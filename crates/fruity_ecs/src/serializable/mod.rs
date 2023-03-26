use crate::entity::EntityId;
use fruity_game_engine::{
    resource::resource_container::ResourceContainer, script_value::ScriptValue, FruityResult,
};
use std::collections::HashMap;

pub use fruity_ecs_macro::Serializable;

/// Implementation of script value conversions for primitives
pub mod impl_primitives;

/// Implementation of script value conversions for types in fruity_game_engine and fruity_ecs
pub mod impl_fruity_game_engine;

/// Implementation of script value conversions for containers (like Vec, Box ...)
pub mod impl_containers;

/// Implementation of script value conversions for tuples
pub mod impl_tuples;

/// Trait to implement a generic constructor from a ScriptValue
pub trait Serializable: Sized {
    /// Identifier of the deserialize object
    /// in the js, it correspond to the class name
    fn get_identifier() -> String;

    /// Serializable an object
    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self>;
}
