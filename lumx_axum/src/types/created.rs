use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Created<I> {
    pub id: I,
}

impl<I> Created<I> {
    pub fn new(id: I) -> Self {
        Self { id }
    }
}
