use crate::convert::FruityFrom;
use crate::settings::Settings;
use crate::FruityResult;
use crate::ResourceContainer;
use std::rc::Rc;

/// A service to manage modules loading
pub mod modules_service;

/// A module for the engine
#[derive(Clone, FruityFrom)]
pub struct Module {
    /// The name of the module
    pub name: String,

    /// The dependencies of the module
    pub dependencies: Vec<String>,

    /// A function that initialize the module
    pub setup: Option<Rc<dyn Fn(ResourceContainer, Settings) -> FruityResult<()>>>,

    /// A function that initialize the module resources
    pub load_resources: Option<Rc<dyn Fn(ResourceContainer, Settings) -> FruityResult<()>>>,

    /// A function that is called when the world enter into the loop
    pub run: Option<Rc<dyn Fn(ResourceContainer, Settings) -> FruityResult<()>>>,
}
