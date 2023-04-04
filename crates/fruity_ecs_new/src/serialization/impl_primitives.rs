use super::{Deserialize, Serialize};
use crate::entity::EntityId;
use fruity_game_engine::{
    resource::resource_container::ResourceContainer, settings::Settings, FruityError, FruityResult,
};
use std::collections::HashMap;

impl Serialize for Settings {
    fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(self.clone())
    }
}

impl Deserialize for Settings {
    fn get_identifier() -> String {
        "Settings".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        _resource_container: &ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        Ok(serialized.clone())
    }
}

macro_rules! impl_fruity_try_from_fruity_any_for_numeric {
    ( $type:ident, $name:literal ) => {
        impl Serialize for $type {
            fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
                Ok(Settings::F64(*self as f64))
            }
        }

        impl Deserialize for $type {
            fn get_identifier() -> String {
                $name.to_string()
            }

            fn deserialize(
                serialized: &Settings,
                _resource_container: &ResourceContainer,
                _local_id_to_entity_id: &HashMap<u64, EntityId>,
            ) -> FruityResult<Self> {
                $type::try_from(serialized.clone())
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

impl Serialize for bool {
    fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Bool(*self))
    }
}

impl Deserialize for bool {
    fn get_identifier() -> String {
        "bool".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        _resource_container: &ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        bool::try_from(serialized.clone())
    }
}

impl Serialize for String {
    fn serialize(&self, _resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::String(self.clone()))
    }
}

impl Deserialize for String {
    fn get_identifier() -> String {
        "String".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        _resource_container: &ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        String::try_from(serialized.clone())
    }
}

impl<T: Serialize> Serialize for [T; 3] {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Array(vec![
            self[0].serialize(resource_container)?,
            self[1].serialize(resource_container)?,
            self[2].serialize(resource_container)?,
        ]))
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
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match serialized {
            Settings::Array(args) => Ok([
                T::deserialize(
                    &args[0].serialize(resource_container)?,
                    resource_container,
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    &args[1].serialize(resource_container)?,
                    resource_container,
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    &args[2].serialize(resource_container)?,
                    resource_container,
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

impl<T: Serialize> Serialize for [T; 4] {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Array(vec![
            self[0].serialize(resource_container)?,
            self[1].serialize(resource_container)?,
            self[2].serialize(resource_container)?,
            self[3].serialize(resource_container)?,
        ]))
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
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match serialized {
            Settings::Array(args) => Ok([
                T::deserialize(
                    &args[0].serialize(resource_container)?,
                    resource_container,
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    &args[1].serialize(resource_container)?,
                    resource_container,
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    &args[2].serialize(resource_container)?,
                    resource_container,
                    local_id_to_entity_id,
                )?,
                T::deserialize(
                    &args[3].serialize(resource_container)?,
                    resource_container,
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
