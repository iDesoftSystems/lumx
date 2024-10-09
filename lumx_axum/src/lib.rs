use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use axum::{Extension, Router};
use lumx_core::{
    app::{App, AppBuilder},
    plugable::plugin::Plugin,
    types::ProgramFailure,
};

#[derive(Clone)]
pub struct AppState {
    /// App Registry Ref
    pub app: Arc<App>,
}

pub struct WebPlugin;

#[async_trait]
impl Plugin for WebPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        app.add_schedule(move |app_c: Arc<App>| Box::new(Self::schedule(app_c)));
    }
}

impl WebPlugin {
    async fn schedule(app: Arc<App>) -> Result<String, ProgramFailure> {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect(format!("bind tcp listener failed: {}", addr).as_str());

        let router = Router::new().layer(Extension(AppState { app }));

        let server = axum::serve(listener, router.into_make_service());

        tracing::debug!("Ctrl-C to shutdown server");

        server
            .await
            .map_err(|err| ProgramFailure::Scheduler(err.to_string()))?;

        Ok("axum schedule finished".to_string())
    }
}
