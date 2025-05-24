use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page_size: u64,
    pub page: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
