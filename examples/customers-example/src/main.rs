use lumx_axum::{
    axum::{routing, Router},
    plugin::WebPlugin,
    router::ProgramRoutable,
};
use lumx_core::{program::Program, tokio};
use lumx_sea_orm::plugin::SeaOrmPlugin;

#[tokio::main]
async fn main() {
    Program::builder()
        .load_envs()
        .collect_tracing()
        .add_plugin(WebPlugin)
        .add_plugin(SeaOrmPlugin)
        .add_router(router())
        .run()
        .await
}

fn router() -> Router {
    Router::new().route(
        "/api/customers",
        routing::post(customers_example::adapters::rest::handler::create_customer),
    )
}
