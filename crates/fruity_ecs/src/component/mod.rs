use self::script_component::ScriptComponent;
use crate::entity::archetype::component_storage::ComponentStorage;
use crate::entity::EntityId;
use crate::serializable::{Deserialize, Serialize};
use crate::serialization_service::SerializationService;
pub use fruity_ecs_macro::Component;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::{IntrospectFields, IntrospectMethods};
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use fruity_game_engine::script_value::impl_containers::ScriptValueHashMap;
use fruity_game_engine::script_value::{ScriptObject, ScriptValue};
use fruity_game_engine::settings::Settings;
use fruity_game_engine::{typescript, FruityError, FruityResult};
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

/// Provides reference over a component
pub mod component_reference;

/// Provides guards over a component
pub mod component_guard;

/// Provide a component that contains a script value
pub mod script_component;

/// An abstraction over a component, should be implemented for every component
pub trait StaticComponent {
    /// Return the class type name
    fn get_component_name() -> &'static str;
}

/// An abstraction over a component, should be implemented for every component
pub trait Component:
    IntrospectFields + IntrospectMethods + Serialize + Debug + Send + Sync
{
    /// Create a new component that is a clone of self
    fn duplicate(&self) -> Box<dyn Component>;

    /// Get a collection to store this component in the archetype
    fn get_storage(&self) -> Box<dyn ComponentStorage>;
}

impl dyn Component {
    /// Get all field values
    pub fn get_field_values(&self) -> FruityResult<Vec<(String, ScriptValue)>> {
        self.get_field_names()?
            .into_iter()
            .map(|field_name| {
                self.get_field_value(&field_name)
                    .map(|field_value| (field_name, field_value))
            })
            .try_collect::<Vec<_>>()
    }
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

impl TryIntoScriptValue for AnyComponent {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self.component)))
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

impl Serialize for AnyComponent {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Object(hashmap!(
            "class_name".to_string() => Settings::String(self.component.get_class_name()?),
            "fields".to_string() =>  self.component.serialize(resource_container)?,
        )))
    }
}

impl Deserialize for AnyComponent {
    fn get_identifier() -> String {
        "AnyComponent".to_string()
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
                        "Couldn't deserialize {:?} to AnyComponent, class_name not found",
                        serialized
                    )));
                };

            let fields = if let Some(fields) = serialized.get("fields") {
                fields.clone()
            } else {
                return Err(FruityError::InvalidArg(format!(
                    "Couldn't deserialize {:?} to AnyComponent, fields not found",
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
                instance
            } else {
                // Fallback as a ScriptValueHashMap
                Box::new(ScriptValueHashMap {
                    class_name,
                    fields: <HashMap<String, ScriptValue>>::deserialize(
                        &fields,
                        resource_container,
                        local_id_to_entity_id,
                    )?,
                })
            };

            match instance.downcast::<Box<dyn Component>>() {
                Ok(instance) => Ok(AnyComponent::from(*instance)),
                Err(instance) => Ok(AnyComponent::from(instance)),
            }
        } else {
            Err(FruityError::InvalidArg(format!(
                "Couldn't deserialize {:?} to AnyComponent",
                serialized
            )))
        }
    }
}
