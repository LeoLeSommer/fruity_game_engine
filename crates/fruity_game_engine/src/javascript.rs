use crate::{
    introspect::IntrospectObject,
    script_value::convert::TryIntoScriptValue,
    script_value::ScriptObject,
    script_value::{ScriptCallback, ScriptValue},
    FruityResult,
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
    Result, ValueType,
};
use napi::{check_status, NapiValue};
use napi::{JsError, NapiRaw};
use std::{ffi::CString, ops::Deref};
use std::{fmt::Debug, vec};
use std::{rc::Rc, sync::Arc};

/// Tool to export javascript modules
pub struct ExportJavascript {
    exports: JsObject,
    env: Env,
}

impl ExportJavascript {
    /// Returns an ExportJavascript
    pub fn new(exports: JsObject, env: Env) -> Self {
        Self { exports, env }
    }

    /// Register a class type
    pub fn export_constructor<T>(&mut self, name: &str, value: T) -> Result<()>
    where
        T: TryIntoScriptValue,
    {
        let js_value = script_value_to_js_value(&self.env, value.into_script_value()?)?;
        self.exports.set_named_property(&name, js_value)?;

        Ok(())
    }

    /// Register a class type
    pub fn export_value<T>(&mut self, name: &str, value: T) -> Result<()>
    where
        T: TryIntoScriptValue,
    {
        let js_value = script_value_to_js_value(&self.env, value.into_script_value()?)?;
        self.exports
            .set_named_property(&name.to_case(Case::Camel), js_value)?;

        Ok(())
    }
}

/// Create a napi js value from a script value
pub fn script_value_to_js_value(env: &Env, value: ScriptValue) -> Result<JsUnknown> {
    Ok(match value.into_script_value()? {
        ScriptValue::I8(value) => env.create_int32(value as i32)?.into_unknown(),
        ScriptValue::I16(value) => env.create_int32(value as i32)?.into_unknown(),
        ScriptValue::I32(value) => env.create_int32(value)?.into_unknown(),
        ScriptValue::I64(value) => env.create_int64(value)?.into_unknown(),
        ScriptValue::ISize(value) => env.create_int32(value as i32)?.into_unknown(),
        ScriptValue::U8(value) => env.create_uint32(value as u32)?.into_unknown(),
        ScriptValue::U16(value) => env.create_uint32(value as u32)?.into_unknown(),
        ScriptValue::U32(value) => env.create_uint32(value)?.into_unknown(),
        ScriptValue::U64(value) => env.create_bigint_from_u64(value)?.into_unknown()?,
        ScriptValue::USize(value) => env.create_uint32(value as u32)?.into_unknown(),
        ScriptValue::F32(value) => env.create_double(value as f64)?.into_unknown(),
        ScriptValue::F64(value) => env.create_double(value as f64)?.into_unknown(),
        ScriptValue::Bool(value) => env.get_boolean(value)?.into_unknown(),
        ScriptValue::String(value) => env.create_string(&value)?.into_unknown(),
        ScriptValue::Array(value) => {
            let mut js_array = env.create_empty_array()?;

            for (index, elem) in value.into_iter().enumerate() {
                js_array.set_element(index as u32, script_value_to_js_value(env, elem)?)?;
            }

            js_array.into_unknown()
        }
        ScriptValue::Null => env.get_null()?.into_unknown(),
        ScriptValue::Undefined => env.get_undefined()?.into_unknown(),
        ScriptValue::Iterator(_value) => {
            todo!()
        }
        ScriptValue::Callback(callback) => env
            .create_function_from_closure("unknown", move |ctx| {
                let args = ctx
                    .get_all()
                    .into_iter()
                    .map(|elem| js_value_to_script_value(ctx.env, elem))
                    .try_collect::<Vec<_>>()?;

                let result = callback.call(args)?;
                script_value_to_js_value(ctx.env, result)
            })?
            .into_unknown(),
        ScriptValue::Object(value) => {
            match value.downcast::<JsIntrospectObject>() {
                // First case, it's a native js value
                Ok(value) => value.inner.into_unknown(),
                // Second case, we wrap the object into a js object
                Err(value) => {
                    let mut js_object = env.create_object()?;

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

                    js_object.add_finalizer((), (), |_| {
                        std::mem::drop(field_names);
                    })?;

                    unsafe {
                        check_status!(napi_sys::napi_define_properties(
                            env.raw(),
                            js_object.raw(),
                            properties.len(),
                            properties.as_ptr(),
                        ))?;
                    }

                    // Define const method accessors
                    value
                        .get_const_method_names()?
                        .into_iter()
                        .try_for_each(|method_name| {
                            js_object.set_named_property(
                                method_name.clone().to_case(Case::Camel).as_str(),
                                env.create_function_from_closure(
                                    &method_name.clone(),
                                    move |ctx| {
                                        // Get args as script value
                                        let args = ctx
                                            .get_all()
                                            .into_iter()
                                            .map(|elem| js_value_to_script_value(ctx.env, elem))
                                            .try_collect::<Vec<_>>()?;

                                        // Get the native value wrapped in the javascript object
                                        let wrapped = ctx
                                            .env
                                            .unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;

                                        // Call the function
                                        let result =
                                            wrapped.call_const_method(&method_name, args)?;

                                        // Returns the result
                                        script_value_to_js_value(ctx.env, result)
                                    },
                                )?,
                            )?;

                            Result::Ok(())
                        })?;

                    // Define mut method accessors
                    value
                        .get_mut_method_names()?
                        .into_iter()
                        .try_for_each(|method_name| {
                            js_object.set_named_property(
                                method_name.clone().to_case(Case::Camel).as_str(),
                                env.create_function_from_closure(
                                    &method_name.clone(),
                                    move |ctx| {
                                        // Get args as script value
                                        let args = ctx
                                            .get_all()
                                            .into_iter()
                                            .map(|elem| js_value_to_script_value(ctx.env, elem))
                                            .try_collect::<Vec<_>>()?;

                                        // Get the native value wrapped in the javascript object
                                        let wrapped = ctx
                                            .env
                                            .unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;

                                        // Call the function
                                        let result = wrapped.call_mut_method(&method_name, args)?;

                                        // Returns the result
                                        script_value_to_js_value(ctx.env, result)
                                    },
                                )?,
                            )?;

                            Result::Ok(())
                        })?;

                    env.wrap(&mut js_object, value)?;
                    js_object.into_unknown()
                }
            }
        }
    })
}

/// Create a script value from a napi js value
pub fn js_value_to_script_value(env: &Env, value: JsUnknown) -> Result<ScriptValue> {
    Ok(match value.get_type()? {
        ValueType::Undefined => ScriptValue::Undefined,
        ValueType::Null => ScriptValue::Null,
        ValueType::Boolean => ScriptValue::Bool(value.coerce_to_bool()?.get_value()?),
        ValueType::Number => ScriptValue::F64(value.coerce_to_number()?.get_double()?),
        ValueType::String => {
            ScriptValue::String(value.coerce_to_string()?.into_utf8()?.as_str()?.to_string())
        }
        ValueType::Symbol => unimplemented!(),
        ValueType::Object => {
            let js_object = unsafe { value.cast::<JsObject>() };

            if js_object.is_array()? {
                // First case, the object is a plain javascript array
                ScriptValue::Array(
                    (0..js_object.get_array_length()?)
                        .map(|index| js_value_to_script_value(env, js_object.get_element(index)?))
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
                            inner: js_object,
                            env: env.clone(),
                        }))
                    }
                }
            }
        }
        ValueType::Function => {
            let js_func = JsFunction::try_from(value)?;

            ScriptValue::Callback(Rc::new(JsFunctionCallback {
                reference: env.create_reference(js_func)?,
                env: env.clone(),
            }))
        }
        ValueType::External => unimplemented!(),
        ValueType::BigInt => ScriptValue::I64(unsafe { value.cast::<JsBigInt>() }.get_i64()?.0),
        ValueType::Unknown => unimplemented!(),
    })
}

struct JsFunctionCallback {
    pub reference: Ref<()>,
    pub env: Env,
}

impl ScriptCallback for JsFunctionCallback {
    fn call(&self, args: Vec<ScriptValue>) -> Result<ScriptValue> {
        // Get the js func from the reference
        let js_func = self
            .env
            .get_reference_value::<JsFunction>(&self.reference)?;

        // Convert all the others args as a JsUnknown
        let args = args
            .into_iter()
            .map(|elem| script_value_to_js_value(&self.env, elem))
            .try_collect::<Vec<_>>()?;

        // Call the function
        let result = js_func.call(None, &args)?;

        // Return the result
        let result = js_value_to_script_value(&self.env, result)?;
        Ok(result)
    }

    fn create_thread_safe_callback(&self) -> Result<Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
        // Get the js func from the reference
        let js_func = self
            .env
            .get_reference_value::<JsFunction>(&self.reference)?;

        // Create the thread safe function
        let thread_safe_func: ThreadsafeFunction<Vec<ScriptValue>, ErrorStrategy::Fatal> = js_func
            .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Vec<ScriptValue>>| {
                // Convert all the others args as a JsUnknown
                let args = ctx
                    .value
                    .into_iter()
                    .map(|elem| script_value_to_js_value(&ctx.env, elem))
                    .try_collect::<Vec<_>>()?;

                Ok(args)
            })?;

        // Create the closure to call the function later
        Ok(Arc::new(move |args| {
            // Execute the function
            thread_safe_func.call(args, ThreadsafeFunctionCallMode::Blocking);
        }))
    }
}

impl Drop for JsFunctionCallback {
    fn drop(&mut self) {
        self.reference.unref(self.env.clone()).unwrap();
    }
}

/// A structure to store a javascript object that can be stored in a ScriptValue
#[derive(FruityAny)]
pub struct JsIntrospectObject {
    pub reference: Ref<()>,
    env: Env,
}

impl Debug for JsIntrospectObject {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl ScriptObject for JsIntrospectObject {
    fn duplicate(&self) -> FruityResult<Box<dyn ScriptObject>> {
        let mut new_js_object = self.env.create_object()?;

        let properties = self.inner.get_property_names()?;
        let len = properties
            .get_named_property::<JsNumber>("length")?
            .get_uint32()?;

        for index in 0..len {
            let name: JsString = properties.get_element(index)?;
            let name = name.into_utf8()?.as_str()?.to_string();

            let value: JsUnknown = self.inner.get_named_property(&name.to_case(Case::Camel))?;
            new_js_object.set_named_property(&name, value)?;
        }

        Ok(Box::new(Self {
            inner: new_js_object,
            env: self.env.clone(),
        }))
    }
}

impl IntrospectObject for JsIntrospectObject {
    fn get_class_name(&self) -> FruityResult<String> {
        // Ok("js_unknown".to_string())

        // TODO: Get the class name from prototype
        let constructor: JsObject = self.inner.get_named_property("constructor")?;
        let name: JsString = constructor.get_named_property("name")?;
        Ok(name.into_utf8()?.as_str()?.to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        let properties = self.inner.get_property_names()?;
        let len = properties
            .get_named_property::<JsNumber>("length")?
            .get_uint32()?;

        (0..len)
            .map(|index| {
                let key = properties.get_element::<JsString>(index)?;
                let key = key.into_utf8()?.as_str()?.to_string();

                Ok(key.to_case(Case::Snake))
            })
            .try_collect()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        let value = script_value_to_js_value(&self.env, value)?;
        self.inner
            .set_named_property(&name.to_case(Case::Camel), value)?;

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        let value = self.inner.get_named_property(&name.to_case(Case::Camel))?;
        js_value_to_script_value(&self.env, value)
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

unsafe extern "C" fn generic_getter(
    raw_env: napi_env,
    callback_info: napi_sys::napi_callback_info,
) -> napi_value {
    unsafe fn generic_getter(
        raw_env: napi_env,
        callback_info: napi_sys::napi_callback_info,
    ) -> Result<napi_value> {
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
        let result = wrapped.get_field_value(&field_name)?;

        // Returns the result
        let result = script_value_to_js_value(&env, result)?;
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
    ) -> Result<napi_value> {
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
        let arg = js_value_to_script_value(&env, arg)?;
        wrapped.set_field_value(&field_name, arg)?;

        // Returns the result
        let result = env.get_undefined()?;
        Ok(result.raw())
    }

    generic_setter(raw_env, callback_info).unwrap_or_else(|e| {
        unsafe { JsError::from(e).throw_into(raw_env) };
        std::ptr::null_mut()
    })
}
