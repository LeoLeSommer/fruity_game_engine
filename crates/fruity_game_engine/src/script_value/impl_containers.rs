use super::IntrospectObjectClone;
use super::ScriptValue;
use crate::convert::FruityFrom;
use crate::convert::FruityInto;
use crate::introspect::IntrospectObject;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use crate::RwLock;
use std::collections::HashMap;
use std::rc::Rc;

impl<T: IntrospectObjectClone> FruityInto<ScriptValue> for T {
  fn fruity_into(self) -> FruityResult<ScriptValue> {
    Ok(ScriptValue::NativeObject(Box::new(self)))
  }
}

impl<T: IntrospectObject + ?Sized> FruityFrom<ScriptValue> for RwLock<Box<T>> {
  fn fruity_try_from(value: ScriptValue) -> FruityResult<Self> {
    match value {
      ScriptValue::NativeObject(value) => match value.as_any_box().downcast::<RwLock<Box<T>>>() {
        Ok(value) => Ok(*value),
        _ => Err(FruityError::new(
          FruityStatus::InvalidArg,
          format!("Couldn't convert a ScriptValue to native object"),
        )),
      },
      _ => Err(FruityError::new(
        FruityStatus::InvalidArg,
        format!("Couldn't convert {:?} to native object", value),
      )),
    }
  }
}

impl<T: IntrospectObject + ?Sized> FruityFrom<ScriptValue> for Rc<T> {
  fn fruity_try_from(value: ScriptValue) -> FruityResult<Self> {
    match value {
      ScriptValue::NativeObject(value) => match value.as_any_box().downcast::<Rc<T>>() {
        Ok(value) => Ok(*value),
        _ => Err(FruityError::new(
          FruityStatus::InvalidArg,
          format!("Couldn't convert a ScriptValue to native object"),
        )),
      },
      _ => Err(FruityError::new(
        FruityStatus::InvalidArg,
        format!("Couldn't convert {:?} to native object", value),
      )),
    }
  }
}

impl<T: FruityInto<ScriptValue>> FruityInto<ScriptValue> for Vec<T> {
  fn fruity_into(self) -> FruityResult<ScriptValue> {
    Ok(ScriptValue::Array(
      self
        .into_iter()
        .map(|elem| elem.fruity_into().unwrap())
        .collect::<Vec<_>>(),
    ))
  }
}

impl<T: FruityInto<ScriptValue>> FruityInto<ScriptValue> for Option<T> {
  fn fruity_into(self) -> FruityResult<ScriptValue> {
    match self {
      Some(value) => value.fruity_into(),
      None => Ok(ScriptValue::Null),
    }
  }
}

impl<T: FruityFrom<ScriptValue>> FruityFrom<ScriptValue> for Option<T> {
  fn fruity_try_from(value: ScriptValue) -> FruityResult<Self> {
    if let ScriptValue::Null = value {
      Ok(None)
    } else {
      T::fruity_try_from(value).map(|value| Some(value))
    }
  }
}

impl<T: FruityFrom<ScriptValue>> FruityFrom<ScriptValue> for HashMap<String, T> {
  fn fruity_try_from(value: ScriptValue) -> FruityResult<Self> {
    if let ScriptValue::Object { fields, .. } = value {
      let mut result = HashMap::<String, T>::new();

      fields.into_iter().for_each(|(key, value)| {
        if let Some(value) = T::fruity_try_from(value).ok() {
          result.insert(key, value);
        }
      });

      Ok(result)
    } else {
      Err(FruityError::new(
        FruityStatus::ObjectExpected,
        format!("Couldn't convert {:?} to HashMap", value),
      ))
    }
  }
}

impl<T: FruityInto<ScriptValue>> FruityInto<ScriptValue> for HashMap<String, T> {
  fn fruity_into(self) -> FruityResult<ScriptValue> {
    let mut fields = HashMap::<String, ScriptValue>::new();

    self.into_iter().for_each(|(key, value)| {
      fields.insert(key, value.fruity_into().unwrap());
    });

    Ok(ScriptValue::Object {
      class_name: "unknown".to_string(),
      fields,
    })
  }
}
