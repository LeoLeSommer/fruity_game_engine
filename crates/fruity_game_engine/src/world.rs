use crate::any::FruityAny;
use crate::export;
use crate::frame_service::FrameService;
use crate::module::Module;
use crate::object_factory_service::ObjectFactoryService;
use crate::settings::Settings;
use crate::FruityResult;
use crate::ModulesService;
use crate::ResourceContainer;
use fruity_game_engine_macro::fruity_export;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

/// A middleware that occurs when entering into the loop
pub type StartMiddleware = Rc<dyn Fn(ResourceContainer, Settings) -> FruityResult<()>>;

/// A middleware that occurs when rendering the loop
pub type FrameMiddleware = Rc<dyn Fn(ResourceContainer, Settings) -> FruityResult<()>>;

/// A middleware that occurs when leaving the loop
pub type EndMiddleware = Rc<dyn Fn(ResourceContainer, Settings) -> FruityResult<()>>;

/// A middleware that occurs when the world runs
pub type RunMiddleware = Rc<
    dyn Fn(
        ResourceContainer,
        Settings,
        &(dyn Fn(ResourceContainer, Settings) -> FruityResult<()>),
        &(dyn Fn(ResourceContainer, Settings) -> FruityResult<()>),
        &(dyn Fn(ResourceContainer, Settings) -> FruityResult<()>),
    ) -> FruityResult<()>,
>;

struct InnerWorld {
    resource_container: ResourceContainer,
    settings: Settings,
    start_middleware: StartMiddleware,
    frame_middleware: FrameMiddleware,
    end_middleware: EndMiddleware,
    run_middleware: RunMiddleware,
}

fruity_export! {
    /// The main container of the ECS
    #[derive(FruityAny, Clone)]
    pub struct World {
        inner: Rc<RefCell<InnerWorld>>,
        module_service: Rc<RefCell<ModulesService>>,
    }

    impl World {
        /// Returns a World
        pub fn new(settings: Settings) -> Self {
            let resource_container = ResourceContainer::new();
            Self::initialize(resource_container.clone(), &settings);
            let module_service = ModulesService::new(resource_container.clone());

            World {
                inner: Rc::new(RefCell::new(InnerWorld {
                    resource_container,
                    settings,
                    start_middleware: Rc::new(|_, _| Ok(())),
                    frame_middleware: Rc::new(|_, _| Ok(())),
                    end_middleware: Rc::new(|_, _| Ok(())),
                    run_middleware: Rc::new(|resource_container, settings, start, frame, end| {
                        start(resource_container.clone(), settings.clone())?;
                        frame(resource_container.clone(), settings.clone())?;
                        end(resource_container.clone(), settings.clone())?;

                        FruityResult::Ok(())
                    }),
                })),
                module_service: Rc::new(RefCell::new(module_service)),
            }
        }

        /// Initialize the world
        pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
            let frame_service = FrameService::new(resource_container.clone());
            resource_container.add::<FrameService>("frame_service", Box::new(frame_service));

            let object_factory_service = ObjectFactoryService::new(resource_container.clone());
            resource_container.add::<ObjectFactoryService>("object_factory_service", Box::new(object_factory_service));
        }

        /// Register a module
        #[export]
        pub fn register_module(&self, module: Module) -> FruityResult<()> {
            self.module_service.deref().borrow_mut().register_module(module);

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
            let run_middleware = self.inner.deref().borrow().run_middleware.clone();
            let resource_container = self.inner.deref().borrow().resource_container.clone();
            let settings = self.inner.deref().borrow().settings.clone();
            let start_middleware = self.inner.deref().borrow().start_middleware.clone();
            let frame_middleware = self.inner.deref().borrow().frame_middleware.clone();
            let end_middleware = self.inner.deref().borrow().end_middleware.clone();

            crate::profile::profile_function!();

            run_middleware(
                resource_container,
                settings,
                start_middleware.deref(),
                frame_middleware.deref(),
                end_middleware.deref(),
            )
        }

        /// Add a run start middleware
        pub fn add_run_start_middleware(&self, middleware: impl Fn(StartMiddleware, ResourceContainer, Settings) -> FruityResult<()> + 'static) {
            let mut this = self.inner.deref().borrow_mut();
            let next_middleware = this.start_middleware.clone();

            this.start_middleware = Rc::new(move |resource_container, settings| {
                middleware(next_middleware.clone(), resource_container, settings)
            });
        }

        /// Add a run frame middleware
        pub fn add_run_frame_middleware(&self, middleware: impl Fn(StartMiddleware, ResourceContainer, Settings) -> FruityResult<()> + 'static) {
            let mut this = self.inner.deref().borrow_mut();
            let next_middleware = this.frame_middleware.clone();

            this.frame_middleware = Rc::new(move |resource_container, settings| {
                middleware(next_middleware.clone(), resource_container, settings)
            });
        }

        /// Add a run end middleware
        pub fn add_run_end_middleware(&self, middleware: impl Fn(StartMiddleware, ResourceContainer, Settings) -> FruityResult<()> + 'static) {
            let mut this = self.inner.deref().borrow_mut();
            let next_middleware = this.end_middleware.clone();

            this.end_middleware = Rc::new(move |resource_container, settings| {
                middleware(next_middleware.clone(), resource_container, settings)
            });
        }

        /// Get resource container
        #[export]
        pub fn get_resource_container(&self) -> ResourceContainer {
            let this = self.inner.deref().borrow();
            this.resource_container.clone()
        }
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
