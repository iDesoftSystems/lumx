use std::env;

use lumx_axum::{
    axum::{async_trait, routing, Router},
    plugin::WebPlugin,
    router::ProgramRoutable,
};
use lumx_core::tracer::InitTracing;
use lumx_core::{plugable::plugin::Plugin, program::ProgramBuilder, tokio};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");

    lumx_core::program::Program::new()
        .add_plugin(HealthyModule)
        .add_plugin(HomeModule)
        .add_plugin(WebPlugin)
        .init_tracing()
        .run()
        .await
}

struct HealthyModule;

#[async_trait]
impl Plugin for HealthyModule {
    async fn build(&self, app: &mut ProgramBuilder) {
        let router: lumx_axum::axum::Router =
            Router::new().route("/health", routing::get(|| async { "all is ok" }));

        app.add_router(router);
    }
}

struct HomeModule;

#[async_trait]
impl Plugin for HomeModule {
    async fn build(&self, app: &mut ProgramBuilder) {
        let router: lumx_axum::axum::Router =
            Router::new().route("/", routing::get(|| async { "Hello, world" }));

        app.add_router(router);
    }
}
