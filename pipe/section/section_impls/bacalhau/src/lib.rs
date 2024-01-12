use std::sync::Arc;

use section::message::{Ack, Chunk, Column, DataFrame, DataType, Message, ValueView};
pub mod api;
pub mod destination;
pub mod jobstore;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Clone, PartialEq)]
pub struct BacalhauPayload {
    pub filepath: String,
}

impl BacalhauPayload {
    fn new(filepath: String) -> Self {
        Self { filepath }
    }
}

impl DataFrame for BacalhauPayload {
    fn columns(&self) -> Vec<section::message::Column<'_>> {
        vec![Column::new(
            "filepath",
            DataType::Str,
            Box::new(std::iter::once(ValueView::from(&self.filepath))),
        )]
    }
}

pub struct BacalhauMessage {
    origin: Arc<str>,
    payload: Option<Box<dyn DataFrame>>,
    ack: Option<Ack>,
}

impl BacalhauMessage {
    fn new(origin: Arc<str>, payload: impl DataFrame, ack: Option<Ack>) -> Self {
        Self {
            origin,
            payload: Some(Box::new(payload)),
            ack,
        }
    }
}

impl std::fmt::Debug for BacalhauMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BacalhauMessage")
            .field("origin", &self.origin)
            .field("payload", &self.payload)
            .finish()
    }
}

impl Message for BacalhauMessage {
    fn origin(&self) -> &str {
        self.origin.as_ref()
    }

    fn next(&mut self) -> section::message::Next<'_> {
        let v = self.payload.take().map(Chunk::DataFrame);
        Box::pin(async move { Ok(v) })
    }

    fn ack(&mut self) -> Ack {
        self.ack.take().unwrap_or(Box::pin(async {}))
    }
}
