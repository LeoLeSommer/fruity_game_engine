#![warn(missing_docs)]

//! ScriptValue
//!
//! Provide a structure that will be used all over the application to send data to scripting
//! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
//! data storage

use crate::any::FruityAny;
use crate::introspect::IntrospectFields;
use crate::introspect::IntrospectMethods;
use crate::script_value::convert::TryFromScriptValue;
use crate::typescript;
use crate::FruityError;
use crate::FruityResult;
use futures::future::Shared;
use lazy_static::__Deref;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Traits similar to TryInto and TryFrom for ScriptValue
pub mod convert;

/// Implementation of script value conversions for primitives
pub mod impl_primitives;

/// Implementation of script value conversions for functions
pub mod impl_functions;

/// Implementation of script value conversions for containers (like Vec, Box ...)
pub mod impl_containers;

/// Implementation of script value conversions for tuples
pub mod impl_tuples;

/// a script value
#[typescript("type ScriptValue = any")]
pub enum ScriptValue {
    /// i8 value
    I8(i8),

    /// i16 value
    I16(i16),

    /// i32 value
    I32(i32),

    /// i64 value
    I64(i64),

    /// isize value
    ISize(isize),

    /// u8 value
    U8(u8),

    /// u16 value
    U16(u16),

    /// u32 value
    U32(u32),

    /// u64 value
    U64(u64),

    /// usize value
    USize(usize),

    /// f32 value
    F32(f32),

    /// f64 value
    F64(f64),

    /// bool value
    Bool(bool),

    /// String value
    String(String),

    /// Array of values
    Array(Vec<ScriptValue>),

    /// A null value, correspond to [’Option::None’]
    Null,

    /// A null value, correspond to ()
    Undefined,

    /// A future
    Future(Shared<Pin<Box<dyn Send + Future<Output = FruityResult<ScriptValue>>>>>),

    /// A callback
    Callback(Arc<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>),

    /// An object created by rust
    Object(Box<dyn ScriptObject>),
}

impl<T: TryFromScriptValue + ?Sized> TryFromScriptValue for Vec<T> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Array(value) => Ok(value
                .into_iter()
                .filter_map(|elem| T::from_script_value(elem).ok())
                .collect()),
            _ => Err(FruityError::ArrayExpected(format!(
                "Couldn't convert {:?} to array",
                value
            ))),
        }
    }
}

/// A trait that can be implemented for an object storable in a ScriptValue
pub trait ScriptObject: IntrospectFields + IntrospectMethods + Send + Sync {
    /// Duplicate the script object
    fn duplicate(&self) -> Box<dyn ScriptObject>;
}

impl dyn ScriptObject {
    /// Downcast a script object like an Any could do, the only difference is the err returns
    pub fn downcast<T: Any>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        let any = self.deref().as_any_ref();
        if any.is::<T>() {
            unsafe { Ok(self.as_any_box().downcast_unchecked::<T>()) }
        } else {
            Err(self)
        }
    }
}

impl<T> ScriptObject for T
where
    T: Clone + IntrospectFields + IntrospectMethods + Send + Sync,
{
    fn duplicate(&self) -> Box<dyn ScriptObject> {
        Box::new(self.clone())
    }
}

impl Debug for ScriptValue {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ScriptValue::I8(value) => value.fmt(formatter),
            ScriptValue::I16(value) => value.fmt(formatter),
            ScriptValue::I32(value) => value.fmt(formatter),
            ScriptValue::I64(value) => value.fmt(formatter),
            ScriptValue::ISize(value) => value.fmt(formatter),
            ScriptValue::U8(value) => value.fmt(formatter),
            ScriptValue::U16(value) => value.fmt(formatter),
            ScriptValue::U32(value) => value.fmt(formatter),
            ScriptValue::U64(value) => value.fmt(formatter),
            ScriptValue::USize(value) => value.fmt(formatter),
            ScriptValue::F32(value) => value.fmt(formatter),
            ScriptValue::F64(value) => value.fmt(formatter),
            ScriptValue::Bool(value) => value.fmt(formatter),
            ScriptValue::String(value) => value.fmt(formatter),
            ScriptValue::Array(value) => value.fmt(formatter),
            ScriptValue::Null => formatter.write_str("null"),
            ScriptValue::Undefined => formatter.write_str("undefined"),
            ScriptValue::Future(_) => formatter.write_str("future"),
            ScriptValue::Callback(_) => formatter.write_str("function"),
            ScriptValue::Object(value) => value.fmt(formatter),
        }
    }
}

impl Clone for ScriptValue {
    fn clone(&self) -> Self {
        match self {
            Self::I8(value) => Self::I8(value.clone()),
            Self::I16(value) => Self::I16(value.clone()),
            Self::I32(value) => Self::I32(value.clone()),
            Self::I64(value) => Self::I64(value.clone()),
            Self::ISize(value) => Self::ISize(value.clone()),
            Self::U8(value) => Self::U8(value.clone()),
            Self::U16(value) => Self::U16(value.clone()),
            Self::U32(value) => Self::U32(value.clone()),
            Self::U64(value) => Self::U64(value.clone()),
            Self::USize(value) => Self::USize(value.clone()),
            Self::F32(value) => Self::F32(value.clone()),
            Self::F64(value) => Self::F64(value.clone()),
            Self::Bool(value) => Self::Bool(value.clone()),
            Self::String(value) => Self::String(value.clone()),
            Self::Array(value) => Self::Array(value.clone()),
            Self::Null => Self::Null,
            Self::Undefined => Self::Undefined,
            Self::Future(value) => Self::Future(value.clone()),
            Self::Callback(value) => Self::Callback(value.clone()),
            Self::Object(value) => Self::Object(value.duplicate()),
        }
    }
}

/// A script object implemented from an hashmap of script values
#[derive(Debug, Clone, FruityAny)]
pub struct HashMapScriptObject {
    /// The object class name
    pub class_name: String,
    /// The object field values
    pub fields: HashMap<String, ScriptValue>,
}

//#[typegen = "type HashMapScriptObject = unknown"]
impl IntrospectFields for HashMapScriptObject {
    fn get_class_name(&self) -> FruityResult<String> {
        Ok(self.class_name.clone())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(self.fields.keys().map(|key| key.clone()).collect())
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.fields.entry(name.to_string()).or_insert_with(|| value);

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        Ok(self
            .fields
            .get(name)
            .unwrap_or_else(|| unreachable!())
            .clone())
    }
}

impl IntrospectMethods for HashMapScriptObject {
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
