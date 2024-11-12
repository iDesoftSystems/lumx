use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FailureReply {
    pub message: String,
    pub errors: Vec<FieldFailure>,
}

impl From<String> for FailureReply {
    fn from(value: String) -> Self {
        Self {
            message: value,
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FieldFailure {
    pub field: String,
    pub error: String,
}

impl FieldFailure {
    pub fn new(field: String, err: String) -> Self {
        Self { field, error: err }
    }
}
