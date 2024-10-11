use std::{net::SocketAddr, sync::Arc};

use axum::{async_trait, Extension, Router};
use lumx_core::{
    plugable::plugin::Plugin,
    program::{Program, ProgramBuilder},
    types::ProgramFailure,
};

use crate::{router::RouterRef, state::AppState};

pub struct WebPlugin;

#[async_trait]
impl Plugin for WebPlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        let router = app.get_component::<RouterRef>();

        let router = match router {
            Some(rs) => {
                let r_val = rs.0.read().unwrap();
                r_val.to_owned()
            }
            None => Router::new(),
        };

        app.add_schedule(move |app_c: Arc<Program>| Box::new(Self::schedule(app_c, router)));
    }
}

impl WebPlugin {
    async fn schedule(app: Arc<Program>, router: axum::Router) -> Result<String, ProgramFailure> {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect(format!("bind tcp listener failed: {}", addr).as_str());

        let router = router.layer(Extension(AppState { app }));

        println!("Listening on {}", listener.local_addr().unwrap());
        println!("Ctrl-C to shutdown server");

        let server = axum::serve(listener, router.into_make_service());
        server
            .await
            .map_err(|err| ProgramFailure::Scheduler(err.to_string()))?;

        Ok("axum schedule finished".to_string())
    }
}
