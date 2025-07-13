use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
pub enum ProgramBuilderError {
    #[error("failed to spawn scheduler with message {0}")]
    Spawn(#[from] JoinError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetComponentFailure {
    #[error("{0} component not exists in registry")]
    ComponentNotExist(&'static str),
}
