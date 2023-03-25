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
use fruity_ecs::deserialize_service::DeserializeService;
use fruity_ecs::system_service::{StartupSystemParams, SystemService};
use fruity_game_engine::module::Module;
use fruity_game_engine::{export_function, typescript_import};
use std::sync::Arc;

/// Components of the module
pub mod components;

/// Systems of the module
pub mod systems;

#[typescript_import({SignalProperty, Module} from "fruity_game_engine")]
#[typescript_import({EntityLocation} from "fruity_ecs")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_hierarchy_module() -> Module {
    Module {
        name: "fruity_hierarchy".to_string(),
        dependencies: vec!["fruity_ecs".to_string()],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let deserialize_service = resource_container.require::<DeserializeService>();
            let mut deserialize_service = deserialize_service.write();

            deserialize_service.register_component::<Parent>();

            let system_service = resource_container.require::<SystemService>();
            let mut system_service = system_service.write();

            system_service.add_startup_system(
                "delete_cascade",
                &delete_cascade as &'static (dyn Fn(_, _) -> _ + Send + Sync),
                Some(StartupSystemParams {
                    ignore_pause: Some(true),
                    ..Default::default()
                }),
            );
            system_service.add_startup_system(
                "update_nested_level",
                &update_nested_level as &'static (dyn Fn(_) -> _ + Send + Sync),
                Some(StartupSystemParams {
                    ignore_pause: Some(true),
                    ..Default::default()
                }),
            );

            Ok(())
        })),
        ..Default::default()
    }
}
