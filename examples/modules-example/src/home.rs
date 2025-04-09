use lumx_axum::axum::{routing, Router};
use lumx_axum::router::ProgramRoutable;
use lumx_core::async_trait::async_trait;
use lumx_core::plugable::plugin::Plugin;
use lumx_core::program::ProgramBuilder;

pub struct HomeModule;

#[async_trait]
impl Plugin for HomeModule {
    async fn build(&self, app: &mut ProgramBuilder) {
        let router: lumx_axum::axum::Router =
            Router::new().route("/", routing::get(|| async { "Hello, world" }));

        app.add_router(router);
    }
}
