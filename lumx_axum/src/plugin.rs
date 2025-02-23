use crate::{router::RouterRef, state::AppState};
use axum::{async_trait, Extension, Router};
use lumx_core::{
    plugable::plugin::Plugin,
    program::{Program, ProgramBuilder},
    types::ProgramFailure,
};
use std::ops::Deref;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing::debug;

pub struct WebPlugin;

#[async_trait]
impl Plugin for WebPlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        let router_ref = app.get_expect_component::<RouterRef>();

        let mutex_router_ref = router_ref.0.to_owned();
        let mutex_guard = mutex_router_ref.lock().unwrap();
        let router = mutex_guard.deref().to_owned();

        app.add_schedule(move |app_c: Arc<Program>| Box::new(Self::schedule(app_c, router)));
    }
}

impl WebPlugin {
    async fn schedule(app: Arc<Program>, router: Router) -> Result<String, ProgramFailure> {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect(format!("bind tcp listener failed: {}", addr).as_str());

        debug!(?router, "registered routes");
        let router = router
            .layer(Extension(AppState { app }))
            .layer(TraceLayer::new_for_http());

        println!("Listening on {}", listener.local_addr().unwrap());
        println!("Ctrl-C to shutdown server");

        let server = axum::serve(listener, router.into_make_service());
        server
            .await
            .map_err(|err| ProgramFailure::Scheduler(err.to_string()))?;

        Ok("axum schedule finished".to_string())
    }
}
