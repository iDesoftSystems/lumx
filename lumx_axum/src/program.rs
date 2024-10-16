use axum::Router;
use lumx_core::program::{Program, ProgramBuilder};

use crate::router::{EndpointsExposer, ProgramRoutable, RouterRef};

impl ProgramRoutable for ProgramBuilder {
    fn add_router(&mut self, other_router: axum::Router) -> &mut Self {
        let router_ref = self.get_component::<RouterRef>();

        if let Some(rs) = router_ref {
            let router_registered = rs.0.read().unwrap().to_owned();

            let mut rs_writer = rs.0.write().unwrap();
            *rs_writer = Router::new().merge(router_registered).merge(other_router);
        } else {
            let router = Router::new().merge(other_router);
            self.add_component(RouterRef::new(router));
        }

        self
    }
}

impl EndpointsExposer for Program {
    fn endpoints(&self) -> axum::Router {
        let router = self.get_component::<RouterRef>();
        let router_registered = match router {
            Some(rs) => {
                let r_val = rs.0.read().unwrap();
                r_val.to_owned()
            }
            None => Router::new(),
        };

        router_registered
    }
}
