use super::component::Component;
use crate::entity::archetype::{
    component_array::ComponentArray, component_collection::ComponentCollection,
};
use fruity_game_engine::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    javascript::JsIntrospectObject,
    script_value::ScriptValue,
    send_wrapper::SendWrapper,
    FruityResult,
};

/// Provide a component that contains a script value
#[derive(FruityAny, Debug, Clone)]
pub struct ScriptComponent(pub SendWrapper<JsIntrospectObject>);

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
    fn get_collection(&self) -> Box<dyn ComponentCollection> {
        Box::new(ComponentArray::<ScriptComponent>::new())
    }

    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
