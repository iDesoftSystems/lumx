use std::ops::Deref;
use std::sync::Arc;

use axum::Extension;
use lumx_axum::{router::RouterRef, state::AppState};
use lumx_core::program::Program;

/// Axum testable endpoints
pub trait IntoTestableEndpoints {
    fn into_testable_endpoints(self) -> axum::Router;
}

impl IntoTestableEndpoints for Arc<Program> {
    fn into_testable_endpoints(self) -> axum::Router {
        let router_ref = self.get_expect_component::<RouterRef>();
        let mutex = router_ref.0.lock().unwrap();
        let router = mutex.deref().to_owned();
        router.layer(Extension(AppState { app: self.clone() }))
    }
}
