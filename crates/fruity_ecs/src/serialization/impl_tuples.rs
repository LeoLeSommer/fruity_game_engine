use super::{Deserialize, Serialize, Settings};
use crate::entity::EntityId;
use fruity_game_engine::{resource::ResourceContainer, FruityError, FruityResult};
use std::collections::HashMap;

impl<T1: Serialize, T2: Serialize> Serialize for (T1, T2) {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Array(vec![
            self.0.serialize(resource_container)?,
            self.1.serialize(resource_container)?,
        ]))
    }
}

impl<T1: Deserialize, T2: Deserialize> Deserialize for (T1, T2) {
    fn get_identifier() -> String {
        format!("({}, {})", T1::get_identifier(), T2::get_identifier())
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        match serialized {
            Settings::Array(args) => Ok((
                T1::deserialize(
                    &args[0].serialize(resource_container)?,
                    resource_container,
                    local_id_to_entity_id,
                )?,
                T2::deserialize(
                    &args[1].serialize(resource_container)?,
                    resource_container,
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
