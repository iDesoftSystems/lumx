use lumx_core::{plugable::component::ComponentRef, program::Program};
use sea_orm::DatabaseConnection;

pub trait ConnectionExposer {
    fn conn(&self) -> ComponentRef<DatabaseConnection>;
}

impl ConnectionExposer for Program {
    fn conn(&self) -> ComponentRef<DatabaseConnection> {
        let conn = self
            .get_component_ref::<DatabaseConnection>()
            .expect("sea-orm must be required");
        conn
    }
}
