use crate::settings::Settings;
use crate::ResourceContainer;
use std::sync::Arc;

/// A service to manage modules loading
pub mod modules_service;

/// A module for the engine
#[derive(Clone)]
pub struct Module {
  /// The name of the module
  pub name: String,

  /// The dependencies of the module
  pub dependencies: Vec<String>,

  /// A function that initialize the module
  pub setup: Option<ModuleCallback>,

  /// A function that initialize the module resources
  pub load_resources: Option<ModuleCallback>,

  /// A function that is called when the world enter into the loop
  pub run: Option<ModuleCallback>,
}

/// A callback for the run function of a module
#[derive(Clone)]
pub struct ModuleCallback(pub Arc<dyn Fn(ResourceContainer, Settings) + Sync + Send>);
