use lambda_example::handlers;
use lambda_http::{Error, run};
use lumx_axum::{
    axum::{Router, routing},
    plugin::WebPlugin,
    router::{IntoService, ProgramRoutable},
};
use lumx_core::{program::Program, tokio};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let program = Program::new()
        .load_envs()
        .collect_tracing()
        .add_plugin(WebPlugin)
        .add_router(router())
        .build()
        .await;

    run(program.into_service()).await
}

fn router() -> Router {
    Router::new().route("/api/articles", routing::get(handlers::read_articles))
}
