use crate::entity::EntityId;
use fruity_game_engine::{resource::ResourceContainer, settings::Settings, FruityResult};
use std::collections::HashMap;

pub use fruity_ecs_macro::Deserialize;
pub use fruity_ecs_macro::Serialize;

/// Implementation of script value conversions for primitives
pub mod impl_primitives;

/// Implementation of script value conversions for types in fruity_game_engine and fruity_ecs
pub mod impl_fruity_game_engine;

/// Implementation of script value conversions for containers (like Vec, Box ...)
pub mod impl_containers;

/// Implementation of script value conversions for tuples
pub mod impl_tuples;

mod serialization_service;
pub use serialization_service::*;

/// Trait to implement a generic constructor from a ScriptValue
pub trait Serialize {
    /// Serialize an object
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings>;
}

/// Trait to implement a generic constructor from a ScriptValue
pub trait Deserialize: Sized {
    /// Identifier of the deserialize object
    /// in the js, it correspond to the class name
    fn get_identifier() -> String;

    /// Deserialize an object
    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self>;
}
