#[tokio::main]
async fn main() {
    lumx_core::program::Program::new()
        .add_plugin(lumx_axum::WebPlugin)
        .run()
        .await
}
