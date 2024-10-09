use std::{collections::HashSet, future::Future, sync::Arc};

use dashmap::DashMap;

use crate::{
    plugable::{
        component::ComponentRef,
        plugin::{Plugin, PluginRef},
    },
    scheduler::Scheduler,
    types::ProgramFailure,
};

pub type Registry<T> = DashMap<String, T>;

pub struct Program {
    /// Components
    components: Registry<ComponentRef>,
}

impl Program {
    pub fn new() -> ProgramBuilder {
        ProgramBuilder::default()
    }
}

pub struct ProgramBuilder {
    /// Components
    components: Registry<ComponentRef>,

    /// Plugin
    pub(crate) plugin_registry: Registry<PluginRef>,

    /// Schedulers
    schedulers: Vec<Box<Scheduler>>,
}

unsafe impl Send for ProgramBuilder {}
unsafe impl Sync for ProgramBuilder {}

impl Default for ProgramBuilder {
    fn default() -> Self {
        Self {
            components: Default::default(),
            plugin_registry: Default::default(),
            schedulers: Default::default(),
        }
    }
}

impl ProgramBuilder {
    /// Add component to the registry
    pub fn add_component<T>(&mut self, component: T) -> &mut Self
    where
        T: std::any::Any + Send + Sync,
    {
        let component_name = std::any::type_name::<T>();
        tracing::debug!("added component: {}", component_name);

        if self.components.contains_key(component_name) {
            panic!("Error adding component {component_name}: component was already added in application");
        }

        let component_name = component_name.to_string();
        self.components
            .insert(component_name, ComponentRef::new(component));
        self
    }

    /// Get the component of the specified type
    pub fn get_component<T>(&self) -> Option<Arc<T>>
    where
        T: std::any::Any + Send + Sync,
    {
        let component_name = std::any::type_name::<T>();
        let pair = self.components.get(component_name)?;
        let component_ref = pair.value().clone();

        component_ref.downcast::<T>()
    }

    /// Add plugin
    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        tracing::debug!("added plugin {}", plugin.name());

        let plugin_name = plugin.name().to_string();
        if self.plugin_registry.contains_key(plugin.name()) {
            panic!("Error adding plugin {plugin_name}: plugin was already added in application");
        }

        self.plugin_registry
            .insert(plugin_name, PluginRef::new(plugin));

        self
    }

    /// Add a scheduled task
    pub fn add_schedule<T>(&mut self, scheduler: T) -> &mut Self
    where
        T: FnOnce(Arc<Program>) -> Box<dyn Future<Output = Result<String, ProgramFailure>> + Send>
            + 'static,
    {
        self.schedulers.push(Box::new(scheduler));
        self
    }

    /// Build registered plugins
    async fn build_plugins(&mut self) {
        let registry = std::mem::take(&mut self.plugin_registry);

        let to_register = registry
            .iter()
            .map(|e| e.value().to_owned())
            .collect::<Vec<_>>();

        let mut registered: HashSet<String> = HashSet::new();

        for plugin in to_register {
            plugin.build(self).await;
            registered.insert(plugin.name().to_string());
            tracing::info!("{} plugin registered", plugin.name());
        }

        // self.plugin_registry = registry;
    }

    /// Running
    pub async fn run(&mut self) {
        match self.inner_run().await {
            Ok(_program) => {}
            Err(err) => {
                tracing::error!("{:?}", err);
            }
        }
    }

    async fn inner_run(&mut self) -> Result<Arc<Program>, ProgramFailure> {
        // 1. read env variables
        dotenvy::dotenv().ok();

        // 2. init tracing
        super::tracing::init();

        // 3. build plugins
        self.build_plugins().await;

        // 4. schedule
        self.schedule_server().await
    }

    fn build_program(&mut self) -> Arc<Program> {
        let components = std::mem::take(&mut self.components);
        Arc::new(Program { components })
    }

    async fn schedule_server(&mut self) -> Result<Arc<Program>, ProgramFailure> {
        let program = self.build_program();

        while let Some(task) = self.schedulers.pop() {
            let poll_future = task(program.clone());
            let poll_future = Box::into_pin(poll_future);

            let spawn_res = tokio::spawn(poll_future)
                .await
                .map_err(|err| ProgramFailure::Scheduler(err.to_string()))?;

            match spawn_res {
                Ok(msg) => tracing::info!("scheduled result: {}", msg),
                Err(err) => tracing::info!("{}", err),
            }
        }

        Ok(program)
    }
}
