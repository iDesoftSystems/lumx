use lumx_axum::plugin::WebPlugin;
use lumx_core::tokio;
use modules_example::{healthy, home};

#[tokio::main]
async fn main() {
    lumx_core::program::Program::new()
        .load_envs()
        .collect_tracing()
        .add_plugin(healthy::HealthyModule)
        .add_plugin(home::HomeModule)
        .add_plugin(WebPlugin)
        .run()
        .await
}
