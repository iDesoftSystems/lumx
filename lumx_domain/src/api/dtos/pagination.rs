pub struct PaginationQuery {
    pub page_size: u64,
    pub page: u64,
}

pub struct Page<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
