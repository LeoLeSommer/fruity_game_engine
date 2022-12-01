#![warn(missing_docs)]

//! Introspect
//!
//! Implements traits and macros to make a structure abe to list it's field and to get/set it with any
//!

use crate::any::FruityAny;
use crate::script_value::ScriptValue;
use crate::FruityResult;
use crate::RwLock;
use parking_lot::RwLockUpgradableReadGuard;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

/// Trait to implement static introspection to an object
pub trait IntrospectObject: Debug + FruityAny {
    /// Return the class type name
    fn get_class_name(&self) -> FruityResult<String>;

    /// Return the class type name
    fn get_field_names(&self) -> FruityResult<Vec<String>>;

    /// Return the class type name
    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()>;

    /// Return the class type name
    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue>;

    /// Return the class type name
    fn get_const_method_names(&self) -> FruityResult<Vec<String>>;

    /// Return the class type name
    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue>;

    /// Return the class type name
    fn get_mut_method_names(&self) -> FruityResult<Vec<String>>;

    /// Return the class type name
    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue>;
}

/// Trait to implement static introspection to an object
pub trait IntrospectStruct: Debug + FruityAny {
    /// Return the class type name
    fn get_class_name(&self) -> FruityResult<String>;

    /// Return the class type name
    fn get_field_names(&self) -> FruityResult<Vec<String>>;

    /// Return the class type name
    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()>;

    /// Return the class type name
    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue>;
}

/// Trait to implement static introspection to an object
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

impl<T> IntrospectObject for T
where
    T: IntrospectMethods + IntrospectStruct + ?Sized,
{
    fn get_class_name(&self) -> FruityResult<String> {
        self.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.deref_mut().set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.get_field_value(name)
    }

    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.call_mut_method(name, args)
    }
}

impl<T: IntrospectObject + ?Sized> IntrospectObject for Box<T> {
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

impl<T: IntrospectObject + ?Sized> IntrospectObject for Rc<T> {
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

impl<T: IntrospectObject> IntrospectObject for RwLock<T> {
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

    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        let reader = self.read();
        let mut result = reader.get_const_method_names()?;
        let mut mut_method_names = reader.get_mut_method_names()?;
        result.append(&mut mut_method_names);

        Ok(result)
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        let reader = self.upgradable_read();
        let const_method_names = reader.get_const_method_names()?;
        let mut_method_names = reader.get_mut_method_names()?;

        if const_method_names.contains(&name.to_string()) {
            reader.call_const_method(name, args)
        } else if mut_method_names.contains(&name.to_string()) {
            let mut writer = RwLockUpgradableReadGuard::<T>::upgrade(reader);
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
