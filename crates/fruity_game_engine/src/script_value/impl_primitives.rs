use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::convert::TryIntoScriptValue;
use crate::script_value::ScriptValue;
use crate::FruityError;
use crate::FruityResult;

impl TryFromScriptValue for ScriptValue {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        Ok(value)
    }
}

impl TryIntoScriptValue for ScriptValue {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(self)
    }
}

macro_rules! impl_fruity_try_from_fruity_any_for_numeric {
    ( $type:ident ) => {
        impl TryFromScriptValue for $type {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as $type),
                    ScriptValue::I16(value) => Ok(value as $type),
                    ScriptValue::I32(value) => Ok(value as $type),
                    ScriptValue::I64(value) => Ok(value as $type),
                    ScriptValue::ISize(value) => Ok(value as $type),
                    ScriptValue::U8(value) => Ok(value as $type),
                    ScriptValue::U16(value) => Ok(value as $type),
                    ScriptValue::U32(value) => Ok(value as $type),
                    ScriptValue::U64(value) => Ok(value as $type),
                    ScriptValue::USize(value) => Ok(value as $type),
                    ScriptValue::F32(value) => Ok(value as $type),
                    ScriptValue::F64(value) => Ok(value as $type),
                    _ => Err(FruityError::NumberExpected(format!(
                        "Couldn't convert {:?} to {}",
                        value, "$type"
                    ))),
                }
            }
        }
    };
}

impl_fruity_try_from_fruity_any_for_numeric!(i8);
impl_fruity_try_from_fruity_any_for_numeric!(i16);
impl_fruity_try_from_fruity_any_for_numeric!(i32);
impl_fruity_try_from_fruity_any_for_numeric!(i64);
impl_fruity_try_from_fruity_any_for_numeric!(isize);
impl_fruity_try_from_fruity_any_for_numeric!(u8);
impl_fruity_try_from_fruity_any_for_numeric!(u16);
impl_fruity_try_from_fruity_any_for_numeric!(u32);
impl_fruity_try_from_fruity_any_for_numeric!(u64);
impl_fruity_try_from_fruity_any_for_numeric!(usize);
impl_fruity_try_from_fruity_any_for_numeric!(f32);
impl_fruity_try_from_fruity_any_for_numeric!(f64);

impl TryFromScriptValue for bool {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Bool(value) => Ok(value),
            _ => Err(FruityError::BooleanExpected(format!(
                "Couldn't convert {:?} to bool",
                value
            ))),
        }
    }
}

impl TryIntoScriptValue for &str {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::String(self.to_string()))
    }
}

impl TryIntoScriptValue for String {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::String(self.clone()))
    }
}

impl TryFromScriptValue for String {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::String(value) => Ok(value.clone()),
            _ => Err(FruityError::StringExpected(format!(
                "Couldn't convert {:?} to string",
                value
            ))),
        }
    }
}

impl TryIntoScriptValue for i8 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I8(self))
    }
}

impl TryIntoScriptValue for i16 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I16(self))
    }
}

impl TryIntoScriptValue for i32 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I32(self))
    }
}

impl TryIntoScriptValue for i64 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I64(self))
    }
}

impl TryIntoScriptValue for isize {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::ISize(self))
    }
}

impl TryIntoScriptValue for u8 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U8(self))
    }
}

impl TryIntoScriptValue for u16 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U16(self))
    }
}

impl TryIntoScriptValue for u32 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U32(self))
    }
}

impl TryIntoScriptValue for u64 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U64(self))
    }
}

impl TryIntoScriptValue for usize {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::USize(self))
    }
}

impl TryIntoScriptValue for f32 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::F32(self))
    }
}

impl TryIntoScriptValue for f64 {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::F64(self))
    }
}

impl TryIntoScriptValue for bool {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Bool(self))
    }
}
