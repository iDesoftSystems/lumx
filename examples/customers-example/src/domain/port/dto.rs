use validator::Validate;

#[derive(Validate)]
pub struct CreateCustomerCommand {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub business_name: String,
    pub fiscal_document_id: i32,

    #[validate(length(min = 1, max = 100))]
    pub fiscal_number: String,
    pub is_natural_person: bool,
}

pub struct AddCustomerChange {
    pub name: String,
    pub business_name: String,
    pub fiscal_document_id: i32,
    pub fiscal_number: String,
    pub is_natural_person: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum CustomerRegisterError {}
