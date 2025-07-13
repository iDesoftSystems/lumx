use std::env;

use async_trait::async_trait;
use lumx_core::{plugable::plugin::Plugin, program::ProgramBuilder};

pub struct SeaOrmPlugin;

#[async_trait]
impl Plugin for SeaOrmPlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in env");

        let db_conn = sea_orm::Database::connect(db_url)
            .await
            .expect("sea-orm connection failed");

        app.add_component(db_conn);
    }
}
