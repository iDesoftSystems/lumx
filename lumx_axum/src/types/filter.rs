use lumx_domain::api::dtos::filter::FilterQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub q: String,
}

impl Into<FilterQuery> for FilterParams {
    fn into(self) -> FilterQuery {
        FilterQuery {
            query: self.q.to_owned(),
        }
    }
}
