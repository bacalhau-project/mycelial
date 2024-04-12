pub mod api;
pub mod destination;
pub mod jobstore;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
