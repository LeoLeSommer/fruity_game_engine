use super::Deserialize;
use crate::{component::Component, entity::EntityId};
use fruity_game_engine::{
    any::FruityAny,
    export_impl, export_struct,
    resource::ResourceContainer,
    script_value::{ScriptValue, TryIntoScriptValue},
    settings::Settings,
    FruityResult,
};
use std::collections::HashMap;

/// Utility used to deserialize objects, mostly used to restore snapshot
#[derive(FruityAny)]
#[export_struct]
pub struct SerializationService {
    resource_container: ResourceContainer,
    factories: HashMap<
        String,
        Box<
            dyn Fn(
                    &Settings,
                    &ResourceContainer,
                    &HashMap<u64, EntityId>,
                ) -> FruityResult<ScriptValue>
                + Send
                + Sync,
        >,
    >,
}

#[export_impl]
impl SerializationService {
    /// Returns an SerializationService
    pub fn new(resource_container: ResourceContainer) -> SerializationService {
        SerializationService {
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
        T: Deserialize + TryIntoScriptValue,
    {
        self.factories.insert(
            T::get_identifier(),
            Box::new(|script_value, resource_container, local_id_to_entity_id| {
                let result =
                    T::deserialize(script_value, &resource_container, local_id_to_entity_id)?;

                result.into_script_value()
            }),
        );
    }

    /// Register a new deserialize type
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    ///
    /// # Generic Arguments
    /// * `T` - The type of the object
    ///
    pub fn register_component<T>(&mut self)
    where
        T: Deserialize + Component + TryIntoScriptValue,
    {
        self.factories.insert(
            T::get_identifier(),
            Box::new(|script_value, resource_container, local_id_to_entity_id| {
                let result = Box::new(T::deserialize(
                    script_value,
                    resource_container,
                    local_id_to_entity_id,
                )?) as Box<dyn Component>;

                result.into_script_value()
            }),
        );
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
            &Settings,
            &ResourceContainer,
            &HashMap<u64, EntityId>,
        ) -> FruityResult<ScriptValue>,
    ) {
        self.factories
            .insert(object_type.to_string(), Box::new(instantiate));
    }

    /// Instantiate an object from it's factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `serialized` - A serialized value that will populate the new component
    ///
    pub fn instantiate(
        &self,
        value: &Settings,
        object_type: String,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Option<ScriptValue>> {
        Ok(match self.factories.get(&object_type) {
            Some(factory) => Some(factory(
                value,
                &self.resource_container,
                local_id_to_entity_id,
            )?),
            None => None,
        })
    }
}

impl std::fmt::Debug for SerializationService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
