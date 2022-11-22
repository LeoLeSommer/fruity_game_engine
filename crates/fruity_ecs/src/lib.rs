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

use crate::entity::entity_service::EntityService;
use crate::extension_component_service::ExtensionComponentService;
use crate::system_service::StartupSystemParams;
use crate::system_service::SystemParams;
use crate::system_service::SystemService;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::resource::resource_container::ResourceContainer;

pub use fruity_ecs_derive::Component;
pub use fruity_ecs_derive::IntrospectObject;
pub use fruity_ecs_derive::SerializableObject;

/// All related with components
pub mod component;

/// All related with entities
pub mod entity;

/// Provides collection for systems
pub mod system_service;

/// A service to store components extensions
pub mod extension_component_service;

/// Create an entity type, use it like entity_type!["Component1", "Component2"])
#[macro_export]
macro_rules! entity_type {
    ($($component:expr),*) => {
        fruity_ecs::entity::entity::EntityTypeIdentifier(vec![$ ($component.to_string()),*])
    };
}

/// Identifier of this extension
#[no_mangle]
pub fn module_identifier() -> &'static str {
    "fruity_ecs"
}

/// List all the dependencies of this extension
#[no_mangle]
pub fn dependencies() -> &'static [&'static str] {
    &[]
}

/// Initialize this extension
#[no_mangle]
pub fn initialize(resource_container: ResourceContainer) {
    let system_service = SystemService::new(resource_container.clone());

    let extension_component_service = ExtensionComponentService::new(resource_container.clone());
    resource_container.add::<ExtensionComponentService>(
        "extension_component_service",
        Box::new(extension_component_service),
    );

    let entity_service = EntityService::new(resource_container.clone());

    resource_container.add::<SystemService>("system_service", Box::new(system_service));
    resource_container.add::<EntityService>("entity_service", Box::new(entity_service));

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<SystemParams>("SystemParams");
    object_factory_service.register::<StartupSystemParams>("StartupSystemParams");
}
