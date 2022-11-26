use crate::{
    introspect::IntrospectObject,
    script_value::convert::TryIntoScriptValue,
    script_value::{ScriptCallback, ScriptObject, ScriptValue},
    FruityResult,
};
use convert_case::{Case, Casing};
use fruity_game_engine_macro::FruityAny;
use napi::{
    threadsafe_function::{
        ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
    },
    Env, JsBigInt, JsFunction, JsNumber, JsObject, JsString, JsUnknown, Ref, Result, ValueType,
};
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
            let value_2 = value.duplicate();

            if let Ok(value) = value.as_any_box().downcast::<JsIntrospectObject>() {
                // First case, it's a native js value
                value.inner.into_unknown()
            } else {
                // Second case, we wrap the object into a js object
                let mut js_object = env.create_object()?;

                // Define const method accessors
                value_2
                    .get_const_method_names()?
                    .into_iter()
                    .try_for_each(|method_name| {
                        js_object.set_named_property(
                            method_name.clone().to_case(Case::Camel).as_str(),
                            env.create_function_from_closure(&method_name.clone(), move |ctx| {
                                // Get args as script value
                                let args = ctx
                                    .get_all()
                                    .into_iter()
                                    .map(|elem| js_value_to_script_value(ctx.env, elem))
                                    .try_collect::<Vec<_>>()?;

                                // Get the native value wrapped in the javascript object
                                let wrapped =
                                    ctx.env.unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;

                                // Call the function
                                let result = wrapped.call_const_method(&method_name, args)?;

                                // Returns the result
                                script_value_to_js_value(ctx.env, result)
                            })?,
                        )?;

                        Result::Ok(())
                    })?;

                // Define mut method accessors
                value_2
                    .get_mut_method_names()?
                    .into_iter()
                    .try_for_each(|method_name| {
                        js_object.set_named_property(
                            method_name.clone().to_case(Case::Camel).as_str(),
                            env.create_function_from_closure(&method_name.clone(), move |ctx| {
                                // Get args as script value
                                let args = ctx
                                    .get_all()
                                    .into_iter()
                                    .map(|elem| js_value_to_script_value(ctx.env, elem))
                                    .try_collect::<Vec<_>>()?;

                                // Get the native value wrapped in the javascript object
                                let wrapped =
                                    ctx.env.unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;

                                // Call the function
                                let result = wrapped.call_mut_method(&method_name, args)?;

                                // Returns the result
                                script_value_to_js_value(ctx.env, result)
                            })?,
                        )?;

                        Result::Ok(())
                    })?;

                env.wrap(&mut js_object, value_2)?;
                js_object.into_unknown()
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
        ValueType::Symbol => todo!(),
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
                        ScriptValue::Object(wrapped.duplicate())
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
        ValueType::External => todo!(),
        ValueType::BigInt => ScriptValue::I64(unsafe { value.cast::<JsBigInt>() }.get_i64()?.0),
        ValueType::Unknown => todo!(),
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

#[derive(FruityAny)]
struct JsIntrospectObject {
    inner: JsObject,
    env: Env,
}

impl Clone for JsIntrospectObject {
    fn clone(&self) -> Self {
        let mut inner = self.env.create_object().unwrap();

        let properties = self.inner.get_property_names().unwrap();
        let len = properties
            .get_named_property::<JsNumber>("length")
            .unwrap()
            .get_uint32()
            .unwrap();

        (0..len)
            .try_for_each(|index| {
                let key = properties.get_element::<JsString>(index)?;
                inner.set_property(
                    key,
                    self.inner
                        .get_property::<JsString, JsUnknown>(key.clone())?,
                )
            })
            .unwrap();

        Self {
            inner,
            env: self.env.clone(),
        }
    }
}

impl Debug for JsIntrospectObject {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectObject for JsIntrospectObject {
    fn get_class_name(&self) -> FruityResult<String> {
        Ok("js_unknown".to_string())

        // TODO: Get the class name from prototype
        /*let constructor: JsObject = self.inner.get_named_property("constructor")?;
        let name: JsString = constructor.get_named_property("name")?;
        Ok(name.into_utf8()?.as_str()?.to_string())*/
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

/*unsafe extern "C" fn generic_getter(
    raw_env: napi_env,
    callback_info: napi_callback_info,
) -> napi_value {
    // Initialize javascript utils
    let env = Env::from_raw(raw_env);
    let callback_info = CallbackInfo::new(raw_env, callback_info, None).unwrap();

    // Get the field info wrapped in the callback info
    let field_info: &JsObject = callback_info.unwrap_borrow().unwrap();
    let field_info = JsObject::from_raw(raw_env, *field_info).unwrap();
    let mut field_info = env.unwrap::<FieldInfo>(&field_info).unwrap();

    // Get the native value wrapped in the javascript object
    let this = JsObject::from_raw(raw_env, callback_info.this()).unwrap();
    let mut wrapped = env.unwrap::<Box<dyn ScriptObject>>(&this).unwrap();

    // Execute the getter
    let result = (field_info.getter)(wrapped.as_any_ref());

    // Returns the result
    let result = script_value_to_js_value(&env, result).unwrap();
    result.raw()
}*/
