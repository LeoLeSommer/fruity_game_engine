use super::Deserialize;
use crate::entity::EntityId;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Range;

impl<T: Deserialize> Deserialize for Vec<T> {
    fn get_identifier() -> String {
        format!("Vec<{}>", T::get_identifier())
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match script_value {
            ScriptValue::Array(value) => Ok(value
                .into_iter()
                .map(|elem| T::deserialize(elem, resource_container.clone(), local_id_to_entity_id))
                .try_collect::<Vec<_>>()?),
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
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match script_value {
            ScriptValue::Array(value) => Ok(value
                .into_iter()
                .map(|elem| T::deserialize(elem, resource_container.clone(), local_id_to_entity_id))
                .try_collect::<HashSet<_>>()?),
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
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        if let ScriptValue::Object(script_value) = script_value {
            let mut result = HashMap::<String, T>::new();

            script_value
                .get_field_names()?
                .into_iter()
                .try_for_each(|name| {
                    let field_value = script_value.get_field_value(&name)?;
                    result.insert(
                        name,
                        T::deserialize(
                            field_value,
                            resource_container.clone(),
                            local_id_to_entity_id,
                        )?,
                    );

                    FruityResult::Ok(())
                })?;

            Ok(result)
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
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match script_value {
            ScriptValue::Null => Ok(None),
            ScriptValue::Undefined => Ok(None),
            _ => Ok(Some(T::deserialize(
                script_value,
                resource_container.clone(),
                local_id_to_entity_id,
            )?)),
        }
    }
}

impl<T: Deserialize> Deserialize for Range<T> {
    fn get_identifier() -> String {
        format!("Range<{}>", T::get_identifier())
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match script_value {
            ScriptValue::Array(mut value) => {
                if value.len() == 2 {
                    Ok(Range {
                        start: T::deserialize(
                            value.remove(0).into_script_value()?,
                            resource_container.clone(),
                            local_id_to_entity_id,
                        )?,
                        end: T::deserialize(
                            value.remove(0).into_script_value()?,
                            resource_container.clone(),
                            local_id_to_entity_id,
                        )?,
                    })
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
