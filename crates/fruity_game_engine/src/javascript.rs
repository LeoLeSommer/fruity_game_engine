use crate::{
  any::FruityAny,
  any_value::{AnyValue, IntrospectObjectClone},
  convert::{FruityFrom, FruityInto},
  introspect::MethodCaller,
  FruityError, FruityStatus,
};
use lazy_static::__Deref;
use napi::{
  bindgen_prelude::{External, FromNapiValue, ToNapiValue},
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  Env, JsBigInt, JsFunction, JsNumber, JsObject, JsString, JsUnknown, NapiRaw, NapiValue, Result,
  ValueType,
};
use std::{
  collections::HashMap,
  ops::DerefMut,
  sync::{
    mpsc::{channel, Receiver, Sender},
    Arc,
  },
};

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
    T: FruityInto<AnyValue>,
  {
    let js_value = any_value_to_js_value(&self.env, value.fruity_into()?)?;
    self.exports.set_named_property(name, js_value)?;

    Ok(())
  }
}

/// Create a napi js value from an any value
pub fn any_value_to_js_value(env: &Env, value: AnyValue) -> Result<JsUnknown> {
  Ok(match value.fruity_into()? {
    AnyValue::I8(value) => env.create_int32(value as i32)?.into_unknown(),
    AnyValue::I16(value) => env.create_int32(value as i32)?.into_unknown(),
    AnyValue::I32(value) => env.create_int32(value)?.into_unknown(),
    AnyValue::I64(value) => env.create_int64(value)?.into_unknown(),
    AnyValue::ISize(value) => env.create_int32(value as i32)?.into_unknown(),
    AnyValue::U8(value) => env.create_uint32(value as u32)?.into_unknown(),
    AnyValue::U16(value) => env.create_uint32(value as u32)?.into_unknown(),
    AnyValue::U32(value) => env.create_uint32(value)?.into_unknown(),
    AnyValue::U64(value) => env.create_bigint_from_u64(value)?.into_unknown()?,
    AnyValue::USize(value) => env.create_uint32(value as u32)?.into_unknown(),
    AnyValue::F32(value) => env.create_double(value as f64)?.into_unknown(),
    AnyValue::F64(value) => env.create_double(value as f64)?.into_unknown(),
    AnyValue::Bool(value) => env.get_boolean(value)?.into_unknown(),
    AnyValue::String(value) => env.create_string(&value)?.into_unknown(),
    AnyValue::Array(value) => {
      let mut js_array = env.create_empty_array()?;

      for (index, elem) in value.into_iter().enumerate() {
        js_array.set_element(index as u32, any_value_to_js_value(env, elem)?)?;
      }

      js_array.into_unknown()
    }
    AnyValue::Null => env.get_null()?.into_unknown(),
    AnyValue::Undefined => env.get_undefined()?.into_unknown(),
    AnyValue::Iterator(_value) => {
      todo!()
    }
    AnyValue::Callback(callback) => env
      .create_function_from_closure("unknown", move |ctx| {
        let args = ctx
          .get_all()
          .into_iter()
          .map(|elem| js_value_to_any_value(ctx.env, elem))
          .try_collect::<Vec<_>>()?;

        let result = callback(args)?;
        any_value_to_js_value(ctx.env, result)
      })?
      .into_unknown(),
    AnyValue::Object { fields, .. } => {
      let mut js_object = env.create_object()?;

      for (key, elem) in fields.into_iter() {
        js_object.set(key, any_value_to_js_value(env, elem)?)?;
      }

      js_object.into_unknown()
    }
    AnyValue::NativeObject(value) => {
      let mut js_object = env.create_object()?;

      // Define property accessors
      /*let properties = value
      .get_field_infos()
      .into_iter()
      .map(|field_info| {
          let property = Property::new(field_info.name.as_str()).unwrap();

          // Store the field infos into the property
          let mut js_field_object = env.create_object().unwrap();
          env.wrap(&mut js_field_object, field_info).unwrap();
          property.with_value(&js_field_object);

          property.with_getter(generic_getter);

          property
      })
      .collect::<Vec<_>>();*/

      // Define method accessors
      value
        .get_method_infos()
        .into_iter()
        .try_for_each(|method_info| {
          js_object.set_named_property(
            method_info.name.clone().as_str(),
            env.create_function_from_closure("unknown", move |ctx| {
              // Get args as any value
              let args = ctx
                .get_all()
                .into_iter()
                .map(|elem| js_value_to_any_value(ctx.env, elem))
                .try_collect::<Vec<_>>()?;

              // Get the native value wrapped in the javascript object
              let wrapped = ctx
                .env
                .unwrap::<Box<dyn IntrospectObjectClone>>(&ctx.this()?)?;

              // Call the function
              let result = match &method_info.call {
                MethodCaller::Const(call) => call(wrapped.deref().as_any_ref(), args),
                MethodCaller::Mut(call) => call(wrapped.deref_mut().as_any_mut(), args),
              }?;

              // Returns the result
              any_value_to_js_value(ctx.env, result)
            })?,
          )?;

          Result::Ok(())
        })?;

      env.wrap(&mut js_object, value)?;
      js_object.into_unknown()
    }
  })
}

/// Create an any value from a napi js value
pub fn js_value_to_any_value(env: &Env, value: JsUnknown) -> Result<AnyValue> {
  Ok(match value.get_type()? {
    ValueType::Undefined => AnyValue::Undefined,
    ValueType::Null => AnyValue::Null,
    ValueType::Boolean => AnyValue::Bool(value.coerce_to_bool()?.get_value()?),
    ValueType::Number => AnyValue::F64(value.coerce_to_number()?.get_double()?),
    ValueType::String => {
      AnyValue::String(value.coerce_to_string()?.into_utf8()?.as_str()?.to_string())
    }
    ValueType::Symbol => todo!(),
    ValueType::Object => {
      let js_object = unsafe { value.cast::<JsObject>() };

      if js_object.is_array()? {
        // First case, the object is a plain javascript array
        AnyValue::Array(
          (0..js_object.get_array_length()?)
            .map(|index| js_value_to_any_value(env, js_object.get_element(index)?))
            .try_collect::<Vec<_>>()?,
        )
      } else {
        match env.unwrap::<Box<dyn IntrospectObjectClone>>(&js_object) {
          Ok(wrapped) => {
            // Second case, a value is wrapped into the object
            AnyValue::NativeObject(wrapped.duplicate())
          }
          Err(_) => {
            // Third case, the object is a plain javascript object
            let mut fields: HashMap<String, AnyValue> = HashMap::new();

            let properties = js_object.get_property_names()?;
            let len = properties
              .get_named_property::<JsNumber>("length")?
              .get_uint32()?;
            for index in 0..len {
              let key = properties.get_element::<JsString>(index)?;
              let key_string = key.into_utf8()?.as_str()?.to_string();

              let js_value: JsUnknown = js_object.get_property(key)?;

              fields.insert(key_string, js_value_to_any_value(env, js_value)?);
            }

            AnyValue::Object {
              class_name: "unknown".to_string(),
              fields: fields,
            }
          }
        }
      }
    }
    ValueType::Function => {
      let js_func = unsafe { value.cast::<JsFunction>() };

      // We wrap the function into a function that use a channel sender to return the callback result
      let js_channel_func = env.create_function_from_closure("unknown", move |ctx| {
        let mut args = ctx.get_all();

        // Get the channel send that is used to return the callback result
        let channel_arg = args.remove(0);
        let channel_arg = unsafe {
          <External<Sender<AnyValue>>>::from_napi_value(ctx.env.raw(), channel_arg.raw())?
        };

        // Execute the function
        let result = js_func.call(None, &args)?;

        // Returns the result trough the channel
        let result = js_value_to_any_value(&ctx.env, result)?;
        channel_arg.send(result).map_err(|_| {
          FruityError::new(
            FruityStatus::CallbackScopeMismatch,
            format!("Channel error while executing a javascript callback"),
          )
        })?;

        ctx.env.get_undefined()
      })?;

      // TODO: think about a good number for max queue size
      let ts_func: ThreadsafeFunction<(Sender<AnyValue>, Vec<AnyValue>), ErrorStrategy::Fatal> =
        js_channel_func.create_threadsafe_function(
          0,
          |ctx: ThreadSafeCallContext<(Sender<AnyValue>, Vec<AnyValue>)>| {
            // Convert the channel sender into a JsUnknown value
            let channel_arg = <External<Sender<AnyValue>>>::new(ctx.value.0);
            let channel_arg =
              unsafe { <External<Sender<AnyValue>>>::to_napi_value(ctx.env.raw(), channel_arg)? };
            let channel_arg = unsafe { JsUnknown::from_napi_value(ctx.env.raw(), channel_arg)? };

            // Convert all the others args as a JsUnknown
            let mut args = ctx
              .value
              .1
              .clone()
              .into_iter()
              .map(|elem| any_value_to_js_value(&ctx.env, elem))
              .try_collect::<Vec<_>>()?;

            // Send to the function the channel and then the other args
            let mut all_args = Vec::new();
            all_args.push(channel_arg);
            all_args.append(&mut args);

            Ok(all_args)
          },
        )?;

      AnyValue::Callback(Arc::new(move |args| {
        println!("call callback");
        let (tx, rx): (Sender<AnyValue>, Receiver<AnyValue>) = channel();

        let ts_func = ts_func.clone();
        // std::thread::spawn(move || {
        ts_func.call((tx, args), ThreadsafeFunctionCallMode::NonBlocking);
        // });

        println!("call callback 1");
        let result = rx.recv().map_err(|_| {
          FruityError::new(
            FruityStatus::CallbackScopeMismatch,
            format!("Channel error while executing a javascript callback"),
          )
        })?;
        println!("call callback 2");

        Ok(result)
      }))
    }
    ValueType::External => todo!(),
    ValueType::BigInt => AnyValue::I64(unsafe { value.cast::<JsBigInt>() }.get_i64()?.0),
    ValueType::Unknown => todo!(),
  })
}

/// Create a napi js value from an any value
pub fn raw_js_value_to_any_value<T>(
  env: napi::sys::napi_env,
  value: napi::sys::napi_value,
) -> Result<T>
where
  T: FruityFrom<AnyValue>,
{
  let value = unsafe { JsUnknown::from_raw(env, value)? };

  let result = js_value_to_any_value(&napi::Env::from(env), value)?;
  FruityFrom::<AnyValue>::fruity_try_from(result)
}

/// Create an any value from a napi js value
pub fn raw_any_value_to_js_value<T>(
  env: napi::sys::napi_env,
  value: T,
) -> Result<napi::sys::napi_value>
where
  T: FruityInto<AnyValue>,
{
  any_value_to_js_value(&napi::Env::from(env), value.fruity_into()?)
    .map(|result| unsafe { result.raw() })
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
    let result = any_value_to_js_value(&env, result).unwrap();
    result.raw()
}*/
