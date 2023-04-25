use crate::settings::Settings;
use crate::world::{RunWorldMiddleware, SetupWorldMiddleware, World};
use crate::{sync::Arc, FruityResult};
use fruity_game_engine_macro::{export_impl, export_struct, FruityAny};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::future::Future;
use std::pin::Pin;

/// A service to manage modules loading
mod modules_service;
pub use modules_service::*;

/// A module for the engine
#[derive(Default, Clone, FruityAny)]
#[export_struct(from_raw_js_object = true)]
pub struct Module {
    /// The name of the module
    pub name: String,

    /// The dependencies of the module
    pub dependencies: Vec<String>,

    /// A function that initialize the module
    pub setup: Option<Arc<dyn Send + Sync + Fn(World, Settings) -> FruityResult<()>>>,

    /// An async function that initialize the module
    pub setup_async: Option<
        Arc<
            dyn Send
                + Sync
                + Fn(World, Settings) -> Pin<Box<dyn Send + Future<Output = FruityResult<()>>>>,
        >,
    >,

    /// A function that initialize the module resources
    pub load_resources: Option<Arc<dyn Send + Sync + Fn(World, Settings) -> FruityResult<()>>>,

    /// An async function that initialize the module resources
    pub load_resources_async: Option<
        Arc<
            dyn Send
                + Sync
                + Fn(World, Settings) -> Pin<Box<dyn Send + Future<Output = FruityResult<()>>>>,
        >,
    >,

    /// A middleware that occurs when the world setups
    pub setup_world_middleware: Option<SetupWorldMiddleware>,

    /// A middleware that occurs when the world runs
    pub run_world_middleware: Option<RunWorldMiddleware>,
}

#[export_impl]
impl Module {}

impl Debug for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name)
            .field("dependencies", &self.dependencies)
            .finish()
    }
}
