use std::{future::Future, sync::Arc};

use crate::{program::Program, types::ProgramFailure};

pub type SchedulerResult = Result<String, ProgramFailure>;
pub type Scheduler = dyn FnOnce(Arc<Program>) -> Box<dyn Future<Output = SchedulerResult> + Send>;
