use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct RouterRef(pub Arc<RwLock<axum::Router>>);

impl RouterRef {
    pub fn new(router: axum::Router) -> Self {
        Self(Arc::new(RwLock::new(router)))
    }
}

/// Axum router configurator
pub trait ProgramRoutable {
    /// add route to app registry
    fn add_router(&mut self, other_router: axum::Router) -> &mut Self;
}

/// Axum endpoints
pub trait EndpointsExposer {
    fn endpoints(&self) -> axum::Router;
}
