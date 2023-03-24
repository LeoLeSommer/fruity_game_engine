use super::Deserialize;
use super::ScriptValue;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::FruityError;

impl Deserialize for () {
    fn get_identifier() -> String {
        "()".to_string()
    }

    fn deserialize(
        _deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        _resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        _local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        Ok(script_value)
    }
}

impl<T1: Deserialize, T2: Deserialize> Deserialize for (T1, T2) {
    fn get_identifier() -> String {
        format!("({}, {})", T1::get_identifier(), T2::get_identifier())
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        match script_value {
            ScriptValue::Array(mut args) => (
                T1::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
                T2::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
            )
                .into_script_value(),
            value => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to tuple",
                value
            ))),
        }
    }
}
