use crate::program::ProgramBuilder;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub trait InitTracing {
    fn init_tracing(&mut self) -> &mut Self;
}

impl InitTracing for ProgramBuilder {
    fn init_tracing(&mut self) -> &mut Self {
        tracing_subscriber::registry()
            .with(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
            )
            .with(fmt::layer())
            .init();

        self
    }
}
