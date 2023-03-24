use super::Deserialize;
use fruity_game_engine::{
    script_value::{convert::TryIntoScriptValue, ScriptValue},
    FruityError,
};

impl Deserialize for ScriptValue {
    fn get_identifier() -> String {
        "ScriptValue".to_string()
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

macro_rules! impl_fruity_try_from_fruity_any_for_numeric {
    ( $type:ident, $name:literal ) => {
        impl Deserialize for $type {
            fn get_identifier() -> String {
                $name.to_string()
            }

            fn deserialize(
                _deserialize_service: &crate::deserialize_service::DeserializeService,
                script_value: ScriptValue,
                _resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
                _local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
            ) -> fruity_game_engine::FruityResult<ScriptValue> {
                Ok(script_value)
            }}
    };
}

impl_fruity_try_from_fruity_any_for_numeric!(i8, "i8");
impl_fruity_try_from_fruity_any_for_numeric!(i16, "i16");
impl_fruity_try_from_fruity_any_for_numeric!(i32, "i32");
impl_fruity_try_from_fruity_any_for_numeric!(i64, "i64");
impl_fruity_try_from_fruity_any_for_numeric!(isize, "isize");
impl_fruity_try_from_fruity_any_for_numeric!(u8, "u8");
impl_fruity_try_from_fruity_any_for_numeric!(u16, "u16");
impl_fruity_try_from_fruity_any_for_numeric!(u32, "u32");
impl_fruity_try_from_fruity_any_for_numeric!(u64, "u64");
impl_fruity_try_from_fruity_any_for_numeric!(usize, "usize");
impl_fruity_try_from_fruity_any_for_numeric!(f32, "f32");
impl_fruity_try_from_fruity_any_for_numeric!(f64, "f64");

impl Deserialize for bool {
    fn get_identifier() -> String {
        "bool".to_string()
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

impl Deserialize for String {
    fn get_identifier() -> String {
        "String".to_string()
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

impl<T: Deserialize> Deserialize for [T; 3] {
    fn get_identifier() -> String {
        format!(
            "[{}, {}, {}]",
            T::get_identifier(),
            T::get_identifier(),
            T::get_identifier()
        )
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        match script_value {
            ScriptValue::Array(mut args) => [
                T::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
                T::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
                T::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
            ]
            .into_script_value(),
            value => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to tuple",
                value
            ))),
        }
    }
}

impl<T: Deserialize> Deserialize for [T; 4] {
    fn get_identifier() -> String {
        format!(
            "[{}, {}, {}, {}]",
            T::get_identifier(),
            T::get_identifier(),
            T::get_identifier(),
            T::get_identifier()
        )
    }

    fn deserialize(
        deserialize_service: &crate::deserialize_service::DeserializeService,
        script_value: ScriptValue,
        resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
        local_id_to_entity_id: &std::collections::HashMap<u64, crate::entity::EntityId>,
    ) -> fruity_game_engine::FruityResult<ScriptValue> {
        match script_value {
            ScriptValue::Array(mut args) => [
                T::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
                T::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
                T::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
                T::deserialize(
                    deserialize_service,
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                ),
            ]
            .into_script_value(),
            value => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to tuple",
                value
            ))),
        }
    }
}
