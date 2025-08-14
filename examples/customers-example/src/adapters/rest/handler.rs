use crate::adapters::rest::dto::CreateCustomerParams;
use crate::domain::port::api::CustomerRegister;
use crate::domain::port::dto::CreateCustomerCommand;
use crate::domain::service::CustomerService;
use lumx_axum::axum::Json;
use lumx_axum::extractor::Component;
use lumx_axum::responses::Created;
use lumx_axum::result::api::ApiError;

pub async fn create_customer(
    Component(uc): Component<CustomerService>,
    Json(payload): Json<CreateCustomerParams>,
) -> Result<Created<i64>, ApiError> {
    let command = CreateCustomerCommand {
        name: payload.name,
        business_name: payload.business_name,
        fiscal_document_id: payload.fiscal_document_id,
        fiscal_number: payload.fiscal_number,
        is_natural_person: payload.is_natural_person,
    };
    let saved_id = uc.register_customer(&command).await?;
    Ok(Created::new(saved_id))
}
