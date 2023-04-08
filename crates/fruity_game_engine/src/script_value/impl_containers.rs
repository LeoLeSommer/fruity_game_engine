use super::ScriptObject;
use super::ScriptValue;
use crate::any::FruityAny;
use crate::introspect::IntrospectFields;
use crate::introspect::IntrospectMethods;
use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::convert::TryIntoScriptValue;
use crate::FruityError;
use crate::FruityResult;
use std::any::type_name;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::ops::Range;
use std::pin::Pin;

impl<T> TryIntoScriptValue for FruityResult<T>
where
    T: TryIntoScriptValue,
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        match self {
            Ok(value) => <T as TryIntoScriptValue>::into_script_value(value),
            Err(err) => Err(err.clone()),
        }
    }
}

impl<T: TryIntoScriptValue + 'static> TryIntoScriptValue
    for Pin<Box<dyn Send + Future<Output = FruityResult<T>>>>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        let future = async move {
            let result = self.await;
            result.into_script_value()
        };

        let future =
            Box::pin(future) as Pin<Box<dyn Send + Future<Output = FruityResult<ScriptValue>>>>;
        Ok(ScriptValue::Future(future))
    }
}

impl<T: TryFromScriptValue> TryFromScriptValue
    for Pin<Box<dyn Send + Future<Output = FruityResult<T>>>>
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        if let ScriptValue::Future(future) = value {
            Ok(Box::pin(async move {
                let result = future.await?;
                T::from_script_value(result)
            }))
        } else {
            Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to future",
                value
            )))
        }
    }
}

impl<T> TryFromScriptValue for FruityResult<T>
where
    T: TryFromScriptValue,
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        Ok(<T as TryFromScriptValue>::from_script_value(value))
    }
}

impl<T: ScriptObject> TryIntoScriptValue for T {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self)))
    }
}

impl<T> TryFromScriptValue for T
where
    T: ScriptObject,
{
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        use lazy_static::__Deref;

        match value {
            ScriptValue::Object(value) => match value.downcast::<T>() {
                Ok(value) => Ok(*value),
                Err(value) => Err(FruityError::InvalidArg(format!(
                    "Couldn't convert a {} to {}",
                    value.deref().get_type_name(),
                    type_name::<T>()
                ))),
            },
            value => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to native object",
                value
            ))),
        }
    }
}

impl<T: TryIntoScriptValue + Clone> TryIntoScriptValue for &'static [T] {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(
            self.iter()
                .map(|elem| elem.clone().into_script_value())
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: TryIntoScriptValue> TryIntoScriptValue for Vec<T> {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(
            self.into_iter()
                .map(|elem| elem.into_script_value())
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: TryIntoScriptValue> TryIntoScriptValue for HashSet<T> {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(
            self.into_iter()
                .map(|elem| elem.into_script_value())
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: TryFromScriptValue + Eq + Hash> TryFromScriptValue for HashSet<T> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Array(value) => value
                .into_iter()
                .map(|elem| T::from_script_value(elem))
                .try_collect::<HashSet<_>>(),
            value => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to native object",
                value
            ))),
        }
    }
}

/// A script value object stored as an hashmap
#[derive(Debug, Clone, FruityAny)]
pub struct ScriptValueHashMap<T: TryIntoScriptValue + TryFromScriptValue + Clone + Debug + 'static>
{
    /// Class name
    pub class_name: String,
    /// Fields
    pub fields: HashMap<String, T>,
}

impl<T: TryIntoScriptValue + TryFromScriptValue + Clone + Debug + 'static> IntrospectFields
    for ScriptValueHashMap<T>
{
    fn is_static(&self) -> FruityResult<bool> {
        Ok(false)
    }

    fn get_class_name(&self) -> FruityResult<String> {
        Ok(self.class_name.clone())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(self.fields.keys().map(|key| key.clone()).collect())
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.fields
            .entry(name.to_string())
            .or_insert_with(|| T::from_script_value(value).unwrap());

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        Ok(self
            .fields
            .get(name)
            .unwrap_or_else(|| unreachable!())
            .clone()
            .into_script_value()?)
    }
}

impl<T: TryIntoScriptValue + TryFromScriptValue + Clone + Debug + 'static> IntrospectMethods
    for ScriptValueHashMap<T>
{
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_const_method(&self, _name: &str, _args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        unreachable!()
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_mut_method(
        &mut self,
        _name: &str,
        _args: Vec<ScriptValue>,
    ) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}

impl<T: TryIntoScriptValue + TryFromScriptValue + Clone + Debug + Send + Sync + 'static>
    TryIntoScriptValue for HashMap<String, T>
{
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(ScriptValueHashMap {
            class_name: type_name::<Self>().to_string(),
            fields: self,
        })))
    }
}

impl<T: TryFromScriptValue> TryFromScriptValue for HashMap<String, T> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        if let ScriptValue::Object(value) = value {
            let mut result = HashMap::<String, T>::new();

            value.get_field_names()?.into_iter().try_for_each(|name| {
                let field_value = value.get_field_value(&name)?;
                result.insert(name, T::from_script_value(field_value)?);

                FruityResult::Ok(())
            })?;

            Ok(result)
        } else {
            Err(FruityError::ObjectExpected(format!(
                "Couldn't convert {:?} to HashMap",
                value
            )))
        }
    }
}

impl<T: TryIntoScriptValue> TryIntoScriptValue for Option<T> {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        match self {
            Some(value) => value.into_script_value(),
            None => Ok(ScriptValue::Null),
        }
    }
}

impl<T: TryFromScriptValue> TryFromScriptValue for Option<T> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        Ok(match value {
            ScriptValue::Null => None,
            ScriptValue::Undefined => None,
            _ => T::from_script_value(value).map(|value| Some(value))?,
        })
    }
}

impl<T: TryIntoScriptValue> TryIntoScriptValue for Range<T> {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(vec![
            self.start.into_script_value()?,
            self.end.into_script_value()?,
        ]))
    }
}

impl<T: TryFromScriptValue> TryFromScriptValue for Range<T> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Array(mut value) => {
                if value.len() == 2 {
                    Ok(Range {
                        start: T::from_script_value(value.remove(0))?,
                        end: T::from_script_value(value.remove(0))?,
                    })
                } else {
                    Err(FruityError::ArrayExpected(format!(
                        "Couldn't convert {:?} to range",
                        value
                    )))
                }
            }
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} range",
                value
            ))),
        }
    }
}
