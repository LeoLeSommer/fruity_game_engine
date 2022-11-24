use super::IntrospectObjectClone;
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

impl<T: IntrospectObjectClone> FruityInto<ScriptValue> for T {
    fn fruity_into(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::NativeObject(Box::new(self)))
    }
}

impl<T> FruityFrom<ScriptValue> for T
where
    T: IntrospectObjectClone,
{
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::NativeObject(value) => match value.as_any_box().downcast::<T>() {
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
        if let ScriptValue::Object { fields, .. } = value {
            let mut result = HashMap::<String, T>::new();

            fields.into_iter().for_each(|(key, value)| {
                if let Some(value) = T::fruity_from(value).ok() {
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
        Ok(ScriptValue::Object {
            class_name: "unknown".to_string(),
            fields: self
                .into_iter()
                .map(|(key, value)| value.fruity_into().map(|value| (key, value)))
                .try_collect()?,
        })
    }
}
