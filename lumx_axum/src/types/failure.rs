use lumx_domain::api::dtos::field::InvalidField;
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

impl From<Vec<InvalidField>> for FailureReply {
    fn from(value: Vec<InvalidField>) -> Self {
        Self {
            message: String::from("validation error"),
            errors: value
                .into_iter()
                .map(|failure| FieldFailure::new(failure.field, failure.error))
                .collect(),
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
