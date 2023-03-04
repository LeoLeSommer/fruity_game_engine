#![warn(missing_docs)]
#![feature(iterator_try_collect)]

//! Hierarchy
//!
//! A module that add a hierarchy the the ECS
//!
//! The hierarchy is achieved trough a Parent component

use crate::components::parent::Parent;
use crate::systems::delete_cascade::delete_cascade;
use crate::systems::update_nested_level::update_nested_level;
use fruity_ecs::system_service::{StartupSystemParams, SystemService};
use fruity_game_engine::module::Module;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::{export_function, typescript_import};
use std::sync::Arc;

/// Components of the module
pub mod components;

/// Systems of the module
pub mod systems;

#[typescript_import({SignalProperty, Module} from "fruity_game_engine")]
#[typescript_import({EntityId} from "fruity_ecs")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_hierarchy_module() -> Module {
    Module {
        name: "fruity_hierarchy".to_string(),
        dependencies: vec!["fruity_ecs".to_string()],
        setup: Some(Arc::new(|world, _settings| {
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
        })),
        ..Default::default()
    }
}
