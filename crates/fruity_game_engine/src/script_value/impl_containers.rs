use super::HashMapScriptObject;
use super::ScriptObject;
use super::ScriptValue;
use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::convert::TryIntoScriptValue;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::collections::HashMap;

impl<T> TryIntoScriptValue for FruityResult<T>
where
    T: TryIntoScriptValue,
{
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        match self {
            Ok(value) => <T as TryIntoScriptValue>::into_script_value(value),
            Err(err) => Err(err.clone()),
        }
    }
}

impl<T> TryFromScriptValue for FruityResult<T>
where
    T: TryFromScriptValue,
{
    fn from_script_value(value: &ScriptValue) -> FruityResult<Self> {
        Ok(<T as TryFromScriptValue>::from_script_value(value))
    }
}

impl<T: ScriptObject> TryIntoScriptValue for T {
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self).duplicate()))
    }
}

impl<T> TryFromScriptValue for T
where
    T: ScriptObject,
{
    fn from_script_value(value: &ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.duplicate().as_any_box().downcast::<T>() {
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

impl<T: TryIntoScriptValue + Clone> TryIntoScriptValue for &'static [T] {
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(
            self.iter()
                .map(|elem| elem.clone().into_script_value())
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: TryIntoScriptValue> TryIntoScriptValue for Vec<T> {
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(
            self.into_iter()
                .map(|elem| elem.into_script_value())
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: TryIntoScriptValue> TryIntoScriptValue for Option<T> {
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        match self {
            Some(value) => value.into_script_value(),
            None => Ok(ScriptValue::Null),
        }
    }
}

impl<T: TryFromScriptValue> TryFromScriptValue for Option<T> {
    fn from_script_value(value: &ScriptValue) -> FruityResult<Self> {
        Ok(match value {
            ScriptValue::Null => None,
            ScriptValue::Undefined => None,
            _ => T::from_script_value(value).map(|value| Some(value))?,
        })
    }
}

impl<T: TryFromScriptValue> TryFromScriptValue for HashMap<String, T> {
    fn from_script_value(value: &ScriptValue) -> FruityResult<Self> {
        if let ScriptValue::Object(value) = value {
            let mut result = HashMap::<String, T>::new();

            value.get_field_names()?.into_iter().try_for_each(|name| {
                let field_value = value.get_field_value(&name)?;
                result.insert(name, T::from_script_value(&field_value)?);

                FruityResult::Ok(())
            })?;

            Ok(result)
        } else {
            Err(FruityError::new(
                FruityStatus::ObjectExpected,
                format!("Couldn't convert {:?} to HashMap", value),
            ))
        }
    }
}

impl<T: TryIntoScriptValue> TryIntoScriptValue for HashMap<String, T> {
    fn into_script_value(&self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(HashMapScriptObject {
            class_name: "unknown".to_string(),
            fields: self
                .into_iter()
                .map(|(key, value)| value.into_script_value().map(|value| (key.clone(), value)))
                .try_collect()?,
        })))
    }
}
