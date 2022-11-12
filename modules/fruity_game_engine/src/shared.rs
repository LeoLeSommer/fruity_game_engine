use crate::RwLock;
use napi::bindgen_prelude::FromNapiValue;
use napi::bindgen_prelude::SharedReference;
use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::TypeName;
use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::NapiRaw;
use napi::NapiValue;
use std::sync::Arc;

/// Shared references that can be exposed to javascript with read and write abilities
pub struct ResourceShared<T: Send + Sync + ?Sized + ToNapiValue + TypeName>(pub Arc<RwLock<T>>);

impl<T: Send + Sync + ?Sized + ToNapiValue + TypeName> Clone for ResourceShared<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: Send + Sync + ToNapiValue + TypeName> ResourceShared<T> {
  /// Returns a new RWShared
  pub fn new(val: T) -> Self {
    Self(Arc::new(RwLock::new(val)))
  }
}

impl<T: Send + Sync + ToNapiValue + TypeName> ToNapiValue for ResourceShared<T> {
  unsafe fn to_napi_value(
    raw_env: *mut napi_sys::napi_env__,
    value: Self,
  ) -> Result<*mut napi_sys::napi_value__, napi::Error> {
    let env = Env::from(raw_env);

    // We create a js object to reference the inner object, it will be used to access it's properties
    // We need to take care that the locks are properly raised when accessing the inner object
    // TODO: Highly unsafe, we need to find a proper way to do that
    let referenced_inner = <*const RwLock<T>>::as_ref(Arc::into_raw(value.0))
      .unwrap()
      .data_ptr();
    let inner_js_value = T::to_napi_value(raw_env, *referenced_inner)?;
    let inner_js_value = JsObject::from_napi_value(raw_env, inner_js_value)?;

    // TODO: Try to find a way to differentiate, read and write access, but if it's not possible, then
    // we should write protect everytime

    let mut reference_js_value = env.create_object()?;

    let inner_keys = JsObject::keys(&inner_js_value)?;
    inner_keys.iter().for_each(|key| {
      let js_value_wrapper = if let Some(inner_property_as_fn) =
        inner_js_value.get::<&String, JsFunction>(key).unwrap()
      {
      } else if let Some(inner_property_as_fn) =
        inner_js_value.get::<&String, JsFunction>(key).unwrap()
      {
      } else {
      };

      reference_js_value.set(key, js_value_wrapper);
    });

    reference_js_value.Ok(reference_js_value.raw())
  }
}
