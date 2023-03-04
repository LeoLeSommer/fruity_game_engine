use crate::any::FruityAny;
use crate::console_log;
use crate::export;
use crate::frame_service::FrameService;
use crate::module::Module;
use crate::object_factory_service::ObjectFactoryService;
use crate::resource::script_resource_container::ScriptResourceContainer;
use crate::settings::Settings;
use crate::utils::asynchronous::block_on;
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

/// A middleware that occurs when the world runs
#[typescript("type RunMiddleware = (world: World, settings: Settings) => void")]
pub type RunMiddleware = Arc<
    dyn Send + Sync + Fn(World, Settings) -> Pin<Box<dyn Send + Future<Output = FruityResult<()>>>>,
>;

struct InnerWorld {
    resource_container: ResourceContainer,
    settings: Settings,
    start_middleware: StartMiddleware,
    frame_middleware: FrameMiddleware,
    end_middleware: EndMiddleware,
    run_middleware: RunMiddleware,
}

/// The main container of the ECS
#[derive(FruityAny, Clone)]
#[export_struct]
pub struct World {
    inner: Arc<RwLock<InnerWorld>>,
    module_service: Arc<RwLock<ModulesService>>,
    script_resource_container: ScriptResourceContainer,
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
                run_middleware: Arc::new(|world, _settings| {
                    Box::pin(async move {
                        world.setup_modules_async().await?;
                        world.load_resources_async().await?;
                        world.start()?;
                        world.frame()?;
                        world.end()?;

                        FruityResult::Ok(())
                    })
                }),
            })),
            module_service: Arc::new(RwLock::new(module_service)),
            script_resource_container: ScriptResourceContainer::new(resource_container),
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
        if let Some(run_middleware) = module.run_middleware.clone() {
            let mut this = self.inner.deref().write();
            this.run_middleware = run_middleware;
        }

        self.module_service.deref().write().register_module(module);

        Ok(())
    }

    /// Load the modules
    pub async fn setup_modules_async(&self) -> FruityResult<()> {
        let settings = self.inner.deref().read().settings.clone();
        let module_service = self.module_service.deref().read();

        module_service
            .traverse_modules_by_dependencies_async(|module: Module| {
                let settings = settings.clone();
                async move {
                    console_log(&module.name);

                    if let Some(setup) = module.setup {
                        setup(self.clone(), settings.clone())?;
                    }

                    if let Some(setup_async) = module.setup_async {
                        let world = self.clone();
                        let settings = settings.clone();

                        setup_async(world.clone(), settings.clone()).await?;
                    }

                    Ok(())
                }
            })
            .await?;

        Ok(())
    }

    /// Load the resources
    pub async fn load_resources_async(&self) -> FruityResult<()> {
        let settings = self.inner.deref().read().settings.clone();
        let module_service = self.module_service.deref().read();

        module_service
            .traverse_modules_by_dependencies_async(|module: Module| {
                let settings = settings.clone();
                async move {
                    if let Some(load_resources) = module.load_resources {
                        load_resources(self.clone(), settings.clone())?;
                    }

                    if let Some(load_resources_async) = module.load_resources_async {
                        let world = self.clone();
                        let settings = settings.clone();

                        load_resources_async(world.clone(), settings.clone()).await?;
                    }

                    Ok(())
                }
            })
            .await
    }

    /// Run the world
    #[export]
    pub fn run(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let settings = self.inner.deref().read().settings.clone();
        let run_middleware = self.inner.deref().read().run_middleware.clone();

        let world = self.clone();
        block_on(Box::pin(async move {
            // TODO: Better catch errors
            run_middleware(world.clone(), settings).await.unwrap();
        }));

        Ok(())
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
    pub fn get_resource_container(&self) -> ResourceContainer {
        let this = self.inner.deref().read();
        this.resource_container.clone()
    }

    /// Get resource container
    #[export(name = "get_resource_container")]
    pub fn get_script_resource_container(&self) -> ScriptResourceContainer {
        self.script_resource_container.clone()
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
