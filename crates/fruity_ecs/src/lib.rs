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

use crate::entity::entity_service::EntityService;
use crate::extension_component_service::ExtensionComponentService;
use crate::system_service::SystemService;
pub use fruity_ecs_macro::Component;
use fruity_game_engine::export_function;
use fruity_game_engine::module::Module;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::typescript_import;

/// All related with components
pub mod component;

/// All related with entities
pub mod entity;

/// Provides collection for systems
pub mod system_service;

/// A service to store components extensions
pub mod extension_component_service;

#[typescript_import({Signal, ScriptCallback, ObserverHandler, Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_ecs_module() -> Module {
    Module {
        name: "fruity_ecs".to_string(),
        dependencies: vec![],
        setup: Some(Rc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let system_service = SystemService::new(resource_container.clone());
            resource_container.add::<SystemService>("system_service", Box::new(system_service));

            // Register system middleware
            let system_service = resource_container.require::<SystemService>();
            world.add_run_start_middleware(move |next, world| {
                let mut system_service_writer = system_service.write();
                system_service_writer.run_start(&world)?;

                next(world)
            });

            let system_service = resource_container.require::<SystemService>();
            world.add_run_frame_middleware(move |next, world| {
                let system_service_reader = system_service.read();
                system_service_reader.run_frame(&world)?;

                next(world)
            });

            let system_service = resource_container.require::<SystemService>();
            world.add_run_end_middleware(move |next, world| {
                let mut system_service_writer = system_service.write();
                system_service_writer.run_end(&world)?;

                next(world)
            });

            let extension_component_service =
                ExtensionComponentService::new(resource_container.clone());
            resource_container.add::<ExtensionComponentService>(
                "extension_component_service",
                Box::new(extension_component_service),
            );

            let entity_service = EntityService::new(resource_container.clone());
            resource_container.add::<EntityService>("entity_service", Box::new(entity_service));

            Ok(())
        })),
        ..Default::default()
    }
}
