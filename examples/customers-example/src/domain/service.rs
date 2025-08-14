use crate::domain::port::api::CustomerRegister;
use crate::domain::port::dto::{AddCustomerChange, CreateCustomerCommand};
use crate::domain::port::spi::CustomerSaver;
use lumx_axum::result::api::ApiError;
use lumx_core::async_trait;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone)]
pub struct CustomerService {
    repo: Arc<dyn CustomerSaver>,
}

impl CustomerService {
    pub fn new(repo: Arc<dyn CustomerSaver>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl CustomerRegister for CustomerService {
    async fn register_customer(&self, command: &CreateCustomerCommand) -> Result<i64, ApiError> {
        command.validate()?;

        let change = AddCustomerChange {
            name: command.name.trim().to_string(),
            business_name: command.business_name.trim().to_string(),
            fiscal_document_id: command.fiscal_document_id,
            fiscal_number: command.fiscal_number.trim().to_string(),
            is_natural_person: command.is_natural_person,
        };
        let saved_id = self.repo.save_customer(&change).await?;

        Ok(saved_id)
    }
}
