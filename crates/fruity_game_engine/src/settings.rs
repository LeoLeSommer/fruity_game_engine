use crate::any::FruityAny;
use crate::any_value::AnyValue;
use crate::convert::FruityFrom;
use crate::convert::FruityInto;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

/// Settings collection
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
  pub fn get<T: FruityFrom<Settings> + ?Sized>(&self, key: &str, default: T) -> T {
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

impl FruityInto<AnyValue> for Settings {
  fn fruity_into(self) -> FruityResult<AnyValue> {
    Ok(match self {
      Settings::F64(value) => AnyValue::F64(value),
      Settings::Bool(value) => AnyValue::Bool(value),
      Settings::String(value) => AnyValue::String(value),
      Settings::Array(value) => AnyValue::Array(
        value
          .into_iter()
          .map(|elem| elem.fruity_into())
          .try_collect::<Vec<_>>()?,
      ),
      Settings::Object(value) => AnyValue::Object {
        class_name: "unknown".to_string(),
        fields: value
          .into_iter()
          .map(|(key, elem)| Ok((key, elem.fruity_into()?)) as FruityResult<(String, AnyValue)>)
          .try_collect::<HashMap<_, _>>()?,
      },
      Settings::Null => AnyValue::Null,
    })
  }
}

impl FruityFrom<AnyValue> for Settings {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    Ok(match value {
      AnyValue::I8(value) => Settings::F64(value as f64),
      AnyValue::I16(value) => Settings::F64(value as f64),
      AnyValue::I32(value) => Settings::F64(value as f64),
      AnyValue::I64(value) => Settings::F64(value as f64),
      AnyValue::ISize(value) => Settings::F64(value as f64),
      AnyValue::U8(value) => Settings::F64(value as f64),
      AnyValue::U16(value) => Settings::F64(value as f64),
      AnyValue::U32(value) => Settings::F64(value as f64),
      AnyValue::U64(value) => Settings::F64(value as f64),
      AnyValue::USize(value) => Settings::F64(value as f64),
      AnyValue::F32(value) => Settings::F64(value as f64),
      AnyValue::F64(value) => Settings::F64(value as f64),
      AnyValue::Bool(value) => Settings::Bool(value),
      AnyValue::String(value) => Settings::String(value),
      AnyValue::Array(value) => Settings::Array(
        value
          .into_iter()
          .map(|elem| FruityFrom::<AnyValue>::fruity_try_from(elem).unwrap())
          .collect::<Vec<_>>(),
      ),
      AnyValue::Null => Settings::Null,
      AnyValue::Undefined => Settings::Null,
      AnyValue::Iterator(_) => unimplemented!(),
      AnyValue::Callback(_) => unimplemented!(),
      AnyValue::Object { fields, .. } => Settings::Object(
        fields
          .into_iter()
          .map(|(key, elem)| (key, FruityFrom::<AnyValue>::fruity_try_from(elem).unwrap()))
          .collect::<HashMap<_, _>>(),
      ),
      AnyValue::NativeObject(_) => unimplemented!(),
    })
  }
}

/// Build a Settings by reading a yaml document
pub fn read_settings(reader: &mut dyn Read) -> Settings {
  let mut buffer = String::new();
  if let Err(err) = reader.read_to_string(&mut buffer) {
    log::error!("{}", err.to_string());
    return Settings::Object(HashMap::new());
  }

  let docs = YamlLoader::load_from_str(&buffer).unwrap();
  let root = &docs[0];

  if let Some(settings) = build_settings_from_yaml(root) {
    settings
  } else {
    return Settings::Object(HashMap::new());
  }
}

/// Build a Settings by reading a yaml document
pub fn read_settings_from_path(path: String) -> Settings {
  let mut file = File::open(&path).unwrap();
  read_settings(&mut file)
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
    impl FruityFrom<Settings> for $type {
      fn fruity_try_from(value: Settings) -> FruityResult<Self> {
        match value {
          Settings::F64(value) => Ok(value as $type),
          _ => Err(FruityError::new(
            FruityStatus::NumberExpected,
            format!("Couldn't convert {:?} to {}", value, "$type"),
          )),
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

impl FruityFrom<Settings> for bool {
  fn fruity_try_from(value: Settings) -> FruityResult<Self> {
    match value {
      Settings::Bool(value) => Ok(value),
      _ => Err(FruityError::new(
        FruityStatus::BooleanExpected,
        format!("Couldn't convert {:?} to bool", value),
      )),
    }
  }
}

impl FruityFrom<Settings> for String {
  fn fruity_try_from(value: Settings) -> FruityResult<Self> {
    match value {
      Settings::String(value) => Ok(value),
      _ => Err(FruityError::new(
        FruityStatus::StringExpected,
        format!("Couldn't convert {:?} to string", value),
      )),
    }
  }
}

impl FruityFrom<Settings> for Settings {
  fn fruity_try_from(value: Settings) -> FruityResult<Self> {
    Ok(value)
  }
}

impl<T: FruityFrom<Settings> + ?Sized> FruityFrom<Settings> for Vec<T> {
  fn fruity_try_from(value: Settings) -> FruityResult<Self> {
    match value {
      Settings::Array(value) => Ok(
        value
          .into_iter()
          .filter_map(|elem| T::fruity_try_from(elem).ok())
          .collect(),
      ),
      _ => Err(FruityError::new(
        FruityStatus::ArrayExpected,
        format!("Couldn't convert {:?} to array", value),
      )),
    }
  }
}

impl<T: FruityFrom<Settings> + ?Sized> FruityFrom<Settings> for Option<T> {
  fn fruity_try_from(value: Settings) -> FruityResult<Self> {
    Ok(T::fruity_try_from(value).ok())
  }
}
