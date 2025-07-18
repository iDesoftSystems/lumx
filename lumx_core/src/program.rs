use crate::banner;
use crate::scheduler::SchedulerResult;
use crate::types::GetComponentFailure;
use crate::{
    plugable::{
        component::ComponentRef,
        plugin::{Plugin, PluginRef},
    },
    scheduler::Scheduler,
    types::ProgramBuilderError,
};
use dashmap::DashMap;
use std::any::TypeId;
use std::{collections::HashSet, future::Future, sync::Arc};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub type Registry<T> = DashMap<TypeId, T>;

pub struct Program {
    /// Components
    components: Registry<ComponentRef>,
}

impl Program {
    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }

    /// Get the component of the specified type
    pub fn get_component<T>(&self) -> Option<Arc<T>>
    where
        T: Clone + Send + Sync + 'static,
    {
        let component_id = TypeId::of::<T>();
        let pair = self.components.get(&component_id)?;
        let component_ref = pair.value().clone();

        component_ref.downcast::<T>()
    }

    /// Get the component reference of the specified type.
    /// If the component does not exist, it will panic.
    pub fn get_expect_component<T>(&self) -> Arc<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        self.get_component().unwrap_or_else(|| {
            panic!(
                "{} component not exists in registry",
                std::any::type_name::<T>()
            )
        })
    }

    /// Ge the component reference of the specified type.
    /// If the component does not exist, it will return GetComponentFailure::ComponentNotExist.
    pub fn try_get_component<T>(&self) -> Result<Arc<T>, GetComponentFailure>
    where
        T: Clone + Send + Sync + 'static,
    {
        self.get_component()
            .ok_or_else(|| GetComponentFailure::ComponentNotExist(std::any::type_name::<T>()))
    }
}

#[derive(Default)]
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

impl ProgramBuilder {
    /// Add component to the registry
    pub fn add_component<T>(&mut self, component: T) -> &mut Self
    where
        T: std::any::Any + Send + Sync,
    {
        let component_id = TypeId::of::<T>();
        let component_name = std::any::type_name::<T>();
        log::debug!("added component: {component_name}");

        if self.components.contains_key(&component_id) {
            panic!(
                "Error adding component {component_name}: component was already added in application"
            );
        }

        self.components
            .insert(component_id, ComponentRef::new(component));
        self
    }

    /// Get the component of the specified type
    pub fn get_component<T>(&self) -> Option<Arc<T>>
    where
        T: std::any::Any + Send + Sync,
    {
        let component_id = TypeId::of::<T>();
        let pair = self.components.get(&component_id)?;
        let component_ref = pair.value().clone();

        component_ref.downcast::<T>()
    }

    /// Get the component reference of the specified type.
    /// If the component does not exist, it will panic.
    pub fn get_expect_component<T>(&self) -> Arc<T>
    where
        T: std::any::Any + Send + Sync,
    {
        self.get_component().unwrap_or_else(|| {
            panic!(
                "{} component not exists in registry",
                std::any::type_name::<T>()
            )
        })
    }

    /// Ge the component reference of the specified type.
    /// If the component does not exist, it will return GetComponentFailure::ComponentNotExist.
    pub fn try_get_component<T>(&self) -> Result<Arc<T>, GetComponentFailure>
    where
        T: std::any::Any + Send + Sync,
    {
        self.get_component()
            .ok_or_else(|| GetComponentFailure::ComponentNotExist(std::any::type_name::<T>()))
    }

    /// Add plugin
    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        log::debug!("adding plugin {}", &plugin.name());

        let plugin_id = TypeId::of::<T>();
        if self.plugin_registry.contains_key(&plugin_id) {
            let plugin_name = plugin.name();
            panic!("Error adding plugin {plugin_name}: plugin was already added in application");
        }

        self.plugin_registry
            .insert(plugin_id, PluginRef::new(plugin));

        self
    }

    /// Add a scheduled task
    pub fn add_schedule<T>(&mut self, scheduler: T) -> &mut Self
    where
        T: FnOnce(Arc<Program>) -> Box<dyn Future<Output = SchedulerResult> + Send> + 'static,
    {
        self.schedulers.push(Box::new(scheduler));
        self
    }

    /// Init and read env variables
    pub fn load_envs(&mut self) -> &mut Self {
        dotenvy::dotenv().ok();

        self
    }

    /// Collect and subscribe tracing
    pub fn collect_tracing(&mut self) -> &mut Self {
        tracing_subscriber::registry()
            .with(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
            )
            .with(fmt::layer())
            .init();

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

    /// The `run` method is suitable for applications that contain scheduling logic.
    pub async fn run(&mut self) {
        match self.inner_run().await {
            Ok(_) => {}
            Err(err) => {
                log::error!("failed to run program with err {err}");
            }
        }
    }

    async fn inner_run(&mut self) -> Result<(), ProgramBuilderError> {
        banner::print_banner();

        // 1. build plugins
        self.build_plugins().await;

        // 2. schedule
        self.schedule().await
    }

    /// Unlike the [`run`] method, the [`build`] method is suitable for applications that do not contain scheduling logic.
    /// This method returns the built Program.
    pub async fn build(&mut self) -> Arc<Program> {
        // 1. build plugins
        self.build_plugins().await;

        // 2. build program
        self.build_program()
    }

    fn build_program(&mut self) -> Arc<Program> {
        let components = std::mem::take(&mut self.components);
        Arc::new(Program { components })
    }

    async fn schedule(&mut self) -> Result<(), ProgramBuilderError> {
        let program = self.build_program();

        while let Some(task) = self.schedulers.pop() {
            let poll_future = task(Arc::clone(&program));
            let poll_future = Box::into_pin(poll_future);

            let spawn_res = tokio::spawn(poll_future).await?;

            match spawn_res {
                Ok(msg) => log::info!("scheduled with result: {msg}"),
                Err(err) => log::error!("failed on schedule with err {err}"),
            }
        }

        Ok(())
    }
}
