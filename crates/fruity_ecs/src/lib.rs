#![warn(missing_docs)]
#![feature(iterator_try_collect)]
#![feature(drain_filter)]
#![feature(pointer_byte_offsets)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

use crate::entity::entity_service::EntityService;
use crate::extension_component_service::ExtensionComponentService;
use crate::system::SystemService;
use entity::EntityId;
pub use fruity_ecs_macro::Component;
use fruity_game_engine::export_function;
use fruity_game_engine::module::Module;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::typescript_import;
use serialization_service::SerializationService;
use std::sync::Arc;

/// All related with components
pub mod component;

/// All related with entities
pub mod entity;

/// Provides systems
pub mod system;

/// A service to store components extensions
pub mod extension_component_service;

/// Provides a factory for the introspect object
/// This will be used by to do the snapshots
pub mod serialization_service;

/// A trait to serialize/deserialize script values with the SerializationService
pub mod serializable;

#[typescript_import({Signal, ObserverHandler, Module, ScriptValue} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_ecs_module() -> Module {
    Module {
        name: "fruity_ecs".to_string(),
        dependencies: vec![],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let serialization_service = SerializationService::new(resource_container.clone());
            resource_container.add::<SerializationService>(
                "serialization_service",
                Box::new(serialization_service),
            );

            let system_service = SystemService::new(resource_container.clone());
            resource_container.add::<SystemService>("system_service", Box::new(system_service));

            let extension_component_service =
                ExtensionComponentService::new(resource_container.clone());
            resource_container.add::<ExtensionComponentService>(
                "extension_component_service",
                Box::new(extension_component_service),
            );

            let entity_service = EntityService::new(resource_container.clone());
            resource_container.add::<EntityService>("entity_service", Box::new(entity_service));

            let serialization_service = resource_container.require::<SerializationService>();
            let mut serialization_service = serialization_service.write();

            serialization_service.register::<EntityId>();

            // Register system middleware
            let system_service = resource_container.require::<SystemService>();
            let entity_service = resource_container.require::<EntityService>();
            world.add_run_start_middleware(move |next, world| {
                {
                    let system_service_reader = system_service.read();
                    system_service_reader.run_start()?;
                }

                {
                    let mut entity_service_writer = entity_service.write();
                    unsafe { entity_service_writer.apply_pending_mutations()? };
                }

                next(world)
            });

            let system_service = resource_container.require::<SystemService>();
            let entity_service = resource_container.require::<EntityService>();
            world.add_run_frame_middleware(move |next, world| {
                {
                    let system_service_reader = system_service.read();
                    system_service_reader.run_frame()?;
                }

                {
                    let mut entity_service_writer = entity_service.write();
                    unsafe { entity_service_writer.apply_pending_mutations()? };
                }

                next(world)
            });

            let system_service = resource_container.require::<SystemService>();
            let entity_service = resource_container.require::<EntityService>();
            world.add_run_end_middleware(move |next, world| {
                {
                    let system_service_reader = system_service.read();
                    system_service_reader.run_end()?;
                }

                {
                    let mut entity_service_writer = entity_service.write();
                    unsafe { entity_service_writer.apply_pending_mutations()? };
                }

                next(world)
            });

            Ok(())
        })),
        ..Default::default()
    }
}
