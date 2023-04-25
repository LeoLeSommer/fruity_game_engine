use super::{component_storage::ComponentStorage, Component, ComponentTypeId, VecComponentStorage};
use crate::serialization::Serialize;
use fruity_game_engine::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    resource::ResourceContainer,
    script_value::{
        ScriptObject, ScriptObjectType, ScriptValue, TryFromScriptValue, TryIntoScriptValue,
    },
    settings::Settings,
    sync::Arc,
    FruityError, FruityResult,
};
use std::ops::Deref;

/// Provide a component that contains a script value
#[derive(FruityAny, Debug, Clone)]
pub struct ScriptComponent(Arc<dyn ScriptObject>);

// Safe cause script components are always called in the main thread
unsafe impl Sync for ScriptComponent {}

// Safe cause script components are always called in the main thread
unsafe impl Send for ScriptComponent {}

impl From<Box<dyn ScriptObject>> for ScriptComponent {
    fn from(value: Box<dyn ScriptObject>) -> Self {
        ScriptComponent(value.into())
    }
}

impl IntrospectFields for ScriptComponent {
    fn is_static(&self) -> FruityResult<bool> {
        Ok(false)
    }

    fn get_class_name(&self) -> FruityResult<String> {
        self.0.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.0.get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.0.set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.0.get_field_value(name)
    }
}

impl IntrospectMethods for ScriptComponent {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.0.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.0.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.0.get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.0.call_mut_method(name, args)
    }
}

impl TryIntoScriptValue for ScriptComponent {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self)))
    }
}

impl TryFromScriptValue for ScriptComponent {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.downcast::<Self>() {
                Ok(value) => Ok(*value),
                Err(value) => Err(FruityError::InvalidArg(format!(
                    "Couldn't convert a {} to {}",
                    value.deref().get_type_name(),
                    std::any::type_name::<Self>()
                ))),
            },
            value => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to native object",
                value
            ))),
        }
    }
}

impl Component for ScriptComponent {
    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }

    fn get_component_type_id(&self) -> FruityResult<ComponentTypeId> {
        Ok(ComponentTypeId::Normal(ScriptObjectType::from_identifier(
            self.0.get_class_name()?,
        )))
    }

    fn get_storage(&self) -> Box<dyn ComponentStorage> {
        Box::new(VecComponentStorage::<Self>::new())
    }
}

impl Serialize for ScriptComponent {
    fn serialize(&self, resource_container: &ResourceContainer) -> FruityResult<Settings> {
        Ok(Settings::Object(
            self.0
                .deref()
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
