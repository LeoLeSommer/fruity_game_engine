use crate::any::FruityAny;
use crate::convert::FruityFrom;
use crate::script_value::ScriptValue;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::iter::Enumerate;
use std::vec::IntoIter as VecIntoIter;

/// Cast an any introspect object
///
/// # Arguments
/// * `any` - The introspect object as an any reference
///
pub fn cast_introspect_ref<T: FruityAny>(any: &dyn FruityAny) -> FruityResult<&T> {
    match any.as_any_ref().downcast_ref::<T>() {
        Some(value) => Ok(value),
        None => Err(FruityError::new(
            FruityStatus::CallbackScopeMismatch,
            format!(
                "Failed to get the native value wrapped into a js object, expected {}, got {}",
                std::any::type_name::<T>(),
                any.get_type_name()
            ),
        )),
    }
}

/// Cast an any introspect object with mutability
///
/// # Arguments
/// * `any` - The introspect object as an any mutable reference
///
pub fn cast_introspect_mut<T: FruityAny>(any: &mut dyn FruityAny) -> FruityResult<&mut T> {
    let any_type_name = any.get_type_name();

    match any.as_any_mut().downcast_mut::<T>() {
        Some(value) => Ok(value),
        None => Err(FruityError::new(
            FruityStatus::CallbackScopeMismatch,
            format!(
                "Failed to get the native value wrapped into a js object, expected {}, got {}",
                std::any::type_name::<T>(),
                any_type_name
            ),
        )),
    }
}

/// A tool that is used to cast serialized arguments, intended to be used into IntrospectMethod implementations
pub struct ArgumentCaster {
    args_count: usize,
    iter: Enumerate<VecIntoIter<ScriptValue>>,
    last_index: usize,
}

impl ArgumentCaster {
    /// Return an ArgumentCaster
    pub fn new<'a>(args: Vec<ScriptValue>) -> ArgumentCaster {
        ArgumentCaster {
            args_count: args.len(),
            iter: args.into_iter().enumerate(),
            last_index: 1,
        }
    }

    /// Get all the remaining script value arguments from an argument list
    pub fn rest(&mut self) -> Vec<ScriptValue> {
        let mut result = Vec::new();
        while let Some(elem) = self.iter.next() {
            result.push(elem.1);
        }

        result
    }

    /// Cast a script value argument from an argument list
    ///
    /// # Generic Arguments
    /// * `T` - The type to cast
    ///
    pub fn cast_next<T: FruityFrom<ScriptValue> + ?Sized>(&mut self) -> FruityResult<T> {
        let test = std::any::type_name::<T>();

        match self.iter.next() {
            Some((index, arg)) => {
                self.last_index = index + 1;
                T::fruity_from(arg)
            }
            None => T::fruity_from(ScriptValue::Undefined).map_err(|_| {
                FruityError::new(
                    FruityStatus::InvalidArg,
                    format!(
                        "Wrong number of arguments, you provided {} and we expect {}",
                        self.last_index,
                        self.args_count + 1
                    ),
                )
            }),
        }
    }
}
