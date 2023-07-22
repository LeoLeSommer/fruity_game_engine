use super::{Deserialize, Serialize};
use crate::entity::EntityId;
use fruity_game_engine::{
    resource::ResourceContainer, settings::Settings, FruityError, FruityResult,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Range,
};

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Array(
            self.into_iter()
                .map(|elem| elem.serialize(resource_container))
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn get_identifier() -> String {
        format!("Vec<{}>", T::get_identifier())
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match serialized {
            Settings::Array(value) => Ok(value
                .into_iter()
                .map(|elem| T::deserialize(elem, resource_container, local_id_to_entity_id))
                .try_collect::<Vec<_>>()?),
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to vec",
                serialized
            ))),
        }
    }
}

impl<T: Serialize + Eq + Hash> Serialize for HashSet<T> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Array(
            self.into_iter()
                .map(|elem| elem.serialize(resource_container))
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: Deserialize + Eq + Hash> Deserialize for HashSet<T> {
    fn get_identifier() -> String {
        format!("HashSet<{}>", T::get_identifier())
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match serialized {
            Settings::Array(value) => Ok(value
                .into_iter()
                .map(|elem| T::deserialize(elem, resource_container, local_id_to_entity_id))
                .try_collect::<HashSet<_>>()?),
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to vec",
                serialized
            ))),
        }
    }
}

impl<T: Serialize> Serialize for HashMap<String, T> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Object(
            self.into_iter()
                .map(|(key, value)| {
                    FruityResult::Ok((key.clone(), value.serialize(resource_container)?))
                })
                .try_collect::<HashMap<_, _>>()?,
        ))
    }
}

impl<T: Deserialize> Deserialize for HashMap<String, T> {
    fn get_identifier() -> String {
        format!("HashMap<String, {}>", T::get_identifier())
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        if let Settings::Object(serialized) = serialized {
            Ok(serialized
                .into_iter()
                .map(|(key, value)| {
                    FruityResult::Ok((
                        key.clone(),
                        T::deserialize(value, resource_container, local_id_to_entity_id)?,
                    ))
                })
                .try_collect::<HashMap<_, _>>()?)
        } else {
            Err(FruityError::ObjectExpected(format!(
                "Couldn't convert {:?} to HashMap",
                serialized
            )))
        }
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        match self {
            Some(value) => value.serialize(resource_container),
            None => Ok(Settings::Null),
        }
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn get_identifier() -> String {
        format!("Option<{}>", T::get_identifier())
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match serialized {
            Settings::Null => Ok(None),
            _ => Ok(Some(T::deserialize(
                serialized,
                resource_container,
                local_id_to_entity_id,
            )?)),
        }
    }
}

impl<T: Serialize> Serialize for Range<T> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Array(vec![
            self.start.serialize(resource_container)?,
            self.end.serialize(resource_container)?,
        ]))
    }
}

impl<T: Deserialize> Deserialize for Range<T> {
    fn get_identifier() -> String {
        format!("Range<{}>", T::get_identifier())
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match serialized {
            Settings::Array(value) => {
                if value.len() == 2 {
                    Ok(Range {
                        start: T::deserialize(
                            &value[0].serialize(resource_container)?,
                            resource_container,
                            local_id_to_entity_id,
                        )?,
                        end: T::deserialize(
                            &value[1].serialize(resource_container)?,
                            resource_container,
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
                serialized
            ))),
        }
    }
}
