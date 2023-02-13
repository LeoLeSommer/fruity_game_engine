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
use std::hash::Hash;

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
        match value {
            ScriptValue::Object(value) => match value.downcast::<T>() {
                Ok(value) => Ok(*value),
                Err(value) => Err(FruityError::InvalidArg(format!(
                    "Couldn't convert a {} to {}",
                    value.get_type_name(),
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

#[derive(Debug, Clone, FruityAny)]
struct ScriptValueHashMap(HashMap<String, ScriptValue>);

//#[typegen = "type ScriptValueHashMap = unknown"]
impl IntrospectFields for ScriptValueHashMap {
    fn get_class_name(&self) -> FruityResult<String> {
        Ok("unknown".to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(self.0.keys().map(|key| key.clone()).collect())
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.0.entry(name.to_string()).or_insert_with(|| value);

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        Ok(self.0.get(name).unwrap_or_else(|| unreachable!()).clone())
    }
}

impl IntrospectMethods for ScriptValueHashMap {
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

impl<T: TryIntoScriptValue> TryIntoScriptValue for HashMap<String, T> {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(ScriptValueHashMap(
            self.into_iter()
                .map(|(key, value)| FruityResult::Ok((key, value.into_script_value()?)))
                .try_collect::<HashMap<_, _>>()?,
        ))))
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
