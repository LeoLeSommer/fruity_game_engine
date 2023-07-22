use crate::{
    entity::EntityId,
    serialization::{Deserialize, SerializationService, Serialize},
};
use fruity_game_engine::{
    any::FruityAny,
    external,
    introspect::{IntrospectFields, IntrospectMethods},
    javascript::JsIntrospectObject,
    resource::ResourceContainer,
    script_value::{ScriptObjectType, ScriptValue},
    settings::Settings,
    FruityError, FruityResult,
};
use maplit::hashmap;
use std::{collections::HashMap, fmt::Debug, ops::Deref};

mod name;
pub use name::*;

mod enabled;
pub use enabled::*;

mod extension_component_service;
pub use extension_component_service::*;

mod component_storage;
pub use component_storage::*;

mod component_guard;
pub use component_guard::*;

mod component_reference;
pub use component_reference::*;

pub use fruity_ecs_macro::Component;

/// A component is a piece of data that can be attached to an entity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, FruityAny)]
#[external]
pub enum ComponentTypeId {
    /// A component that is implemented in Rust
    Normal(ScriptObjectType),
    /// A component that is implemented in Rust and have an archetype order
    OrderedRust(ScriptObjectType, u8),
}

impl ComponentTypeId {
    /// Get the script object type of the component type id
    pub fn get_script_object_type(self) -> ScriptObjectType {
        match self {
            ComponentTypeId::Normal(script_object_type) => script_object_type,
            ComponentTypeId::OrderedRust(script_object_type, _) => script_object_type,
        }
    }
}

/// An abstraction over a component, should be implemented for every component
pub trait Component:
    IntrospectFields + IntrospectMethods + Serialize + Debug + Send + Sync
{
    /// Create a new instance of the component with the same values
    fn duplicate(&self) -> Box<dyn Component>;

    /// Get the component type id
    fn get_component_type_id(&self) -> FruityResult<ComponentTypeId>;

    /// Get a collection to store this component in the archetype
    fn get_storage(&self) -> Box<dyn ComponentStorage>;
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        Component::duplicate(self.as_ref())
    }
}

impl Serialize for Box<dyn Component> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Object(hashmap!(
            "class_name".to_string() => Settings::String(self.get_class_name()?),
            "fields".to_string() =>  self.deref().serialize(resource_container)?,
        )))
    }
}

impl Deserialize for Box<dyn Component> {
    fn get_identifier() -> String {
        "Box<dyn Component>".to_string()
    }

    fn deserialize(
        serialized: &Settings,
        resource_container: &ResourceContainer,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        let serialization_service = resource_container.require::<SerializationService>();
        let serialization_service_reader = serialization_service.read();

        let instance =
            serialization_service_reader.instantiate(serialized, local_id_to_entity_id)?;

        let instance = if let ScriptValue::Object(instance) = instance {
            // Component can be instantiated as a native component
            Ok(instance)
        } else {
            Err(FruityError::InvalidArg(format!(
                "Couldn't deserialize {:?} to Box<dyn Component>",
                serialized
            )))
        }?;

        match instance.downcast::<Box<dyn Component>>() {
            Ok(instance) => Ok(<Box<dyn Component>>::from(*instance)),
            Err(instance) => match instance.downcast::<JsIntrospectObject>() {
                Ok(instance) => Ok(<Box<dyn Component>>::from(instance)),
                Err(_) => Err(FruityError::InvalidArg(format!(
                    "Couldn't deserialize {:?} to Box<dyn Component>",
                    serialized
                ))),
            },
        }
    }
}

impl Component for JsIntrospectObject {
    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }

    fn get_component_type_id(&self) -> FruityResult<ComponentTypeId> {
        Ok(ComponentTypeId::Normal(ScriptObjectType::from_identifier(
            self.get_class_name()?,
        )))
    }

    fn get_storage(&self) -> Box<dyn ComponentStorage> {
        Box::new(VecComponentStorage::<Self>::new())
    }
}

impl Serialize for JsIntrospectObject {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Object(
            (self as &dyn IntrospectFields)
                .get_field_values()?
                .into_iter()
                .map(|(key, value)| {
                    value
                        .serialize(resource_container)
                        .map(|serialized| (key, serialized))
                })
                .try_collect()?,
        ))
    }
}

impl Deserialize for JsIntrospectObject {
    fn get_identifier() -> String {
        "JsIntrospectObject".to_string()
    }

    fn deserialize(
        _serialized: &Settings,
        _resource_container: &ResourceContainer,
        _local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        todo!()
    }
}
