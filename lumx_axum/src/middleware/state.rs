use axum::extract::Request;
use lumx_core::program::Program;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct StateLayer {
    program: Arc<Program>,
}

impl StateLayer {
    pub fn new(program: Arc<Program>) -> Self {
        Self { program }
    }
}

impl<S> Layer<S> for StateLayer {
    type Service = StateLayerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        StateLayerService {
            inner,
            program: self.program.clone(),
        }
    }
}

#[derive(Clone)]
pub struct StateLayerService<S> {
    inner: S,
    program: Arc<Program>,
}

impl<S, B> Service<Request<B>> for StateLayerService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        req.extensions_mut().insert(self.program.clone());

        self.inner.call(req)
    }
}
