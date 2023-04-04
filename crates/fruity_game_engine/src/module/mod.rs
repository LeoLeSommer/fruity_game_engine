use crate::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use crate::settings::Settings;
use crate::world::{RunWorldMiddleware, SetupWorldMiddleware, World};
use crate::Arc;
use crate::FruityResult;
use std::future::Future;
use std::pin::Pin;

/// A service to manage modules loading
pub mod modules_service;

/// A module for the engine
#[derive(Clone, TryFromScriptValue, TryIntoScriptValue, Default)]
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
