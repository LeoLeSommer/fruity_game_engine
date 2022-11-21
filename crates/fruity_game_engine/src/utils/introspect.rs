use crate::any_value::AnyValue;
use crate::convert::FruityFrom;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::any::Any;
use std::iter::Enumerate;
use std::vec::IntoIter as VecIntoIter;

/// Cast an any introspect object
///
/// # Arguments
/// * `any` - The introspect object as an any reference
///
pub fn cast_introspect_ref<T: Any>(any: &dyn Any) -> FruityResult<&T> {
  match any.downcast_ref::<T>() {
    Some(value) => Ok(value),
    None => Err(FruityError::new(
      FruityStatus::CallbackScopeMismatch,
      format!("Failed to get the native value wrapped into a js object"),
    )),
  }
}

/// Cast an any introspect object with mutability
///
/// # Arguments
/// * `any` - The introspect object as an any mutable reference
///
pub fn cast_introspect_mut<T: Any>(any: &mut dyn Any) -> FruityResult<&mut T> {
  match any.downcast_mut::<T>() {
    Some(value) => Ok(value),
    None => Err(FruityError::new(
      FruityStatus::CallbackScopeMismatch,
      format!("Failed to get the native value wrapped into a js object"),
    )),
  }
}

/// A tool that is used to cast serialized arguments, intended to be used into IntrospectMethod implementations
pub struct ArgumentCaster {
  args_count: usize,
  iter: Enumerate<VecIntoIter<AnyValue>>,
  last_index: usize,
}

impl ArgumentCaster {
  /// Return an ArgumentCaster
  pub fn new<'a>(args: Vec<AnyValue>) -> ArgumentCaster {
    ArgumentCaster {
      args_count: args.len(),
      iter: args.into_iter().enumerate(),
      last_index: 1,
    }
  }

  /// Get an any value argument from an argument list
  pub fn next(&mut self) -> FruityResult<AnyValue> {
    match self.iter.next() {
      Some((index, arg)) => {
        self.last_index = index + 1;
        Ok(arg)
      }
      None => Err(FruityError::new(
        FruityStatus::InvalidArg,
        format!(
          "Wrong number of arguments, you provided {} and we expect {}",
          self.last_index, self.args_count
        ),
      )),
    }
  }

  /// Get all the remaining any value arguments from an argument list
  pub fn rest(&mut self) -> Vec<AnyValue> {
    let mut result = Vec::new();
    while let Some(elem) = self.iter.next() {
      result.push(elem.1);
    }

    result
  }

  /// Cast an any value argument from an argument list
  ///
  /// # Generic Arguments
  /// * `T` - The type to cast
  ///
  pub fn cast_next<T: FruityFrom<AnyValue> + ?Sized>(&mut self) -> FruityResult<T> {
    match self.iter.next() {
      Some((index, arg)) => {
        self.last_index = index + 1;
        T::fruity_try_from(arg)
      }
      None => Err(FruityError::new(
        FruityStatus::InvalidArg,
        format!(
          "Wrong number of arguments, you provided {} and we expect {}",
          self.last_index, self.args_count
        ),
      )),
    }
  }
}
