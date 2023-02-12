use crate::{
    any::FruityAny,
    resource::{resource_container::ResourceContainer, Resource},
    script_value::ScriptValue,
    FruityResult,
};
use crate::{export, export_impl, export_struct};
use std::{collections::HashMap, sync::Arc};

pub use fruity_game_engine_macro::ObjectFactory;

/// Trait to implement a generic constructor from a ScriptValue
pub trait ObjectFactory {
    /// Get a constructor to instantiate an object
    fn get_constructor() -> Constructor;
}

/// A setter caller
pub type Constructor =
    Arc<dyn Fn(ResourceContainer, Vec<ScriptValue>) -> FruityResult<ScriptValue> + Send + Sync>;

/// Provides a factory for the introspect types
/// This will be used by to do the snapshots
#[derive(FruityAny, Resource)]
#[export_struct]
pub struct ObjectFactoryService {
    resource_container: ResourceContainer,
    factories: HashMap<String, Constructor>,
}

#[export_impl]
impl ObjectFactoryService {
    /// Returns an ObjectFactoryService
    pub fn new(resource_container: ResourceContainer) -> ObjectFactoryService {
        ObjectFactoryService {
            resource_container,
            factories: HashMap::new(),
        }
    }

    /// Register a new object factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    ///
    /// # Generic Arguments
    /// * `T` - The type of the object
    ///
    pub fn register<T>(&mut self, object_type: &str)
    where
        T: ObjectFactory,
    {
        self.factories
            .insert(object_type.to_string(), T::get_constructor());
    }

    /// Register a new object factory from a function constructor
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `constructor` - The constructor
    ///
    pub fn register_func(
        &mut self,
        object_type: &str,
        constructor: impl Fn(ResourceContainer, Vec<ScriptValue>) -> FruityResult<ScriptValue>
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
    #[export]
    pub fn instantiate(&self, object_type: String, args: Vec<ScriptValue>) -> Option<ScriptValue> {
        let factory = self.factories.get(&object_type)?;
        let instantied = factory(self.resource_container.clone(), args).ok()?;
        Some(instantied)
    }

    /// Iterate over all object factories
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Constructor)> {
        self.factories.iter()
    }
}

impl std::fmt::Debug for ObjectFactoryService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
