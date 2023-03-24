use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;

use super::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Range;

impl<T: Deserialize> Deserialize for Vec<T> {
    fn get_identifier() -> String {
        format!("Vec<{}>", T::get_identifier())
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: fruity_game_engine::script_value::ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<fruity_game_engine::script_value::ScriptValue> {
        match script_value {
            ScriptValue::Array(value) => value
                .into_iter()
                .map(|elem| {
                    T::deserialize(
                        deserialize_service,
                        elem,
                        resource_container.clone(),
                        local_id_to_entity_id,
                    )
                })
                .try_collect::<Vec<_>>()?
                .into_script_value(),
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to vec",
                script_value
            ))),
        }
    }
}

impl<T: Deserialize + Eq + Hash> Deserialize for HashSet<T> {
    fn get_identifier() -> String {
        format!("HashSet<{}>", T::get_identifier())
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: fruity_game_engine::script_value::ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<fruity_game_engine::script_value::ScriptValue> {
        match script_value {
            ScriptValue::Array(value) => value
                .into_iter()
                .map(|elem| {
                    T::deserialize(
                        deserialize_service,
                        elem,
                        resource_container.clone(),
                        local_id_to_entity_id,
                    )
                })
                .try_collect::<Vec<_>>()?
                .into_script_value(),
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to vec",
                script_value
            ))),
        }
    }
}

impl<T: Deserialize> Deserialize for HashMap<String, T> {
    fn get_identifier() -> String {
        format!("HashMap<String, {}>", T::get_identifier())
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: fruity_game_engine::script_value::ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<fruity_game_engine::script_value::ScriptValue> {
        if let ScriptValue::Object(script_value) = script_value {
            let mut result = HashMap::<String, ScriptValue>::new();

            script_value
                .get_field_names()?
                .into_iter()
                .try_for_each(|name| {
                    let field_value = script_value.get_field_value(&name)?;
                    result.insert(
                        name,
                        T::deserialize(
                            deserialize_service,
                            field_value,
                            resource_container.clone(),
                            local_id_to_entity_id,
                        )?,
                    );

                    FruityResult::Ok(())
                })?;

            result.into_script_value()
        } else {
            Err(FruityError::ObjectExpected(format!(
                "Couldn't convert {:?} to HashMap",
                script_value
            )))
        }
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn get_identifier() -> String {
        format!("Option<{}>", T::get_identifier())
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: fruity_game_engine::script_value::ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<fruity_game_engine::script_value::ScriptValue> {
        match script_value {
            ScriptValue::Null => Ok(ScriptValue::Null),
            ScriptValue::Undefined => Ok(ScriptValue::Null),
            _ => T::deserialize(
                deserialize_service,
                script_value,
                resource_container.clone(),
                local_id_to_entity_id,
            ),
        }
    }
}

impl<T: Deserialize> Deserialize for Range<T> {
    fn get_identifier() -> String {
        format!("Range<{}>", T::get_identifier())
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: fruity_game_engine::script_value::ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<fruity_game_engine::script_value::ScriptValue> {
        match script_value {
            ScriptValue::Array(mut value) => {
                if value.len() == 2 {
                    Range {
                        start: T::deserialize(
                            deserialize_service,
                            value.remove(0).into_script_value()?,
                            resource_container.clone(),
                            local_id_to_entity_id,
                        )?,
                        end: T::deserialize(
                            deserialize_service,
                            value.remove(0).into_script_value()?,
                            resource_container.clone(),
                            local_id_to_entity_id,
                        )?,
                    }
                    .into_script_value()
                } else {
                    Err(FruityError::ArrayExpected(format!(
                        "Couldn't convert {:?} to range",
                        value
                    )))
                }
            }
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to range",
                script_value
            ))),
        }
    }
}
