use crate::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    script_value::{
        ScriptObject, ScriptObjectType, ScriptValue, TryFromScriptValue, TryIntoScriptValue,
    },
    FruityError, FruityResult,
};
use convert_case::{Case, Casing};
use js_sys::{JsString, Object, Reflect};
use send_wrapper::SendWrapper;
use std::{fmt::Debug, future::Future, ops::Deref, pin::Pin};
use wasm_bindgen::{JsCast, JsError, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};

/// Create a wasm js value from a script value
pub fn script_value_to_js_value(value: ScriptValue) -> FruityResult<JsValue> {
    puffin::profile_scope!("script_value_to_js_value");

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
        ScriptValue::Future(future) => {
            let future = async {
                match future
                    .await
                    .map_err(|err| JsValue::from(JsError::from(err)))
                {
                    Ok(result) => script_value_to_js_value(result)
                        .map_err(|err| JsValue::from(JsError::from(err))),
                    Err(err) => Err(err),
                }
            };

            let promise = future_to_promise(future);
            promise.into()
        }
        ScriptValue::Array(value) => {
            let js_array = js_sys::Array::new();

            value.into_iter().try_for_each(|elem| {
                js_array.push(&script_value_to_js_value(elem)?);
                FruityResult::Ok(())
            })?;

            js_array.into()
        }
        ScriptValue::Callback { callback, .. } => {
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

                    let result = callback(args).map_err(|err| JsError::from(err))?;

                    script_value_to_js_value(result).map_err(|err| err.into())
                },
            )
                as Box<dyn Fn(_, _, _, _, _, _, _) -> _ + 'static>);

            closure.into_js_value()
        }
        ScriptValue::Object(rust_object) => {
            let js_object = js_sys::Object::new();

            // Store the shared ptr of the native object into the js_object
            // TODO: This is highly unsafe and should be reworked
            let rust_object_ptr = Box::into_raw(Box::new(rust_object));
            let rust_object_ptr_value = rust_object_ptr as *const () as u32;
            let rust_object_ptr_js_value = JsValue::from_f64(rust_object_ptr_value.into());

            js_sys::Reflect::set(
                &js_object,
                &"__rust_reference".into(),
                &rust_object_ptr_js_value,
            )
            .map_err(|err| FruityError::from(err))?;

            // Define const method accessors
            unsafe { rust_object_ptr.as_ref().unwrap() }
                .get_field_names()?
                .into_iter()
                .try_for_each(|field_name| {
                    // Define getter
                    let field_name_2 = field_name.clone();
                    let getter = wasm_bindgen::closure::Closure::wrap(Box::new(
                        move || -> Result<JsValue, JsError> {
                            let result = unsafe { rust_object_ptr.as_ref().unwrap() }
                                .get_field_value(&field_name_2)
                                .map_err(|err| JsError::from(err))?;

                            script_value_to_js_value(result).map_err(|err| JsError::from(err))
                        },
                    )
                        as Box<dyn Fn() -> _ + 'static>);

                    // Define setter
                    let field_name_2 = field_name.clone();
                    let setter = wasm_bindgen::closure::Closure::wrap(Box::new(
                        move |arg: JsValue| -> Result<(), JsError> {
                            let arg =
                                js_value_to_script_value(arg).map_err(|err| JsError::from(err))?;

                            unsafe { rust_object_ptr.as_mut().unwrap() }
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
            unsafe { rust_object_ptr.as_ref().unwrap() }
                .get_const_method_names()?
                .into_iter()
                .try_for_each(|method_name| {
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

                            let result = unsafe { rust_object_ptr.as_ref().unwrap() }
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
            unsafe { rust_object_ptr.as_ref().unwrap() }
                .get_mut_method_names()?
                .into_iter()
                .try_for_each(|method_name| {
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

                            let result = unsafe { rust_object_ptr.as_mut().unwrap() }
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
    puffin::profile_scope!("js_value_to_script_value");

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

        // Get the function identifier
        let identifier = {
            let fruity_get_type =
                js_sys::Reflect::get(&js_function, &JsValue::from_str("fruityGetType"))
                    .map_err(|err| FruityError::from(err))?;

            if fruity_get_type.is_undefined() {
                ScriptObjectType::Script(js_function.name().as_string().ok_or(
                    FruityError::GenericFailure(
                        "Couldn't extract javascript function name".to_string(),
                    ),
                )?)
            } else {
                let fruity_get_type: js_sys::Function = fruity_get_type.into();
                let type_id_value = fruity_get_type
                    .call0(&JsValue::undefined())
                    .map_err(|err| FruityError::from(err))?;

                let type_id_value = type_id_value
                    .as_string()
                    .ok_or(FruityError::GenericFailure(
                        "Couldn't extract type".to_string(),
                    ))?
                    .parse::<u64>()
                    .map_err(|e| FruityError::GenericFailure(e.to_string()))?;

                ScriptObjectType::from_type_id_value(type_id_value as u64)
            }
        };

        let js_function = SendWrapper::new(js_function);
        let closure = move |args: Vec<ScriptValue>| {
            // Convert all the others args as a JsUnknown
            let args = args
                .into_iter()
                .map(|elem| script_value_to_js_value(elem))
                .try_collect::<Vec<_>>()?;

            let js_array = js_sys::Array::new();
            args.into_iter()
                .try_for_each(|elem| {
                    js_array.push(&elem);
                    FruityResult::Ok(())
                })
                .map_err(|err| JsError::from(err))?;

            // Call the function
            let result = js_function
                .deref()
                .apply(&JsValue::undefined(), &js_array)
                .map_err(|err| FruityError::from(err))?;

            // Return the result
            let result = js_value_to_script_value(result)?;
            Ok(result)
        };

        ScriptValue::Callback {
            identifier: Some(identifier),
            callback: Box::new(closure),
        }
    } else if js_sys::Array::is_array(&value) {
        let js_array: js_sys::Array = value.into();
        ScriptValue::Array(
            js_array
                .iter()
                .map(|elem| js_value_to_script_value(elem))
                .try_collect::<Vec<_>>()?,
        )
    } else if is_promise(&value)? {
        // First case, the object is a promise
        let promise = js_sys::Promise::from(value);
        let future = JsFuture::from(promise);
        let future = SendWrapper::new(future);
        let future = Box::pin(async move {
            match future.await.map(|result| js_value_to_script_value(result)) {
                Ok(result) => result,
                Err(err) => Err(FruityError::from(err)),
            }
        }) as Pin<Box<dyn Send + Future<Output = FruityResult<ScriptValue>>>>;

        ScriptValue::Future(future)
    } else if value.is_bigint() {
        // Second case, the object is a big int
        todo!()
    } else if value.is_object() {
        // Try to get the wrapped native value
        match js_sys::Reflect::get(&value, &"__rust_reference".into()) {
            Ok(ref_ptr_js_value) => {
                if !ref_ptr_js_value.is_undefined() && !ref_ptr_js_value.is_null() {
                    // Third case, the object is a native object
                    // Get the shared ptr of the native object from the js_object
                    // TODO: This is highly unsafe and should be reworked
                    let rust_object_ptr_value = ref_ptr_js_value.as_f64().unwrap() as u32;
                    let native_object = *unsafe {
                        Box::from_raw(
                            rust_object_ptr_value as *const () as *const Box<dyn ScriptObject>
                                as *mut Box<dyn ScriptObject>,
                        )
                    };

                    ScriptValue::Object(native_object)
                } else {
                    // Fourth case, the object is a js object
                    let js_object: js_sys::Object = value.into();
                    ScriptValue::Object(Box::new(JsIntrospectObject {
                        reference: js_object,
                    }))
                }
            }
            Err(_) => {
                // Fourth case, the object is a js object
                let js_object: js_sys::Object = value.into();
                ScriptValue::Object(Box::new(JsIntrospectObject {
                    reference: js_object,
                }))
            }
        }
    } else {
        ScriptValue::Undefined
    })
}

fn is_promise(value: &JsValue) -> FruityResult<bool> {
    Ok(if value.is_falsy() {
        false
    } else {
        let then =
            js_sys::Reflect::get(value, &"then".into()).map_err(|err| FruityError::from(err))?;

        js_sys::Function::is_type_of(&then)
    })
}

/// A structure to store a javascript object that can be stored in a ScriptValue
#[derive(FruityAny, Clone)]
pub struct JsIntrospectObject {
    reference: js_sys::Object,
}

// Safe cause wasm is mono-threaded
unsafe impl Send for JsIntrospectObject {}

// Safe cause wasm is mono-threaded
unsafe impl Sync for JsIntrospectObject {}

impl Debug for JsIntrospectObject {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl JsIntrospectObject {
    pub fn new(class_name: String) -> FruityResult<Self> {
        let js_object = Object::new();

        // Assign the class name to the new object
        let js_constructor: JsValue = Object::new().into();
        Reflect::set(
            &js_constructor,
            &JsString::from("name").into(),
            &JsString::from(class_name).into(),
        )?;

        Reflect::set(
            &js_object,
            &JsString::from("constructor").into(),
            &js_constructor,
        )?;

        // TODO: Use the js class prototype

        Ok(Self {
            reference: js_object,
        })
    }
}

impl IntrospectFields for JsIntrospectObject {
    fn is_static(&self) -> FruityResult<bool> {
        Ok(false)
    }

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
            .map(|name| name.to_case(Case::Snake))
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
        let prototype = js_sys::Object::get_prototype_of(&self.reference);
        let keys = js_sys::Object::get_own_property_names(prototype.unchecked_ref());

        Ok(keys
            .iter()
            .filter_map(|key| {
                let key: js_sys::JsString = key.into();
                key.as_string()
            })
            .collect())
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        // Convert all the others args as a JsUnknown
        let args = args
            .into_iter()
            .map(|elem| script_value_to_js_value(elem))
            .try_collect::<Vec<_>>()?;

        let js_array = js_sys::Array::new();
        args.into_iter()
            .try_for_each(|elem| {
                js_array.push(&elem);
                FruityResult::Ok(())
            })
            .map_err(|err| JsError::from(err))?;

        // Call the function
        let prototype = js_sys::Object::get_prototype_of(&self.reference);
        let method = js_sys::Reflect::get(
            prototype.unchecked_ref(),
            &name.clone().to_case(Case::Camel).into(),
        )
        .map_err(|err| FruityError::from(err))?;
        let method: js_sys::Function = method.into();

        let result = method
            .apply(&self.reference, &js_array)
            .map_err(|err| FruityError::from(err))?;

        // Return the result
        let result = js_value_to_script_value(result)?;
        Ok(result)
    }
}

impl TryIntoScriptValue for JsIntrospectObject {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self)))
    }
}

impl TryFromScriptValue for JsIntrospectObject {
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
            FruityError::NoExternalBuffersAllowed(message) => JsError::new(&message),
            FruityError::Unknown(message) => JsError::new(&message),
        }
    }
}
