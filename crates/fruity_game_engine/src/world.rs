use crate::any::FruityAny;
use crate::export;
use crate::frame_service::FrameService;
use crate::module::Module;
use crate::object_factory_service::ObjectFactoryService;
use crate::settings::Settings;
use crate::FruityResult;
use crate::ModulesService;
use crate::ResourceContainer;
use crate::RwLock;
use crate::{export_constructor, export_impl, export_struct};
use fruity_game_engine_macro::typescript;
use std::fmt::Debug;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

/// A middleware that occurs when entering into the loop
#[typescript("type StartMiddleware = (world: World) => void")]
pub type StartMiddleware = Arc<dyn Send + Sync + Fn(World) -> FruityResult<()>>;

/// A middleware that occurs when rendering the loop
#[typescript("type FrameMiddleware = (world: World) => void")]
pub type FrameMiddleware = Arc<dyn Send + Sync + Fn(World) -> FruityResult<()>>;

/// A middleware that occurs when leaving the loop
#[typescript("type EndMiddleware = (world: World) => void")]
pub type EndMiddleware = Arc<dyn Send + Sync + Fn(World) -> FruityResult<()>>;

/// The next argument of SetupWorldMiddleware
#[typescript("type SetupWorldMiddlewareNext = (world: World, settings: Settings) => void")]
pub type SetupWorldMiddlewareNext = Arc<dyn Send + Sync + Fn(World, Settings) -> FruityResult<()>>;

/// The next argument of RunWorldMiddleware
#[typescript("type RunWorldMiddlewareNext = (world: World, settings: Settings) => void")]
pub type RunWorldMiddlewareNext = Arc<dyn Send + Sync + Fn(World, Settings) -> FruityResult<()>>;

/// A middleware that occurs when the world runs
#[typescript("type SetupWorldMiddleware = (world: World, settings: Settings, next: SetupWorldMiddlewareNext) => void")]
pub type SetupWorldMiddleware =
    Arc<dyn Send + Sync + Fn(World, Settings, SetupWorldMiddlewareNext) -> FruityResult<()>>;

/// A middleware that occurs when the world runs
#[typescript(
    "type RunWorldMiddleware = (world: World, settings: Settings, next: RunWorldMiddlewareNext) => void"
)]
pub type RunWorldMiddleware =
    Arc<dyn Send + Sync + Fn(World, Settings, RunWorldMiddlewareNext) -> FruityResult<()>>;

struct InnerWorld {
    resource_container: ResourceContainer,
    settings: Settings,
    start_middleware: StartMiddleware,
    frame_middleware: FrameMiddleware,
    end_middleware: EndMiddleware,
    setup_world: Arc<dyn Send + Sync + Fn(World, Settings) -> FruityResult<()>>,
    run_world: Arc<dyn Send + Sync + Fn(World, Settings) -> FruityResult<()>>,
}

/// The main container of the ECS
#[derive(FruityAny, Clone)]
#[export_struct]
pub struct World {
    inner: Arc<RwLock<InnerWorld>>,
    module_service: Arc<RwLock<ModulesService>>,
}

#[export_impl]
impl World {
    /// Returns a World
    #[export_constructor]
    pub fn new(settings: Settings) -> World {
        let resource_container = ResourceContainer::new();
        Self::initialize(resource_container.clone(), &settings);
        let module_service = ModulesService::new(resource_container.clone());

        World {
            inner: Arc::new(RwLock::new(InnerWorld {
                resource_container: resource_container.clone(),
                settings,
                start_middleware: Arc::new(move |_| FruityResult::Ok(())),
                frame_middleware: Arc::new(move |_| FruityResult::Ok(())),
                end_middleware: Arc::new(move |_| FruityResult::Ok(())),
                setup_world: Arc::new(|_world, _settings| FruityResult::Ok(())),
                run_world: Arc::new(|world, _settings| {
                    world.start()?;
                    world.frame()?;
                    world.end()?;

                    FruityResult::Ok(())
                }),
            })),
            module_service: Arc::new(RwLock::new(module_service)),
        }
    }

    /// Initialize the world
    pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
        #[cfg(target_arch = "wasm32")]
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let frame_service = FrameService::new(resource_container.clone());
        resource_container.add::<FrameService>("frame_service", Box::new(frame_service));

        let object_factory_service = ObjectFactoryService::new(resource_container.clone());
        resource_container.add::<ObjectFactoryService>(
            "object_factory_service",
            Box::new(object_factory_service),
        );
    }

    /// Register a module
    #[export]
    pub fn register_module(&self, module: Module) -> FruityResult<()> {
        self.module_service
            .deref()
            .write()
            .register_module(module.clone());

        let ordered_modules = {
            let module_service = self.module_service.deref().read();
            module_service.get_modules_ordered_by_dependencies()
        };

        if let Ok(ordered_modules) = ordered_modules {
            // Rebuild setup_world_middleware taking care of dependency arborescence if needed
            if let Some(_) = module.setup_world_middleware.clone() {
                {
                    let mut this = self.inner.deref().write();
                    this.setup_world = Arc::new(|_world, _settings| FruityResult::Ok(()));
                }

                for module in ordered_modules.clone().into_iter() {
                    if let Some(setup_world_middleware) = module.setup_world_middleware.clone() {
                        let mut this = self.inner.deref().write();
                        let previous_setup_world = this.setup_world.clone();

                        this.setup_world = Arc::new(move |world, settings| {
                            setup_world_middleware(world, settings, previous_setup_world.clone())
                        });
                    }
                }
            }

            // Rebuild run_world_middleware arborescence if needed
            if let Some(_) = module.run_world_middleware.clone() {
                {
                    let mut this = self.inner.deref().write();
                    this.run_world = Arc::new(|_world, _settings| FruityResult::Ok(()));
                }

                for module in ordered_modules.into_iter() {
                    if let Some(run_world_middleware) = module.run_world_middleware.clone() {
                        let mut this = self.inner.deref().write();
                        let previous_run_world = this.run_world.clone();

                        this.run_world = Arc::new(move |world, settings| {
                            run_world_middleware(world, settings, previous_run_world.clone())
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the modules
    #[export]
    pub fn setup_modules_async(&self) -> Pin<Box<dyn Send + Future<Output = FruityResult<()>>>> {
        let world = self.clone();
        Box::pin(async move {
            let settings = world.inner.deref().read().settings.clone();
            let ordered_modules = {
                let module_service = world.module_service.deref().read();
                module_service.get_modules_ordered_by_dependencies()?
            };

            for module in ordered_modules.into_iter() {
                if let Some(setup) = module.setup {
                    setup(world.clone(), settings.clone())?;
                }

                if let Some(setup_async) = module.setup_async {
                    let world = world.clone();
                    let settings = settings.clone();

                    setup_async(world.clone(), settings.clone()).await?;
                }
            }

            Ok(())
        })
    }

    /// Load the resources
    #[export]
    pub fn load_resources_async(&self) -> Pin<Box<dyn Send + Future<Output = FruityResult<()>>>> {
        let world = self.clone();
        Box::pin(async move {
            let settings = world.inner.deref().read().settings.clone();
            let ordered_modules = {
                let module_service = world.module_service.deref().read();
                module_service.get_modules_ordered_by_dependencies()?
            };

            for module in ordered_modules.into_iter() {
                if let Some(load_resources) = module.load_resources {
                    load_resources(world.clone(), settings.clone())?;
                }

                if let Some(load_resources_async) = module.load_resources_async {
                    let world = world.clone();
                    let settings = settings.clone();

                    load_resources_async(world.clone(), settings.clone()).await?;
                }
            }

            Ok(())
        })
    }

    /// Run the world
    #[export]
    pub fn setup(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let settings = self.inner.deref().read().settings.clone();
        let setup = self.inner.deref().read().setup_world.clone();

        setup(self.clone(), settings)
    }

    /// Run the world
    #[export]
    pub fn run(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let settings = self.inner.deref().read().settings.clone();
        let run = self.inner.deref().read().run_world.clone();

        run(self.clone(), settings)
    }

    /// Run the start middleware
    pub fn start(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let start_middleware = self.inner.deref().read().start_middleware.clone();
        start_middleware(self.clone())
    }

    /// Run the frame middleware
    pub fn frame(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let frame_middleware = self.inner.deref().read().frame_middleware.clone();
        frame_middleware(self.clone())
    }

    /// Run the end middleware
    pub fn end(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let end_middleware = self.inner.deref().read().end_middleware.clone();
        end_middleware(self.clone())
    }

    /// Add a run start middleware
    pub fn add_run_start_middleware(
        &self,
        middleware: impl Send + Sync + Fn(StartMiddleware, World) -> FruityResult<()> + 'static,
    ) {
        let mut this = self.inner.deref().write();
        let next_middleware = this.start_middleware.clone();

        this.start_middleware = Arc::new(move |world| middleware(next_middleware.clone(), world));
    }

    /// Add a run frame middleware
    pub fn add_run_frame_middleware(
        &self,
        middleware: impl Send + Sync + Fn(StartMiddleware, World) -> FruityResult<()> + 'static,
    ) {
        let mut this = self.inner.deref().write();
        let next_middleware = this.frame_middleware.clone();

        this.frame_middleware = Arc::new(move |world| middleware(next_middleware.clone(), world));
    }

    /// Add a run end middleware
    pub fn add_run_end_middleware(
        &self,
        middleware: impl Send + Sync + Fn(StartMiddleware, World) -> FruityResult<()> + 'static,
    ) {
        let mut this = self.inner.deref().write();
        let next_middleware = this.end_middleware.clone();

        this.end_middleware = Arc::new(move |world| middleware(next_middleware.clone(), world));
    }

    /// Get resource container
    #[export]
    pub fn get_resource_container(&self) -> ResourceContainer {
        let this = self.inner.deref().read();
        this.resource_container.clone()
    }
}

impl Debug for World {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        let this = self.inner.deref().read();
        this.resource_container.fmt(formatter)
    }
}
