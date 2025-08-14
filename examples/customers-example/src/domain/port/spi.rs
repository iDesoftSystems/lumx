use crate::domain::port::dto::AddCustomerChange;
use lumx_axum::result::api::ApiError;
use lumx_core::async_trait;

#[async_trait::async_trait]
pub trait CustomerSaver: Send + Sync + 'static {
    async fn save_customer(&self, change: &AddCustomerChange) -> Result<i64, ApiError>;
}
