use super::ScriptValue;
use super::Serializable;
use crate::entity::EntityId;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use std::collections::HashMap;

impl Serializable for () {
    fn get_identifier() -> String {
        "()".to_string()
    }

    fn deserialize(
        _script_value: ScriptValue,
        _resource_container: ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        Ok(())
    }
}

impl<T1: Serializable, T2: Serializable> Serializable for (T1, T2) {
    fn get_identifier() -> String {
        format!("({}, {})", T1::get_identifier(), T2::get_identifier())
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match script_value {
            ScriptValue::Array(mut args) => Ok((
                T1::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
                T2::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
            )),
            value => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to tuple",
                value
            ))),
        }
    }
}
