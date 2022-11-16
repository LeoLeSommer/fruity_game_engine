use crate::javascript::FromJavascript;
use crate::javascript::JavascriptCallable;
use crate::ToJavascript;
use neon::prelude::FunctionContext;
use neon::prelude::Handle;
use neon::result::NeonResult;
use neon::types::JsValue;

impl<R: ToJavascript + 'static> JavascriptCallable for &'static dyn Fn() -> R {
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            self().to_js(cx)
        })
    }
}

impl<T1: FromJavascript + 'static, R: ToJavascript + 'static> JavascriptCallable
    for &'static dyn Fn(T1) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;

            self(T1::from_js(arg1, cx)?).to_js(cx)
        })
    }
}

impl<T1: FromJavascript + 'static, T2: FromJavascript + 'static, R: ToJavascript + 'static>
    JavascriptCallable for &'static dyn Fn(T1, T2) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;

            self(T1::from_js(arg1, cx)?, T2::from_js(arg2, cx)?).to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4, T5) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4, T5, T6) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        T14: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;
            let arg14 = cx.argument(13)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
                T14::from_js(arg14, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        T14: FromJavascript + 'static,
        T15: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;
            let arg14 = cx.argument(13)?;
            let arg15 = cx.argument(14)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
                T14::from_js(arg14, cx)?,
                T15::from_js(arg15, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        T14: FromJavascript + 'static,
        T15: FromJavascript + 'static,
        T16: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;
            let arg14 = cx.argument(13)?;
            let arg15 = cx.argument(14)?;
            let arg16 = cx.argument(15)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
                T14::from_js(arg14, cx)?,
                T15::from_js(arg15, cx)?,
                T16::from_js(arg16, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        T14: FromJavascript + 'static,
        T15: FromJavascript + 'static,
        T16: FromJavascript + 'static,
        T17: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
    ) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;
            let arg14 = cx.argument(13)?;
            let arg15 = cx.argument(14)?;
            let arg16 = cx.argument(15)?;
            let arg17 = cx.argument(16)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
                T14::from_js(arg14, cx)?,
                T15::from_js(arg15, cx)?,
                T16::from_js(arg16, cx)?,
                T17::from_js(arg17, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        T14: FromJavascript + 'static,
        T15: FromJavascript + 'static,
        T16: FromJavascript + 'static,
        T17: FromJavascript + 'static,
        T18: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
    ) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;
            let arg14 = cx.argument(13)?;
            let arg15 = cx.argument(14)?;
            let arg16 = cx.argument(15)?;
            let arg17 = cx.argument(16)?;
            let arg18 = cx.argument(17)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
                T14::from_js(arg14, cx)?,
                T15::from_js(arg15, cx)?,
                T16::from_js(arg16, cx)?,
                T17::from_js(arg17, cx)?,
                T18::from_js(arg18, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        T14: FromJavascript + 'static,
        T15: FromJavascript + 'static,
        T16: FromJavascript + 'static,
        T17: FromJavascript + 'static,
        T18: FromJavascript + 'static,
        T19: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
    ) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;
            let arg14 = cx.argument(13)?;
            let arg15 = cx.argument(14)?;
            let arg16 = cx.argument(15)?;
            let arg17 = cx.argument(16)?;
            let arg18 = cx.argument(17)?;
            let arg19 = cx.argument(18)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
                T14::from_js(arg14, cx)?,
                T15::from_js(arg15, cx)?,
                T16::from_js(arg16, cx)?,
                T17::from_js(arg17, cx)?,
                T18::from_js(arg18, cx)?,
                T19::from_js(arg19, cx)?,
            )
            .to_js(cx)
        })
    }
}

impl<
        T1: FromJavascript + 'static,
        T2: FromJavascript + 'static,
        T3: FromJavascript + 'static,
        T4: FromJavascript + 'static,
        T5: FromJavascript + 'static,
        T6: FromJavascript + 'static,
        T7: FromJavascript + 'static,
        T8: FromJavascript + 'static,
        T9: FromJavascript + 'static,
        T10: FromJavascript + 'static,
        T11: FromJavascript + 'static,
        T12: FromJavascript + 'static,
        T13: FromJavascript + 'static,
        T14: FromJavascript + 'static,
        T15: FromJavascript + 'static,
        T16: FromJavascript + 'static,
        T17: FromJavascript + 'static,
        T18: FromJavascript + 'static,
        T19: FromJavascript + 'static,
        T20: FromJavascript + 'static,
        R: ToJavascript + 'static,
    > JavascriptCallable
    for &'static dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
        T20,
    ) -> R
{
    fn into_js(self) -> Box<dyn Fn(FunctionContext) -> NeonResult<Handle<JsValue>>> {
        Box::new(move |mut cx| {
            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            let arg1 = cx.argument(0)?;
            let arg2 = cx.argument(1)?;
            let arg3 = cx.argument(2)?;
            let arg4 = cx.argument(3)?;
            let arg5 = cx.argument(4)?;
            let arg6 = cx.argument(5)?;
            let arg7 = cx.argument(6)?;
            let arg8 = cx.argument(7)?;
            let arg9 = cx.argument(8)?;
            let arg10 = cx.argument(9)?;
            let arg11 = cx.argument(10)?;
            let arg12 = cx.argument(11)?;
            let arg13 = cx.argument(12)?;
            let arg14 = cx.argument(13)?;
            let arg15 = cx.argument(14)?;
            let arg16 = cx.argument(15)?;
            let arg17 = cx.argument(16)?;
            let arg18 = cx.argument(17)?;
            let arg19 = cx.argument(18)?;
            let arg20 = cx.argument(19)?;

            self(
                T1::from_js(arg1, cx)?,
                T2::from_js(arg2, cx)?,
                T3::from_js(arg3, cx)?,
                T4::from_js(arg4, cx)?,
                T5::from_js(arg5, cx)?,
                T6::from_js(arg6, cx)?,
                T7::from_js(arg7, cx)?,
                T8::from_js(arg8, cx)?,
                T9::from_js(arg9, cx)?,
                T10::from_js(arg10, cx)?,
                T11::from_js(arg11, cx)?,
                T12::from_js(arg12, cx)?,
                T13::from_js(arg13, cx)?,
                T14::from_js(arg14, cx)?,
                T15::from_js(arg15, cx)?,
                T16::from_js(arg16, cx)?,
                T17::from_js(arg17, cx)?,
                T18::from_js(arg18, cx)?,
                T19::from_js(arg19, cx)?,
                T20::from_js(arg20, cx)?,
            )
            .to_js(cx)
        })
    }
}
