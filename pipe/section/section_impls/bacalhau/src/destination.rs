use arrow_msg::df_to_recordbatch;
use section::{
    command_channel::{Command, SectionChannel},
    futures::{self, FutureExt, Sink, Stream, StreamExt},
    message::Chunk,
    pretty_print::pretty_print,
    section::Section,
    SectionError, SectionMessage,
};

use std::collections::HashMap;
use std::future::Future;
use std::pin::{pin, Pin};

use crate::api::submit;
use crate::jobstore::JobStore;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub struct Bacalhau {
    pub job: String,
    pub jobstore: JobStore,
}

impl Bacalhau {
    pub fn new(job: impl Into<String>, jobstore: impl Into<String>) -> Self {
        Self {
            job: job.into(),
            jobstore: JobStore::new(jobstore).expect("should be able to create jobstore"),
        }
    }

    pub async fn submit_job(&self, data: &HashMap<String, String>) -> Result<String, StdError> {
        // We'll either get an Err from the call to render, or we'll get either
        // an Ok or Err from the call to submit.
        match self.jobstore.render(self.job.clone(), data) {
            Ok(output) => submit(&output).await,
            Err(m) => Err(m),
        }
    }

    async fn enter_loop<Input, Output, SectionChan>(
        self,
        input: Input,
        _output: Output,
        mut section_channel: SectionChan,
    ) -> Result<(), SectionError>
    where
        Input: Stream<Item = SectionMessage> + Send + 'static,
        Output: Sink<SectionMessage, Error = SectionError> + Send + 'static,
        SectionChan: SectionChannel + Send + 'static,
    {
        let mut input = pin!(input.fuse());
        //let output = pin!(output);
        loop {
            futures::select_biased! {
                cmd = section_channel.recv().fuse() => {
                    if let Command::Stop = cmd? {
                        return Ok(())
                    }
                }
                stream = input.next() => {
                    let mut msg = match stream {
                        Some(msg) => msg,
                        None => Err("input stream closed")?
                    };


                    loop {
                        futures::select! {
                            chunk = msg.next().fuse() => {
                                let frame = match chunk? {
                                    None => break,
                                    Some(Chunk::DataFrame(df)) => df,
                                    Some(_ch) => continue,
                                };

                                section_channel.log(format!("got dataframe chunk from {}:\n{}", msg.origin(), pretty_print(&*frame))).await?;


                                let batch = df_to_recordbatch(frame.as_ref()).unwrap();
                                let json_rows = arrow_json::writer::record_batches_to_json_rows(&[&batch]).unwrap();
                                let first = json_rows.first().unwrap();
                                let args: HashMap<String, String> = first.iter().map(|(k, v)| {
                                    (k.clone(), v.as_str().unwrap().to_string())
                                }).collect();

                                section_channel.log(format!("Arguments: {:?}", &args)).await?;
                                self.submit_job(&args).await?;

                                msg.ack().await;
                            }
                        }

                    }

                    // // let payload: crate::BacalhauPayload = msg.into();
                    // // let origin = &msg.origin();
                    // // self.submit_job(&payload.message).await?;

                    // // section_channel.log(&format!("Message from '{:?}' received! {:?}", origin, payload)).await?;
                    // output.send(msg).await?;
                    //         }
                    //         cmd = section_channel.recv().fuse() => {
                    //             if let Command::Stop = cmd? {
                    //                 return Ok(())
                    //             }
                    //         }

                },
            }
        }
    }
}

impl<Input, Output, SectionChan> Section<Input, Output, SectionChan> for Bacalhau
where
    Input: Stream<Item = SectionMessage> + Send + 'static,
    Output: Sink<SectionMessage, Error = SectionError> + Send + 'static,
    SectionChan: SectionChannel + Send + 'static,
{
    type Error = SectionError;
    type Future = Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'static>>;

    fn start(self, input: Input, output: Output, command: SectionChan) -> Self::Future {
        Box::pin(async move { self.enter_loop(input, output, command).await })
    }
}
