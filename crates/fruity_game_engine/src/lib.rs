#![warn(missing_docs)]
#![feature(iterator_try_collect)]
#![feature(downcast_unchecked)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

use crate::module::modules_service::ModulesService;
use crate::resource::resource_container::ResourceContainer;
pub use error::FruityError;
pub use error::FruityResult;
pub use fruity_game_engine_macro::export;
pub use fruity_game_engine_macro::export_constructor;
pub use fruity_game_engine_macro::export_function;
pub use fruity_game_engine_macro::export_impl;
pub use fruity_game_engine_macro::export_struct;
pub use fruity_game_engine_macro::export_trait;
pub use lazy_static::lazy_static;
pub use parking_lot::*;
pub use send_wrapper;

#[cfg(feature = "wasm-module")]
use web_sys::console;

#[cfg(all(feature = "napi-module", feature = "wasm-module"))]
compile_error!("wasm-module and wasm-module are mutually exclusive and cannot be enabled together");

#[cfg(feature = "napi-module")]
pub use napi;

#[cfg(feature = "wasm-module")]
pub use wasm_bindgen;

#[cfg(feature = "wasm-module")]
pub use web_sys;

pub mod error;

/// The any trait
pub mod any;

/// Add introspection into the types exported to the scripting
pub mod introspect;

/// a script value
pub mod script_value;

/// Tools to export javascript modules
pub mod javascript;

/// Tools to load dynamicaly modules
pub mod module;

/// All related with resources
pub mod resource;

/// Provides a tool to inject resources into functions
pub mod inject;

/// Provides a factory for the introspect object
/// This will be used by to do the snapshots
pub mod object_factory_service;

/// Provides tools to profile functions/blocks
pub mod profile;

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

#[cfg(feature = "wasm-module")]
/// Log a message into a console
pub fn console_log(message: &str) {
    console::log_1(&message.into());
}

#[cfg(not(feature = "wasm-module"))]
/// Log a message into a console
pub fn console_log(message: &str) {
    println!("{}", message);
}
