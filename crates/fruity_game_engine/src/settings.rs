use crate::any::FruityAny;
use crate::export_function;
use crate::introspect::IntrospectFields;
use crate::introspect::IntrospectMethods;
use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::convert::TryIntoScriptValue;
use crate::script_value::ScriptValue;
use crate::FruityError;
use crate::FruityResult;
use napi::JsUnknown;
use napi::NapiValue;
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

/// Build a Settings by reading a yaml document
#[export_function]
pub fn read_settings(path: String) -> FruityResult<Settings> {
    // Open the file
    let mut reader = File::open(&path)
        .map_err(|_| FruityError::GenericFailure(format!("File couldn't be opened: {:?}", path)))?;

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .map_err(|_| FruityError::GenericFailure(format!("File couldn't be read: {:?}", path)))?;

    // Parse the yaml
    let docs = YamlLoader::load_from_str(&buffer)
        .map_err(|_| FruityError::GenericFailure(format!("Incorrect Yaml file: {:?}", path)))?;

    let root = &docs[0];

    // Build the setting object
    Ok(if let Some(settings) = build_settings_from_yaml(root) {
        settings
    } else {
        Settings::Object(HashMap::new())
    })
}
/*
#[doc(hidden)]
#[allow(non_snake_case)]
extern "C" fn __napi__read_settings(
    raw_env: napi::bindgen_prelude::sys::napi_env,
    cb: napi::bindgen_prelude::sys::napi_callback_info,
) -> napi::bindgen_prelude::sys::napi_value {
    unsafe {
        let env = napi::Env::from_raw(raw_env);
        napi::bindgen_prelude::CallbackInfo::<2usize>::new(raw_env, cb, None)
            .and_then(|cb| {
                let arg0 = {
                    let arg = crate::javascript::js_value_to_script_value(
                        &env,
                        napi::JsUnknown::from_raw(raw_env, cb.get_arg(0usize))?,
                    )
                    .map_err(|e| e.into_napi())?;
                    <String>::from_script_value(arg).map_err(|e| e.into_napi())?
                };

                napi::bindgen_prelude::within_runtime_if_available(move || {
                    let _ret = { read_settings(arg0) };
                    let _ret = <FruityResult<Settings>>::into_script_value(_ret)
                        .map_err(|e| e.into_napi())?;
                    let _ret = crate::javascript::script_value_to_js_value(&env, _ret)
                        .map_err(|e| e.into_napi())?;

                    <JsUnknown as napi::bindgen_prelude::ToNapiValue>::to_napi_value(raw_env, _ret)
                })
            })
            .unwrap_or_else(|e| {
                napi::bindgen_prelude::JsError::from(e).throw_into(raw_env);
                std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
            })
    }
}

#[doc(hidden)]
#[allow(dead_code)]
unsafe fn read_settings_js_function(
    raw_env: napi::bindgen_prelude::sys::napi_env,
) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
    let mut fn_ptr = std::ptr::null_mut();
    {
        let c = napi::bindgen_prelude::sys::napi_create_function(
            raw_env,
            "readSettings\0".as_ptr() as *const _,
            13usize,
            Some(__napi__read_settings),
            std::ptr::null_mut(),
            &mut fn_ptr,
        );
        match c {
            ::napi::sys::Status::napi_ok => Ok(()),
            _ => Err(::napi::Error::new(
                ::napi::Status::from(c),
                format!("Failed to register function `{}`", "readSettings"),
            )),
        }
    }?;
    napi::bindgen_prelude::register_js_function(
        "readSettings\0",
        read_settings_js_function,
        Some(__napi__read_settings),
    );
    Ok(fn_ptr)
}

#[doc(hidden)]
#[allow(non_snake_case)]
extern "C" fn __napi_register__read_settings() {
    napi::bindgen_prelude::register_module_export(
        None,
        "readSettings\0",
        read_settings_js_function,
    );
}

#[used]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[doc(hidden)]
#[link_section = "__DATA,__mod_init_func"]
static __napi_register__read_settings___rust_ctor___ctor: unsafe extern "C" fn() = {
    unsafe extern "C" fn __napi_register__read_settings___rust_ctor___ctor() {
        __napi_register__read_settings()
    }
    __napi_register__read_settings___rust_ctor___ctor
};*/

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
            ScriptValue::Iterator(_) => unimplemented!(),
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
