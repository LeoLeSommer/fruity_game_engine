#![warn(missing_docs)]

//! AnyValue
//!
//! Provide a structure that will be used all over the application to send data to scripting
//! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
//! data storage

/// Implementation of any value conversions for primitives
pub mod impl_primitives;

/// Implementation of any value conversions for functions
pub mod impl_functions;

/// Implementation of any value conversions for containers (like Vec, Box ...)
pub mod impl_containers;

/// Implementation of any value conversions for tuples
pub mod impl_tuples;

use crate::convert::FruityFrom;
use crate::introspect::IntrospectObject;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use crate::RwLock;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// An any value
#[derive(Clone)]
pub enum AnyValue {
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
  Array(Vec<AnyValue>),

  /// A null value, correspond to [’Option::None’]
  Null,

  /// A null value, correspond to ()
  Undefined,

  /// Iterator over values
  Iterator(Arc<RwLock<dyn Iterator<Item = AnyValue> + Send + Sync>>),

  /// A callback
  Callback(Arc<dyn Fn(Vec<AnyValue>) -> FruityResult<AnyValue> + Sync + Send + 'static>),

  /// An object stored as an hashmap, mostly used to grab objects from the scripting runtime
  Object {
    /// The object class name
    class_name: String,

    /// The object fields
    fields: HashMap<String, AnyValue>,
  },

  /// An object created by rust
  NativeObject(Box<dyn IntrospectObjectClone>),
}

impl<T: FruityFrom<AnyValue> + ?Sized> FruityFrom<AnyValue> for Vec<T> {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::Array(value) => Ok(
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

impl FruityFrom<AnyValue>
  for Arc<dyn Fn(Vec<AnyValue>) -> FruityResult<AnyValue> + Sync + Send + 'static>
{
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::Callback(value) => Ok(value.clone()),
      _ => Err(FruityError::new(
        FruityStatus::FunctionExpected,
        format!("Couldn't convert {:?} to callback", value),
      )),
    }
  }
}

impl FruityFrom<AnyValue> for Box<dyn IntrospectObjectClone> {
  fn fruity_try_from(value: AnyValue) -> FruityResult<Self> {
    match value {
      AnyValue::NativeObject(value) => Ok(value),
      _ => Err(FruityError::new(
        FruityStatus::InvalidArg,
        format!("Couldn't convert {:?} to native object", value),
      )),
    }
  }
}

impl Debug for AnyValue {
  fn fmt(
    &self,
    formatter: &mut std::fmt::Formatter<'_>,
  ) -> std::result::Result<(), std::fmt::Error> {
    match self {
      AnyValue::I8(value) => value.fmt(formatter),
      AnyValue::I16(value) => value.fmt(formatter),
      AnyValue::I32(value) => value.fmt(formatter),
      AnyValue::I64(value) => value.fmt(formatter),
      AnyValue::ISize(value) => value.fmt(formatter),
      AnyValue::U8(value) => value.fmt(formatter),
      AnyValue::U16(value) => value.fmt(formatter),
      AnyValue::U32(value) => value.fmt(formatter),
      AnyValue::U64(value) => value.fmt(formatter),
      AnyValue::USize(value) => value.fmt(formatter),
      AnyValue::F32(value) => value.fmt(formatter),
      AnyValue::F64(value) => value.fmt(formatter),
      AnyValue::Bool(value) => value.fmt(formatter),
      AnyValue::String(value) => value.fmt(formatter),
      AnyValue::Array(value) => value.fmt(formatter),
      AnyValue::Null => formatter.write_str("null"),
      AnyValue::Undefined => formatter.write_str("undefined"),
      AnyValue::Iterator(_) => formatter.write_str("iterator"),
      AnyValue::Callback(_) => formatter.write_str("function"),
      AnyValue::Object { fields, .. } => fields.fmt(formatter),
      AnyValue::NativeObject(value) => value.fmt(formatter),
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

impl Clone for Box<dyn IntrospectObjectClone> {
  fn clone(&self) -> Self {
    self.duplicate()
  }
}
