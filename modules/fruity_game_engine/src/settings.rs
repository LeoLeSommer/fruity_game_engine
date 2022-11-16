use crate::any::FruityAny;
use crate::convert::FruityTryFrom;
use crate::resource::Resource;
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

impl crate::javascript::ToJavascript for Settings {
  fn to_js<'a>(
    self,
    cx: &'a mut neon::prelude::FunctionContext,
  ) -> neon::result::NeonResult<neon::handle::Handle<'a, neon::prelude::JsValue>> {
    use neon::context::Context;
    use neon::object::Object;
    use neon::prelude::Value;

    Ok(match self {
      Settings::F64(value) => cx.number(value).as_value(cx),
      Settings::Bool(value) => cx.boolean(value).as_value(cx),
      Settings::String(value) => cx.string(value).as_value(cx),
      Settings::Array(value) => {
        let array = cx.empty_array();

        // TODO: Find a way to remove this
        let cx_2 = unsafe {
          std::mem::transmute::<
            &mut neon::prelude::FunctionContext,
            &mut neon::prelude::FunctionContext,
          >(cx)
        };

        for (index, elem) in value.into_iter().enumerate() {
          array.set(cx, index as u32, elem.to_js(cx_2)?)?;
        }

        array.as_value(cx)
      }
      Settings::Object(value) => {
        let object = cx.empty_object();

        // TODO: Find a way to remove this
        let cx_2 = unsafe {
          std::mem::transmute::<
            &mut neon::prelude::FunctionContext,
            &mut neon::prelude::FunctionContext,
          >(cx)
        };

        for (key, elem) in value.into_iter() {
          object.set(cx, key.as_str(), elem.to_js(cx_2)?)?;
        }

        object.as_value(cx)
      }
      Settings::Null => neon::prelude::JsNull::new(cx).as_value(cx),
    })
  }
}

impl crate::javascript::FromJavascript for Settings {
  fn from_js(
    value: neon::handle::Handle<neon::prelude::JsValue>,
    cx: &mut neon::prelude::FunctionContext,
  ) -> neon::result::NeonResult<Self> {
    use neon::object::Object;

    Ok(
      if let Ok(value) =
        value.downcast::<neon::prelude::JsNumber, neon::prelude::FunctionContext>(cx)
      {
        Settings::F64(value.value(cx))
      } else if let Ok(value) =
        value.downcast::<neon::prelude::JsBoolean, neon::prelude::FunctionContext>(cx)
      {
        Settings::Bool(value.value(cx))
      } else if let Ok(value) =
        value.downcast::<neon::prelude::JsString, neon::prelude::FunctionContext>(cx)
      {
        Settings::String(value.value(cx))
      } else if let Ok(value) =
        value.downcast::<neon::prelude::JsArray, neon::prelude::FunctionContext>(cx)
      {
        let mut result = Vec::<Settings>::new();

        for index in [0, value.len(cx)] {
          result.push(Self::from_js(value.get(cx, index.clone())?, cx)?);
        }

        Settings::Array(result)
      } else if let Ok(value) =
        value.downcast::<neon::prelude::JsObject, neon::prelude::FunctionContext>(cx)
      {
        let mut result = HashMap::<String, Settings>::new();

        let js_keys = value.get_own_property_names(cx)?;
        let keys = [0, js_keys.len(cx)]
          .iter()
          .map(|index| {
            js_keys
              .get::<neon::prelude::JsString, neon::prelude::FunctionContext, u32>(
                cx,
                index.clone(),
              )
              .unwrap()
              .value(cx)
          })
          .collect::<Vec<_>>();

        for key in keys.into_iter() {
          result.insert(
            key.clone(),
            Self::from_js(value.get(cx, key.as_str())?, cx)?,
          );
        }

        Settings::Object(result)
      } else {
        Settings::default()
      },
    )
  }
}
