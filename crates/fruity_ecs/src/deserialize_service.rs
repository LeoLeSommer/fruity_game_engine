use crate::entity::EntityId;
use fruity_game_engine::{
    any::FruityAny, resource::resource_container::ResourceContainer, script_value::ScriptValue,
    FruityResult,
};
use fruity_game_engine::{export_impl, export_struct};
use std::{collections::HashMap, sync::Arc};

pub use fruity_ecs_macro::DeserializeFactory;

/// Trait to implement a generic constructor from a ScriptValue
pub trait DeserializeFactory {
    /// Get a constructor to instantiate an object
    fn get_factory() -> Factory;
}

/// A setter caller
pub type Factory = Arc<
    dyn Fn(
            &DeserializeService,
            ScriptValue,
            ResourceContainer,
            &HashMap<u64, EntityId>,
        ) -> FruityResult<ScriptValue>
        + Send
        + Sync,
>;

/// Provides a factory for the introspect types
/// This will be used by to do the snapshots
#[derive(FruityAny)]
#[export_struct]
pub struct DeserializeService {
    resource_container: ResourceContainer,
    factories: HashMap<String, Factory>,
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

    /// Register a new deserialize factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    ///
    /// # Generic Arguments
    /// * `T` - The type of the object
    ///
    pub fn register<T>(&mut self, object_type: &str)
    where
        T: DeserializeFactory,
    {
        self.factories
            .insert(object_type.to_string(), T::get_factory());
    }

    /// Register a new deserialize factory from a function constructor
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `constructor` - The constructor
    ///
    pub fn register_func(
        &mut self,
        object_type: &str,
        constructor: impl Fn(
                &DeserializeService,
                ScriptValue,
                ResourceContainer,
                &HashMap<u64, EntityId>,
            ) -> FruityResult<ScriptValue>
            + Send
            + Sync
            + 'static,
    ) {
        self.factories
            .insert(object_type.to_string(), Arc::new(constructor));
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

    /// Iterate over all object factories
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Factory)> {
        self.factories.iter()
    }
}

impl std::fmt::Debug for DeserializeService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
