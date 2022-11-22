use super::{IntrospectObjectClone, ScriptValue};
use crate::convert::{FruityFrom, FruityInto};
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::rc::Rc;

impl<R: FruityFrom<ScriptValue>> FruityFrom<ScriptValue> for Rc<dyn Fn() -> FruityResult<R>> {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move || {
                let args: Vec<ScriptValue> = vec![];
                let result = value(args)?;

                <R>::fruity_from(result)
            })),
            _ => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to native callback ", value),
            )),
        }
    }
}

impl<T1: FruityInto<ScriptValue>, R: FruityFrom<ScriptValue> + IntrospectObjectClone>
    FruityFrom<ScriptValue> for Rc<dyn Fn(T1) -> FruityResult<R>>
{
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move |arg1| {
                let args: Vec<ScriptValue> = vec![arg1.fruity_into()?];
                let result = value(args)?;

                <R>::fruity_from(result.fruity_into()?)
            })),
            _ => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to native callback ", value),
            )),
        }
    }
}

impl<T1: FruityFrom<ScriptValue>, R: FruityInto<ScriptValue>> FruityInto<ScriptValue>
    for &'static (dyn Fn(T1) -> R + Send + Sync)
{
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Callback(Rc::new(|mut args| {
            let arg1 = <T1 as FruityFrom<ScriptValue>>::fruity_from(args.remove(0))?;
            let result = self(arg1);

            result.fruity_into()
        })))
    }
}

impl<T1: FruityInto<ScriptValue>, T2: FruityInto<ScriptValue>, R: FruityFrom<ScriptValue>>
    FruityFrom<ScriptValue> for Rc<dyn Fn(T1, T2) -> FruityResult<R>>
{
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(Rc::new(move |arg1, arg2| {
                let args: Vec<ScriptValue> = vec![arg1.fruity_into()?, arg2.fruity_into()?];
                let result = value(args)?;

                <R>::fruity_from(result)
            })),
            _ => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to native callback ", value),
            )),
        }
    }
}
