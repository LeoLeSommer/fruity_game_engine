use crate::entity::archetype::component_storage::ComponentStorage;
use crate::entity::EntityId;
use crate::serializable::{Deserialize, Serialize};
use crate::serialization_service::SerializationService;
pub use fruity_ecs_macro::Component;
use fruity_game_engine::introspect::{IntrospectFields, IntrospectMethods};
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use fruity_game_engine::script_value::impl_containers::ScriptValueHashMap;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::{FruityError, FruityResult};
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt::Debug;

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

    /// Always returns 0, if this is not 0, the same component with different order
    /// will be stored in a different archetype ordered by this value
    /// It used to implements hierarchical components
    /// There is a limit of 256 components with different order
    fn archetype_order(&self) -> FruityResult<u8>;

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

impl TryIntoScriptValue for Box<dyn Component> {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self.component)))
    }
}

impl TryFromScriptValue for Box<dyn Component> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.downcast::<Box<dyn Component>>() {
                Ok(value) => Ok(<Box<dyn Component>>::from(*value)),
                Err(value) => Ok(<Box<dyn Component>>::from(value)),
            },
            value => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to native object",
                value
            ))),
        }
    }
}

impl Serialize for Box<dyn Component> {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Object(hashmap!(
            "class_name".to_string() => Settings::String(self.component.get_class_name()?),
            "fields".to_string() =>  self.component.serialize(resource_container)?,
        )))
    }
}

impl Deserialize for Box<dyn Component> {
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
                Ok(instance) => Ok(<Box<dyn Component>>::from(*instance)),
                Err(instance) => Ok(<Box<dyn Component>>::from(instance)),
            }
        } else {
            Err(FruityError::InvalidArg(format!(
                "Couldn't deserialize {:?} to AnyComponent",
                serialized
            )))
        }
    }
}
