#![warn(missing_docs)]

//! ScriptValue
//!
//! Provide a structure that will be used all over the application to send data to scripting
//! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
//! data storage

/// Implementation of script value conversions for primitives
pub mod impl_primitives;

/// Implementation of script value conversions for functions
pub mod impl_functions;

/// Implementation of script value conversions for containers (like Vec, Box ...)
pub mod impl_containers;

/// Implementation of script value conversions for tuples
pub mod impl_tuples;

/// Implementation of script value conversions for tuples
pub mod yaml;

use crate::convert::FruityFrom;
use crate::introspect::IntrospectObject;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use crate::RwLock;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

/// a script value
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

    /// Iterator over values
    Iterator(Rc<RwLock<dyn Iterator<Item = ScriptValue>>>),

    /// A callback
    Callback(Rc<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>),

    /// An object stored as an hashmap, mostly used to grab objects from the scripting runtime
    Object {
        /// The object class name
        class_name: String,

        /// The object fields
        fields: HashMap<String, ScriptValue>,
    },

    /// An object created by rust
    NativeObject(Box<dyn IntrospectObjectClone>),
}

impl<T: FruityFrom<ScriptValue> + ?Sized> FruityFrom<ScriptValue> for Vec<T> {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Array(value) => Ok(value
                .into_iter()
                .filter_map(|elem| T::fruity_from(elem).ok())
                .collect()),
            _ => Err(FruityError::new(
                FruityStatus::ArrayExpected,
                format!("Couldn't convert {:?} to array", value),
            )),
        }
    }
}

impl FruityFrom<ScriptValue> for Rc<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>> {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(value.clone()),
            _ => Err(FruityError::new(
                FruityStatus::FunctionExpected,
                format!("Couldn't convert {:?} to callback", value),
            )),
        }
    }
}

impl FruityFrom<ScriptValue> for Box<dyn IntrospectObjectClone> {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::NativeObject(value) => Ok(value),
            _ => Err(FruityError::new(
                FruityStatus::InvalidArg,
                format!("Couldn't convert {:?} to native object", value),
            )),
        }
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
            ScriptValue::Iterator(_) => formatter.write_str("iterator"),
            ScriptValue::Callback(_) => formatter.write_str("function"),
            ScriptValue::Object { fields, .. } => fields.fmt(formatter),
            ScriptValue::NativeObject(value) => value.fmt(formatter),
        }
    }
}

/// Provides trait to implement a self duplication for an introspect object that can be stored in serialized
pub trait IntrospectObjectClone: IntrospectObject {
    /// Create a copy of self
    fn duplicate(&self) -> Box<dyn IntrospectObjectClone>;
}

impl<T: Clone + IntrospectObject> IntrospectObjectClone for T {
    fn duplicate(&self) -> Box<dyn IntrospectObjectClone> {
        Box::new(self.clone())
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
            Self::Iterator(value) => Self::Iterator(value.clone()),
            Self::Callback(value) => Self::Callback(value.clone()),
            Self::Object { class_name, fields } => Self::Object {
                class_name: class_name.clone(),
                fields: fields.clone(),
            },
            Self::NativeObject(value) => Self::NativeObject(value.duplicate()),
        }
    }
}
