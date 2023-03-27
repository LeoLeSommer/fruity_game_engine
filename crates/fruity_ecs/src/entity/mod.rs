use crate::component::AnyComponent;
use crate::component::Component;
use fruity_ecs_macro::{Deserialize, Serialize};
use fruity_game_engine::script_value::convert::TryFromScriptValue;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::typescript;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use std::fmt::Debug;
use std::hash::Hash;

/// Provides a query over entities with given types
pub mod entity_query;

/// Provides a reference to an entity
pub mod entity_reference;

/// Provides guards for an entity
pub mod entity_guard;

/// Provides a collections to store archetypes
pub mod entity_service;

/// Provides a collections to store entities
pub mod archetype;

/// A module for the engine
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct SerializedEntity {
    /// Entity id, it will not be the definitively used id but is internal to the serialization
    /// so the same id can be found in different serializations
    pub local_id: u64,
    /// Name
    pub name: String,
    /// Enabled
    pub enabled: bool,
    /// Components
    pub components: Vec<Settings>,
}

/// An identifier to an entity type, is composed be the identifier of the contained components
#[derive(Debug, Clone)]
pub struct EntityTypeIdentifier(pub Vec<String>);

impl PartialEq for EntityTypeIdentifier {
    fn eq(&self, other: &EntityTypeIdentifier) -> bool {
        let matching = self
            .0
            .iter()
            .zip(other.0.iter())
            .filter(|&(a, b)| a == b)
            .count();
        matching == self.0.len() && matching == other.0.len()
    }
}

impl Eq for EntityTypeIdentifier {}

impl Hash for EntityTypeIdentifier {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.0.hash(state)
    }
}

impl EntityTypeIdentifier {
    /// Check if an entity identifier contains an other one
    /// For example ["c1", "c2", "c3"] contains ["c3", "c2"]
    pub fn contains(&self, other: &String) -> bool {
        self.0.contains(other)
    }
}

/// Id of an entity
#[typescript("type EntityId = number")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct EntityId(pub u64);

impl TryIntoScriptValue for EntityId {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U64(self.0))
    }
}

impl TryFromScriptValue for EntityId {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::I8(value) => Ok(EntityId(value as u64)),
            ScriptValue::I16(value) => Ok(EntityId(value as u64)),
            ScriptValue::I32(value) => Ok(EntityId(value as u64)),
            ScriptValue::I64(value) => Ok(EntityId(value as u64)),
            ScriptValue::ISize(value) => Ok(EntityId(value as u64)),
            ScriptValue::U8(value) => Ok(EntityId(value as u64)),
            ScriptValue::U16(value) => Ok(EntityId(value as u64)),
            ScriptValue::U32(value) => Ok(EntityId(value as u64)),
            ScriptValue::U64(value) => Ok(EntityId(value as u64)),
            ScriptValue::USize(value) => Ok(EntityId(value as u64)),
            ScriptValue::F32(value) => Ok(EntityId(value as u64)),
            ScriptValue::F64(value) => Ok(EntityId(value as u64)),
            _ => Err(FruityError::NumberExpected(format!(
                "Couldn't convert {:?} to EntityId",
                value
            ))),
        }
    }
}

/// Location of an entity in an archetype
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EntityLocation {
    pub(crate) archetype_index: usize,
    pub(crate) entity_index: usize,
}

/// Get the entity type identifier from a list of components
pub fn get_type_identifier_by_any(
    components: &[AnyComponent],
) -> FruityResult<EntityTypeIdentifier> {
    let identifier = components
        .iter()
        .map(|component| component.get_class_name())
        .try_collect::<Vec<_>>()?;

    Ok(EntityTypeIdentifier(identifier))
}

/// Get the entity type identifier from a list of components
pub fn get_type_identifier(components: &[&dyn Component]) -> FruityResult<EntityTypeIdentifier> {
    let identifier = components
        .iter()
        .map(|component| component.get_class_name())
        .try_collect::<Vec<_>>()?;

    Ok(EntityTypeIdentifier(identifier))
}
