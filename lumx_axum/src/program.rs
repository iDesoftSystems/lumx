use axum::Router;
use lumx_core::program::ProgramBuilder;

use crate::router::{ProgramRoutable, RouterRef};

impl ProgramRoutable for ProgramBuilder {
    fn add_router(&mut self, other_router: Router) -> &mut Self {
        let router_ref = self.get_component::<RouterRef>();

        if let Some(rs) = router_ref {
            rs.add_router(other_router)
        } else {
            let router = Router::new().merge(other_router);
            self.add_component(RouterRef::new(router));
        }

        self
    }
}
