use crate::{
    entity::EntityId,
    serialization::{Deserialize, SerializationService, Serialize},
};
use fruity_game_engine::{
    any::FruityAny,
    external,
    introspect::{IntrospectFields, IntrospectMethods},
    resource::ResourceContainer,
    script_value::{ScriptObjectType, ScriptValue},
    settings::Settings,
    typescript, FruityError, FruityResult,
};
use maplit::hashmap;
use std::{collections::HashMap, fmt::Debug};

mod name;
pub use name::*;

mod enabled;
pub use enabled::*;

mod script_component;
pub use script_component::*;

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
            "fields".to_string() =>  self.serialize(resource_container)?,
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
        if let Settings::Object(serialized) = serialized {
            let serialization_service = resource_container.require::<SerializationService>();
            let serialization_service_reader = serialization_service.read();

            let class_name =
                if let Some(Settings::String(class_name)) = serialized.get("class_name") {
                    class_name.clone()
                } else {
                    return Err(FruityError::InvalidArg(format!(
                        "Couldn't deserialize {:?} to Box<dyn Component>, class_name not found",
                        serialized
                    )));
                };

            let fields = if let Some(fields) = serialized.get("fields") {
                fields.clone()
            } else {
                return Err(FruityError::InvalidArg(format!(
                    "Couldn't deserialize {:?} to Box<dyn Component>, fields not found",
                    serialized
                )));
            };

            let instance = serialization_service_reader.instantiate(
                &fields,
                class_name.clone(),
                local_id_to_entity_id,
            )?;

            let instance = if let Some(ScriptValue::Object(instance)) = instance {
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
                Err(instance) => Ok(<Box<dyn Component>>::from(Box::new(ScriptComponent::from(
                    instance,
                )))),
            }
        } else {
            Err(FruityError::InvalidArg(format!(
                "Couldn't deserialize {:?} to Box<dyn Component>",
                serialized
            )))
        }
    }
}
