use crate::javascript::FromJavascript;
use crate::ToJavascript;
use neon::context::Context;
use neon::prelude::CallContext;
use neon::prelude::FunctionContext;
use neon::prelude::Handle;
use neon::prelude::JsResultExt;
use neon::prelude::Value;
use neon::result::NeonResult;
use neon::types::JsObject;
use neon::types::JsValue;

impl ToJavascript for () {
    fn to_js<'a>(self, cx: &'a mut FunctionContext) -> NeonResult<Handle<'a, JsValue>> {
        Ok(cx.undefined().as_value(cx))
    }
}

impl FromJavascript for f32 {
    fn from_js(value: Handle<JsValue>, cx: &mut CallContext<JsObject>) -> NeonResult<Self> {
        let value = value
            .downcast::<neon::prelude::JsNumber, neon::prelude::FunctionContext>(cx)
            .or_throw(cx)?;

        Ok(value.value(cx) as f32)
    }
}

impl ToJavascript for f32 {
    fn to_js<'a>(self, cx: &'a mut FunctionContext) -> NeonResult<Handle<'a, JsValue>> {
        Ok(cx.number(self).as_value(cx))
    }
}

impl FromJavascript for f64 {
    fn from_js(value: Handle<JsValue>, cx: &mut CallContext<JsObject>) -> NeonResult<Self> {
        let value = value
            .downcast::<neon::prelude::JsNumber, neon::prelude::FunctionContext>(cx)
            .or_throw(cx)?;

        Ok(value.value(cx))
    }
}

impl ToJavascript for f64 {
    fn to_js<'a>(self, cx: &'a mut FunctionContext) -> NeonResult<Handle<'a, JsValue>> {
        Ok(cx.number(self).as_value(cx))
    }
}
