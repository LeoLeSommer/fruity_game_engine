use super::{ScriptValue, TryFromScriptValue, TryIntoScriptValue};
use crate::{sync::Arc, utils::ArgumentCaster, FruityError, FruityResult};
use futures::Future;
use std::pin::Pin;

macro_rules! impl_try_into_script_value_for_fn {
    () => {
        #[allow(non_snake_case)]
        impl<R: TryIntoScriptValue + Send + 'static> TryIntoScriptValue for &'static (dyn Send + Sync + Fn() -> R) {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Callback(Box::new(move |_args: Vec<ScriptValue>| {
                    let result = self();
                    result.into_script_value()
                })))
            }
        }

        #[allow(non_snake_case)]
        impl<R: TryIntoScriptValue + Send + 'static> TryIntoScriptValue for Box<dyn Send + Sync + Fn() -> R> {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Callback(Box::new(move |_args: Vec<ScriptValue>| {
                    let result = self();
                    result.into_script_value()
                })))
            }
        }

        #[allow(non_snake_case)]
        impl<R: TryIntoScriptValue + Send + 'static> TryIntoScriptValue for Arc<dyn Send + Sync + Fn() -> R> {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Callback(Box::new(move |_args: Vec<ScriptValue>| {
                    let result = self();
                    result.into_script_value()
                })))
            }
        }
    };
    ($($arg:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($arg: TryFromScriptValue + 'static,)* R: TryIntoScriptValue + Send + 'static> TryIntoScriptValue for &'static (dyn Send + Sync + Fn($($arg),*) -> R) {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Callback(Box::new(move |args: Vec<ScriptValue>| {
                    let mut caster = ArgumentCaster::new(args);
                    $(
                        let $arg = caster.cast_next::<$arg>()?;
                    )*

                    let result = self($($arg),*);

                    result.into_script_value()
                })))
            }
        }

        #[allow(non_snake_case)]
        impl<$($arg: TryFromScriptValue + 'static,)* R: TryIntoScriptValue + Send + 'static> TryIntoScriptValue for Box<dyn Send + Sync + Fn($($arg),*) -> R> {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Callback(Box::new(move |args: Vec<ScriptValue>| {
                    let mut caster = ArgumentCaster::new(args);
                    $(
                        let $arg = caster.cast_next::<$arg>()?;
                    )*

                    let result = self($($arg),*);

                    result.into_script_value()
                })))
            }
        }

        #[allow(non_snake_case)]
        impl<$($arg: TryFromScriptValue + 'static,)* R: TryIntoScriptValue + Send + 'static> TryIntoScriptValue for Arc<dyn Send + Sync + Fn($($arg),*) -> R> {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Callback(Box::new(move |args: Vec<ScriptValue>| {
                    let mut caster = ArgumentCaster::new(args);
                    $(
                        let $arg = caster.cast_next::<$arg>()?;
                    )*

                    let result = self($($arg),*);

                    result.into_script_value()
                })))
            }
        }
    };
}

macro_rules! impl_try_from_script_value_for_fn {
    ($($arg:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($arg: TryIntoScriptValue + 'static,)* R: TryFromScriptValue + Send + 'static> TryFromScriptValue for Box<dyn Send + Fn($($arg),*) -> FruityResult<R>> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => Ok(Box::new(move |$($arg),*| {
                        let args: Vec<ScriptValue> = vec![$($arg.into_script_value()?,)*];
                        let result = value(args)?;

                        <R>::from_script_value(result)
                    })),
                    _ => Err(FruityError::FunctionExpected(format!(
                        "Couldn't convert {:?} to native callback ",
                        value
                    ))),
                }
            }
        }

        #[allow(non_snake_case)]
        impl<$($arg: TryIntoScriptValue + 'static,)* R: TryFromScriptValue + Send + 'static> TryFromScriptValue for Box<dyn Send + Sync + Fn($($arg),*) -> FruityResult<R>> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => Ok(Box::new(move |$($arg),*| {
                        let args: Vec<ScriptValue> = vec![$($arg.into_script_value()?,)*];
                        let result = value(args)?;

                        <R>::from_script_value(result)
                    })),
                    _ => Err(FruityError::FunctionExpected(format!(
                        "Couldn't convert {:?} to native callback ",
                        value
                    ))),
                }
            }
        }

        #[allow(non_snake_case)]
        impl<$($arg: TryIntoScriptValue + 'static,)* R: TryFromScriptValue + Send + 'static> TryFromScriptValue for Arc<dyn Send + Sync + Fn($($arg),*) -> FruityResult<R>> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => Ok(Arc::new(move |$($arg: $arg),*| {
                        let args: Vec<ScriptValue> = vec![$($arg.into_script_value()?,)*];
                        let result = value(args)?;

                        <R>::from_script_value(result)
                    })),
                    _ => Err(FruityError::FunctionExpected(format!(
                        "Couldn't convert {:?} to native callback ",
                        value
                    ))),
                }
            }
        }

        #[allow(non_snake_case)]
        impl<
                $($arg: TryIntoScriptValue + Send + 'static,)*
                R: TryFromScriptValue,
            > TryFromScriptValue
            for Box<dyn Send + Fn($($arg),*) -> Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>
        {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        let shared_callback = Arc::new(value);

                        Ok(Box::new(move |$($arg: $arg),*| {
                            let shared_callback = shared_callback.clone();
                            Box::pin(async move {
                                let args: Vec<ScriptValue> = vec![$($arg.into_script_value()?,)*];
                                let result = shared_callback(args)?;
                                <Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>::from_script_value(
                                    result,
                                )?
                                .await
                            })
                        }))
                    }
                    _ => Err(FruityError::FunctionExpected(format!(
                        "Couldn't convert {:?} to native callback ",
                        value
                    ))),
                }
            }
        }

        #[allow(non_snake_case)]
        impl<
                $($arg: TryIntoScriptValue + Send + 'static,)*
                R: TryFromScriptValue,
            > TryFromScriptValue
            for Box<dyn Send + Sync + Fn($($arg),*) -> Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>
        {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        let shared_callback = Arc::new(value);

                        Ok(Box::new(move |$($arg: $arg),*| {
                            let shared_callback = shared_callback.clone();
                            Box::pin(async move {
                                let args: Vec<ScriptValue> = vec![$($arg.into_script_value()?,)*];
                                let result = shared_callback(args)?;
                                <Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>::from_script_value(
                                    result,
                                )?
                                .await
                            })
                        }))
                    }
                    _ => Err(FruityError::FunctionExpected(format!(
                        "Couldn't convert {:?} to native callback ",
                        value
                    ))),
                }
            }
        }

        #[allow(non_snake_case)]
        impl<
                $($arg: TryIntoScriptValue + Send + 'static,)*
                R: TryFromScriptValue,
            > TryFromScriptValue
            for Arc<dyn Send + Sync + Fn($($arg),*) -> Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>
        {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        let shared_callback = Arc::new(value);

                        Ok(Arc::new(move |$($arg: $arg),*| {
                            let shared_callback = shared_callback.clone();
                            Box::pin(async move {
                                let args: Vec<ScriptValue> = vec![$($arg.into_script_value()?,)*];
                                let result = shared_callback(args)?;
                                <Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>::from_script_value(
                                    result,
                                )?
                                .await
                            })
                        }))
                    }
                    _ => Err(FruityError::FunctionExpected(format!(
                        "Couldn't convert {:?} to native callback ",
                        value
                    ))),
                }
            }
        }

        #[allow(non_snake_case)]
        impl<$($arg: TryIntoScriptValue + Send + 'static,)* R: TryFromScriptValue + 'static> TryFromScriptValue
            for Box<dyn FnOnce($($arg),*) -> FruityResult<R> + Send + Sync + 'static>
        {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        Ok(Box::new(move |$($arg: $arg),*| {
                            let args: Vec<ScriptValue> = vec![$($arg.into_script_value()?,)*];
                            let result = value(args)?;

                            <R>::from_script_value(result)
                        }))
                    }
                    _ => Err(FruityError::FunctionExpected(format!(
                        "Couldn't convert {:?} to native callback ",
                        value
                    ))),
                }
            }
        }
    };
}

macro_rules! impl_for_fn {
    ($($arg:ident),*) => {
        impl_try_into_script_value_for_fn!($($arg),*);
        impl_try_from_script_value_for_fn!($($arg),*);
    };
}

impl_for_fn!();
impl_for_fn!(T1);
impl_for_fn!(T1, T2);
impl_for_fn!(T1, T2, T3);
impl_for_fn!(T1, T2, T3, T4);
impl_for_fn!(T1, T2, T3, T4, T5);
impl_for_fn!(T1, T2, T3, T4, T5, T6);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
impl_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
impl_for_fn!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
