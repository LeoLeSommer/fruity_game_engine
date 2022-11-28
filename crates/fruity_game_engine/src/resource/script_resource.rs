use std::{cell::RefCell, rc::Rc};

use crate::{
    introspect::IntrospectObject, javascript::JsIntrospectObject, script_value::ScriptValue,
    FruityResult,
};
use fruity_game_engine_macro::FruityAny;

/// A resource created by the script
#[derive(FruityAny, Debug, Clone)]
pub struct ScriptResource {
    script_object: Rc<RefCell<JsIntrospectObject>>,
}

impl From<JsIntrospectObject> for ScriptResource {
    fn from(script_object: JsIntrospectObject) -> Self {
        Self {
            script_object: Rc::new(RefCell::new(script_object)),
        }
    }
}

impl IntrospectObject for ScriptResource {
    fn get_class_name(&self) -> FruityResult<String> {
        self.script_object.borrow().get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.script_object.borrow().get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.script_object.borrow_mut().set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.script_object.borrow().get_field_value(name)
    }

    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.script_object.borrow().get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.script_object.borrow().call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.script_object.borrow().get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.script_object.borrow_mut().call_mut_method(name, args)
    }
}
