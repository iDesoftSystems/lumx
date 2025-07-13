use std::{ops::Deref, sync::Arc};

use axum::{Extension, Router};
use lumx_core::program::{Program, ProgramBuilder};

use crate::{
    router::{IntoService, ProgramRoutable, RouterRef},
    state::AppState,
};

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

impl IntoService for Arc<Program> {
    fn into_service(self) -> axum::Router {
        let router_ref = self.get_expect_component::<RouterRef>();

        let mutex = router_ref.0.lock().unwrap();
        let router = mutex.deref().to_owned();

        router.layer(Extension(AppState { app: self.clone() }))
    }
}
