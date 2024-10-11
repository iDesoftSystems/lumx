use lumx_axum::{
    axum::{routing, Router},
    plugin::WebPlugin,
    router::ProgramRoutable,
};
use lumx_core::{program::Program, tokio};

#[tokio::main]
async fn main() {
    Program::new()
        .add_router(router())
        .add_plugin(WebPlugin)
        .run()
        .await
}

fn router() -> Router {
    Router::new().route("/", routing::get(|| async { "Hello, world" }))
}
