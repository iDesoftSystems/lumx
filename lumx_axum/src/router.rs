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
        println!("adding router: {:?}", other_router);

        let router_cloned = self.0.clone();
        let mut router_data = router_cloned.lock().unwrap();

        //let wrapper = router_data.merge(other_router);
        let router = router_data.deref().to_owned();
        let merged_router = router.merge(other_router);

        println!("post adding router: {:?}", &merged_router);

        *router_data = merged_router;

        // let mut data_mutex_clone = Arc::clone(&self.0);

        //*data_mutex_clone.lock().unwrap() = *data

        // data.merge(other_router);

        //let router_registered = self.0.read().unwrap().to_owned();

        //let mut writer = self.0.write().unwrap();

        //*writer = router_registered.merge(other_router);
        //self.0.merge(other_router)
    }
}

/// Axum router configurator
pub trait ProgramRoutable {
    /// add route to app registry
    fn add_router(&mut self, other_router: axum::Router) -> &mut Self;
}
