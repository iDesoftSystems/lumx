use std::{future::Future, sync::Arc};

use crate::program::Program;

#[derive(thiserror::Error, Debug)]
pub enum SchedulerError {
    #[error("IO failure")]
    IOError(#[from] std::io::Error),
}

pub type SchedulerResult = Result<String, SchedulerError>;
pub type Scheduler = dyn FnOnce(Arc<Program>) -> Box<dyn Future<Output = SchedulerResult> + Send>;
