use super::Deserialize;
use crate::entity::{entity_reference::EntityReference, entity_service::EntityService};
use fruity_game_engine::{
    introspect::{IntrospectFields, IntrospectMethods},
    resource::resource_reference::{AnyResourceReference, ResourceReference},
    script_value::{
        convert::{TryFromScriptValue, TryIntoScriptValue},
        ScriptValue,
    },
    signal::{Signal, SignalProperty},
    FruityError,
};

/*impl<T: Deserialize + 'static> Deserialize for Signal<T> {
    fn get_identifier() -> String {
        format!("Signal<{}>", T::get_identifier())
    }

    fn deserialize(
        _deserialize_service: &crate::deserialize_service::DeserializeService,
        _script_value: ScriptValue,
        _resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        _local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        Self::default().into_script_value()
    }
}*/

impl<T: Deserialize + Send + Sync + Clone + 'static> Deserialize for SignalProperty<T> {
    fn get_identifier() -> String {
        format!("SignalProperty<{}>", T::get_identifier())
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        SignalProperty::new(T::deserialize(
            deserialize_service,
            script_value,
            resource_container.clone(),
            local_id_to_entity_id,
        )?)
        .into_script_value()
    }
}

impl Deserialize for EntityReference {
    fn get_identifier() -> String {
        "EntityReference".to_string()
    }

    fn deserialize(
        _deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
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

        entity_reference.into_script_value()
    }
}

impl Deserialize for AnyResourceReference {
    fn get_identifier() -> String {
        "AnyResourceReference".to_string()
    }

    fn deserialize(
        _deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        _local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        if let ScriptValue::String(script_value) = script_value {
            let resource = resource_container.get_untyped(script_value.clone()).ok_or(
                FruityError::GenericFailure(format!(
                    "Resource with identifier {} don't exists",
                    &script_value
                )),
            )?;

            resource.into_script_value()
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
        _deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        _local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        if let ScriptValue::String(script_value) = script_value {
            let resource =
                resource_container
                    .get::<T>(&script_value)
                    .ok_or(FruityError::GenericFailure(format!(
                        "Resource with identifier {} don't exists",
                        &script_value
                    )))?;

            resource.into_script_value()
        } else {
            Err(FruityError::GenericFailure(
                "Cannot deserialize a resource, a string is expected".to_string(),
            ))
        }
    }
}
