use crate::any::FruityAny;
use crate::resource::resource_container::ResourceContainer;
use crate::resource::Resource;
use std::cell::RefCell;
use std::fmt::Debug;
use std::time::Instant;

#[deny(missing_fragment_specifier)]

macro fruity_export_method_binding(
  $method_visibility:vis fn $method_name:ident(
    &$self_1:tt $($self_2:tt)?
    $(,$($param_name:ident: $param_type:ty)*)?
  ) $(-> $return_type:ty)?
  $method_body:block
) {}

macro fruity_export_field_binding(
  $field_visibility:vis $field_name:ident: $field_type:ty
) {}

macro fruity_export {
  (
    $(#[$attribute:meta])*
    $visibility:vis struct $struct_name:ident {
      $($field_visibility:vis $field_name:ident: $field_type:ty),*$(,)?
    }

    impl {
      $($impl_function:item)*
    }

    export {
      $(
        $(#[$method_attribute:meta])*
        $method_visibility:vis fn $method_name:ident(
          &$self_1:tt $($self_2:tt)?
          $(,$($param_name:ident: $param_type:ty)*)?
        ) $(-> $return_type:ty)?
        $method_body:block
      )*
    }
  ) => {
    // Structure
    $(#[$attribute])*
    $visibility struct $struct_name {
      $($visibility $field_name: $field_type),+
    }

    // Impl
    impl $struct_name {
      $($impl_function)+

      $(
        $(#[$method_attribute])+
        $method_visibility fn $method_name(
          &$self_1 $($self_2)?
          $(,$($param_name: $param_type)*)?
        ) $(-> $return_type)?
        $method_body
      )+
    }

    $(
      fruity_export_field_binding!(
        $field_visibility $field_name: $field_type
      );
    )+

    $(
      fruity_export_method_binding!(
        $method_visibility fn $method_name(
          &$self_1 $($self_2)?
          $(,$($param_name: $param_type)*)?
        ) $(-> $return_type)?
        $method_body
      );
    )+
  },
}

fruity_export! {
  /// A service for frame management
  #[derive(FruityAny, Resource)]
  pub struct FrameService {
    last_frame_instant: Instant,
    pub delta: f32,
  }

  impl {
    /// Returns a FrameService
    pub fn new(_resource_container: ResourceContainer) -> FrameService {
      FrameService {
        delta: 0.0,
        last_frame_instant: Instant::now(),
      }
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

  export {
    /// Get the time before the previous frame
    pub fn get_delta(&self) -> f32 {
      self.delta
    }

    /// Please remove it
    pub fn set_delta(&mut self, value: f32) {
      self.delta = value;
    }
  }
}

impl crate::neon::types::Finalize for FrameService {}

impl crate::javascript::ToJavascript for FrameService {
    fn to_js<'a>(
        self,
        cx: &'a mut crate::neon::prelude::FunctionContext,
    ) -> crate::neon::result::NeonResult<
        crate::neon::prelude::Handle<'a, crate::neon::types::JsValue>,
    > {
        use crate::javascript::FromJavascript;
        use crate::neon::context::Context;
        use crate::neon::object::Object;
        use crate::neon::prelude::Value;

        // Store the reference to the rust object
        let boxed: crate::neon::prelude::Handle<crate::neon::types::JsBox<RefCell<Self>>> =
            cx.boxed(RefCell::new(self));

        // Generate method wrappers
        let get_delta = crate::neon::types::JsFunction::new(cx, |mut cx: crate::neon::prelude::FunctionContext| -> crate::neon::result::JsResult<crate::neon::types::JsValue> {
      let this: crate::neon::prelude::Handle<crate::neon::types::JsBox<RefCell<Self>>> = cx.this().downcast(&mut cx).unwrap();
      let this = this.borrow();

      let result = this.get_delta();

      // TODO: Find a way to remove this
      let cx = unsafe {
        std::mem::transmute::<
          &mut neon::prelude::FunctionContext,
          &mut neon::prelude::FunctionContext,
        >(&mut cx)
      };

      Ok(crate::javascript::ToJavascript::to_js(result, cx)?)
    })?;

        boxed.set(cx, "getDelta", get_delta)?;

        // Generate method wrappers
        let set_delta = crate::neon::types::JsFunction::new(cx, |mut cx: crate::neon::prelude::FunctionContext| -> crate::neon::result::JsResult<crate::neon::types::JsValue> {
          let this: crate::neon::prelude::Handle<crate::neon::types::JsBox<RefCell<Self>>> = cx.this().downcast(&mut cx).unwrap();
          let mut this = this.borrow_mut();

      let arg1 = f32::from_js(cx.argument(0)?, &mut cx)?;

      let result = this.set_delta(arg1);

      // TODO: Find a way to remove this
      let cx = unsafe {
        std::mem::transmute::<
          &mut neon::prelude::FunctionContext,
          &mut neon::prelude::FunctionContext,
        >(&mut cx)
      };

      Ok(crate::javascript::ToJavascript::to_js(result, cx)?)
    })?;

        boxed.set(cx, "setDelta", set_delta)?;

        Ok(boxed.as_value(cx))
    }
}

impl Debug for FrameService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
