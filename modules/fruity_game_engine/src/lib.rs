#![warn(missing_docs)]
#![feature(decl_macro)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

use crate::javascript::JavascriptContext;
use crate::javascript::ToJavascript;
use crate::module::modules_service::ModulesService;
use crate::resource::resource_container::ResourceContainer;
use crate::settings::read_settings;

mod neon {
    pub use ::neon::*;
}

pub use ::neon::main as javascript_module;
pub use ::neon::result::NeonResult as ExportResult;
pub use fruity_game_engine_macro::fruity_export_class;
pub use fruity_game_engine_macro::fruity_module_export;
pub use parking_lot::*;

#[macro_use]
extern crate lazy_static;

/// The any trait
pub mod any;

/// All related with interfacing with javascript
pub mod javascript;

/// Tools to load dynamicaly modules
pub mod module;

/// All related with resources
pub mod resource;

/// Provides a tool to inject resources into functions
pub mod inject;

/// Traits similar to into and from but without some limitations
pub mod convert;

/// An observer pattern
pub mod signal;

/// Provides a collection for settings
pub mod settings;

/// Provides some utils for the game engine
pub mod utils;

/// Provides a main object for the game engine
pub mod world;

/// A service for frame management
pub mod frame_service;

#[fruity_module_export]
fn export(mut ctx: JavascriptContext) -> ExportResult<()> {
    ctx.register_function("readSettings", &read_settings as &dyn Fn() -> _)?;

    Ok(())
}
