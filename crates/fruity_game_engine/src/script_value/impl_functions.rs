use super::{ScriptCallback, ScriptValue};
use crate::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use crate::utils::introspect::ArgumentCaster;
use crate::FruityError;
use crate::FruityResult;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

impl ScriptCallback for Box<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>> {
    fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self(args)
    }

    fn create_thread_safe_callback(
        &self,
    ) -> FruityResult<std::sync::Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
        unimplemented!()
    }
}

impl<R: TryIntoScriptValue> TryIntoScriptValue for &'static (dyn Fn() -> R) {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Rc::new(Box::new(|_| {
            let result = self();

            result.into_script_value()
        })
            as Box<
                dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>,
            >)))
    }
}

impl<T1: TryFromScriptValue, R: TryIntoScriptValue> TryIntoScriptValue
    for &'static (dyn Fn(T1) -> R)
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Rc::new(
            Box::new(|args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;

                let result = self(arg1);

                result.into_script_value()
            }) as Box<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}

impl<T1: TryFromScriptValue, R: TryIntoScriptValue> ScriptCallback for &'static (dyn Fn(T1) -> R) {
    fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        let mut caster = ArgumentCaster::new(args);
        let arg1 = caster.cast_next::<T1>()?;

        let result = self(arg1);

        result.into_script_value()
    }

    fn create_thread_safe_callback(
        &self,
    ) -> FruityResult<std::sync::Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
        unimplemented!()
    }
}

impl<R: TryFromScriptValue> TryFromScriptValue for Rc<dyn Fn() -> FruityResult<R>> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move || {
                let args: Vec<ScriptValue> = vec![];
                let result = value.call(args)?;

                <R>::from_script_value(result)
            })),
            _ => Err(FruityError::FunctionExpected(format!(
                "Couldn't convert {:?} to native callback ",
                value
            ))),
        }
    }
}

impl<T1: TryIntoScriptValue, R: TryFromScriptValue> TryFromScriptValue
    for Rc<dyn Fn(T1) -> FruityResult<R>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move |arg1| {
                let args: Vec<ScriptValue> = vec![arg1.into_script_value()?];
                let result = value.call(args)?;

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
    for Rc<dyn Fn(T1) -> R>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Rc::new(
            Box::new(move |args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;

                let result = self(arg1);

                result.into_script_value()
            }) as Box<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}

impl<T1: TryIntoScriptValue, T2: TryIntoScriptValue, R: TryFromScriptValue> TryFromScriptValue
    for Rc<dyn Fn(T1, T2) -> FruityResult<R>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move |arg1, arg2| {
                let args: Vec<ScriptValue> =
                    vec![arg1.into_script_value()?, arg2.into_script_value()?];
                let result = value.call(args)?;

                <R>::from_script_value(result)
            })),
            _ => Err(FruityError::FunctionExpected(format!(
                "Couldn't convert {:?} to native callback ",
                value
            ))),
        }
    }
}

/*impl<
        T1: TryFromScriptValue + 'static,
        T2: TryFromScriptValue + 'static,
        R: TryIntoScriptValue + 'static,
    > TryIntoScriptValue
    for Rc<dyn Fn(T1, T2) -> Pin<Box<dyn Future<Output = FruityResult<R>>>>>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Rc::new(
            Box::new(move |args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let arg2 = caster.cast_next::<T2>()?;

                let result = self(arg1, arg2);

                result.into_script_value()
            }) as Box<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}*/

impl<T1: TryIntoScriptValue, T2: TryIntoScriptValue, R: TryFromScriptValue> TryFromScriptValue
    for Rc<dyn Fn(T1, T2) -> Pin<Box<dyn Future<Output = FruityResult<R>>>>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        // TODO: Better catch errors
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move |arg1: T1, arg2: T2| {
                let args: Vec<ScriptValue> = vec![
                    arg1.into_script_value().unwrap(),
                    arg2.into_script_value().unwrap(),
                ];

                let result = value.call(args).unwrap();
                <Pin<Box<dyn Future<Output = FruityResult<R>>>>>::from_script_value(result).unwrap()
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
    > TryIntoScriptValue for Rc<dyn Fn(T1, T2) -> R>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Rc::new(
            Box::new(move |args: Vec<ScriptValue>| {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let arg2 = caster.cast_next::<T2>()?;

                let result = self(arg1, arg2);

                result.into_script_value()
            }) as Box<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
        )))
    }
}

impl<T1: TryFromScriptValue, T2: TryFromScriptValue, R: TryIntoScriptValue> ScriptCallback
    for &'static (dyn Fn(T1, T2) -> R)
{
    fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        let mut caster = ArgumentCaster::new(args);
        let arg1 = caster.cast_next::<T1>()?;
        let arg2 = caster.cast_next::<T2>()?;

        let result = self(arg1, arg2);

        result.into_script_value()
    }

    fn create_thread_safe_callback(
        &self,
    ) -> FruityResult<std::sync::Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
        unimplemented!()
    }
}
