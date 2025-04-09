use lumx_axum::axum::extract::Path;
use lumx_axum::axum::http::StatusCode;
use lumx_axum::axum::response::IntoResponse;
use lumx_axum::axum::{routing, Json, Router};
use lumx_axum::router::ProgramRoutable;
use lumx_core::async_trait::async_trait;
use lumx_core::plugable::plugin::Plugin;
use lumx_core::program::ProgramBuilder;
use serde::Serialize;

pub struct HealthyModule;

#[async_trait]
impl Plugin for HealthyModule {
    async fn build(&self, app: &mut ProgramBuilder) {
        let router: Router = Router::new()
            .route("/health", routing::get(|| async { "all is ok" }))
            .route("/health/{resource_id}", routing::get(find_resource));

        app.add_router(router);
    }
}

#[derive(Serialize, Debug)]
struct ResourceInfo {
    id: i32,
}

pub async fn find_resource(Path(resource_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(ResourceInfo { id: resource_id }))
}
