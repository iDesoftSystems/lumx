#[derive(Debug, PartialEq)]
pub struct InvalidField {
    pub field: String,
    pub error: String,
}

impl InvalidField {
    pub fn new(field: String, err: String) -> Self {
        Self { field, error: err }
    }
}
