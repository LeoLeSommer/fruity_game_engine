use crate::any::FruityAny;
use crate::introspect::IntrospectFields;
use crate::introspect::IntrospectMethods;
use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::convert::TryIntoScriptValue;
use crate::script_value::ScriptValue;
use crate::typescript;
use crate::FruityError;
use crate::FruityResult;
use std::collections::HashMap;

/// Settings collection
#[typescript(
    "type SettingsElem =
  | number
  | boolean
  | string
  | SettingsElem[]
  | Settings
  | null"
)]
#[typescript("type Settings = { [key: string]: SettingsElem }")]
#[derive(Debug, Clone, FruityAny)]
pub enum Settings {
    /// f64 value
    F64(f64),

    /// bool value
    Bool(bool),

    /// String value
    String(String),

    /// Array of values
    Array(Vec<Settings>),

    /// An object stored as an hashmap, mostly used to grab objects from the scripting runtime
    Object(HashMap<String, Settings>),

    /// null value
    Null,
}

impl Settings {
    /// Get a field into the params
    ///
    /// # Arguments
    /// * `key` - The field identifier
    /// * `default` - The default value, if not found or couldn't serialize
    ///
    /// # Generic Arguments
    /// * `T` - The type to cast the value
    ///
    pub fn get<T: TryFrom<Settings> + ?Sized>(&self, key: &str, default: T) -> T {
        match self {
            Settings::Object(fields) => match fields.get(key) {
                Some(value) => T::try_from(value.clone()).unwrap_or(default),
                None => default,
            },
            _ => default,
        }
    }

    /// Get a field into the params as settings
    ///
    /// # Arguments
    /// * `key` - The field identifier
    ///
    pub fn get_settings(&self, key: &str) -> Settings {
        match self {
            Settings::Object(fields) => match fields.get(key) {
                Some(value) => value.clone(),
                None => Settings::default(),
            },
            _ => Settings::default(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::Null
    }
}

macro_rules! impl_numeric_from_settings {
    ( $type:ident ) => {
        impl TryFrom<Settings> for $type {
            type Error = FruityError;

            fn try_from(value: Settings) -> FruityResult<Self> {
                match value {
                    Settings::F64(value) => Ok(value as $type),
                    _ => Err(FruityError::NumberExpected(format!(
                        "Couldn't convert {:?} to {}",
                        value, "$type"
                    ))),
                }
            }
        }
    };
}

impl_numeric_from_settings!(i8);
impl_numeric_from_settings!(i16);
impl_numeric_from_settings!(i32);
impl_numeric_from_settings!(i64);
impl_numeric_from_settings!(isize);
impl_numeric_from_settings!(u8);
impl_numeric_from_settings!(u16);
impl_numeric_from_settings!(u32);
impl_numeric_from_settings!(u64);
impl_numeric_from_settings!(usize);
impl_numeric_from_settings!(f32);
impl_numeric_from_settings!(f64);

impl TryFrom<Settings> for bool {
    type Error = FruityError;

    fn try_from(value: Settings) -> FruityResult<Self> {
        match value {
            Settings::Bool(value) => Ok(value),
            _ => Err(FruityError::BooleanExpected(format!(
                "Couldn't convert {:?} to bool",
                value
            ))),
        }
    }
}

impl TryFrom<Settings> for String {
    type Error = FruityError;

    fn try_from(value: Settings) -> FruityResult<Self> {
        match value {
            Settings::String(value) => Ok(value),
            _ => Err(FruityError::StringExpected(format!(
                "Couldn't convert {:?} to string",
                value
            ))),
        }
    }
}

impl<T: TryFrom<Settings> + ?Sized> TryFrom<Settings> for Vec<T> {
    type Error = FruityError;

    fn try_from(value: Settings) -> FruityResult<Self> {
        match value {
            Settings::Array(value) => Ok(value
                .into_iter()
                .filter_map(|elem| T::try_from(elem).ok())
                .collect()),
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to array",
                value
            ))),
        }
    }
}

impl TryFrom<Settings> for Option<String> {
    type Error = FruityError;

    fn try_from(value: Settings) -> FruityResult<Self> {
        match String::try_from(value) {
            Ok(e) => Ok(Some(e)),
            Err(_) => Ok(None),
        }
    }
}

impl TryIntoScriptValue for Settings {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(match self {
            Settings::F64(value) => ScriptValue::F64(value),
            Settings::Bool(value) => ScriptValue::Bool(value),
            Settings::String(value) => ScriptValue::String(value.clone()),
            Settings::Array(value) => ScriptValue::Array(
                value
                    .into_iter()
                    .map(|elem| elem.into_script_value())
                    .try_collect::<Vec<_>>()?,
            ),
            Settings::Object(value) => ScriptValue::Object(Box::new(SettingsHashMap(value))),
            Settings::Null => ScriptValue::Null,
        })
    }
}

impl TryFromScriptValue for Settings {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        Ok(match value {
            ScriptValue::I8(value) => Settings::F64(value as f64),
            ScriptValue::I16(value) => Settings::F64(value as f64),
            ScriptValue::I32(value) => Settings::F64(value as f64),
            ScriptValue::I64(value) => Settings::F64(value as f64),
            ScriptValue::ISize(value) => Settings::F64(value as f64),
            ScriptValue::U8(value) => Settings::F64(value as f64),
            ScriptValue::U16(value) => Settings::F64(value as f64),
            ScriptValue::U32(value) => Settings::F64(value as f64),
            ScriptValue::U64(value) => Settings::F64(value as f64),
            ScriptValue::USize(value) => Settings::F64(value as f64),
            ScriptValue::F32(value) => Settings::F64(value as f64),
            ScriptValue::F64(value) => Settings::F64(value as f64),
            ScriptValue::Bool(value) => Settings::Bool(value),
            ScriptValue::String(value) => Settings::String(value.clone()),
            ScriptValue::Array(value) => Settings::Array(
                value
                    .into_iter()
                    .map(|elem| TryFromScriptValue::from_script_value(elem))
                    .try_collect::<Vec<_>>()?,
            ),
            ScriptValue::Null => Settings::Null,
            ScriptValue::Undefined => Settings::Null,
            ScriptValue::Future(_) => unimplemented!(),
            ScriptValue::Callback(_) => unimplemented!(),
            ScriptValue::Object(value) => Settings::Object(
                value
                    .get_field_names()?
                    .into_iter()
                    .map(|name| {
                        let field_value = value.get_field_value(&name)?;
                        TryFromScriptValue::from_script_value(field_value)
                            .map(|value| (name, value))
                    })
                    .try_collect::<HashMap<_, _>>()?,
            ),
        })
    }
}

#[derive(Debug, Clone, FruityAny)]
struct SettingsHashMap(HashMap<String, Settings>);

//#[typegen = "type SettingsHashMap = unknown"]
impl IntrospectFields for SettingsHashMap {
    fn get_class_name(&self) -> FruityResult<String> {
        Ok("Settings".to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(self.0.keys().map(|key| key.clone()).collect())
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        let value = <Settings>::from_script_value(value)?;
        self.0.entry(name.to_string()).or_insert_with(|| value);

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.0
            .get(name)
            .unwrap_or_else(|| unreachable!())
            .clone()
            .into_script_value()
    }
}

impl IntrospectMethods for SettingsHashMap {
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
