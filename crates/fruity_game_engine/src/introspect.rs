#![warn(missing_docs)]

//! Introspect
//!
//! Implements traits and macros to make a structure abe to list it's field and to get/set it with any
//!

use crate::{
    any::FruityAny,
    script_value::ScriptValue,
    sync::{Arc, RwLock},
    FruityResult,
};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

/// Trait to implement fields introspection to a struct
pub trait IntrospectFields: Debug + FruityAny {
    /// Is the list of fields and methods supposed to change
    fn is_static(&self) -> FruityResult<bool>;

    /// Return the class type name
    fn get_class_name(&self) -> FruityResult<String>;

    /// Return the class type name
    fn get_field_names(&self) -> FruityResult<Vec<String>>;

    /// Return the class type name
    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()>;

    /// Return the class type name
    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue>;
}

impl dyn IntrospectFields {
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
}

/// Trait to implement methods introspection on a type
pub trait IntrospectMethods: Debug + FruityAny {
    /// Return the class type name
    fn get_const_method_names(&self) -> FruityResult<Vec<String>>;

    /// Return the class type name
    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue>;

    /// Return the class type name
    fn get_mut_method_names(&self) -> FruityResult<Vec<String>>;

    /// Return the class type name
    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue>;
}

impl<T: IntrospectFields + ?Sized> IntrospectFields for Box<T> {
    fn is_static(&self) -> FruityResult<bool> {
        self.deref().is_static()
    }

    fn get_class_name(&self) -> FruityResult<String> {
        self.deref().get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.deref().get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.deref_mut().set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.deref().get_field_value(name)
    }
}

impl<T: IntrospectMethods + ?Sized> IntrospectMethods for Box<T> {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.deref().get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.deref().call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.deref().get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.deref_mut().call_mut_method(name, args)
    }
}

impl<T: IntrospectFields + ?Sized> IntrospectFields for Arc<T> {
    fn is_static(&self) -> FruityResult<bool> {
        self.deref().is_static()
    }

    fn get_class_name(&self) -> FruityResult<String> {
        self.deref().get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.deref().get_field_names()
    }

    fn set_field_value(&mut self, _name: &str, _value: ScriptValue) -> FruityResult<()> {
        unreachable!()
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.deref().get_field_value(name)
    }
}

impl<T: IntrospectMethods + ?Sized> IntrospectMethods for Arc<T> {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.deref().get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.deref().call_const_method(name, args)
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

impl<T: IntrospectFields> IntrospectFields for RwLock<T> {
    fn is_static(&self) -> FruityResult<bool> {
        self.read().is_static()
    }

    fn get_class_name(&self) -> FruityResult<String> {
        self.read().get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.read().get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        let mut writer = self.write();
        writer.set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        let reader = self.read();
        reader.get_field_value(name)
    }
}

impl<T: IntrospectMethods> IntrospectMethods for RwLock<T> {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        let reader = self.read();
        let mut result = reader.get_const_method_names()?;
        let mut mut_method_names = reader.get_mut_method_names()?;
        result.append(&mut mut_method_names);

        Ok(result)
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        let (const_method_names, mut_method_names) = {
            let reader = self.read();
            (
                reader.get_const_method_names()?,
                reader.get_mut_method_names()?,
            )
        };

        if const_method_names.contains(&name.to_string()) {
            let reader = self.read();
            reader.call_const_method(name, args)
        } else if mut_method_names.contains(&name.to_string()) {
            let mut writer = self.write();
            writer.call_mut_method(name, args)
        } else {
            unreachable!()
        }
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
