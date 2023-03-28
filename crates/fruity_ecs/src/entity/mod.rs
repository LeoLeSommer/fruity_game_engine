use either::Either;
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
use std::ptr::NonNull;

use crate::component::component_guard::AnyComponentReadGuard;
use crate::component::component_guard::AnyComponentWriteGuard;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::Component;
use crate::component::StaticComponent;

use self::archetype::Archetype;

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

/// A struct that allow you to read an entity
#[derive(Clone)]
pub struct Entity<'a> {
    pub(crate) entity_index: usize,
    pub(crate) archetype: &'a Archetype,
}

impl<'a> Debug for Entity<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Entity<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        self.archetype
            .entity_id_array
            .get(self.entity_index)
            .map(|entity_id| *entity_id)
            .unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.archetype
            .name_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        self.archetype
            .enabled_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> impl Iterator<Item = AnyComponentReadGuard<'_>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        self.archetype
            .component_storages
            .iter()
            .map(|(_, storage)| storage.get(self.entity_index))
            .flatten()
            .map(|component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            })
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<impl Iterator<Item = ComponentReadGuard<'_, T>>> {
        Ok(self
            .iter_components_by_type_identifier(T::get_component_name())?
            .into_iter()
            .map(|component_guard| component_guard.try_into())
            .filter_map(|entity_guard| entity_guard.ok()))
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_identifier(
        &self,
        component_identifier: &str,
    ) -> FruityResult<impl Iterator<Item = AnyComponentReadGuard<'_>>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        let storage = if let Some(storage) = self
            .archetype
            .component_storages
            .get(&component_identifier.to_string())
        {
            storage
        } else {
            return Ok(Either::Left(vec![].into_iter()));
        };

        Ok(Either::Right(storage.get(self.entity_index).map(
            |component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            },
        )))
    }

    /// Read a single component with a given type
    pub fn get_component_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<Option<ComponentReadGuard<'_, T>>> {
        Ok(self.iter_components_by_type()?.next())
    }
}

/// A struct that allow you to write an entity
#[derive(Clone)]
pub struct EntityMut<'a> {
    pub(crate) entity_index: usize,
    pub(crate) archetype: &'a Archetype,
}

impl<'a> Debug for EntityMut<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityMut<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        self.archetype
            .entity_id_array
            .get(self.entity_index)
            .map(|entity_id| *entity_id)
            .unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.archetype
            .name_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Set the entity name
    ///
    /// # Arguments
    /// * `value` - The name value
    ///
    pub fn set_name(&self, value: &str) {
        let name = &self.archetype.name_array[self.entity_index];

        // Safe cause it is protected by self.entity_guard
        #[allow(mutable_transmutes)]
        let name = unsafe { std::mem::transmute::<&String, &mut String>(name) };

        *name = value.to_string();
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        self.archetype
            .enabled_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Set the entity enabled state
    ///
    /// # Arguments
    /// * `value` - Is the entity enabled
    ///
    pub fn set_enabled(&self, value: bool) {
        let enabled = &self.archetype.enabled_array[self.entity_index];

        // Safe cause it is protected by self.entity_guard
        #[allow(mutable_transmutes)]
        let enabled = unsafe { std::mem::transmute::<&bool, &mut bool>(enabled) };

        *enabled = value;
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> impl Iterator<Item = AnyComponentReadGuard<'_>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        self.archetype
            .component_storages
            .iter()
            .map(|(_, storage)| storage.get(self.entity_index))
            .flatten()
            .map(|component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            })
    }

    /// Write all components of the entity
    pub fn write_all_components(&self) -> impl Iterator<Item = AnyComponentWriteGuard<'_>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        self.archetype
            .component_storages
            .iter()
            .map(|(_, storage)| storage.get(self.entity_index))
            .flatten()
            .map(|component| AnyComponentWriteGuard {
                entity_guard: lock.write(),
                component_ptr: NonNull::from(component),
            })
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<impl Iterator<Item = ComponentReadGuard<'_, T>>> {
        Ok(self
            .iter_components_by_type_identifier(T::get_component_name())?
            .into_iter()
            .map(|component_guard| component_guard.try_into())
            .filter_map(|entity_guard| entity_guard.ok()))
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_mut<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<impl Iterator<Item = ComponentWriteGuard<'_, T>>> {
        Ok(self
            .iter_components_by_type_identifier_mut(T::get_component_name())?
            .into_iter()
            .map(|component_guard| component_guard.try_into())
            .filter_map(|entity_guard| entity_guard.ok()))
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_identifier(
        &self,
        component_identifier: &str,
    ) -> FruityResult<impl Iterator<Item = AnyComponentReadGuard<'_>>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        let storage = if let Some(storage) = self
            .archetype
            .component_storages
            .get(&component_identifier.to_string())
        {
            storage
        } else {
            return Ok(Either::Left(vec![].into_iter()));
        };

        Ok(Either::Right(storage.get(self.entity_index).map(
            |component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            },
        )))
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_identifier_mut(
        &self,
        component_identifier: &str,
    ) -> FruityResult<impl Iterator<Item = AnyComponentWriteGuard<'_>>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        let storage = if let Some(storage) = self
            .archetype
            .component_storages
            .get(&component_identifier.to_string())
        {
            storage
        } else {
            return Ok(Either::Left(vec![].into_iter()));
        };

        Ok(Either::Right(storage.get(self.entity_index).map(
            |component| AnyComponentWriteGuard {
                entity_guard: lock.write(),
                component_ptr: NonNull::from(component),
            },
        )))
    }

    /// Read a single component with a given type
    pub fn get_component_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<Option<ComponentReadGuard<'_, T>>> {
        Ok(self.iter_components_by_type()?.next())
    }

    /// Write a single component with a given type
    pub fn get_component_by_type_mut<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<Option<ComponentWriteGuard<'_, T>>> {
        Ok(self.iter_components_by_type_mut()?.next())
    }
}
