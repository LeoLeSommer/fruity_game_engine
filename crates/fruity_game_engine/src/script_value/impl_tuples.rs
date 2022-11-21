use super::ScriptValue;
use crate::convert::FruityFrom;
use crate::convert::FruityInto;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;

impl FruityInto<ScriptValue> for () {
  fn fruity_into(self) -> FruityResult<ScriptValue> {
    Ok(ScriptValue::Undefined)
  }
}

impl FruityFrom<ScriptValue> for () {
  fn fruity_try_from(_value: ScriptValue) -> FruityResult<Self> {
    Ok(())
  }
}

impl<T: FruityInto<ScriptValue>, U: FruityInto<ScriptValue>> FruityInto<ScriptValue> for (T, U) {
  fn fruity_into(self) -> FruityResult<ScriptValue> {
    Ok(ScriptValue::Array(vec![
      self.0.fruity_into()?,
      self.1.fruity_into()?,
    ]))
  }
}

impl<T: FruityFrom<ScriptValue>, U: FruityFrom<ScriptValue>> FruityFrom<ScriptValue> for (T, U) {
  fn fruity_try_from(value: ScriptValue) -> FruityResult<Self> {
    match value {
      ScriptValue::Array(mut value) => {
        if value.len() < 2 {
          return Err(FruityError::new(
            FruityStatus::ArrayExpected,
            format!("Couldn't convert {:?} to tuple", value),
          ));
        };

        let value1 = if let Ok(value1) = T::fruity_try_from(value.remove(0)) {
          value1
        } else {
          return Err(FruityError::new(
            FruityStatus::ArrayExpected,
            format!("Couldn't convert {:?} to tuple", value),
          ));
        };

        let value2 = if let Ok(value2) = U::fruity_try_from(value.remove(0)) {
          value2
        } else {
          return Err(FruityError::new(
            FruityStatus::ArrayExpected,
            format!("Couldn't convert {:?} to tuple", value),
          ));
        };

        Ok((value1, value2))
      }
      _ => Err(FruityError::new(
        FruityStatus::ArrayExpected,
        format!("Couldn't convert {:?} to tuple", value),
      )),
    }
  }
}
