#![warn(missing_docs)]

//! ScriptValue
//!
//! Provide a structure that will be used all over the application to send data to scripting
//! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
//! data storage

use crate::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    typescript, FruityError, FruityResult,
};
use lazy_static::__Deref;
use std::{any::Any, fmt::Debug, future::Future, pin::Pin};

/// Traits similar to TryInto and TryFrom for ScriptValue
mod convert;
pub use convert::*;

/// Implementation of script value conversions for primitives
mod impl_primitives;
pub use impl_primitives::*;

/// Implementation of script value conversions for functions
mod impl_functions;
pub use impl_functions::*;

/// Implementation of script value conversions for containers (like Vec, Box ...)
mod impl_containers;
pub use impl_containers::*;

/// Implementation of script value conversions for tuples
mod impl_tuples;
pub use impl_tuples::*;

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
    Future(Pin<Box<dyn Send + Future<Output = FruityResult<ScriptValue>>>>),

    /// A callback
    Callback {
        /// The type of the object that will be passed as `this` to the callback
        identifier: Option<ScriptObjectType>,
        /// The callback
        callback: Box<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
    },

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
#[typescript("type ScriptObject = {[key: string]: ScriptValue}")]
pub trait ScriptObject: IntrospectFields + IntrospectMethods + Send {}

impl dyn ScriptObject {
    /// Get all field values
    pub fn get_field_values(&self) -> FruityResult<Vec<(String, ScriptValue)>> {
        self.get_field_names()?
            .into_iter()
            .map(|field_name| {
                self.get_field_value(&field_name)
                    .map(|field_value| (field_name, field_value))
            })
            .try_collect::<Vec<_>>()
    }

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

impl<T> ScriptObject for T where T: IntrospectFields + IntrospectMethods + Send + Sync {}

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
            ScriptValue::Callback { .. } => formatter.write_str("function"),
            ScriptValue::Object(value) => value.fmt(formatter),
        }
    }
}

/// A type associated with a [ScriptObject]
#[typescript("type ScriptObjectType = string | number")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, FruityAny)]
pub enum ScriptObjectType {
    /// A object that is implemented in Rust
    Rust(std::any::TypeId),
    /// A object that is implemented in JavaScript
    Script(String),
}

impl ScriptObjectType {
    /// Create a new component type id from a javascript type identifier
    pub fn from_identifier(string: String) -> Self {
        Self::Script(string)
    }

    /// Create a new component type id from a rust type
    pub fn from_type_id_value(value: u64) -> Self {
        let transmuted_type_id = unsafe {
            std::mem::transmute::<TransmutedTypeId, std::any::TypeId>(TransmutedTypeId {
                t: value as u64,
            })
        };

        ScriptObjectType::Rust(transmuted_type_id)
    }

    /// Create a new component type id from a rust type
    pub fn of<T: 'static>() -> Self {
        Self::Rust(std::any::TypeId::of::<T>())
    }
}

struct TransmutedTypeId {
    t: u64,
}

impl TryIntoScriptValue for ScriptObjectType {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        match self {
            ScriptObjectType::Rust(value) => {
                let transmuted_type_id =
                    unsafe { std::mem::transmute::<std::any::TypeId, TransmutedTypeId>(value) };

                Ok(ScriptValue::U64(transmuted_type_id.t))
            }
            ScriptObjectType::Script(value) => Ok(ScriptValue::String(value)),
        }
    }
}

impl TryFromScriptValue for ScriptObjectType {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::String(value) => Ok(ScriptObjectType::Script(value)),
            ScriptValue::U8(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::U16(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::U32(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::U64(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::USize(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::I8(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::I16(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::I32(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::I64(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::ISize(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::F32(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::F64(value) => Ok(Self::from_type_id_value(value as u64)),
            ScriptValue::Callback {
                identifier: Some(identifier),
                ..
            } => Ok(identifier),
            _ => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to ScriptObjectType",
                value
            ))),
        }
    }
}
