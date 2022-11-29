use crate::{
    any::FruityAny,
    introspect::IntrospectObject,
    object_factory_service::ObjectFactory,
    script_value::{convert::TryIntoScriptValue, ScriptValue},
    FruityResult,
};

/// Tool to export javascript modules
pub struct ExportJavascript {}

impl ExportJavascript {
    /// Returns an ExportJavascript
    pub fn new() -> Self {
        Self {}
    }

    /// Register a class type
    pub fn export_constructor<T>(&mut self, _name: &str, _value: T) -> FruityResult<()>
    where
        T: ObjectFactory,
    {
        Ok(())
    }

    /// Register a class type
    pub fn export_function_as_constructor<T>(&mut self, _name: &str, _value: T) -> FruityResult<()>
    where
        T: TryIntoScriptValue,
    {
        Ok(())
    }

    /// Register a class type
    pub fn export_value<T>(&mut self, _name: &str, _value: T) -> FruityResult<()>
    where
        T: TryIntoScriptValue,
    {
        Ok(())
    }
}

/// A structure to store a javascript object that can be stored in a ScriptValue
#[derive(FruityAny, Clone, Debug)]
pub struct JsIntrospectObject {}

impl IntrospectObject for JsIntrospectObject {
    fn get_class_name(&self) -> FruityResult<String> {
        Ok("unknown".to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn set_field_value(&mut self, _name: &str, _value: ScriptValue) -> FruityResult<()> {
        unreachable!()
    }

    fn get_field_value(&self, _name: &str) -> FruityResult<ScriptValue> {
        unreachable!()
    }

    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_const_method(&self, _name: &str, _args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        unreachable!()
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_mut_method(
        &mut self,
        _name: &str,
        _args: Vec<ScriptValue>,
    ) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}
