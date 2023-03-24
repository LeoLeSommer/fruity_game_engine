use crate::deserialize::Deserialize;
use crate::entity::EntityId;
use fruity_game_engine::{
    any::FruityAny, resource::resource_container::ResourceContainer, script_value::ScriptValue,
    FruityResult,
};
use fruity_game_engine::{export_impl, export_struct};
use std::collections::HashMap;

/// Utility used to deserialize objects, mostly used to restore snapshot
#[derive(FruityAny)]
#[export_struct]
pub struct DeserializeService {
    resource_container: ResourceContainer,
    factories: HashMap<
        String,
        fn(
            deserialize_service: &DeserializeService,
            script_value: ScriptValue,
            resource_container: ResourceContainer,
            local_id_to_entity_id: &HashMap<u64, EntityId>,
        ) -> FruityResult<ScriptValue>,
    >,
}

#[export_impl]
impl DeserializeService {
    /// Returns an DeserializeService
    pub fn new(resource_container: ResourceContainer) -> DeserializeService {
        DeserializeService {
            resource_container,
            factories: HashMap::new(),
        }
    }

    /// Register a new deserialize type
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    ///
    /// # Generic Arguments
    /// * `T` - The type of the object
    ///
    pub fn register<T>(&mut self)
    where
        T: Deserialize,
    {
        self.factories.insert(T::get_identifier(), T::deserialize);
    }

    /// Register a new deserialize type from a function
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `constructor` - The constructor
    ///
    pub fn register_func(
        &mut self,
        object_type: &str,
        instantiate: fn(
            &DeserializeService,
            ScriptValue,
            ResourceContainer,
            &HashMap<u64, EntityId>,
        ) -> FruityResult<ScriptValue>,
    ) {
        self.factories.insert(object_type.to_string(), instantiate);
    }

    /// Instantiate an object from it's factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `serialized` - A serialized value that will populate the new component
    ///
    pub fn instantiate(
        &self,
        value: ScriptValue,
        object_type: String,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Option<ScriptValue>> {
        Ok(match self.factories.get(&object_type) {
            Some(factory) => Some(factory(
                &self,
                value,
                self.resource_container.clone(),
                local_id_to_entity_id,
            )?),
            None => None,
        })
    }
}

impl std::fmt::Debug for DeserializeService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
