use fruity_game_engine_macro::typescript;

use crate::any::FruityAny;
use crate::export;
use crate::frame_service::FrameService;
use crate::module::Module;
use crate::object_factory_service::ObjectFactoryService;
use crate::resource::script_resource_container::ScriptResourceContainer;
use crate::settings::Settings;
use crate::FruityResult;
use crate::ModulesService;
use crate::ResourceContainer;
use crate::{export_constructor, export_impl, export_struct};
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

/// A middleware that occurs when entering into the loop
#[typescript("type StartMiddleware = (world: World) => void")]
pub type StartMiddleware = Rc<dyn Fn(World) -> FruityResult<()>>;

/// A middleware that occurs when rendering the loop
#[typescript("type FrameMiddleware = (world: World) => void")]
pub type FrameMiddleware = Rc<dyn Fn(World) -> FruityResult<()>>;

/// A middleware that occurs when leaving the loop
#[typescript("type EndMiddleware = (world: World) => void")]
pub type EndMiddleware = Rc<dyn Fn(World) -> FruityResult<()>>;

/// A middleware that occurs when the world runs
#[typescript("type RunMiddleware = (world: World, settings: Settings, setupModules: (world: World) => void, loadResources: (world: World) => void, start: StartMiddleware, frame: FrameMiddleware, end: EndMiddleware) => void")]
pub type RunMiddleware = Rc<dyn Fn(World, Settings) -> FruityResult<()>>;

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
    inner: Rc<RefCell<InnerWorld>>,
    module_service: Rc<RefCell<ModulesService>>,
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
            inner: Rc::new(RefCell::new(InnerWorld {
                resource_container: resource_container.clone(),
                settings,
                start_middleware: Rc::new(|_| Ok(())),
                frame_middleware: Rc::new(|_| Ok(())),
                end_middleware: Rc::new(|_| Ok(())),
                run_middleware: Rc::new(|world, _settings| {
                    world.setup_modules()?;
                    world.load_resources()?;
                    world.start()?;
                    world.frame()?;
                    world.end()?;

                    FruityResult::Ok(())
                }),
            })),
            module_service: Rc::new(RefCell::new(module_service)),
            script_resource_container: ScriptResourceContainer::new(resource_container),
        }
    }

    /// Initialize the world
    pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
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
            let mut this = self.inner.deref().borrow_mut();
            this.run_middleware = run_middleware;
        }

        self.module_service
            .deref()
            .borrow_mut()
            .register_module(module);

        Ok(())
    }

    /// Load the modules
    #[export]
    pub fn setup_modules(&self) -> FruityResult<()> {
        let settings = self.inner.deref().borrow().settings.clone();
        let module_service = self.module_service.deref().borrow();

        module_service.traverse_modules_by_dependencies(&Box::new(|module: Module| {
            if let Some(setup) = module.setup {
                setup(self.clone(), settings.clone())?;
            }

            Ok(())
        }))
    }

    /// Load the resources
    #[export]
    pub fn load_resources(&self) -> FruityResult<()> {
        let settings = self.inner.deref().borrow().settings.clone();
        let module_service = self.module_service.deref().borrow();

        module_service.traverse_modules_by_dependencies(&Box::new(|module: Module| {
            if let Some(load_resources) = module.load_resources {
                load_resources(self.clone(), settings.clone())?;
            }

            Ok(())
        }))
    }

    /// Run the world
    #[export]
    pub fn run(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let settings = self.inner.deref().borrow().settings.clone();
        let run_middleware = self.inner.deref().borrow().run_middleware.clone();

        run_middleware(self.clone(), settings)
    }

    /// Run the start middleware
    #[export]
    pub fn start(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let start_middleware = self.inner.deref().borrow().start_middleware.clone();
        start_middleware(self.clone())
    }

    /// Run the frame middleware
    #[export]
    pub fn frame(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let frame_middleware = self.inner.deref().borrow().frame_middleware.clone();
        frame_middleware(self.clone())
    }

    /// Run the end middleware
    #[export]
    pub fn end(&self) -> FruityResult<()> {
        crate::profile::profile_function!();

        let end_middleware = self.inner.deref().borrow().end_middleware.clone();
        end_middleware(self.clone())
    }

    /// Add a run start middleware
    pub fn add_run_start_middleware(
        &self,
        middleware: impl Fn(StartMiddleware, World) -> FruityResult<()> + 'static,
    ) {
        let mut this = self.inner.deref().borrow_mut();
        let next_middleware = this.start_middleware.clone();

        this.start_middleware = Rc::new(move |world| middleware(next_middleware.clone(), world));
    }

    /// Add a run frame middleware
    pub fn add_run_frame_middleware(
        &self,
        middleware: impl Fn(StartMiddleware, World) -> FruityResult<()> + 'static,
    ) {
        let mut this = self.inner.deref().borrow_mut();
        let next_middleware = this.frame_middleware.clone();

        this.frame_middleware = Rc::new(move |world| middleware(next_middleware.clone(), world));
    }

    /// Add a run end middleware
    pub fn add_run_end_middleware(
        &self,
        middleware: impl Fn(StartMiddleware, World) -> FruityResult<()> + 'static,
    ) {
        let mut this = self.inner.deref().borrow_mut();
        let next_middleware = this.end_middleware.clone();

        this.end_middleware = Rc::new(move |world| middleware(next_middleware.clone(), world));
    }

    /// Get resource container
    pub fn get_resource_container(&self) -> ResourceContainer {
        let this = self.inner.deref().borrow();
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
        let this = self.inner.deref().borrow();
        this.resource_container.fmt(formatter)
    }
}
