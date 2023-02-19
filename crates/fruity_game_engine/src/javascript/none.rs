use crate::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    script_value::ScriptValue,
    typescript, FruityResult,
};

/// A structure to store a javascript object that can be stored in a ScriptValue
#[derive(FruityAny, Clone, Debug)]
#[typescript("type JsIntrospectObject = { [key: string]: any }")]
pub struct JsIntrospectObject {}

impl IntrospectFields for JsIntrospectObject {
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
}

impl IntrospectMethods for JsIntrospectObject {
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
