use crate::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use crate::settings::Settings;
use crate::world::{RunMiddleware, World};
use crate::FruityResult;
use std::rc::Rc;

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
    pub setup: Option<Rc<dyn Fn(World, Settings) -> FruityResult<()>>>,

    /// A function that initialize the module resources
    pub load_resources: Option<Rc<dyn Fn(World, Settings) -> FruityResult<()>>>,

    /// A middleware that occurs when the world runs
    pub run_middleware: Option<RunMiddleware>,
}
