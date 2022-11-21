use super::{AnyValue, IntrospectObjectClone};
use crate::convert::{FruityFrom, FruityInto};
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::sync::Arc;

impl<R: FruityFrom<AnyValue> + 'static> FruityFrom<AnyValue> for Arc<dyn Fn() -> R + Sync + Send> {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::Callback(value) => Ok(Arc::new(move || {
        let args: Vec<AnyValue> = vec![];
        let result = value(args).unwrap();

        <R>::fruity_try_from(result).unwrap()
      })),
      _ => Err(FruityError::new(
        FruityStatus::FunctionExpected,
        format!("Couldn't convert {:?} to native callback ", value),
      )),
    }
  }
}

impl<T1: FruityInto<AnyValue>, R: FruityFrom<AnyValue> + IntrospectObjectClone + 'static>
  FruityFrom<AnyValue> for Arc<dyn Fn(T1) -> R + Sync + Send>
{
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::Callback(value) => Ok(Arc::new(move |arg1| {
        let args: Vec<AnyValue> = vec![arg1.fruity_into().unwrap()];
        let result = value(args).unwrap();

        <R>::fruity_try_from(result.fruity_into().unwrap()).unwrap()
      })),
      _ => Err(FruityError::new(
        FruityStatus::FunctionExpected,
        format!("Couldn't convert {:?} to native callback ", value),
      )),
    }
  }
}

impl<T1: FruityFrom<AnyValue>, R: FruityInto<AnyValue> + 'static> FruityInto<AnyValue>
  for &'static (dyn Fn(T1) -> R + Send + Sync)
{
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(AnyValue::Callback(Arc::new(|mut args| {
      let arg1 = <T1 as FruityFrom<AnyValue>>::fruity_try_from(args.remove(0)).unwrap();
      let result = self(arg1);

      result.fruity_into()
    })))
  }
}

impl<T1: FruityInto<AnyValue>, T2: FruityInto<AnyValue>, R: FruityFrom<AnyValue> + 'static>
  FruityFrom<AnyValue> for Arc<dyn Fn(T1, T2) -> R + Sync + Send>
{
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::Callback(value) => Ok(Arc::new(move |arg1, arg2| {
        let args: Vec<AnyValue> = vec![arg1.fruity_into().unwrap(), arg2.fruity_into().unwrap()];
        let result = value(args).unwrap();

        <R>::fruity_try_from(result).unwrap()
      })),
      _ => Err(FruityError::new(
        FruityStatus::FunctionExpected,
        format!("Couldn't convert {:?} to native callback ", value),
      )),
    }
  }
}
