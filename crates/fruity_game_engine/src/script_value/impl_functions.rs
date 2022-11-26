use super::{ScriptCallback, ScriptValue};
use crate::introspect::IntrospectObject;
use crate::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
use crate::utils::introspect::ArgumentCaster;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
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

impl TryFromScriptValue for Rc<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move |args| value.call(args))),
            _ => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to native callback ", value),
            )),
        }
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
            _ => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to native callback ", value),
            )),
        }
    }
}

impl<T1: TryIntoScriptValue, R: TryFromScriptValue + IntrospectObject> TryFromScriptValue
    for Rc<dyn Fn(T1) -> FruityResult<R>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move |arg1| {
                let args: Vec<ScriptValue> = vec![arg1.into_script_value()?];
                let result = value.call(args)?;

                <R>::from_script_value(result.into_script_value()?)
            })),
            value => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to native callback ", value),
            )),
        }
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
            _ => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to native callback ", value),
            )),
        }
    }
}
