use async_trait::async_trait;
use std::{any::Any, ops::Deref, sync::Arc};

use crate::app::AppBuilder;

#[derive(Clone)]
pub struct PluginRef(Arc<dyn Plugin>);

#[async_trait]
pub trait Plugin: Any + Send + Sync {
    /// Configure the [`App`] to which this plugin is added.
    async fn build(&self, app: &mut AppBuilder);

    /// Configures a name for the [`Plugin`] which is primarily used for checking plugin
    /// uniqueness and debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

impl PluginRef {
    pub(crate) fn new<T: Plugin>(plugin: T) -> Self {
        Self(Arc::new(plugin))
    }
}

impl Deref for PluginRef {
    type Target = dyn Plugin;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
