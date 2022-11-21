use crate::{
  convert::FruityInto,
  introspect::MethodCaller,
  script_value::{IntrospectObjectClone, ScriptValue},
};
use napi::{Env, JsBigInt, JsFunction, JsNumber, JsObject, JsString, JsUnknown, Result, ValueType};
use std::{collections::HashMap, ops::DerefMut, rc::Rc};

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
  pub fn export_value<T>(&mut self, name: &str, value: T) -> Result<()>
  where
    T: FruityInto<ScriptValue>,
  {
    let js_value = script_value_to_js_value(&self.env, value.fruity_into()?)?;
    self.exports.set_named_property(name, js_value)?;

    Ok(())
  }
}

/// Create a napi js value from a script value
pub fn script_value_to_js_value(env: &Env, value: ScriptValue) -> Result<JsUnknown> {
  Ok(match value.fruity_into()? {
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

        let result = callback(args)?;
        script_value_to_js_value(ctx.env, result)
      })?
      .into_unknown(),
    ScriptValue::Object { fields, .. } => {
      let mut js_object = env.create_object()?;

      for (key, elem) in fields.into_iter() {
        js_object.set(key, script_value_to_js_value(env, elem)?)?;
      }

      js_object.into_unknown()
    }
    ScriptValue::NativeObject(value) => {
      let mut js_object = env.create_object()?;

      // Define method accessors
      value
        .get_method_infos()
        .into_iter()
        .try_for_each(|method_info| {
          js_object.set_named_property(
            method_info.name.clone().as_str(),
            env.create_function_from_closure("unknown", move |ctx| {
              // Get args as script value
              let args = ctx
                .get_all()
                .into_iter()
                .map(|elem| js_value_to_script_value(ctx.env, elem))
                .try_collect::<Vec<_>>()?;

              // Get the native value wrapped in the javascript object
              let wrapped = ctx
                .env
                .unwrap::<Box<dyn IntrospectObjectClone>>(&ctx.this()?)?;

              // Call the function
              let result = match &method_info.call {
                MethodCaller::Const(call) => call(wrapped.deref_mut().as_fruity_any_ref(), args),
                MethodCaller::Mut(call) => call(wrapped.deref_mut().as_fruity_any_mut(), args),
              }?;

              // Returns the result
              script_value_to_js_value(ctx.env, result)
            })?,
          )?;

          Result::Ok(())
        })?;

      env.wrap(&mut js_object, value)?;
      js_object.into_unknown()
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
        match env.unwrap::<Box<dyn IntrospectObjectClone>>(&js_object) {
          Ok(wrapped) => {
            // Second case, a value is wrapped into the object
            ScriptValue::NativeObject(wrapped.duplicate())
          }
          Err(_) => {
            // Third case, the object is a plain javascript object
            let mut fields: HashMap<String, ScriptValue> = HashMap::new();

            let properties = js_object.get_property_names()?;
            let len = properties
              .get_named_property::<JsNumber>("length")?
              .get_uint32()?;
            for index in 0..len {
              let key = properties.get_element::<JsString>(index)?;
              let key_string = key.into_utf8()?.as_str()?.to_string();

              let js_value: JsUnknown = js_object.get_property(key)?;

              fields.insert(key_string, js_value_to_script_value(env, js_value)?);
            }

            ScriptValue::Object {
              class_name: "unknown".to_string(),
              fields: fields,
            }
          }
        }
      }
    }
    ValueType::Function => {
      let js_func = unsafe { value.cast::<JsFunction>() };

      let env = env.clone();
      ScriptValue::Callback(Rc::new(move |args| {
        // Convert all the others args as a JsUnknown
        let args = args
          .into_iter()
          .map(|elem| script_value_to_js_value(&env, elem))
          .try_collect::<Vec<_>>()?;

        // Execute the function
        let result = js_func.call(None, &args)?;

        // Returns the result
        let result = js_value_to_script_value(&env, result)?;
        Ok(result)
      }))
    }
    ValueType::External => todo!(),
    ValueType::BigInt => ScriptValue::I64(unsafe { value.cast::<JsBigInt>() }.get_i64()?.0),
    ValueType::Unknown => todo!(),
  })
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
    let mut wrapped = env.unwrap::<Box<dyn IntrospectObjectClone>>(&this).unwrap();

    // Execute the getter
    let result = (field_info.getter)(wrapped.as_any_ref());

    // Returns the result
    let result = script_value_to_js_value(&env, result).unwrap();
    result.raw()
}*/
