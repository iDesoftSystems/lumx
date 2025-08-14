use crate::domain::port::dto;
use lumx_axum::result::api::ApiError;
use lumx_core::async_trait;

#[async_trait::async_trait]
pub trait CustomerRegister: Send + Sync {
    async fn register_customer(
        &self,
        command: &dto::CreateCustomerCommand,
    ) -> Result<i64, ApiError>;
}
