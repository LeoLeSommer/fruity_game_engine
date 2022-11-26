#![warn(missing_docs)]

//! ScriptValue
//!
//! Provide a structure that will be used all over the application to send data to scripting
//! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
//! data storage

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

use lazy_static::__Deref;

use crate::any::FruityAny;
use crate::introspect::IntrospectObject;
/// Implementation of script value conversions for tuples
// pub mod yaml;
use crate::script_value::convert::TryFromScriptValue;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use crate::RwLock;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

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
    Callback(Rc<dyn ScriptCallback>),

    /// An object created by rust
    Object(Box<dyn IntrospectObject>),
}

impl<T: TryFromScriptValue + ?Sized> TryFromScriptValue for Vec<T> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Array(value) => Ok(value
                .into_iter()
                .filter_map(|elem| T::from_script_value(elem).ok())
                .collect()),
            _ => Err(FruityError::new(
                FruityStatus::ArrayExpected,
                format!("Couldn't convert {:?} to array", value),
            )),
        }
    }
}

/// A trait that can be implemented for a callback
pub trait ScriptCallback {
    /// Call the callback
    fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue>;

    /// Turn the callback into a thread safe callback than can be called
    /// in every thread of the application
    /// It change the callback behavior, you can now call it from every thread but it will
    /// not be synchronously called and you can't receive the callback return anymore
    ///
    /// Note that not every callbacks can be turned into a thread safe callback, in general only the
    /// callbacks from the scripting language can be turned into a thread safe callback, an error will
    /// be raised if the callback cannot be turned into a thread safe callback
    ///
    fn create_thread_safe_callback(
        &self,
    ) -> FruityResult<Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>>;
}

impl TryFromScriptValue for Rc<dyn ScriptCallback> {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Callback(value) => Ok(value.clone()),
            _ => Err(FruityError::new(
                FruityStatus::InvalidArg,
                format!("Couldn't convert {:?} to callback", value),
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
            ScriptValue::Object(value) => value.fmt(formatter),
        }
    }
}

impl dyn IntrospectObject {
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

/// An hash map object for any object created from rust
#[derive(FruityAny, Debug)]
pub struct HashMapScriptObject {
    /// The type identifier
    pub class_name: String,
    /// The fields
    pub fields: HashMap<String, ScriptValue>,
}

impl IntrospectObject for HashMapScriptObject {
    fn get_class_name(&self) -> FruityResult<String> {
        Ok(self.class_name.clone())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(self.fields.keys().map(|e| e.clone()).collect())
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        *self.fields.get_mut(name).unwrap() = value;
        FruityResult::Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        #[allow(mutable_transmutes)]
        let this =
            unsafe { std::mem::transmute::<&HashMapScriptObject, &mut HashMapScriptObject>(self) };

        match this.fields.remove(name) {
            Some(value) => Ok(value),
            None => Err(FruityError::new(
                FruityStatus::GenericFailure,
                format!("Cannot get twice the same field from an HashMapScriptObject, the field where occured the error is {}", name),
            )),
        }
    }

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
