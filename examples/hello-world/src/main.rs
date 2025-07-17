use lumx_axum::{
    axum::{Router, routing},
    plugin::WebPlugin,
    router::ProgramRoutable,
};
use lumx_core::{program::Program, tokio};

#[tokio::main]
async fn main() {
    Program::builder()
        .load_envs()
        .collect_tracing()
        .add_plugin(WebPlugin)
        .add_router(router())
        .run()
        .await
}

fn router() -> Router {
    Router::new().route("/", routing::get(|| async { "Hello, world" }))
}
