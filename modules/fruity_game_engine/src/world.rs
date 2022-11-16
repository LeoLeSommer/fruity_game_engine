use crate::frame_service::FrameService;
use crate::module::Module;
use crate::settings::Settings;
use crate::ModulesService;
use crate::ResourceContainer;
use std::fmt::Debug;
use std::sync::Arc;

/// A middleware that occurs when entering into the loop
pub type StartMiddleware =
    Arc<dyn Fn(ResourceContainer, Settings, &(dyn Fn() + Sync + Send)) + Sync + Send>;

/// A middleware that occurs when rendering the loop
pub type FrameMiddleware =
    Arc<dyn Fn(ResourceContainer, Settings, &(dyn Fn() + Sync + Send)) + Sync + Send>;

/// A middleware that occurs when leaving the loop
pub type EndMiddleware =
    Arc<dyn Fn(ResourceContainer, Settings, &(dyn Fn() + Sync + Send)) + Sync + Send>;

/// A middleware that occurs when the world runs
pub type RunMiddleware = Arc<
    dyn Fn(
        ResourceContainer,
        Settings,
        &(dyn Fn() + Sync + Send),
        &(dyn Fn() + Sync + Send),
        &(dyn Fn() + Sync + Send),
    ),
>;

/// The main container of the ECS
pub struct World {
    /// The resource container
    resource_container: ResourceContainer,
    settings: Settings,
    start_middleware: Option<StartMiddleware>,
    frame_middleware: Option<FrameMiddleware>,
    end_middleware: Option<EndMiddleware>,
    run_middleware: Option<RunMiddleware>,
}

impl Debug for World {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.resource_container.fmt(formatter)
    }
}

impl<'s> World {
    /// Returns a World
    pub fn new(settings: Settings) -> World {
        let resource_container = ResourceContainer::new();
        Self::initialize(resource_container.clone(), &settings);

        World {
            resource_container,
            settings,
            start_middleware: None,
            frame_middleware: None,
            end_middleware: None,
            run_middleware: None,
        }
    }

    /// Initialize the world
    pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
        let frame_service = FrameService::new(resource_container.clone());
        let module_service = ModulesService::new(resource_container.clone());

        resource_container.add::<FrameService>("frame_service", Box::new(frame_service));
        // resource_container.add::<ModulesService>("module_service", Box::new(module_service));
    }

    /// Register a module
    pub fn register_modules(&self, module: Module) {
        /*let module_service = self.resource_container.require::<ModulesService>();
        let mut module_service_writer = module_service.write();
        module_service_writer.register_module(module);*/
    }

    /// Load the modules
    pub fn setup_modules(&self) {
        /*let module_service = self.resource_container.require::<ModulesService>();
        let module_service_reader = module_service.read();

        module_service_reader.traverse_modules_by_dependencies(&Box::new(|module: Module| {
          if let Some(setup) = module.setup {
            (setup.0)(self.resource_container.clone(), self.settings.clone());
          }
        }));*/
    }

    /// Load the resources
    pub fn load_resources(&self) {
        /*let module_service = self.resource_container.require::<ModulesService>();
        let module_service_reader = module_service.read();

        module_service_reader.traverse_modules_by_dependencies(&Box::new(|module: Module| {
          if let Some(load_resources) = module.load_resources {
            (load_resources.0)(self.resource_container.clone(), self.settings.clone());
          }
        }));*/
    }

    /// Run the world
    pub fn run(&self) {
        puffin::profile_function!();

        if let Some(run_middleware) = &self.run_middleware {
            let resource_container_1 = self.resource_container.clone();
            let resource_container_2 = self.resource_container.clone();
            let resource_container_3 = self.resource_container.clone();
            let settings_1 = self.settings.clone();
            let settings_2 = self.settings.clone();
            let settings_3 = self.settings.clone();

            let start_middleware = self.start_middleware.clone();
            let frame_middleware = self.frame_middleware.clone();
            let end_middleware = self.end_middleware.clone();

            run_middleware(
                self.resource_container.clone(),
                self.settings.clone(),
                &Box::new(move || {
                    Self::run_start(
                        &start_middleware,
                        resource_container_1.clone(),
                        settings_1.clone(),
                    );
                }),
                &Box::new(move || {
                    Self::run_frame(
                        &frame_middleware,
                        resource_container_2.clone(),
                        settings_2.clone(),
                    );
                }),
                &Box::new(move || {
                    Self::run_end(
                        &end_middleware,
                        resource_container_3.clone(),
                        settings_3.clone(),
                    );
                }),
            )
        } else {
            Self::run_start(
                &self.start_middleware,
                self.resource_container.clone(),
                self.settings.clone(),
            );
            Self::run_frame(
                &self.frame_middleware,
                self.resource_container.clone(),
                self.settings.clone(),
            );
            Self::run_end(
                &self.end_middleware,
                self.resource_container.clone(),
                self.settings.clone(),
            );
        }
    }

    /// Run the world on start
    pub fn run_start(
        middleware: &Option<StartMiddleware>,
        resource_container: ResourceContainer,
        settings: Settings,
    ) {
        puffin::profile_function!();

        if let Some(middleware) = middleware {
            middleware(resource_container, settings, &Box::new(|| {}));
        } else {
        }
    }

    /// Run the world on frame
    pub fn run_frame(
        middleware: &Option<FrameMiddleware>,
        resource_container: ResourceContainer,
        settings: Settings,
    ) {
        puffin::profile_function!();

        if let Some(middleware) = middleware {
            middleware(resource_container, settings, &Box::new(|| {}));
        } else {
        }
    }

    /// Run the world on end
    pub fn run_end(
        middleware: &Option<EndMiddleware>,
        resource_container: ResourceContainer,
        settings: Settings,
    ) {
        puffin::profile_function!();

        if let Some(middleware) = middleware {
            middleware(resource_container, settings, &Box::new(|| {}));
        } else {
        }
    }
}
