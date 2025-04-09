use lumx_axum::plugin::WebPlugin;
use lumx_core::tokio;
use modules_example::{healthy, home};
use std::env;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");

    lumx_core::program::Program::new()
        .with_envs()
        .collect_tracing()
        .add_plugin(healthy::HealthyModule)
        .add_plugin(home::HomeModule)
        .add_plugin(WebPlugin)
        .run()
        .await
}
