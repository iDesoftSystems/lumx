use core::fmt;

#[derive(Debug)]
pub enum ProgramFailure {
    Config(String),
    Database(String),
    Serve(String),
    Unknown(String),
    Scheduler(String),
    ComponentNotExist(String),
}

impl fmt::Display for ProgramFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramFailure::Config(msg) => write!(f, "{msg}"),
            ProgramFailure::Database(msg) => write!(f, "{msg}"),
            ProgramFailure::Serve(msg) => write!(f, "{msg}"),
            ProgramFailure::Unknown(msg) => write!(f, "{msg}"),
            ProgramFailure::Scheduler(msg) => write!(f, "{msg}"),
            ProgramFailure::ComponentNotExist(msg) => write!(f, "{msg}"),
        }
    }
}
