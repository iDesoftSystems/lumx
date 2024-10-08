use std::{future::Future, sync::Arc};

use crate::{app::App, types::ProgramFailure};

pub type SchedulerResult = Result<String, ProgramFailure>;
pub type Scheduler = dyn FnOnce(Arc<App>) -> Box<dyn Future<Output = SchedulerResult> + Send>;
