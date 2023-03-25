use self::script_component::ScriptComponent;
use crate::deserialize_service::DeserializeService;
use crate::entity::archetype::component_storage::ComponentStorage;
use crate::entity::EntityId;
pub use fruity_ecs_macro::Component;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::{IntrospectFields, IntrospectMethods};
use fruity_game_engine::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use fruity_game_engine::script_value::impl_containers::ScriptValueHashMap;
use fruity_game_engine::script_value::{ScriptObject, ScriptValue};
use fruity_game_engine::{typescript, FruityError, FruityResult};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

/// Provides reference over a component
pub mod component_reference;

/// Provides guards over a component
pub mod component_guard;

/// Provide a component that contains a script value
pub mod script_component;

/// A module for the engine
#[derive(Clone, TryFromScriptValue, TryIntoScriptValue, Default)]
pub struct SerializedAnyComponent {
    /// The class identifier, is used to create the component trough DeserializeService
    pub class_name: String,

    /// The field values
    pub fields: HashMap<String, ScriptValue>,
}

/// An abstraction over a component, should be implemented for every component
pub trait StaticComponent {
    /// Return the class type name
    fn get_component_name() -> &'static str;
}

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectFields + IntrospectMethods + Debug + Send + Sync {
    /// Create a new component that is a clone of self
    fn duplicate(&self) -> Box<dyn Component>;

    /// Get a collection to store this component in the archetype
    fn get_storage(&self) -> Box<dyn ComponentStorage>;
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        Component::duplicate(self.as_ref())
    }
}

/// An container for a component without knowing the instancied type
#[derive(FruityAny, Debug, Clone)]
#[typescript("type AnyComponent = { [key: string]: any }")]
pub struct AnyComponent {
    component: Box<dyn Component>,
}

impl AnyComponent {
    /// Returns an AnyComponent
    pub fn new(component: impl Component) -> AnyComponent {
        AnyComponent {
            component: Box::new(component),
        }
    }

    /// Returns an AnyComponent
    pub fn from_box(component: Box<dyn Component>) -> AnyComponent {
        AnyComponent { component }
    }

    /// Returns an AnyComponent
    pub fn into_box(self) -> Box<dyn Component> {
        self.component
    }

    /// Serialize an AnyComponent
    pub fn serialize(&self) -> FruityResult<SerializedAnyComponent> {
        Ok(SerializedAnyComponent {
            class_name: self.component.get_class_name()?,
            fields: self
                .component
                .get_field_names()?
                .into_iter()
                .map(|field_name| {
                    self.component
                        .get_field_value(&field_name)
                        .map(|value| (field_name, value))
                })
                .try_collect::<HashMap<_, _>>()?,
        })
    }

    /// Deserialize an AnyComponent
    pub fn deserialize(
        value: ScriptValue,
        deserialize_service: &DeserializeService,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<Self> {
        let serialized_component = SerializedAnyComponent::from_script_value(value)?;

        let new_object = deserialize_service
            .instantiate(
                serialized_component.fields.clone().into_script_value()?,
                serialized_component.class_name.clone(),
                local_id_to_entity_id,
            )?
            .map(|component| Ok(component) as FruityResult<ScriptValue>)
            .unwrap_or(
                ScriptValue::Object(Box::new(ScriptValueHashMap {
                    class_name: serialized_component.class_name.clone(),
                    fields: serialized_component.fields,
                }))
                .into_script_value(),
            )?;

        AnyComponent::from_script_value(new_object)
    }
}

impl From<Box<dyn Component>> for AnyComponent {
    fn from(component: Box<dyn Component>) -> Self {
        Self { component }
    }
}

impl From<Box<dyn ScriptObject>> for AnyComponent {
    fn from(value: Box<dyn ScriptObject>) -> Self {
        Self {
            component: Box::new(ScriptComponent::from(value)),
        }
    }
}

impl Deref for AnyComponent {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.component.as_ref()
    }
}

impl TryFromScriptValue for AnyComponent {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.downcast::<Box<dyn Component>>() {
                Ok(value) => Ok(AnyComponent::from(*value)),
                Err(value) => Ok(AnyComponent::from(value)),
            },
            value => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to native object",
                value
            ))),
        }
    }
}
