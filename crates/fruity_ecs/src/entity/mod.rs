use crate::serialization::{Deserialize, Serialize};
use fruity_game_engine::{
    script_value::{
        convert::{TryFromScriptValue, TryIntoScriptValue},
        ScriptValue,
    },
    settings::Settings,
    FruityError, FruityResult,
};

mod archetype;
pub(crate) use archetype::*;

mod entity_service;
pub use entity_service::*;

mod entity_reference;
pub use entity_reference::*;

mod entity_storage;
pub use entity_storage::*;

/// An entity is a unique identifier for an object in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// The location of an entity in the world
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityLocation {
    /// Archetype id
    pub(crate) archetype: ArchetypeId,
    /// Index in the archetype
    pub(crate) index: usize,
}

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
