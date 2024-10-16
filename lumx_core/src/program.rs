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

    pub fn get_component<T>(&self) -> Option<Arc<T>>
    where
        T: Clone + Send + Sync + 'static,
    {
        let component_name = std::any::type_name::<T>();
        let pair = self.components.get(component_name)?;
        let component_ref = pair.value().clone();

        component_ref.downcast::<T>()
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
        log::debug!("added component: {}", component_name);

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
        log::debug!("added plugin {}", plugin.name());

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

        let mut to_register = registry
            .iter()
            .map(|e| e.value().to_owned())
            .collect::<Vec<_>>();

        let mut registered: HashSet<String> = HashSet::new();

        while !to_register.is_empty() {
            let mut progress = false;
            let mut next_round = vec![];

            for plugin in to_register {
                let deps = plugin.dependencies();
                if deps.iter().all(|dep| registered.contains(*dep)) {
                    plugin.build(self).await;
                    registered.insert(plugin.name().to_string());
                    log::info!("{} plugin registered", plugin.name());
                    progress = true;
                } else {
                    next_round.push(plugin);
                }
            }

            if !progress {
                panic!("Cyclic dependency detected or missing dependencies for some plugins");
            }

            to_register = next_round;
        }

        self.plugin_registry = registry;
    }

    /// Running
    pub async fn run(&mut self) {
        match self.inner_run().await {
            Ok(_program) => {}
            Err(err) => {
                log::error!("{:?}", err);
            }
        }
    }

    async fn inner_run(&mut self) -> Result<Arc<Program>, ProgramFailure> {
        // 1. read env variables
        dotenvy::dotenv().ok();

        // 2. build plugins
        self.build_plugins().await;

        // 3. schedule
        self.schedule().await
    }

    pub async fn configure(&mut self) -> Arc<Program> {
        // 1. read env variables
        dotenvy::dotenv().ok();

        // 2. build plugins
        self.build_plugins().await;

        // 3. build program
        self.build_program()
    }

    fn build_program(&mut self) -> Arc<Program> {
        let components = std::mem::take(&mut self.components);
        Arc::new(Program { components })
    }

    async fn schedule(&mut self) -> Result<Arc<Program>, ProgramFailure> {
        let program = self.build_program();

        while let Some(task) = self.schedulers.pop() {
            let poll_future = task(program.clone());
            let poll_future = Box::into_pin(poll_future);

            let spawn_res = tokio::spawn(poll_future)
                .await
                .map_err(|err| ProgramFailure::Scheduler(err.to_string()))?;

            match spawn_res {
                Ok(msg) => log::info!("scheduled result: {}", msg),
                Err(err) => log::info!("{}", err),
            }
        }

        Ok(program)
    }
}
