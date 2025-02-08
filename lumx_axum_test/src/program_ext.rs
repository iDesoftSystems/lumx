use std::sync::Arc;

use axum::Extension;
use lumx_axum::{router::RouterRef, state::AppState};
use lumx_core::program::Program;

/// Axum testable endpoints
pub trait IntoTestableEndpoints {
    fn into_testable_endpoints(&self) -> axum::Router;
}

impl IntoTestableEndpoints for Arc<Program> {
    fn into_testable_endpoints(&self) -> axum::Router {
        let router_ref = self.get_expect_component::<RouterRef>();
        let router_rw = router_ref.0.read().unwrap();

        router_rw
            .to_owned()
            .layer(Extension(AppState { app: self.clone() }))
    }
}
