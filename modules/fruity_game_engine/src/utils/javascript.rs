use neon::object::{Object, This};
use neon::prelude::{CallContext, Context, Handle};
use neon::result::NeonResult;
use neon::types::{JsObject, JsString, JsValue};

pub fn js_object_keys<'a, C>(cx: &mut C, js_object: &Handle<JsObject>) -> NeonResult<Vec<String>>
where
    C: Context<'a>,
{
    let js_keys = js_object.get_own_property_names(cx)?;
    Ok([0, js_keys.len(cx)]
        .iter()
        .map(|index| {
            js_keys
                .get::<JsString, C, u32>(cx, index.clone())
                .unwrap()
                .value(cx)
        })
        .collect::<Vec<_>>())
}

pub fn get_context_args<'a, T>(cx: &mut CallContext<'a, T>) -> NeonResult<Vec<Handle<'a, JsValue>>>
where
    T: This,
{
    Ok([0, cx.len()]
        .into_iter()
        .filter_map(|index| cx.argument_opt(*index))
        .collect::<Vec<_>>())
}
