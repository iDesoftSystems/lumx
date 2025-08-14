use crate::adapters::repository::CustomerSeaRepository;
use crate::domain::service::CustomerService;
use lumx_core::program::ProgramBuilder;
use lumx_core::{async_trait, plugable::plugin::Plugin};
use lumx_sea_orm::sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct CustomerPlugin;

#[async_trait::async_trait]
impl Plugin for CustomerPlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        let conn = app.get_expect_component::<DatabaseConnection>();
        let customer_repo = CustomerSeaRepository::new(conn);
        let customer_service = CustomerService::new(Arc::new(customer_repo));

        app.add_component(customer_service);
    }
}
