use super::Deserialize;
use crate::entity::{entity_reference::EntityReference, entity_service::EntityService, EntityId};
use fruity_game_engine::{
    introspect::{IntrospectFields, IntrospectMethods},
    resource::{
        resource_container::ResourceContainer,
        resource_reference::{AnyResourceReference, ResourceReference},
    },
    script_value::{
        convert::{TryFromScriptValue, TryIntoScriptValue},
        ScriptValue,
    },
    signal::{Signal, SignalProperty},
    FruityError, FruityResult,
};
use std::collections::HashMap;

impl<T: Deserialize + TryFromScriptValue + TryIntoScriptValue + Clone + 'static> Deserialize
    for Signal<T>
{
    fn get_identifier() -> String {
        format!("Signal<{}>", T::get_identifier())
    }

    fn deserialize(
        _script_value: ScriptValue,
        _resource_container: ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        Ok(Self::default())
    }
}

impl<T: Deserialize + Send + Sync + Clone + 'static> Deserialize for SignalProperty<T> {
    fn get_identifier() -> String {
        format!("SignalProperty<{}>", T::get_identifier())
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        Ok(SignalProperty::new(T::deserialize(
            script_value,
            resource_container.clone(),
            local_id_to_entity_id,
        )?))
    }
}

impl Deserialize for EntityReference {
    fn get_identifier() -> String {
        "EntityReference".to_string()
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        let local_id = u64::from_script_value(script_value)?;
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

impl Deserialize for AnyResourceReference {
    fn get_identifier() -> String {
        "AnyResourceReference".to_string()
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        if let ScriptValue::String(script_value) = script_value {
            let resource = resource_container.get_untyped(script_value.clone()).ok_or(
                FruityError::GenericFailure(format!(
                    "Resource with identifier {} don't exists",
                    &script_value
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

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Deserialize
    for ResourceReference<T>
{
    fn get_identifier() -> String {
        "ResourceReference".to_string()
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        if let ScriptValue::String(script_value) = script_value {
            let resource =
                resource_container
                    .get::<T>(&script_value)
                    .ok_or(FruityError::GenericFailure(format!(
                        "Resource with identifier {} don't exists",
                        &script_value
                    )))?;

            Ok(resource)
        } else {
            Err(FruityError::GenericFailure(
                "Cannot deserialize a resource, a string is expected".to_string(),
            ))
        }
    }
}

impl Deserialize for EntityId {
    fn get_identifier() -> String {
        "EntityId".to_string()
    }

    fn deserialize(
        script_value: ScriptValue,
        _resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        let local_id = <u64 as TryFromScriptValue>::from_script_value(script_value)?;
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
