use super::AnyValue;
use super::IntrospectObjectClone;
use crate::convert::FruityFrom;
use crate::convert::FruityInto;
use crate::introspect::IntrospectObject;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use crate::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

impl<T: IntrospectObjectClone> FruityInto<AnyValue> for T {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::NativeObject(Box::new(self)))
  }
}

impl<T: IntrospectObject + ?Sized> FruityFrom<AnyValue> for RwLock<Box<T>> {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::NativeObject(value) => match value.as_any_box().downcast::<RwLock<Box<T>>>() {
        Ok(value) => Ok(*value),
        _ => Err(FruityError::new(
          FruityStatus::InvalidArg,
          format!("Couldn't convert a AnyValue to native object"),
        )),
      },
      _ => Err(FruityError::new(
        FruityStatus::InvalidArg,
        format!("Couldn't convert {:?} to native object", value),
      )),
    }
  }
}

impl<T: IntrospectObject + ?Sized> FruityFrom<AnyValue> for Arc<T> {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::NativeObject(value) => match value.as_any_box().downcast::<Arc<T>>() {
        Ok(value) => Ok(*value),
        _ => Err(FruityError::new(
          FruityStatus::InvalidArg,
          format!("Couldn't convert a AnyValue to native object"),
        )),
      },
      _ => Err(FruityError::new(
        FruityStatus::InvalidArg,
        format!("Couldn't convert {:?} to native object", value),
      )),
    }
  }
}

impl<T: FruityInto<AnyValue>> FruityInto<AnyValue> for Vec<T> {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::Array(
      self
        .into_iter()
        .map(|elem| elem.fruity_into().unwrap())
        .collect::<Vec<_>>(),
    ))
  }
}

impl<T: FruityInto<AnyValue>> FruityInto<AnyValue> for Option<T> {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    match self {
      Some(value) => value.fruity_into(),
      None => Ok(AnyValue::Null),
    }
  }
}

impl<T: FruityFrom<AnyValue> + 'static> FruityFrom<AnyValue> for Option<T> {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    if let AnyValue::Null = value {
      Ok(None)
    } else {
      T::fruity_try_from(value).map(|value| Some(value))
    }
  }
}

impl<T: FruityFrom<AnyValue> + 'static> FruityFrom<AnyValue> for HashMap<String, T> {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    if let AnyValue::Object { fields, .. } = value {
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

impl<T: FruityInto<AnyValue>> FruityInto<AnyValue> for HashMap<String, T> {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    let mut fields = HashMap::<String, AnyValue>::new();

    self.into_iter().for_each(|(key, value)| {
      fields.insert(key, value.fruity_into().unwrap());
    });

    Ok(AnyValue::Object {
      class_name: "unknown".to_string(),
      fields,
    })
  }
}
