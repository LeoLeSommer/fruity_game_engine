use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::ScriptValue;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::iter::Enumerate;
use std::vec::IntoIter as VecIntoIter;

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
    pub fn cast_next<T: TryFromScriptValue + ?Sized>(&mut self) -> FruityResult<T> {
        match self.iter.next() {
            Some((index, arg)) => {
                self.last_index = index + 1;
                T::from_script_value(&arg)
            }
            None => T::from_script_value(&ScriptValue::Undefined).map_err(|_| {
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
