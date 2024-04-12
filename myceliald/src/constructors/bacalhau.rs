use std::path::Path;

use pipe::{config::Map, types::DynSection};
use section::{command_channel::SectionChannel, SectionError};

pub fn destination_ctor<S: SectionChannel>(
    config: &Map,
) -> Result<Box<dyn DynSection<S>>, SectionError> {
    let job = config
        .get("job")
        .ok_or("bacalhau section requires 'job'")?
        .as_str()
        .ok_or("'job' should be a string")?;
    let jobstore = config
        .get("jobstore")
        .ok_or("bacalhau section requires 'jobstore'")?
        .as_str()
        .ok_or("'jobstore' should be a string")?;

    // Verify jobstore folder exists
    if !Path::new(jobstore).exists() {
        return Err(format!("'jobstore' path '{}' does not exist", jobstore).into());
    }

    Ok(Box::new(bacalhau::destination::Bacalhau::new(
        job, jobstore,
    )))
}
