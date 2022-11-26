use super::Resource;
use crate::{introspect::IntrospectObject, script_value::ScriptValue, FruityResult};
use fruity_game_engine_macro::FruityAny;
use std::{any::Any, sync::Arc};

/// A resource created by the script
#[derive(FruityAny, Debug)]
pub struct ScriptResource {
    script_object: Box<dyn IntrospectObject>,
}

// Normally, a script resource is always called only by the scripting language
//
// TODO: Find a safer way to do that, for example by splitting system service into a multi threads
// service for native systems and a single threaded service for scripting systems
//
unsafe impl Sync for ScriptResource {}
unsafe impl Send for ScriptResource {}

impl From<Box<dyn IntrospectObject>> for ScriptResource {
    fn from(script_object: Box<dyn IntrospectObject>) -> Self {
        Self { script_object }
    }
}

impl IntrospectObject for ScriptResource {
    fn get_class_name(&self) -> FruityResult<String> {
        self.script_object.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.script_object.get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.script_object.set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.script_object.get_field_value(name)
    }

    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.script_object.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.script_object.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.script_object.get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.script_object.call_mut_method(name, args)
    }
}

impl Resource for ScriptResource {
    fn as_resource_box(self: Box<Self>) -> Box<dyn Resource> {
        self
    }

    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}
