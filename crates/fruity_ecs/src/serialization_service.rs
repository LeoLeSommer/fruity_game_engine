use crate::component::Component;
use crate::entity::EntityId;
use crate::serializable::Serializable;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::{
    any::FruityAny, resource::resource_container::ResourceContainer, script_value::ScriptValue,
    FruityResult,
};
use fruity_game_engine::{export_impl, export_struct};
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
                    ScriptValue,
                    ResourceContainer,
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
        T: Serializable + TryIntoScriptValue,
    {
        self.factories.insert(
            T::get_identifier(),
            Box::new(|script_value, resource_container, local_id_to_entity_id| {
                let result =
                    T::deserialize(script_value, resource_container, local_id_to_entity_id)?;

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
        T: Serializable + Component + TryIntoScriptValue,
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
            ScriptValue,
            ResourceContainer,
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
        value: ScriptValue,
        object_type: String,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Option<ScriptValue>> {
        Ok(match self.factories.get(&object_type) {
            Some(factory) => Some(factory(
                value,
                self.resource_container.clone(),
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
