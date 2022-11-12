#![feature(prelude_import)]
#![warn(missing_docs)]
//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use crate::module::modules_service::ModulesService;
use crate::resource::resource_container::ResourceContainer;
pub use parking_lot::*;
#[macro_use]
extern crate lazy_static;

/// A service for frame management
pub mod frame_service {
  use crate::any::FruityAny;
  use crate::resource::resource_container::ResourceContainer;
  use crate::resource::Resource;
  use napi_derive::napi;
  use std::fmt::Debug;
  use std::time::Instant;
  /// A service for frame management
  pub struct FrameService {
    last_frame_instant: Instant,
    delta: f32,
  }
  impl napi::bindgen_prelude::TypeName for FrameService {
    fn type_name() -> &'static str {
      "FrameService"
    }
    fn value_type() -> napi::ValueType {
      napi::ValueType::Function
    }
  }
  impl napi::bindgen_prelude::TypeName for &FrameService {
    fn type_name() -> &'static str {
      "FrameService"
    }
    fn value_type() -> napi::ValueType {
      napi::ValueType::Object
    }
  }
  impl napi::bindgen_prelude::TypeName for &mut FrameService {
    fn type_name() -> &'static str {
      "FrameService"
    }
    fn value_type() -> napi::ValueType {
      napi::ValueType::Object
    }
  }
  impl napi::bindgen_prelude::ToNapiValue for FrameService {
    unsafe fn to_napi_value(
      env: napi::sys::napi_env,
      val: FrameService,
    ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
      if let Some(ctor_ref) = napi::__private::get_class_constructor("FrameService\0") {
        let wrapped_value = Box::into_raw(Box::new(val));
        let instance_value =
          FrameService::new_instance(env, wrapped_value as *mut std::ffi::c_void, ctor_ref)?;
        Ok(instance_value)
      } else {
        Err(napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
              &["Failed to get constructor of class `", "` in `ToNapiValue`"],
              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
            ));
            res
          },
        ))
      }
    }
  }
  impl napi::bindgen_prelude::ObjectFinalize for FrameService {}
  impl FrameService {
    pub fn instance_of<V: napi::NapiRaw>(env: napi::Env, value: V) -> napi::Result<bool> {
      if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("FrameService\0") {
        let mut ctor = std::ptr::null_mut();
        {
          let c = unsafe { napi::sys::napi_get_reference_value(env.raw(), ctor_ref, &mut ctor) };
          match c {
            ::napi::sys::Status::napi_ok => Ok(()),
            _ => Err(::napi::Error::new(::napi::Status::from(c), {
              let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Failed to get constructor reference of class `", "`"],
                &[::core::fmt::ArgumentV1::new_display(&"FrameService\0")],
              ));
              res
            })),
          }
        }?;
        let mut is_instance_of = false;
        {
          let c = unsafe {
            napi::sys::napi_instanceof(env.raw(), value.raw(), ctor, &mut is_instance_of)
          };
          match c {
            ::napi::sys::Status::napi_ok => Ok(()),
            _ => Err(::napi::Error::new(::napi::Status::from(c), {
              let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Failed to run instanceof for class `", "`"],
                &[::core::fmt::ArgumentV1::new_display(&"FrameService\0")],
              ));
              res
            })),
          }
        }?;
        Ok(is_instance_of)
      } else {
        Err(napi::Error::new(napi::Status::GenericFailure, {
          let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["Failed to get constructor of class `", "`"],
            &[::core::fmt::ArgumentV1::new_display(&"FrameService\0")],
          ));
          res
        }))
      }
    }
  }
  impl FrameService {
    pub fn into_reference(
      val: FrameService,
      env: napi::Env,
    ) -> napi::Result<napi::bindgen_prelude::Reference<FrameService>> {
      if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("FrameService\0") {
        unsafe {
          let wrapped_value = Box::into_raw(Box::new(val));
          let instance_value = FrameService::new_instance(
            env.raw(),
            wrapped_value as *mut std::ffi::c_void,
            ctor_ref,
          )?;
          {
            let env = env.raw();
          }
          napi::bindgen_prelude::Reference::<FrameService>::from_value_ptr(
            wrapped_value as *mut std::ffi::c_void,
            env.raw(),
          )
        }
      } else {
        Err(napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
              &["Failed to get constructor of class `", "`"],
              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
            ));
            res
          },
        ))
      }
    }
    pub fn into_instance(
      self,
      env: napi::Env,
    ) -> napi::Result<napi::bindgen_prelude::ClassInstance<FrameService>> {
      if let Some(ctor_ref) = napi::bindgen_prelude::get_class_constructor("FrameService\0") {
        unsafe {
          let wrapped_value = Box::leak(Box::new(self));
          let instance_value = FrameService::new_instance(
            env.raw(),
            wrapped_value as *mut _ as *mut std::ffi::c_void,
            ctor_ref,
          )?;
          Ok(napi::bindgen_prelude::ClassInstance::<FrameService>::new(
            instance_value,
            wrapped_value,
          ))
        }
      } else {
        Err(napi::bindgen_prelude::Error::new(
          napi::bindgen_prelude::Status::InvalidArg,
          {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
              &["Failed to get constructor of class `", "`"],
              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
            ));
            res
          },
        ))
      }
    }
    unsafe fn new_instance(
      env: napi::sys::napi_env,
      wrapped_value: *mut std::ffi::c_void,
      ctor_ref: napi::sys::napi_ref,
    ) -> napi::Result<napi::bindgen_prelude::sys::napi_value> {
      let mut ctor = std::ptr::null_mut();
      {
        let c = napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor);
        match c {
          ::napi::sys::Status::napi_ok => Ok(()),
          _ => Err(::napi::Error::new(::napi::Status::from(c), {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
              &["Failed to get constructor reference of class `", "`"],
              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
            ));
            res
          })),
        }
      }?;
      let mut result = std::ptr::null_mut();
      let inner = napi::__private::___CALL_FROM_FACTORY.get_or_default();
      inner.store(true, std::sync::atomic::Ordering::Relaxed);
      {
        let c = napi::sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result);
        match c {
          ::napi::sys::Status::napi_ok => Ok(()),
          _ => Err(::napi::Error::new(::napi::Status::from(c), {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
              &["Failed to construct class `", "`"],
              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
            ));
            res
          })),
        }
      }?;
      inner.store(false, std::sync::atomic::Ordering::Relaxed);
      let mut object_ref = std::ptr::null_mut();
      let initial_finalize: Box<dyn FnOnce()> = Box::new(|| {});
      let finalize_callbacks_ptr = std::rc::Rc::into_raw(std::rc::Rc::new(std::cell::Cell::new(
        Box::into_raw(initial_finalize),
      )));
      {
        let c = napi::sys::napi_wrap(
          env,
          result,
          wrapped_value,
          Some(napi::bindgen_prelude::raw_finalize_unchecked::<FrameService>),
          std::ptr::null_mut(),
          &mut object_ref,
        );
        match c {
          ::napi::sys::Status::napi_ok => Ok(()),
          _ => Err(::napi::Error::new(::napi::Status::from(c), {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
              &["Failed to wrap native object of class `", "`"],
              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
            ));
            res
          })),
        }
      }?;
      napi::bindgen_prelude::Reference::<FrameService>::add_ref(
        env,
        wrapped_value,
        (wrapped_value, object_ref, finalize_callbacks_ptr),
      );
      Ok(result)
    }
  }
  /*impl napi::bindgen_prelude::FromNapiRef for FrameService {
      unsafe fn from_napi_ref(
          env: napi::bindgen_prelude::sys::napi_env,
          napi_val: napi::bindgen_prelude::sys::napi_value,
      ) -> napi::bindgen_prelude::Result<&'static Self> {
          let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
          {
              let c = napi::bindgen_prelude::sys::napi_unwrap(
                  env,
                  napi_val,
                  &mut wrapped_val,
              );
              match c {
                  ::napi::sys::Status::napi_ok => Ok(()),
                  _ => {
                      Err(
                          ::napi::Error::new(
                              ::napi::Status::from(c),
                              {
                                  let res = ::alloc::fmt::format(
                                      ::core::fmt::Arguments::new_v1(
                                          &["Failed to recover `", "` type from napi value"],
                                          &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                      ),
                                  );
                                  res
                              },
                          ),
                      )
                  }
              }
          }?;
          Ok(&*(wrapped_val as *const FrameService))
      }
  }
  impl napi::bindgen_prelude::FromNapiMutRef for FrameService {
      unsafe fn from_napi_mut_ref(
          env: napi::bindgen_prelude::sys::napi_env,
          napi_val: napi::bindgen_prelude::sys::napi_value,
      ) -> napi::bindgen_prelude::Result<&'static mut Self> {
          let mut wrapped_val: *mut std::ffi::c_void = std::ptr::null_mut();
          {
              let c = napi::bindgen_prelude::sys::napi_unwrap(
                  env,
                  napi_val,
                  &mut wrapped_val,
              );
              match c {
                  ::napi::sys::Status::napi_ok => Ok(()),
                  _ => {
                      Err(
                          ::napi::Error::new(
                              ::napi::Status::from(c),
                              {
                                  let res = ::alloc::fmt::format(
                                      ::core::fmt::Arguments::new_v1(
                                          &["Failed to recover `", "` type from napi value"],
                                          &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                      ),
                                  );
                                  res
                              },
                          ),
                      )
                  }
              }
          }?;
          Ok(&mut *(wrapped_val as *mut FrameService))
      }
  }
  impl napi::bindgen_prelude::FromNapiValue for &FrameService {
      unsafe fn from_napi_value(
          env: napi::bindgen_prelude::sys::napi_env,
          napi_val: napi::bindgen_prelude::sys::napi_value,
      ) -> napi::bindgen_prelude::Result<Self> {
          napi::bindgen_prelude::FromNapiRef::from_napi_ref(env, napi_val)
      }
  }
  impl napi::bindgen_prelude::FromNapiValue for &mut FrameService {
      unsafe fn from_napi_value(
          env: napi::bindgen_prelude::sys::napi_env,
          napi_val: napi::bindgen_prelude::sys::napi_value,
      ) -> napi::bindgen_prelude::Result<Self> {
          napi::bindgen_prelude::FromNapiMutRef::from_napi_mut_ref(env, napi_val)
      }
  }
  impl napi::bindgen_prelude::ValidateNapiValue for &FrameService {
      unsafe fn validate(
          env: napi::sys::napi_env,
          napi_val: napi::sys::napi_value,
      ) -> napi::Result<napi::sys::napi_value> {
          if let Some(ctor_ref)
              = napi::bindgen_prelude::get_class_constructor("FrameService\0") {
              let mut ctor = std::ptr::null_mut();
              {
                  let c = napi::sys::napi_get_reference_value(
                      env,
                      ctor_ref,
                      &mut ctor,
                  );
                  match c {
                      ::napi::sys::Status::napi_ok => Ok(()),
                      _ => {
                          Err(
                              ::napi::Error::new(
                                  ::napi::Status::from(c),
                                  {
                                      let res = ::alloc::fmt::format(
                                          ::core::fmt::Arguments::new_v1(
                                              &["Failed to get constructor reference of class `", "`"],
                                              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                          ),
                                      );
                                      res
                                  },
                              ),
                          )
                      }
                  }
              }?;
              let mut is_instance_of = false;
              {
                  let c = napi::sys::napi_instanceof(
                      env,
                      napi_val,
                      ctor,
                      &mut is_instance_of,
                  );
                  match c {
                      ::napi::sys::Status::napi_ok => Ok(()),
                      _ => {
                          Err(
                              ::napi::Error::new(
                                  ::napi::Status::from(c),
                                  {
                                      let res = ::alloc::fmt::format(
                                          ::core::fmt::Arguments::new_v1(
                                              &["Failed to get external value of class `", "`"],
                                              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                          ),
                                      );
                                      res
                                  },
                              ),
                          )
                      }
                  }
              }?;
              if is_instance_of {
                  Ok(std::ptr::null_mut())
              } else {
                  Err(
                      napi::Error::new(
                          napi::Status::InvalidArg,
                          {
                              let res = ::alloc::fmt::format(
                                  ::core::fmt::Arguments::new_v1(
                                      &["Value is not instanceof class `", "`"],
                                      &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                  ),
                              );
                              res
                          },
                      ),
                  )
              }
          } else {
              Err(
                  napi::Error::new(
                      napi::Status::InvalidArg,
                      {
                          let res = ::alloc::fmt::format(
                              ::core::fmt::Arguments::new_v1(
                                  &["Failed to get constructor of class `", "`"],
                                  &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                              ),
                          );
                          res
                      },
                  ),
              )
          }
      }
  }
  impl napi::bindgen_prelude::ValidateNapiValue for &mut FrameService {
      unsafe fn validate(
          env: napi::sys::napi_env,
          napi_val: napi::sys::napi_value,
      ) -> napi::Result<napi::sys::napi_value> {
          if let Some(ctor_ref)
              = napi::bindgen_prelude::get_class_constructor("FrameService\0") {
              let mut ctor = std::ptr::null_mut();
              {
                  let c = napi::sys::napi_get_reference_value(
                      env,
                      ctor_ref,
                      &mut ctor,
                  );
                  match c {
                      ::napi::sys::Status::napi_ok => Ok(()),
                      _ => {
                          Err(
                              ::napi::Error::new(
                                  ::napi::Status::from(c),
                                  {
                                      let res = ::alloc::fmt::format(
                                          ::core::fmt::Arguments::new_v1(
                                              &["Failed to get constructor reference of class `", "`"],
                                              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                          ),
                                      );
                                      res
                                  },
                              ),
                          )
                      }
                  }
              }?;
              let mut is_instance_of = false;
              {
                  let c = napi::sys::napi_instanceof(
                      env,
                      napi_val,
                      ctor,
                      &mut is_instance_of,
                  );
                  match c {
                      ::napi::sys::Status::napi_ok => Ok(()),
                      _ => {
                          Err(
                              ::napi::Error::new(
                                  ::napi::Status::from(c),
                                  {
                                      let res = ::alloc::fmt::format(
                                          ::core::fmt::Arguments::new_v1(
                                              &["Failed to get external value of class `", "`"],
                                              &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                          ),
                                      );
                                      res
                                  },
                              ),
                          )
                      }
                  }
              }?;
              if is_instance_of {
                  Ok(std::ptr::null_mut())
              } else {
                  Err(
                      napi::Error::new(
                          napi::Status::InvalidArg,
                          {
                              let res = ::alloc::fmt::format(
                                  ::core::fmt::Arguments::new_v1(
                                      &["Value is not instanceof class `", "`"],
                                      &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                                  ),
                              );
                              res
                          },
                      ),
                  )
              }
          } else {
              Err(
                  napi::Error::new(
                      napi::Status::InvalidArg,
                      {
                          let res = ::alloc::fmt::format(
                              ::core::fmt::Arguments::new_v1(
                                  &["Failed to get constructor of class `", "`"],
                                  &[::core::fmt::ArgumentV1::new_display(&"FrameService")],
                              ),
                          );
                          res
                      },
                  ),
              )
          }
      }
  }
  impl napi::NapiRaw for &FrameService {
      unsafe fn raw(&self) -> napi::sys::napi_value {
          ::core::panicking::panic("internal error: entered unreachable code")
      }
  }
  impl napi::NapiRaw for &mut FrameService {
      unsafe fn raw(&self) -> napi::sys::napi_value {
          ::core::panicking::panic("internal error: entered unreachable code")
      }
  }
  #[allow(clippy::all)]
  #[allow(non_snake_case)]
  mod __napi_helper__FrameService {
      use std::ptr;
      use super::*;
      #[allow(non_snake_case)]
      #[allow(clippy::all)]
      #[cfg(all(not(test), not(feature = "noop")))]
      extern fn __napi_register__FrameService_struct() {
          napi::__private::register_class(
              "FrameService",
              None,
              "FrameService\0",
              ::alloc::vec::Vec::new(),
          );
      }
      #[used]
      #[allow(non_upper_case_globals)]
      #[doc(hidden)]
      #[link_section = "__DATA,__mod_init_func"]
      static __napi_register__FrameService_struct___rust_ctor___ctor: unsafe extern "C" fn() = {
          unsafe extern "C" fn __napi_register__FrameService_struct___rust_ctor___ctor() {
              __napi_register__FrameService_struct()
          }
          __napi_register__FrameService_struct___rust_ctor___ctor
      };
  }*/
  impl FrameService {
    /// Returns a FrameService
    pub fn new(_resource_container: ResourceContainer) -> FrameService {
      FrameService {
        delta: 0.0,
        last_frame_instant: Instant::now(),
      }
    }
    /// Get the time before the previous frame
    pub fn get_delta(&self) -> f32 {
      self.delta
    }
    /// A function that needs to be called on new frame
    /// Intended to be used in the render pipeline
    pub fn begin_frame(&mut self) {
      let now = Instant::now();
      let delta = now.duration_since(self.last_frame_instant);
      self.delta = delta.as_secs_f32();
      self.last_frame_instant = now;
    }
  }
  #[allow(non_snake_case)]
  #[allow(clippy::all)]
  mod __napi_impl_helper__FrameService__2 {
    use super::*;
    /// Get the time before the previous frame
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(clippy::all)]
    extern "C" fn __napi__get_delta(
      env: napi::bindgen_prelude::sys::napi_env,
      cb: napi::bindgen_prelude::sys::napi_callback_info,
    ) -> napi::bindgen_prelude::sys::napi_value {
      unsafe {
        napi::bindgen_prelude::CallbackInfo::<0usize>::new(env, cb, None)
          .and_then(|mut cb| {
            let this_ptr = unsafe { cb.unwrap_raw::<FrameService>()? };
            let this: &FrameService = Box::leak(Box::from_raw(this_ptr));
            napi::bindgen_prelude::within_runtime_if_available(move || {
              let _ret = { this.get_delta() };
              <f32 as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, _ret)
            })
          })
          .unwrap_or_else(|e| {
            napi::bindgen_prelude::JsError::from(e).throw_into(env);
            std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
          })
      }
    }
    #[cfg(all(not(test), not(feature = "noop")))]
    extern "C" fn __napi_register__FrameService_impl() {
      napi::__private::register_class(
        "FrameService",
        None,
        "FrameService\0",
        <[_]>::into_vec(box [napi::bindgen_prelude::Property::new("getDelta")
          .unwrap()
          .with_property_attributes(
            napi::bindgen_prelude::PropertyAttributes::from_bits(7i32).unwrap(),
          )
          .with_method(__napi__get_delta)]),
      );
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __napi_register__FrameService_impl___rust_ctor___ctor: unsafe extern "C" fn() = {
      unsafe extern "C" fn __napi_register__FrameService_impl___rust_ctor___ctor() {
        __napi_register__FrameService_impl()
      }
      __napi_register__FrameService_impl___rust_ctor___ctor
    };
  }
}
