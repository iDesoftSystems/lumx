use std::env;

use async_trait::async_trait;
use lumx_core::{plugable::plugin::Plugin, program::AppBuilder, types::ProgramFailure};

pub struct SeaOrmPlugin;

impl SeaOrmPlugin {
    fn database_url(&self) -> Result<String, ProgramFailure> {
        env::var("DATABASE_URL")
            .map_err(|_| ProgramFailure::Config(String::from("DATABASE_URL is not set in env")))
    }
}

#[async_trait]
impl Plugin for SeaOrmPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let db_url = self.database_url().expect("DATABASE_URL is not set in env");

        let db_conn = sea_orm::Database::connect(db_url)
            .await
            .expect("sea-orm connection failed");

        app.add_component(db_conn);
    }
}
