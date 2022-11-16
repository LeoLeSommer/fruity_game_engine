use neon::prelude::FunctionContext;
use neon::prelude::Handle;
use neon::prelude::ModuleContext;
use neon::result::NeonResult;
use neon::types::JsValue;

/// Implementations of JavascriptCallable for rust functions
pub mod callable;

/// Implementations of javascript traits for primary types
pub mod primary;

/// A trait that is implemented by functions that are exposed to javascript
pub trait JavascriptCallable {
    /// Get a function that proceed the injection
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>>;
}

/// A trait that all types sent to javascript should implements
pub trait ToJavascript {
    /// Convert a rust value to a js value
    fn to_js<'a>(self, cx: &'a mut FunctionContext) -> NeonResult<Handle<'a, JsValue>>;
}

/// A trait that all types importer from javascript should implements
pub trait FromJavascript: Sized {
    /// Convert a js value to a rust value
    fn from_js(value: Handle<JsValue>, cx: &mut FunctionContext) -> NeonResult<Self>;
}

/// A tool to register functions, values ... to the js context
pub struct JavascriptContext<'a>(ModuleContext<'a>);

impl<'a> JavascriptContext<'a> {
    /// Returns a ModuleContext
    pub fn new(ctx: ModuleContext<'a>) -> Self {
        Self(ctx)
    }

    /// Register a function to expose it to the javascript
    pub fn register_function(
        &mut self,
        name: &str,
        function: impl JavascriptCallable,
    ) -> NeonResult<()> {
        self.0.export_function(name, function.into_js())?;

        Ok(())
    }
}
