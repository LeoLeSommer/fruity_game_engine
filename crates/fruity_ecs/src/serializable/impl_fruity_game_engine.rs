use super::{Deserialize, Serialize};
use crate::entity::{entity_reference::EntityReference, entity_service::EntityService, EntityId};
use fruity_game_engine::{
    introspect::{IntrospectFields, IntrospectMethods},
    resource::{
        resource_container::ResourceContainer,
        resource_reference::{AnyResourceReference, ResourceReference},
    },
    script_value::{HashMapScriptObject, ScriptValue},
    settings::Settings,
    signal::SignalProperty,
    FruityError, FruityResult,
};
use std::{collections::HashMap, ops::Deref};

impl Serialize for ScriptValue {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(match self {
            ScriptValue::I8(value) => Settings::F64(*value as f64),
            ScriptValue::I16(value) => Settings::F64(*value as f64),
            ScriptValue::I32(value) => Settings::F64(*value as f64),
            ScriptValue::I64(value) => Settings::F64(*value as f64),
            ScriptValue::ISize(value) => Settings::F64(*value as f64),
            ScriptValue::U8(value) => Settings::F64(*value as f64),
            ScriptValue::U16(value) => Settings::F64(*value as f64),
            ScriptValue::U32(value) => Settings::F64(*value as f64),
            ScriptValue::U64(value) => Settings::F64(*value as f64),
            ScriptValue::USize(value) => Settings::F64(*value as f64),
            ScriptValue::F32(value) => Settings::F64(*value as f64),
            ScriptValue::F64(value) => Settings::F64(*value as f64),
            ScriptValue::Bool(value) => Settings::Bool(*value),
            ScriptValue::String(value) => Settings::String(value.clone()),
            ScriptValue::Array(value) => Settings::Array(
                value
                    .into_iter()
                    .map(|value| value.serialize(resource_container))
                    .try_collect::<_>()?,
            ),
            ScriptValue::Null => Settings::Null,
            ScriptValue::Undefined => Settings::Null,
            ScriptValue::Future(_) => unimplemented!(),
            ScriptValue::Callback(_) => unimplemented!(),
            ScriptValue::Object(value) => Settings::Object(
                value
                    .get_field_values()?
                    .into_iter()
                    .map(|(key, value)| {
                        value
                            .serialize(resource_container)
                            .map(|value| (key, value))
                    })
                    .try_collect::<_>()?,
            ),
        })
    }
}

impl Deserialize for ScriptValue {
    fn get_identifier() -> String {
        "ScriptValue".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        Ok(match serialized {
            Settings::F64(value) => ScriptValue::F64(*value),
            Settings::Bool(value) => ScriptValue::Bool(*value),
            Settings::String(value) => ScriptValue::String(value.clone()),
            Settings::Array(value) => ScriptValue::Array(
                value
                    .into_iter()
                    .map(|value| {
                        ScriptValue::deserialize(value, resource_container, local_id_to_entity_id)
                    })
                    .try_collect::<_>()?,
            ),
            Settings::Object(value) => ScriptValue::Object(Box::new(HashMapScriptObject {
                class_name: "unknown".to_string(),
                fields: value
                    .into_iter()
                    .map(|(key, value)| {
                        ScriptValue::deserialize(value, resource_container, local_id_to_entity_id)
                            .map(|value| (key.clone(), value))
                    })
                    .try_collect::<_>()?,
            })),
            Settings::Null => ScriptValue::Null,
        })
    }
}

impl<T: Serialize + Send + Sync + Clone + 'static> Serialize for SignalProperty<T> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(self.deref().clone().serialize(resource_container)?)
    }
}

impl<T: Deserialize + Send + Sync + Clone + 'static> Deserialize for SignalProperty<T> {
    fn get_identifier() -> String {
        format!("SignalProperty<{}>", T::get_identifier())
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        Ok(SignalProperty::new(T::deserialize(
            serialized,
            resource_container,
            local_id_to_entity_id,
        )?))
    }
}

impl Serialize for EntityReference {
    fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::F64(self.get_entity_id()?.0 as f64))
    }
}

impl Deserialize for EntityReference {
    fn get_identifier() -> String {
        "EntityReference".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        let local_id = u64::try_from(serialized.clone())?;
        let entity_id = local_id_to_entity_id
            .get(&local_id)
            .ok_or(FruityError::GenericFailure(format!(
                "Entity with local id {} don't exists",
                local_id
            )))?;
        let entity_service = resource_container.require::<EntityService>();
        let entity_service_reader = entity_service.read();

        let entity_reference = entity_service_reader
            .get_entity_reference(*entity_id)
            .ok_or(FruityError::GenericFailure(format!(
                "Entity with id {:?} don't exists",
                entity_id
            )))?;

        Ok(entity_reference)
    }
}

impl Serialize for AnyResourceReference {
    fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::String(self.get_name()))
    }
}

impl Deserialize for AnyResourceReference {
    fn get_identifier() -> String {
        "AnyResourceReference".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        if let Settings::String(serialized) = serialized {
            let resource = resource_container.get_untyped(serialized.clone()).ok_or(
                FruityError::GenericFailure(format!(
                    "Resource with identifier {} don't exists",
                    &serialized
                )),
            )?;

            Ok(resource)
        } else {
            Err(FruityError::GenericFailure(
                "Cannot deserialize a resource, a string is expected".to_string(),
            ))
        }
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Serialize
    for ResourceReference<T>
{
    fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::String(self.get_name()))
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Deserialize
    for ResourceReference<T>
{
    fn get_identifier() -> String {
        "ResourceReference".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        if let Settings::String(serialized) = serialized {
            let resource =
                resource_container
                    .get::<T>(&serialized)
                    .ok_or(FruityError::GenericFailure(format!(
                        "Resource with identifier {} don't exists",
                        &serialized
                    )))?;

            Ok(resource)
        } else {
            Err(FruityError::GenericFailure(
                "Cannot deserialize a resource, a string is expected".to_string(),
            ))
        }
    }
}

impl Serialize for EntityId {
    fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::F64(self.0 as f64))
    }
}

impl Deserialize for EntityId {
    fn get_identifier() -> String {
        "EntityId".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        _resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        let local_id = u64::try_from(serialized.clone())?;
        let entity_id = local_id_to_entity_id
            .get(&local_id)
            .map(|entity_id| entity_id.clone())
            .ok_or(FruityError::NumberExpected(format!(
                "You try to refer an entity that doesn't exists with local id {:?}",
                local_id
            )))?;

        Ok(entity_id)
    }
}
