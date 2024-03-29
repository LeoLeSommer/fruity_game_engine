use super::{ScriptValue, TryFromScriptValue, TryIntoScriptValue};
use crate::{utils::ArgumentCaster, FruityError, FruityResult};

impl TryIntoScriptValue for () {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Undefined)
    }
}

impl TryFromScriptValue for () {
    fn from_script_value(_value: ScriptValue) -> FruityResult<Self> {
        Ok(())
    }
}

impl<T: TryIntoScriptValue, U: TryIntoScriptValue> TryIntoScriptValue for (T, U) {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(vec![
            self.0.into_script_value()?,
            self.1.into_script_value()?,
        ]))
    }
}

impl<T1: TryFromScriptValue, T2: TryFromScriptValue> TryFromScriptValue for (T1, T2) {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Array(args) => {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let arg2 = caster.cast_next::<T2>()?;

                Ok((arg1, arg2))
            }
            value => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to tuple",
                value
            ))),
        }
    }
}
