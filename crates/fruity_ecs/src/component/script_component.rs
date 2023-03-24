use super::Component;
use crate::entity::archetype::component_storage::{ComponentStorage, VecComponentStorage};
use fruity_game_engine::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    script_value::{ScriptObject, ScriptValue},
    FruityResult,
};

/// Provide a component that contains a script value
#[derive(FruityAny, Debug)]
pub struct ScriptComponent(Box<dyn ScriptObject>);

impl From<Box<dyn ScriptObject>> for ScriptComponent {
    fn from(value: Box<dyn ScriptObject>) -> Self {
        ScriptComponent(value)
    }
}

impl IntrospectFields for ScriptComponent {
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

impl Component for ScriptComponent {
    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(ScriptComponent(self.0.duplicate()))
    }

    fn get_storage(&self) -> Box<dyn ComponentStorage> {
        Box::new(VecComponentStorage::<Self>::new())
    }
}
