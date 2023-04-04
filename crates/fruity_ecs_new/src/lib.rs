#![warn(missing_docs)]
#![feature(iterator_try_collect)]
#![feature(drain_filter)]
#![feature(pointer_byte_offsets)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

use component::ExtensionComponentService;
use entity::EntityId;
use fruity_game_engine::{export_function, module::Module, typescript_import, Arc};
use serialization::SerializationService;
use system::SystemService;

/// Components module
pub mod component;

/// Entities module
pub mod entity;

/// Queries module
// pub mod query;

/// Serialization module
pub mod serialization;

/// Storage module
pub mod storage;

/// Systems module
pub mod system;

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

            /*let entity_service = EntityService::new(resource_container.clone());
            resource_container.add::<EntityService>("entity_service", Box::new(entity_service));*/

            let serialization_service = resource_container.require::<SerializationService>();
            let mut serialization_service = serialization_service.write();

            serialization_service.register::<EntityId>();

            // Register system middleware
            /*let system_service = resource_container.require::<SystemService>();
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
            });*/

            Ok(())
        })),
        ..Default::default()
    }
}
