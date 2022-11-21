use crate::any_value::AnyValue;
use crate::convert::FruityFrom;
use crate::convert::FruityInto;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;

impl FruityFrom<AnyValue> for AnyValue {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    Ok(value)
  }
}

macro_rules! impl_fruity_try_from_fruity_any_for_numeric {
  ( $type:ident ) => {
    impl FruityFrom<AnyValue> for $type {
      fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
        match value {
          AnyValue::I8(value) => Ok(value as $type),
          AnyValue::I16(value) => Ok(value as $type),
          AnyValue::I32(value) => Ok(value as $type),
          AnyValue::I64(value) => Ok(value as $type),
          AnyValue::ISize(value) => Ok(value as $type),
          AnyValue::U8(value) => Ok(value as $type),
          AnyValue::U16(value) => Ok(value as $type),
          AnyValue::U32(value) => Ok(value as $type),
          AnyValue::U64(value) => Ok(value as $type),
          AnyValue::USize(value) => Ok(value as $type),
          AnyValue::F32(value) => Ok(value as $type),
          AnyValue::F64(value) => Ok(value as $type),
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

impl FruityFrom<AnyValue> for bool {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::Bool(value) => Ok(value),
      _ => Err(FruityError::new(
        FruityStatus::BooleanExpected,
        format!("Couldn't convert {:?} to bool", value),
      )),
    }
  }
}

impl FruityInto<AnyValue> for &str {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::String(self.to_string()))
  }
}

impl FruityInto<AnyValue> for String {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::String(self))
  }
}

impl FruityFrom<AnyValue> for String {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::String(value) => Ok(value),
      _ => Err(FruityError::new(
        FruityStatus::StringExpected,
        format!("Couldn't convert {:?} to bool", value),
      )),
    }
  }
}

impl FruityInto<AnyValue> for AnyValue {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(self)
  }
}

impl FruityInto<AnyValue> for i8 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::I8(self))
  }
}

impl FruityInto<AnyValue> for i16 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::I16(self))
  }
}

impl FruityInto<AnyValue> for i32 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::I32(self))
  }
}

impl FruityInto<AnyValue> for i64 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::I64(self))
  }
}

impl FruityInto<AnyValue> for isize {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::ISize(self))
  }
}

impl FruityInto<AnyValue> for u8 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::U8(self))
  }
}

impl FruityInto<AnyValue> for u16 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::U16(self))
  }
}

impl FruityInto<AnyValue> for u32 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::U32(self))
  }
}

impl FruityInto<AnyValue> for u64 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::U64(self))
  }
}

impl FruityInto<AnyValue> for usize {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::USize(self))
  }
}

impl FruityInto<AnyValue> for f32 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::F32(self))
  }
}

impl FruityInto<AnyValue> for f64 {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::F64(self))
  }
}

impl FruityInto<AnyValue> for bool {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::Bool(self))
  }
}
