use super::ScriptValue;
use crate::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use crate::utils::introspect::ArgumentCaster;
use crate::Arc;
use crate::FruityError;
use crate::FruityResult;
use std::future::Future;
use std::pin::Pin;

impl<R: TryIntoScriptValue> TryIntoScriptValue for &'static (dyn Send + Sync + Fn() -> R) {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Arc::new(Box::new(|_| {
            let result = self();

            result.into_script_value()
        })
            as Box<
                dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>,
            >)))
    }
}

impl<T1: TryFromScriptValue, R: TryIntoScriptValue> TryIntoScriptValue
    for &'static (dyn Send + Sync + Fn(T1) -> R)
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Arc::new(
            Box::new(|args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;

                let result = self(arg1);

                result.into_script_value()
            })
                as Box<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}

impl<R: TryFromScriptValue + 'static> TryFromScriptValue
    for Box<dyn FnOnce() -> FruityResult<R> + Send + Sync + 'static>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        let arc_callback =
            <Arc<dyn Fn() -> FruityResult<R> + Send + Sync>>::from_script_value(value)?;

        Ok(Box::new(move || {
            let result = arc_callback();
            result
        }))
    }
}

impl<R: TryFromScriptValue> TryFromScriptValue for Arc<dyn Send + Sync + Fn() -> FruityResult<R>> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Arc::new(move || {
                let args: Vec<ScriptValue> = vec![];
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

impl<
        T1: TryIntoScriptValue + Send + 'static,
        T2: TryIntoScriptValue + Send + 'static,
        R: TryFromScriptValue,
    > TryFromScriptValue
    for Arc<dyn Send + Sync + Fn(T1, T2) -> Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Arc::new(move |arg1: T1, arg2: T2| {
                let value = value.clone();
                Box::pin(async move {
                    let args: Vec<ScriptValue> =
                        vec![arg1.into_script_value()?, arg2.into_script_value()?];

                    let result = value(args)?;
                    <Pin<Box<dyn Send + Future<Output = FruityResult<R>>>>>::from_script_value(
                        result,
                    )?
                    .await
                })
            })),
            _ => Err(FruityError::FunctionExpected(format!(
                "Couldn't convert {:?} to native callback ",
                value
            ))),
        }
    }
}

impl<T1: TryIntoScriptValue, R: TryFromScriptValue> TryFromScriptValue
    for Arc<dyn Send + Sync + Fn(T1) -> FruityResult<R>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Arc::new(move |arg1| {
                let args: Vec<ScriptValue> = vec![arg1.into_script_value()?];
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

impl<T1: TryFromScriptValue + 'static, R: TryIntoScriptValue + 'static> TryIntoScriptValue
    for Arc<dyn Send + Sync + Fn(T1) -> R>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Arc::new(
            Box::new(move |args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;

                let result = self(arg1);

                result.into_script_value()
            })
                as Box<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}

impl<T1: TryIntoScriptValue, T2: TryIntoScriptValue, R: TryFromScriptValue> TryFromScriptValue
    for Arc<dyn Send + Sync + Fn(T1, T2) -> FruityResult<R>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Arc::new(move |arg1, arg2| {
                let args: Vec<ScriptValue> =
                    vec![arg1.into_script_value()?, arg2.into_script_value()?];
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

impl<
        T1: TryFromScriptValue + 'static,
        T2: TryFromScriptValue + 'static,
        R: TryIntoScriptValue + 'static,
    > TryIntoScriptValue for Arc<dyn Send + Sync + Fn(T1, T2) -> R>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Arc::new(
            Box::new(move |args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let arg2 = caster.cast_next::<T2>()?;

                let result = self(arg1, arg2);

                result.into_script_value()
            })
                as Box<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}

impl<
        T1: TryIntoScriptValue,
        T2: TryIntoScriptValue,
        T3: TryIntoScriptValue,
        R: TryFromScriptValue,
    > TryFromScriptValue for Arc<dyn Send + Sync + Fn(T1, T2, T3) -> FruityResult<R>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Arc::new(move |arg1, arg2, arg3| {
                let args: Vec<ScriptValue> = vec![
                    arg1.into_script_value()?,
                    arg2.into_script_value()?,
                    arg3.into_script_value()?,
                ];
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

impl<
        T1: TryFromScriptValue + 'static,
        T2: TryFromScriptValue + 'static,
        T3: TryFromScriptValue + 'static,
        R: TryIntoScriptValue + 'static,
    > TryIntoScriptValue for Arc<dyn Send + Sync + Fn(T1, T2, T3) -> R>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Arc::new(
            Box::new(move |args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let arg2 = caster.cast_next::<T2>()?;
                let arg3 = caster.cast_next::<T3>()?;

                let result = self(arg1, arg2, arg3);

                result.into_script_value()
            })
                as Box<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}
