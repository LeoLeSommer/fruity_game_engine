use crate::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    script_value::{ScriptCallback, ScriptObject, ScriptValue},
    FruityError, FruityResult,
};
use convert_case::{Case, Casing};
use lazy_static::__Deref;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use wasm_bindgen::{JsCast, JsError, JsValue};

/// Create a wasm js value from a script value
pub fn script_value_to_js_value(value: ScriptValue) -> FruityResult<JsValue> {
    Ok(match value {
        ScriptValue::I8(value) => JsValue::from(value),
        ScriptValue::I16(value) => JsValue::from(value),
        ScriptValue::I32(value) => JsValue::from(value),
        ScriptValue::I64(value) => JsValue::from(value),
        ScriptValue::ISize(value) => JsValue::from(value),
        ScriptValue::U8(value) => JsValue::from(value),
        ScriptValue::U16(value) => JsValue::from(value),
        ScriptValue::U32(value) => JsValue::from(value),
        ScriptValue::U64(value) => JsValue::from(value),
        ScriptValue::USize(value) => JsValue::from(value),
        ScriptValue::F32(value) => JsValue::from(value),
        ScriptValue::F64(value) => JsValue::from(value),
        ScriptValue::Bool(value) => JsValue::from(value),
        ScriptValue::String(value) => JsValue::from(&value),
        ScriptValue::Null => JsValue::NULL,
        ScriptValue::Undefined => JsValue::UNDEFINED,
        ScriptValue::Array(value) => {
            let js_array = js_sys::Array::new();

            value.into_iter().try_for_each(|elem| {
                js_array.push(&script_value_to_js_value(elem)?);
                FruityResult::Ok(())
            })?;

            js_array.into()
        }
        ScriptValue::Iterator(_) => todo!(),
        ScriptValue::Callback(value) => {
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                move |arg0: JsValue,
                      arg1: JsValue,
                      arg2: JsValue,
                      arg3: JsValue,
                      arg4: JsValue,
                      arg5: JsValue,
                      arg6: JsValue|
                      -> Result<JsValue, JsError> {
                    let args = vec![arg0, arg1, arg2, arg3, arg4, arg5, arg6];
                    let args = args
                        .into_iter()
                        .map(|arg| js_value_to_script_value(arg))
                        .try_collect::<Vec<_>>()
                        .map_err(|err| JsError::from(err))?;

                    let result = value.call(args).map_err(|err| JsError::from(err))?;

                    script_value_to_js_value(result).map_err(|err| err.into())
                },
            )
                as Box<dyn Fn(_, _, _, _, _, _, _) -> _ + 'static>);

            closure.into_js_value()
        }
        ScriptValue::Object(value) => {
            let js_object = js_sys::Object::new();
            let rust_object: Rc<RefCell<Box<dyn ScriptObject>>> = Rc::from(RefCell::new(value));

            // Define const method accessors
            rust_object
                .borrow()
                .get_field_names()?
                .into_iter()
                .try_for_each(|field_name| {
                    let rust_object = rust_object.clone();

                    // Define getter
                    let rust_object_2 = rust_object.clone();
                    let field_name_2 = field_name.clone();
                    let getter = wasm_bindgen::closure::Closure::wrap(Box::new(
                        move || -> Result<JsValue, JsError> {
                            let result = rust_object_2
                                .borrow()
                                .get_field_value(&field_name_2)
                                .map_err(|err| JsError::from(err))?;

                            script_value_to_js_value(result).map_err(|err| JsError::from(err))
                        },
                    )
                        as Box<dyn Fn() -> _ + 'static>);

                    // Define setter
                    let rust_object_2 = rust_object.clone();
                    let field_name_2 = field_name.clone();
                    let setter = wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |arg: JsValue| -> Result<(), JsError> {
                            let arg =
                                js_value_to_script_value(arg).map_err(|err| JsError::from(err))?;

                            rust_object_2
                                .borrow_mut()
                                .set_field_value(&field_name_2, arg)
                                .map_err(|err| JsError::from(err))?;

                            Ok(())
                        },
                    )
                        as Box<dyn Fn(_) -> _ + 'static>);

                    // Define accessors object for define_property
                    let js_descriptor = js_sys::Object::new();
                    js_sys::Reflect::set(&js_descriptor, &"get".into(), &getter.into_js_value())
                        .map_err(|err| FruityError::from(err))?;
                    js_sys::Reflect::set(&js_descriptor, &"set".into(), &setter.into_js_value())
                        .map_err(|err| FruityError::from(err))?;

                    js_sys::Object::define_property(
                        &js_object,
                        &field_name.clone().to_case(Case::Camel).into(),
                        &js_descriptor,
                    );

                    FruityResult::Ok(())
                })?;

            // Define const method accessors
            rust_object
                .borrow()
                .get_const_method_names()?
                .into_iter()
                .try_for_each(|method_name| {
                    let rust_object = rust_object.clone();
                    let rust_object_2 = rust_object.clone();
                    let method_name_2 = method_name.clone();

                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |arg0: JsValue,
                              arg1: JsValue,
                              arg2: JsValue,
                              arg3: JsValue,
                              arg4: JsValue,
                              arg5: JsValue,
                              arg6: JsValue|
                              -> Result<JsValue, JsError> {
                            let args = vec![arg0, arg1, arg2, arg3, arg4, arg5, arg6];
                            let args = args
                                .into_iter()
                                .map(|arg| js_value_to_script_value(arg))
                                .try_collect::<Vec<_>>()
                                .map_err(|err| JsError::from(err))?;

                            let result = rust_object_2
                                .borrow()
                                .call_const_method(&method_name_2, args)
                                .map_err(|err| JsError::from(err))?;

                            script_value_to_js_value(result).map_err(|err| err.into())
                        },
                    )
                        as Box<dyn Fn(_, _, _, _, _, _, _) -> _ + 'static>);

                    js_sys::Reflect::set(
                        &js_object,
                        &method_name.clone().to_case(Case::Camel).into(),
                        &closure.into_js_value(),
                    )
                    .map_err(|err| FruityError::from(err))?;

                    FruityResult::Ok(())
                })?;

            // Define mut method accessors
            rust_object
                .borrow()
                .get_mut_method_names()?
                .into_iter()
                .try_for_each(|method_name| {
                    let rust_object = rust_object.clone();
                    let rust_object_2 = rust_object.clone();
                    let method_name_2 = method_name.clone();

                    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |arg0: JsValue,
                              arg1: JsValue,
                              arg2: JsValue,
                              arg3: JsValue,
                              arg4: JsValue,
                              arg5: JsValue,
                              arg6: JsValue|
                              -> Result<JsValue, JsError> {
                            let args = vec![arg0, arg1, arg2, arg3, arg4, arg5, arg6];
                            let args = args
                                .into_iter()
                                .map(|arg| js_value_to_script_value(arg))
                                .try_collect::<Vec<_>>()
                                .map_err(|err| JsError::from(err))?;

                            let result = rust_object_2
                                .borrow_mut()
                                .call_mut_method(&method_name_2, args)
                                .map_err(|err| JsError::from(err))?;

                            script_value_to_js_value(result).map_err(|err| err.into())
                        },
                    )
                        as Box<dyn Fn(_, _, _, _, _, _, _) -> _ + 'static>);

                    js_sys::Reflect::set(
                        &js_object,
                        &method_name.clone().to_case(Case::Camel).into(),
                        &closure.into_js_value(),
                    )
                    .map_err(|err| FruityError::from(err))?;

                    FruityResult::Ok(())
                })?;

            js_object.into()
        }
    })
}

/// Create a script value from a wasm js value
pub fn js_value_to_script_value(value: JsValue) -> FruityResult<ScriptValue> {
    Ok(if value.is_null() {
        ScriptValue::Null
    } else if value.is_undefined() {
        ScriptValue::Undefined
    } else if let Some(value) = value.as_f64() {
        ScriptValue::F64(value)
    } else if let Some(value) = value.as_bool() {
        ScriptValue::Bool(value)
    } else if let Some(value) = value.as_string() {
        ScriptValue::String(value)
    } else if value.is_function() {
        let js_function: js_sys::Function = value.into();
        ScriptValue::Callback(Rc::new(JsFunctionCallback {
            reference: Rc::new(js_function),
        }))
    } else if js_sys::Array::is_array(&value) {
        let js_array: js_sys::Array = value.into();
        ScriptValue::Array(
            js_array
                .iter()
                .map(|elem| js_value_to_script_value(elem))
                .try_collect::<Vec<_>>()?,
        )
    } else if value.is_object() {
        let js_object: js_sys::Object = value.into();
        ScriptValue::Object(Box::new(JsIntrospectObject {
            reference: Rc::new(js_object),
        }))
    } else if value.is_bigint() {
        todo!()
    } else {
        ScriptValue::Undefined
    })
}

struct JsFunctionCallback {
    reference: Rc<js_sys::Function>,
}

impl ScriptCallback for JsFunctionCallback {
    fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        web_sys::console::log_1(&"CALL1".into());
        // Convert all the others args as a JsUnknown
        let args = args
            .into_iter()
            .map(|elem| script_value_to_js_value(elem))
            .try_collect::<Vec<_>>()?;

        web_sys::console::log_1(&"CALL2".into());
        let js_array = js_sys::Array::new();
        args.into_iter()
            .try_for_each(|elem| {
                js_array.push(&elem);
                FruityResult::Ok(())
            })
            .map_err(|err| JsError::from(err))?;

        // Call the function
        web_sys::console::log_1(&"CALL3".into());
        let result = self
            .reference
            .deref()
            .apply(&JsValue::undefined(), &js_array)
            .map_err(|err| FruityError::from(err))?;

        // Return the result
        web_sys::console::log_1(&"CALL4".into());
        let result = js_value_to_script_value(result)?;
        Ok(result)
    }

    fn create_thread_safe_callback(
        &self,
    ) -> FruityResult<Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
        todo!()
    }
}

/// A structure to store a javascript object that can be stored in a ScriptValue
#[derive(FruityAny, Clone)]
pub struct JsIntrospectObject {
    reference: Rc<js_sys::Object>,
}

impl Debug for JsIntrospectObject {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectFields for JsIntrospectObject {
    fn get_class_name(&self) -> FruityResult<String> {
        // Get the js func object the reference
        let constructor: js_sys::Function =
            js_sys::Reflect::get(&self.reference.unchecked_ref(), &"constructor".into())
                .map_err(|err| FruityError::from(err))?
                .into();
        let name: js_sys::JsString =
            js_sys::Reflect::get(constructor.unchecked_ref(), &"name".into())
                .map_err(|err| FruityError::from(err))?
                .into();

        Ok(name.as_string().unwrap_or("unknown".to_string()))
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        let keys = js_sys::Object::keys(&self.reference);
        Ok(keys
            .iter()
            .filter_map(|key| {
                let key: js_sys::JsString = key.into();
                key.as_string()
            })
            .collect())
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        js_sys::Reflect::set(
            &self.reference,
            &name.clone().to_case(Case::Camel).into(),
            &script_value_to_js_value(value)?,
        )
        .map_err(|err| FruityError::from(err))?;

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        let result =
            js_sys::Reflect::get(&self.reference, &name.clone().to_case(Case::Camel).into())
                .map_err(|err| FruityError::from(err))?;

        js_value_to_script_value(result)
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

impl From<JsValue> for FruityError {
    fn from(err: JsValue) -> Self {
        FruityError::Unknown(err.as_string().unwrap_or("".to_string()))
    }
}

impl From<JsError> for FruityError {
    fn from(err: JsError) -> Self {
        let err: JsValue = err.into();
        FruityError::Unknown(err.as_string().unwrap_or("".to_string()))
    }
}

impl From<FruityError> for JsError {
    fn from(val: FruityError) -> Self {
        match val {
            FruityError::Ok(message) => JsError::new(&message),
            FruityError::InvalidArg(message) => JsError::new(&message),
            FruityError::ObjectExpected(message) => JsError::new(&message),
            FruityError::StringExpected(message) => JsError::new(&message),
            FruityError::NameExpected(message) => JsError::new(&message),
            FruityError::FunctionExpected(message) => JsError::new(&message),
            FruityError::NumberExpected(message) => JsError::new(&message),
            FruityError::BooleanExpected(message) => JsError::new(&message),
            FruityError::ArrayExpected(message) => JsError::new(&message),
            FruityError::GenericFailure(message) => JsError::new(&message),
            FruityError::PendingException(message) => JsError::new(&message),
            FruityError::Cancelled(message) => JsError::new(&message),
            FruityError::EscapeCalledTwice(message) => JsError::new(&message),
            FruityError::HandleScopeMismatch(message) => JsError::new(&message),
            FruityError::CallbackScopeMismatch(message) => JsError::new(&message),
            FruityError::QueueFull(message) => JsError::new(&message),
            FruityError::Closing(message) => JsError::new(&message),
            FruityError::BigintExpected(message) => JsError::new(&message),
            FruityError::DateExpected(message) => JsError::new(&message),
            FruityError::ArrayBufferExpected(message) => JsError::new(&message),
            FruityError::DetachableArraybufferExpected(message) => JsError::new(&message),
            FruityError::WouldDeadlock(message) => JsError::new(&message),
            FruityError::Unknown(message) => JsError::new(&message),
        }
    }
}
