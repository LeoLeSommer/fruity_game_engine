use crate::any::FruityAny;
use crate::convert::FruityTryFrom;
use crate::resource::Resource;
use napi::bindgen_prelude::Array;
use napi::bindgen_prelude::FromNapiValue;
use napi::bindgen_prelude::ToNapiValue;
use napi::Env;
use napi::JsBoolean;
use napi::JsNumber;
use napi::JsObject;
use napi::JsString;
use napi::JsUnknown;
use napi::NapiRaw;
use napi::NapiValue;
use napi::ValueType;
use napi_derive::napi;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

/// Settings collection
#[derive(Debug, Clone, FruityAny, Resource)]
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
  pub fn get<T: FruityTryFrom<Settings> + ?Sized>(&self, key: &str, default: T) -> T {
    match self {
      Settings::Object(fields) => match fields.get(key) {
        Some(value) => T::fruity_try_from(value.clone()).unwrap_or(default),
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

/// Build a Settings by reading a yaml document
#[napi]
pub fn read_settings() -> Settings {
  let mut reader = File::open("examples/test/settings.yaml").unwrap();

  let mut buffer = String::new();
  if let Err(err) = reader.read_to_string(&mut buffer) {
    log::error!("{}", err.to_string());
    return Settings::default();
  }

  let docs = YamlLoader::load_from_str(&buffer).unwrap();
  let root = &docs[0];

  if let Some(settings) = build_settings_from_yaml(root) {
    settings
  } else {
    return Settings::default();
  }
}

/// Build a Settings by reading a yaml document
pub fn build_settings_from_yaml(yaml: &Yaml) -> Option<Settings> {
  match yaml {
    Yaml::Real(string) => match string.parse::<f64>() {
      Ok(value) => Some(Settings::F64(value)),
      Err(_) => None,
    },
    Yaml::Integer(value) => Some(Settings::F64(*value as f64)),
    Yaml::String(value) => Some(Settings::String(value.clone())),
    Yaml::Boolean(value) => Some(Settings::Bool(*value)),
    Yaml::Array(array) => {
      let settings_array = array
        .iter()
        .filter_map(|elem| build_settings_from_yaml(elem))
        .collect::<Vec<_>>();

      Some(Settings::Array(settings_array))
    }
    Yaml::Hash(hashmap) => {
      let mut fields = HashMap::new();

      for (key, value) in hashmap {
        if let Yaml::String(key) = key {
          if let Some(settings) = build_settings_from_yaml(value) {
            fields.insert(key.clone(), settings);
          }
        }
      }

      Some(Settings::Object(fields))
    }
    Yaml::Alias(_) => None,
    Yaml::Null => None,
    Yaml::BadValue => None,
  }
}

macro_rules! impl_numeric_from_settings {
  ( $type:ident ) => {
    impl FruityTryFrom<Settings> for $type {
      type Error = String;

      fn fruity_try_from(value: Settings) -> Result<Self, Self::Error> {
        match value {
          Settings::F64(value) => Ok(value as $type),
          _ => Err(format!("Couldn't convert {:?} to {}", value, "$type")),
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

impl FruityTryFrom<Settings> for bool {
  type Error = String;

  fn fruity_try_from(value: Settings) -> Result<Self, Self::Error> {
    match value {
      Settings::Bool(value) => Ok(value),
      _ => Err(format!("Couldn't convert {:?} to bool", value)),
    }
  }
}

impl FruityTryFrom<Settings> for String {
  type Error = String;

  fn fruity_try_from(value: Settings) -> Result<Self, Self::Error> {
    match value {
      Settings::String(value) => Ok(value),
      _ => Err(format!("Couldn't convert {:?} to bool", value)),
    }
  }
}

impl FruityTryFrom<Settings> for Settings {
  type Error = String;

  fn fruity_try_from(value: Settings) -> Result<Self, Self::Error> {
    Ok(value)
  }
}

impl<T: FruityTryFrom<Settings> + ?Sized> FruityTryFrom<Settings> for Vec<T> {
  type Error = String;

  fn fruity_try_from(value: Settings) -> Result<Self, Self::Error> {
    match value {
      Settings::Array(value) => Ok(
        value
          .into_iter()
          .filter_map(|elem| T::fruity_try_from(elem).ok())
          .collect(),
      ),
      _ => Err(format!("Couldn't convert {:?} to array", value)),
    }
  }
}

impl<T: FruityTryFrom<Settings> + ?Sized> FruityTryFrom<Settings> for Option<T> {
  type Error = String;

  fn fruity_try_from(value: Settings) -> Result<Self, Self::Error> {
    Ok(T::fruity_try_from(value).ok())
  }
}

impl ToNapiValue for Settings {
  unsafe fn to_napi_value(
    raw_env: *mut napi_sys::napi_env__,
    value: Self,
  ) -> Result<*mut napi_sys::napi_value__, napi::Error> {
    let env = Env::from(raw_env);

    match value {
      Settings::F64(value) => Ok(env.create_double(value)?.raw()),
      Settings::Bool(value) => Ok(env.get_boolean(value)?.raw()),
      Settings::String(value) => Ok(env.create_string_from_std(value)?.raw()),
      Settings::Array(value) => Array::to_napi_value(raw_env, Array::from_vec(&env, value)?),
      Settings::Object(value) => {
        let mut object = env.create_object()?;

        value.into_iter().for_each(|(key, value)| {
          object.set(key, value).unwrap();
        });

        Ok(object.raw())
      }
      Settings::Null => Ok(env.get_null()?.raw()),
    }
  }
}

impl FromNapiValue for Settings {
  unsafe fn from_napi_value(
    raw_env: *mut napi_sys::napi_env__,
    raw_value: *mut napi_sys::napi_value__,
  ) -> Result<Self, napi::Error> {
    let value = JsUnknown::from_raw(raw_env, raw_value)?;

    Ok(match value.get_type()? {
      ValueType::Number => {
        let value = JsNumber::from_raw(raw_env, raw_value)?;
        Settings::F64(value.get_double()?)
      }
      ValueType::Boolean => {
        let value = JsBoolean::from_raw(raw_env, raw_value)?;
        Settings::Bool(value.get_value()?)
      }
      ValueType::String => {
        let value = JsString::from_raw(raw_env, raw_value)?;
        Settings::String(value.into_utf8()?.into_owned()?)
      }
      ValueType::Object => {
        let value = JsObject::from_raw(raw_env, raw_value)?;

        if value.is_array()? {
          let value = Array::from_napi_value(raw_env, raw_value)?;
          let mut result = Vec::<Settings>::new();

          [0, value.len()]
            .into_iter()
            .for_each(|index| result.push(value.get(index).unwrap().unwrap()));

          Settings::Array(result)
        } else {
          let mut result = HashMap::<String, Settings>::new();

          JsObject::keys(&value)?.into_iter().for_each(|key| {
            result.insert(key.clone(), value.get(&key).unwrap().unwrap());
          });

          Settings::Object(result)
        }
      }
      _ => Settings::default(),
    })
  }
}
