use crate::settings::Settings;
use crate::ResourceContainer;
use napi::bindgen_prelude::FromNapiValue;
use napi::bindgen_prelude::ToNapiValue;
use napi::Env;
use napi_derive::napi;
use std::sync::Arc;

/// A service to manage modules loading
pub mod modules_service;

/// A module for the engine
#[derive(Clone)]
#[napi]
pub struct Module {
  /// The name of the module
  pub name: String,

  /// The dependencies of the module
  pub dependencies: Vec<String>,

  /// A function that initialize the module
  pub setup: Option<ModuleCallback>,

  /// A function that initialize the module resources
  pub load_resources: Option<ModuleCallback>,

  /// A function that is called when the world enter into the loop
  pub run: Option<ModuleCallback>,
}

#[derive(Clone)]
pub struct ModuleCallback(pub Arc<dyn Fn(ResourceContainer, Settings) + Sync + Send>);

impl ToNapiValue for ModuleCallback {
  unsafe fn to_napi_value(
    raw_env: *mut napi_sys::napi_env__,
    value: Self,
  ) -> Result<*mut napi_sys::napi_value__, napi::Error> {
    todo!();
    /*let env = Env::from(raw_env);

    Ok(
      env
        .create_function_from_closure("callback", move |ctx| {
          let arg1 = ctx.get(0)?;
          let arg2 = ctx.get(1)?;

          (value.0)(arg1, arg2);

          Ok(ctx.env.get_undefined()?)
        })?
        .raw(),
    )*/
  }
}

impl FromNapiValue for ModuleCallback {
  unsafe fn from_napi_value(
    raw_env: *mut napi_sys::napi_env__,
    raw_value: *mut napi_sys::napi_value__,
  ) -> Result<Self, napi::Error> {
    todo!();
  }
}
