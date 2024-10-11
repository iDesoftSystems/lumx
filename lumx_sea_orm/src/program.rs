use lumx_core::program::Program;
use sea_orm::DatabaseConnection;

pub trait ConnectionExposer {
    fn conn(&self) -> std::sync::Arc<DatabaseConnection>;
}

impl ConnectionExposer for Program {
    fn conn(&self) -> std::sync::Arc<DatabaseConnection> {
        let conn = self
            .get_component::<DatabaseConnection>()
            .expect("sea-orm must be required");
        conn
    }
}
