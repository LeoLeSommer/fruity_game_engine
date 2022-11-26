use super::HashMapScriptObject;
use super::ScriptObject;
use super::ScriptValue;
use crate::convert::FruityFrom;
use crate::convert::FruityInto;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::collections::HashMap;

impl<T> FruityInto<ScriptValue> for FruityResult<T>
where
    T: FruityInto<ScriptValue>,
{
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        match self {
            Ok(value) => <T as FruityInto<ScriptValue>>::fruity_into(value),
            Err(err) => Err(err),
        }
    }
}

impl<T> FruityFrom<ScriptValue> for FruityResult<T>
where
    T: FruityFrom<ScriptValue>,
{
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        Ok(<T as FruityFrom<ScriptValue>>::fruity_from(value))
    }
}

impl<T: ScriptObject> FruityInto<ScriptValue> for T {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self)))
    }
}

impl<T> FruityFrom<ScriptValue> for T
where
    T: ScriptObject,
{
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.as_any_box().downcast::<T>() {
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

impl<T: FruityInto<ScriptValue> + Clone> FruityInto<ScriptValue> for &'static [T] {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(
            self.iter()
                .map(|elem| elem.clone().fruity_into())
                .try_collect::<Vec<_>>()?,
        ))
    }
}

impl<T: FruityInto<ScriptValue>> FruityInto<ScriptValue> for Vec<T> {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Array(
            self.into_iter()
                .map(|elem| elem.fruity_into())
                .try_collect::<Vec<_>>()?,
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
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        Ok(match value {
            ScriptValue::Null => None,
            ScriptValue::Undefined => None,
            _ => T::fruity_from(value).map(|value| Some(value))?,
        })
    }
}

impl<T: FruityFrom<ScriptValue>> FruityFrom<ScriptValue> for HashMap<String, T> {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        if let ScriptValue::Object(value) = value {
            let mut result = HashMap::<String, T>::new();

            value.get_field_names()?.into_iter().try_for_each(|name| {
                let field_value = value.get_field_value(&name)?;
                result.insert(name, T::fruity_from(field_value)?);

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

impl<T: FruityInto<ScriptValue>> FruityInto<ScriptValue> for HashMap<String, T> {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(HashMapScriptObject {
            class_name: "unknown".to_string(),
            fields: self
                .into_iter()
                .map(|(key, value)| value.fruity_into().map(|value| (key, value)))
                .try_collect()?,
        })))
    }
}
