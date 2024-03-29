#![warn(missing_docs)]
#![feature(iterator_try_collect)]
#![feature(downcast_unchecked)]
#![feature(auto_traits)]
#![feature(async_closure)]
#![feature(arbitrary_self_types)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

pub use error::{FruityError, FruityResult};
pub use fruity_game_engine_macro::{
    export, export_constructor, export_enum, export_function, export_impl, export_struct,
    export_trait, external, typescript, typescript_import,
};
pub use lazy_static::lazy_static;
pub use send_wrapper;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(not(target_arch = "wasm32"))]
pub use napi;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen;

#[cfg(target_arch = "wasm32")]
pub use web_sys;

/// Synchronization tools
pub mod sync;

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

#[cfg(target_arch = "wasm32")]
/// Log a message into a console
pub fn console_log(message: &str) {
    console::log_1(&message.into());
}

#[cfg(not(target_arch = "wasm32"))]
/// Log a message into a console
pub fn console_log(message: &str) {
    println!("{}", message);
}

#[cfg(target_arch = "wasm32")]
/// Log a message into a console
pub fn console_err(message: &str) {
    console::error_1(&message.into());
}

#[cfg(not(target_arch = "wasm32"))]
/// Log a message into a console
pub fn console_err(message: &str) {
    eprintln!("{}", message);
}

/// Profile a scope
#[macro_export]
macro_rules! profile_scope {
    (
        $arg:expr
    ) => {
        let _scope = $crate::profile::intern_profile_scope($arg);
    };
}

/// Profile a scope
#[macro_export]
macro_rules! profile_start {
    () => {
        let _server = $crate::profile::intern_profile_start();
    };
}
