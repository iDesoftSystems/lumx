use axum::Router;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RouterRef(pub Arc<Mutex<Router>>);

impl RouterRef {
    pub fn new(router: Router) -> Self {
        Self(Arc::new(Mutex::new(router)))
    }

    pub fn add_router(&self, other_router: Router) {
        let router_cloned = self.0.clone();
        let mut router_data = router_cloned.lock().unwrap();

        let router = router_data.deref().to_owned();
        let merged_router = router.merge(other_router);

        *router_data = merged_router;
    }
}

/// Axum router configurator
pub trait ProgramRoutable {
    /// add route to app registry
    fn add_router(&mut self, other_router: Router) -> &mut Self;
}

pub trait IntoService {
    fn into_service(self) -> axum::Router;
}
