use super::ScriptValue;
use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::convert::TryIntoScriptValue;
use crate::utils::introspect::ArgumentCaster;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;

impl TryIntoScriptValue for () {
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Undefined)
    }
}

impl TryFromScriptValue for () {
    fn from_script_value(_value: &ScriptValue) -> FruityResult<Self> {
        Ok(())
    }
}

impl<T: TryIntoScriptValue, U: TryIntoScriptValue> TryIntoScriptValue for (T, U) {
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(vec![
            self.0.into_script_value()?,
            self.1.into_script_value()?,
        ]))
    }
}

impl<T1: TryFromScriptValue, T2: TryFromScriptValue> TryFromScriptValue for (T1, T2) {
    fn from_script_value(value: &ScriptValue) -> FruityResult<Self> {
        match value.clone() {
            ScriptValue::Array(args) => {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let arg2 = caster.cast_next::<T2>()?;

                Ok((arg1, arg2))
            }
            _ => Err(FruityError::new(
                FruityStatus::ArrayExpected,
                format!("Couldn't convert {:?} to tuple", value),
            )),
        }
    }
}
