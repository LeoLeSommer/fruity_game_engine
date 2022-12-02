#![warn(missing_docs)]
#![feature(iterator_try_collect)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

use std::rc::Rc;

use fruity_ecs::system_service::{StartupSystemParams, SystemService};
use fruity_game_engine::module::Module;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::send_wrapper::SendWrapper;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::world::World;
use fruity_game_engine::FruityResult;
use fruity_game_engine::{export_function, export_value, lazy_static};

use crate::components::parent::Parent;
use crate::systems::delete_cascade::delete_cascade;
use crate::systems::update_nested_level::update_nested_level;

/// Components of the module
pub mod components;

/// Systems of the module
pub mod systems;

/// Name of the module
#[export_value]
pub fn name() -> String {
    "fruity_hierarchy".to_string()
}

/// Dependencies of the module
#[export_value]
pub fn dependencies() -> Vec<String> {
    vec!["fruity_ecs".to_string()]
}

/// Setup the module
#[export_function]
pub fn setup(world: World, _settings: Settings) -> FruityResult<()> {
    let resource_container = world.get_resource_container();

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<Parent>("Parent");

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_startup_system(
        "delete_cascade",
        &delete_cascade as &'static (dyn Fn(_, _) -> _ + Send + Sync),
        Some(StartupSystemParams { ignore_pause: true }),
    );
    system_service.add_startup_system(
        "update_nested_level",
        &update_nested_level as &'static (dyn Fn(_, _) -> _ + Send + Sync),
        Some(StartupSystemParams { ignore_pause: true }),
    );

    Ok(())
}

lazy_static! {
    /// The ecs module, ready to be registered into the fruity_game_engine
    pub static ref FRUITY_ECS_MODULE: SendWrapper<Module> = SendWrapper::new(Module {
        name: name(),
        dependencies: dependencies(),
        setup: Some(Rc::new(setup)),
        load_resources: None,
    });
}
