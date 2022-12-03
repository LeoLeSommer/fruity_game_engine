use crate::{
    introspect::{IntrospectFields, IntrospectMethods},
    script_value::convert::TryIntoScriptValue,
    script_value::ScriptObject,
    script_value::{ScriptCallback, ScriptValue},
    FruityError, FruityResult,
};
use convert_case::{Case, Casing};
use fruity_game_engine_macro::FruityAny;
use napi::{
    bindgen_prelude::CallbackInfo,
    sys::{napi_env, napi_value},
    threadsafe_function::{
        ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
    },
    Env, JsBigInt, JsFunction, JsNumber, JsObject, JsString, JsUnknown, PropertyAttributes, Ref,
    ValueType,
};
use napi::{check_status, NapiValue};
use napi::{JsError, NapiRaw};
use std::{ffi::CString, marker::PhantomData, ops::Deref};
use std::{fmt::Debug, vec};
use std::{rc::Rc, sync::Arc};

/// Create a napi js value from a script value
pub fn script_value_to_js_value(env: &Env, value: ScriptValue) -> FruityResult<JsUnknown> {
    Ok(match value.into_script_value()? {
        ScriptValue::I8(value) => env
            .create_int32(value as i32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::I16(value) => env
            .create_int32(value as i32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::I32(value) => env
            .create_int32(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::I64(value) => env
            .create_int64(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::ISize(value) => env
            .create_int32(value as i32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U8(value) => env
            .create_uint32(value as u32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U16(value) => env
            .create_uint32(value as u32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U32(value) => env
            .create_uint32(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U64(value) => env
            .create_bigint_from_u64(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown()
            .map_err(|e| FruityError::from_napi(e))?,
        ScriptValue::USize(value) => env
            .create_uint32(value as u32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::F32(value) => env
            .create_double(value as f64)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::F64(value) => env
            .create_double(value as f64)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Bool(value) => env
            .get_boolean(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::String(value) => env
            .create_string(&value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Array(value) => {
            let mut js_array = env
                .create_empty_array()
                .map_err(|e| FruityError::from_napi(e))?;

            for (index, elem) in value.into_iter().enumerate() {
                js_array
                    .set_element(index as u32, script_value_to_js_value(env, elem)?)
                    .map_err(|e| FruityError::from_napi(e))?;
            }

            js_array.into_unknown()
        }
        ScriptValue::Null => env
            .get_null()
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Undefined => env
            .get_undefined()
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Iterator(_value) => {
            todo!()
        }
        ScriptValue::Callback(callback) => env
            .create_function_from_closure("unknown", move |ctx| {
                let args = ctx
                    .get_all()
                    .into_iter()
                    .map(|elem| js_value_to_script_value(ctx.env, elem))
                    .try_collect::<Vec<_>>()
                    .map_err(|e| e.into_napi())?;

                let result = callback.call(args).map_err(|e| e.into_napi())?;
                script_value_to_js_value(ctx.env, result).map_err(|e| e.into_napi())
            })
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Object(value) => {
            match value.downcast::<JsIntrospectObject>() {
                // First case, it's a native js value
                Ok(value) => {
                    let js_object: JsObject = value.reference.inner();
                    js_object.into_unknown()
                }
                // Second case, we wrap the object into a js object
                Err(value) => {
                    let mut js_object =
                        env.create_object().map_err(|e| FruityError::from_napi(e))?;

                    // Defined property accessors
                    let field_names = value
                        .get_field_names()?
                        .into_iter()
                        .map(|field_name| CString::new(field_name).unwrap())
                        .collect::<Vec<_>>();

                    let properties = field_names
                        .iter()
                        .map(|field_name| napi_sys::napi_property_descriptor {
                            utf8name: field_name.as_ptr(),
                            name: std::ptr::null_mut(),
                            method: None,
                            getter: Some(generic_getter),
                            setter: Some(generic_setter),
                            value: std::ptr::null_mut(),
                            attributes: (PropertyAttributes::Default
                                | PropertyAttributes::Writable
                                | PropertyAttributes::Enumerable)
                                .bits(),
                            data: field_name.as_ptr() as *mut std::ffi::c_void,
                        })
                        .collect::<Vec<napi_sys::napi_property_descriptor>>();

                    js_object
                        .add_finalizer((), (), |_| {
                            std::mem::drop(field_names);
                        })
                        .map_err(|e| FruityError::from_napi(e))?;

                    unsafe {
                        check_status!(napi_sys::napi_define_properties(
                            env.raw(),
                            js_object.raw(),
                            properties.len(),
                            properties.as_ptr(),
                        ))
                        .map_err(|e| FruityError::from_napi(e))?;
                    }

                    // Define const method accessors
                    value
                        .get_const_method_names()?
                        .into_iter()
                        .try_for_each(|method_name| {
                            js_object
                                .set_named_property(
                                    method_name.clone().to_case(Case::Camel).as_str(),
                                    env.create_function_from_closure(
                                        &method_name.clone(),
                                        move |ctx| {
                                            // Get args as script value
                                            let args = ctx
                                                .get_all()
                                                .into_iter()
                                                .map(|elem| js_value_to_script_value(ctx.env, elem))
                                                .try_collect::<Vec<_>>()
                                                .map_err(|e| e.into_napi())?;

                                            // Get the native value wrapped in the javascript object
                                            let wrapped = ctx
                                                .env
                                                .unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;

                                            // Call the function
                                            let result = wrapped
                                                .call_const_method(&method_name, args)
                                                .map_err(|e| e.into_napi())?;

                                            // Returns the result
                                            script_value_to_js_value(ctx.env, result)
                                                .map_err(|e| e.into_napi())
                                        },
                                    )
                                    .map_err(|e| FruityError::from_napi(e))?,
                                )
                                .map_err(|e| FruityError::from_napi(e))?;

                            FruityResult::Ok(())
                        })?;

                    // Define mut method accessors
                    value
                        .get_mut_method_names()?
                        .into_iter()
                        .try_for_each(|method_name| {
                            js_object
                                .set_named_property(
                                    method_name.clone().to_case(Case::Camel).as_str(),
                                    env.create_function_from_closure(
                                        &method_name.clone(),
                                        move |ctx| {
                                            // Get args as script value
                                            let args = ctx
                                                .get_all()
                                                .into_iter()
                                                .map(|elem| js_value_to_script_value(ctx.env, elem))
                                                .try_collect::<Vec<_>>()
                                                .map_err(|e| e.into_napi())?;

                                            // Get the native value wrapped in the javascript object
                                            let wrapped = ctx
                                                .env
                                                .unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;

                                            // Call the function
                                            let result = wrapped
                                                .call_mut_method(&method_name, args)
                                                .map_err(|e| e.into_napi())?;

                                            // Returns the result
                                            script_value_to_js_value(ctx.env, result)
                                                .map_err(|e| e.into_napi())
                                        },
                                    )
                                    .map_err(|e| FruityError::from_napi(e))?,
                                )
                                .map_err(|e| FruityError::from_napi(e))?;

                            FruityResult::Ok(())
                        })?;

                    env.wrap(&mut js_object, value)
                        .map_err(|e| FruityError::from_napi(e))?;
                    js_object.into_unknown()
                }
            }
        }
    })
}

/// Create a script value from a napi js value
pub fn js_value_to_script_value(env: &Env, value: JsUnknown) -> FruityResult<ScriptValue> {
    Ok(
        match value.get_type().map_err(|e| FruityError::from_napi(e))? {
            ValueType::Undefined => ScriptValue::Undefined,
            ValueType::Null => ScriptValue::Null,
            ValueType::Boolean => ScriptValue::Bool(
                value
                    .coerce_to_bool()
                    .map_err(|e| FruityError::from_napi(e))?
                    .get_value()
                    .map_err(|e| FruityError::from_napi(e))?,
            ),
            ValueType::Number => ScriptValue::F64(
                value
                    .coerce_to_number()
                    .map_err(|e| FruityError::from_napi(e))?
                    .get_double()
                    .map_err(|e| FruityError::from_napi(e))?,
            ),
            ValueType::String => ScriptValue::String(
                value
                    .coerce_to_string()
                    .map_err(|e| FruityError::from_napi(e))?
                    .into_utf8()
                    .map_err(|e| FruityError::from_napi(e))?
                    .as_str()
                    .map_err(|e| FruityError::from_napi(e))?
                    .to_string(),
            ),
            ValueType::Symbol => unimplemented!(),
            ValueType::Object => {
                let js_object = unsafe { value.cast::<JsObject>() };

                if js_object
                    .is_array()
                    .map_err(|e| FruityError::from_napi(e))?
                {
                    // First case, the object is a plain javascript array
                    ScriptValue::Array(
                        (0..js_object
                            .get_array_length()
                            .map_err(|e| FruityError::from_napi(e))?)
                            .map(|index| {
                                js_value_to_script_value(
                                    env,
                                    js_object
                                        .get_element(index)
                                        .map_err(|e| FruityError::from_napi(e))?,
                                )
                            })
                            .try_collect::<Vec<_>>()?,
                    )
                } else {
                    match env.unwrap::<Box<dyn ScriptObject>>(&js_object) {
                        Ok(wrapped) => {
                            // Second case, a value is wrapped into the object
                            ScriptValue::Object(wrapped.deref().duplicate()?)
                        }
                        Err(_) => {
                            // Third case, the object is a plain javascript object
                            ScriptValue::Object(Box::new(JsIntrospectObject {
                                reference: JsSharedRef::new(env, js_object)?,
                                env: env.clone(),
                            }))
                        }
                    }
                }
            }
            ValueType::Function => {
                let js_func = JsFunction::try_from(value).map_err(|e| FruityError::from_napi(e))?;

                ScriptValue::Callback(Rc::new(JsFunctionCallback {
                    reference: JsSharedRef::new(env, js_func)?,
                    env: env.clone(),
                }))
            }
            ValueType::External => unimplemented!(),
            ValueType::BigInt => ScriptValue::I64(
                unsafe { value.cast::<JsBigInt>() }
                    .get_i64()
                    .map_err(|e| FruityError::from_napi(e))?
                    .0,
            ),
            ValueType::Unknown => unimplemented!(),
        },
    )
}

struct RefWrapper {
    reference: Ref<()>,
    env: Env,
}

impl Drop for RefWrapper {
    fn drop(&mut self) {
        self.reference.unref(self.env.clone()).unwrap();
    }
}

struct JsSharedRef<T>
where
    T: NapiRaw,
{
    reference: Rc<RefWrapper>,
    env: Env,
    phantom: PhantomData<T>,
}

impl<T> Debug for JsSharedRef<T>
where
    T: NapiRaw,
{
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<T> Clone for JsSharedRef<T>
where
    T: NapiRaw,
{
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
            env: self.env.clone(),
            phantom: Default::default(),
        }
    }
}

impl<T> JsSharedRef<T>
where
    T: NapiRaw + NapiValue,
{
    pub fn new(env: &Env, value: T) -> FruityResult<Self> {
        Ok(Self {
            reference: Rc::new(RefWrapper {
                reference: env
                    .create_reference(value)
                    .map_err(|e| FruityError::from_napi(e))?,
                env: env.clone(),
            }),
            env: env.clone(),
            phantom: Default::default(),
        })
    }

    pub fn inner(&self) -> T {
        self.env
            .get_reference_value::<T>(&self.reference.reference)
            .unwrap()
    }
}

struct JsFunctionCallback {
    reference: JsSharedRef<JsFunction>,
    env: Env,
}

impl ScriptCallback for JsFunctionCallback {
    fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        // Get the js func from the reference
        let js_func = self.reference.inner();

        // Convert all the others args as a JsUnknown
        let args = args
            .into_iter()
            .map(|elem| script_value_to_js_value(&self.env, elem))
            .try_collect::<Vec<_>>()?;

        // Call the function
        let result = js_func
            .call(None, &args)
            .map_err(|e| FruityError::from_napi(e))?;

        // Return the result
        let result = js_value_to_script_value(&self.env, result)?;
        Ok(result)
    }

    fn create_thread_safe_callback(
        &self,
    ) -> FruityResult<Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
        // Get the js func from the reference
        let js_func = self.reference.inner();

        // Create the thread safe function
        let thread_safe_func: ThreadsafeFunction<Vec<ScriptValue>, ErrorStrategy::Fatal> = js_func
            .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Vec<ScriptValue>>| {
                // Convert all the others args as a JsUnknown
                let args = ctx
                    .value
                    .into_iter()
                    .map(|elem| script_value_to_js_value(&ctx.env, elem))
                    .try_collect::<Vec<_>>()
                    .map_err(|e| e.into_napi())?;

                Ok(args)
            })
            .map_err(|e| FruityError::from_napi(e))?;

        // Create the closure to call the function later
        Ok(Arc::new(move |args| {
            // Execute the function
            thread_safe_func.call(args, ThreadsafeFunctionCallMode::Blocking);
        }))
    }
}

/// A structure to store a javascript object that can be stored in a ScriptValue
#[derive(FruityAny, Clone)]
pub struct JsIntrospectObject {
    reference: JsSharedRef<JsObject>,
    env: Env,
}

impl Debug for JsIntrospectObject {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectFields for JsIntrospectObject {
    fn get_class_name(&self) -> FruityResult<String> {
        // Get the js func object the reference
        let js_object = self.reference.inner();

        let constructor: JsFunction = js_object
            .get_named_property("constructor")
            .map_err(|e| FruityError::from_napi(e))?;
        let constructor = constructor
            .coerce_to_object()
            .map_err(|e| FruityError::from_napi(e))?;
        let name: JsString = constructor
            .get_named_property("name")
            .map_err(|e| FruityError::from_napi(e))?;
        Ok(name
            .into_utf8()
            .map_err(|e| FruityError::from_napi(e))?
            .as_str()
            .map_err(|e| FruityError::from_napi(e))?
            .to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        // Get the js func object the reference
        let js_object = self.reference.inner();

        let properties = js_object
            .get_property_names()
            .map_err(|e| FruityError::from_napi(e))?;
        let len = properties
            .get_named_property::<JsNumber>("length")
            .map_err(|e| FruityError::from_napi(e))?
            .get_uint32()
            .map_err(|e| FruityError::from_napi(e))?;

        (0..len)
            .map(|index| {
                let key = properties
                    .get_element::<JsString>(index)
                    .map_err(|e| FruityError::from_napi(e))?;
                let key = key
                    .into_utf8()
                    .map_err(|e| FruityError::from_napi(e))?
                    .as_str()
                    .map_err(|e| FruityError::from_napi(e))?
                    .to_string();

                Ok(key.to_case(Case::Snake))
            })
            .try_collect()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        // Get the js func object the reference
        let mut js_object = self.reference.inner();

        let value = script_value_to_js_value(&self.env, value)?;
        js_object
            .set_named_property(&name.to_case(Case::Camel), value)
            .map_err(|e| FruityError::from_napi(e))?;

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        // Get the js func object the reference
        let js_object = self.reference.inner();

        let value = js_object
            .get_named_property(&name.to_case(Case::Camel))
            .map_err(|e| FruityError::from_napi(e))?;
        js_value_to_script_value(&self.env, value)
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

unsafe extern "C" fn generic_getter(
    raw_env: napi_env,
    callback_info: napi_sys::napi_callback_info,
) -> napi_value {
    unsafe fn generic_getter(
        raw_env: napi_env,
        callback_info: napi_sys::napi_callback_info,
    ) -> napi::Result<napi_value> {
        // Get the field name passed as data
        let field_name = {
            let mut this = std::ptr::null_mut();
            let mut args = [std::ptr::null_mut(); 1];
            let mut argc = 1;
            let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

            check_status!(napi_sys::napi_get_cb_info(
                raw_env,
                callback_info,
                &mut argc,
                args.as_mut_ptr(),
                &mut this,
                &mut data_ptr,
            ))?;

            let data_ptr = std::ffi::CStr::from_ptr(data_ptr as *mut std::ffi::c_char);
            data_ptr.to_str().unwrap().to_string()
        };

        // Initialize javascript utils
        let env = Env::from_raw(raw_env);
        let callback_info = CallbackInfo::<3>::new(raw_env, callback_info, None)?;

        // Get the wrapped object
        let this = JsObject::from_raw(raw_env, callback_info.this())?;
        let wrapped = env.unwrap::<Box<dyn ScriptObject>>(&this)?;

        // Execute the getter
        let result = wrapped
            .get_field_value(&field_name)
            .map_err(|e| e.into_napi())?;

        // Returns the result
        let result = script_value_to_js_value(&env, result).map_err(|e| e.into_napi())?;
        Ok(result.raw())
    }

    generic_getter(raw_env, callback_info).unwrap_or_else(|e| {
        unsafe { JsError::from(e).throw_into(raw_env) };
        std::ptr::null_mut()
    })
}

unsafe extern "C" fn generic_setter(
    raw_env: napi_env,
    callback_info: napi_sys::napi_callback_info,
) -> napi_value {
    unsafe fn generic_setter(
        raw_env: napi_env,
        callback_info: napi_sys::napi_callback_info,
    ) -> napi::Result<napi_value> {
        // Get the field name passed as data
        let field_name = {
            let mut this = std::ptr::null_mut();
            let mut args = [std::ptr::null_mut(); 1];
            let mut argc = 1;
            let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

            check_status!(napi_sys::napi_get_cb_info(
                raw_env,
                callback_info,
                &mut argc,
                args.as_mut_ptr(),
                &mut this,
                &mut data_ptr,
            ))?;

            let data_ptr = std::ffi::CStr::from_ptr(data_ptr as *mut std::ffi::c_char);
            data_ptr.to_str().unwrap().to_string()
        };

        // Initialize javascript utils
        let env = Env::from_raw(raw_env);
        let callback_info = CallbackInfo::<3>::new(raw_env, callback_info, None)?;

        // Get the wrapped object
        let this = JsObject::from_raw(raw_env, callback_info.this())?;
        let wrapped = env.unwrap::<Box<dyn ScriptObject>>(&this)?;

        // Execute the setter
        let arg = JsUnknown::from_raw(raw_env, callback_info.get_arg(0))?;
        let arg = js_value_to_script_value(&env, arg).map_err(|e| e.into_napi())?;
        wrapped
            .set_field_value(&field_name, arg)
            .map_err(|e| e.into_napi())?;

        // Returns the result
        let result = env.get_undefined()?;
        Ok(result.raw())
    }

    generic_setter(raw_env, callback_info).unwrap_or_else(|e| {
        unsafe { JsError::from(e).throw_into(raw_env) };
        std::ptr::null_mut()
    })
}

impl FruityError {
    /// Convert a js error to a fruity_game_engine error
    pub fn from_napi(err: napi::Error) -> Self {
        match err.status {
            napi::Status::Ok => FruityError::Ok(err.reason.to_string()),
            napi::Status::InvalidArg => FruityError::InvalidArg(err.reason.to_string()),
            napi::Status::ObjectExpected => FruityError::ObjectExpected(err.reason.to_string()),
            napi::Status::StringExpected => FruityError::StringExpected(err.reason.to_string()),
            napi::Status::NameExpected => FruityError::NameExpected(err.reason.to_string()),
            napi::Status::FunctionExpected => FruityError::FunctionExpected(err.reason.to_string()),
            napi::Status::NumberExpected => FruityError::NumberExpected(err.reason.to_string()),
            napi::Status::BooleanExpected => FruityError::BooleanExpected(err.reason.to_string()),
            napi::Status::ArrayExpected => FruityError::ArrayExpected(err.reason.to_string()),
            napi::Status::GenericFailure => FruityError::GenericFailure(err.reason.to_string()),
            napi::Status::PendingException => FruityError::PendingException(err.reason.to_string()),
            napi::Status::Cancelled => FruityError::Cancelled(err.reason.to_string()),
            napi::Status::EscapeCalledTwice => {
                FruityError::EscapeCalledTwice(err.reason.to_string())
            }
            napi::Status::HandleScopeMismatch => {
                FruityError::HandleScopeMismatch(err.reason.to_string())
            }
            napi::Status::CallbackScopeMismatch => {
                FruityError::CallbackScopeMismatch(err.reason.to_string())
            }
            napi::Status::QueueFull => FruityError::QueueFull(err.reason.to_string()),
            napi::Status::Closing => FruityError::Closing(err.reason.to_string()),
            napi::Status::BigintExpected => FruityError::BigintExpected(err.reason.to_string()),
            napi::Status::DateExpected => FruityError::DateExpected(err.reason.to_string()),
            napi::Status::ArrayBufferExpected => {
                FruityError::ArrayBufferExpected(err.reason.to_string())
            }
            napi::Status::DetachableArraybufferExpected => {
                FruityError::DetachableArraybufferExpected(err.reason.to_string())
            }
            napi::Status::WouldDeadlock => FruityError::WouldDeadlock(err.reason.to_string()),
            napi::Status::Unknown => FruityError::Unknown(err.reason.to_string()),
        }
    }

    /// Convert a fruity_game_engine error to a js error
    pub fn into_napi(self) -> napi::Error {
        match self {
            FruityError::Ok(message) => napi::Error::new(napi::Status::Ok, message),
            FruityError::InvalidArg(message) => napi::Error::new(napi::Status::InvalidArg, message),
            FruityError::ObjectExpected(message) => {
                napi::Error::new(napi::Status::ObjectExpected, message)
            }
            FruityError::StringExpected(message) => {
                napi::Error::new(napi::Status::StringExpected, message)
            }
            FruityError::NameExpected(message) => {
                napi::Error::new(napi::Status::NameExpected, message)
            }
            FruityError::FunctionExpected(message) => {
                napi::Error::new(napi::Status::FunctionExpected, message)
            }
            FruityError::NumberExpected(message) => {
                napi::Error::new(napi::Status::NumberExpected, message)
            }
            FruityError::BooleanExpected(message) => {
                napi::Error::new(napi::Status::BooleanExpected, message)
            }
            FruityError::ArrayExpected(message) => {
                napi::Error::new(napi::Status::ArrayExpected, message)
            }
            FruityError::GenericFailure(message) => {
                napi::Error::new(napi::Status::GenericFailure, message)
            }
            FruityError::PendingException(message) => {
                napi::Error::new(napi::Status::PendingException, message)
            }
            FruityError::Cancelled(message) => napi::Error::new(napi::Status::Cancelled, message),
            FruityError::EscapeCalledTwice(message) => {
                napi::Error::new(napi::Status::EscapeCalledTwice, message)
            }
            FruityError::HandleScopeMismatch(message) => {
                napi::Error::new(napi::Status::HandleScopeMismatch, message)
            }
            FruityError::CallbackScopeMismatch(message) => {
                napi::Error::new(napi::Status::CallbackScopeMismatch, message)
            }
            FruityError::QueueFull(message) => napi::Error::new(napi::Status::QueueFull, message),
            FruityError::Closing(message) => napi::Error::new(napi::Status::Closing, message),
            FruityError::BigintExpected(message) => {
                napi::Error::new(napi::Status::BigintExpected, message)
            }
            FruityError::DateExpected(message) => {
                napi::Error::new(napi::Status::DateExpected, message)
            }
            FruityError::ArrayBufferExpected(message) => {
                napi::Error::new(napi::Status::ArrayBufferExpected, message)
            }
            FruityError::DetachableArraybufferExpected(message) => {
                napi::Error::new(napi::Status::DetachableArraybufferExpected, message)
            }
            FruityError::WouldDeadlock(message) => {
                napi::Error::new(napi::Status::WouldDeadlock, message)
            }
            FruityError::Unknown(message) => napi::Error::new(napi::Status::Unknown, message),
        }
    }
}
