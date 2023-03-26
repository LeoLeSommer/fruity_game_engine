use super::Serializable;
use crate::entity::EntityId;
use fruity_game_engine::{
    resource::resource_container::ResourceContainer,
    script_value::{
        convert::{TryFromScriptValue, TryIntoScriptValue},
        ScriptValue,
    },
    FruityError, FruityResult,
};
use std::collections::HashMap;

impl Serializable for ScriptValue {
    fn get_identifier() -> String {
        "ScriptValue".to_string()
    }

    fn deserialize(
        script_value: ScriptValue,
        _resource_container: ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        Ok(script_value)
    }
}

macro_rules! impl_fruity_try_from_fruity_any_for_numeric {
    ( $type:ident, $name:literal ) => {
        impl Serializable for $type {
            fn get_identifier() -> String {
                $name.to_string()
            }

            fn deserialize(
                script_value: ScriptValue,
                _resource_container: ResourceContainer,
                _local_id_to_entity_id: &HashMap<u64, EntityId>,
            ) -> FruityResult<Self> {
                $type::from_script_value(script_value)
            }
        }
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

impl Serializable for bool {
    fn get_identifier() -> String {
        "bool".to_string()
    }

    fn deserialize(
        script_value: ScriptValue,
        _resource_container: ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        bool::from_script_value(script_value)
    }
}

impl Serializable for String {
    fn get_identifier() -> String {
        "String".to_string()
    }

    fn deserialize(
        script_value: ScriptValue,
        _resource_container: ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        String::from_script_value(script_value)
    }
}

impl<T: Serializable> Serializable for [T; 3] {
    fn get_identifier() -> String {
        format!(
            "[{}, {}, {}]",
            T::get_identifier(),
            T::get_identifier(),
            T::get_identifier()
        )
    }

    fn deserialize(
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match script_value {
            ScriptValue::Array(mut args) => Ok([
                T::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
            ]),
            value => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to tuple",
                value
            ))),
        }
    }
}

impl<T: Serializable> Serializable for [T; 4] {
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
        script_value: ScriptValue,
        resource_container: ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match script_value {
            ScriptValue::Array(mut args) => Ok([
                T::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    args.remove(0).into_script_value()?,
                    resource_container.clone(),
                    local_id_to_entity_id,
                )?,
            ]),
            value => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to tuple",
                value
            ))),
        }
    }
}
