use crate::convert::FruityFrom;
use crate::convert::FruityInto;
use crate::script_value::ScriptValue;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;

impl FruityFrom<ScriptValue> for ScriptValue {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        Ok(value)
    }
}

impl FruityInto<ScriptValue> for ScriptValue {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(self)
    }
}

macro_rules! impl_fruity_try_from_fruity_any_for_numeric {
    ( $type:ident ) => {
        impl FruityFrom<ScriptValue> for $type {
            fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
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
                    _ => Err(FruityError::new(
                        FruityStatus::NumberExpected,
                        format!("Couldn't convert {:?} to {}", value, "$type"),
                    )),
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

impl FruityFrom<ScriptValue> for bool {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Bool(value) => Ok(value),
            _ => Err(FruityError::new(
                FruityStatus::BooleanExpected,
                format!("Couldn't convert {:?} to bool", value),
            )),
        }
    }
}

impl FruityInto<ScriptValue> for &str {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::String(self.to_string()))
    }
}

impl FruityInto<ScriptValue> for String {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::String(self))
    }
}

impl FruityFrom<ScriptValue> for String {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::String(value) => Ok(value),
            _ => Err(FruityError::new(
                FruityStatus::StringExpected,
                format!("Couldn't convert {:?} to string", value),
            )),
        }
    }
}

impl FruityInto<ScriptValue> for i8 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I8(self))
    }
}

impl FruityInto<ScriptValue> for i16 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I16(self))
    }
}

impl FruityInto<ScriptValue> for i32 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I32(self))
    }
}

impl FruityInto<ScriptValue> for i64 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::I64(self))
    }
}

impl FruityInto<ScriptValue> for isize {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::ISize(self))
    }
}

impl FruityInto<ScriptValue> for u8 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U8(self))
    }
}

impl FruityInto<ScriptValue> for u16 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U16(self))
    }
}

impl FruityInto<ScriptValue> for u32 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U32(self))
    }
}

impl FruityInto<ScriptValue> for u64 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::U64(self))
    }
}

impl FruityInto<ScriptValue> for usize {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::USize(self))
    }
}

impl FruityInto<ScriptValue> for f32 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::F32(self))
    }
}

impl FruityInto<ScriptValue> for f64 {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::F64(self))
    }
}

impl FruityInto<ScriptValue> for bool {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Bool(self))
    }
}
