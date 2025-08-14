use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCustomerParams {
    pub name: String,
    pub business_name: String,
    pub fiscal_document_id: i32,
    pub fiscal_number: String,
    pub is_natural_person: bool,
}
