use axum::Router;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct RouterRef(pub Arc<RwLock<Router>>);

impl RouterRef {
    pub fn new(router: Router) -> Self {
        Self(Arc::new(RwLock::new(router)))
    }

    pub fn add_router(&self, other_router: Router) {
        println!("adding router: {:?}", other_router);

        let router_registered = self.0.read().unwrap().to_owned();

        let mut writer = self.0.write().unwrap();

        *writer = router_registered.merge(other_router);
    }
}

/// Axum router configurator
pub trait ProgramRoutable {
    /// add route to app registry
    fn add_router(&mut self, other_router: axum::Router) -> &mut Self;
}
