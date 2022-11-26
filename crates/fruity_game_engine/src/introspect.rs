#![warn(missing_docs)]

//! Introspect
//!
//! Implements traits and macros to make a structure abe to list it's field and to get/set it with any
//!

use parking_lot::RwLockUpgradableReadGuard;

use crate::any::FruityAny;
use crate::script_value::ScriptValue;
use crate::FruityResult;
use crate::RwLock;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

/// A setter caller
#[derive(Clone)]
pub enum SetterCaller {
    /// Without mutability
    Const(Rc<dyn Fn(&dyn FruityAny, ScriptValue) -> FruityResult<()>>),

    /// With mutability
    Mut(Rc<dyn Fn(&mut dyn FruityAny, ScriptValue) -> FruityResult<()>>),

    /// No setter
    None,
}

/// Informations about a field of an introspect object
#[derive(Clone)]
pub struct FieldInfo {
    /// The name of the field
    pub name: String,

    /// Function to get one of the entry field value as FruityAny
    ///
    /// # Arguments
    /// * `property` - The field name
    ///
    pub getter: Rc<dyn Fn(&dyn FruityAny) -> FruityResult<ScriptValue>>,

    /// Function to set one of the entry field
    ///
    /// # Arguments
    /// * `property` - The field name
    /// * `value` - The new field value as FruityAny
    ///
    pub setter: SetterCaller,
}

/// A method caller
#[derive(Clone)]
pub enum MethodCaller {
    /// Without mutability
    Const(Rc<dyn Fn(&dyn FruityAny, Vec<ScriptValue>) -> FruityResult<ScriptValue>>),

    /// With mutability
    Mut(Rc<dyn Fn(&mut dyn FruityAny, Vec<ScriptValue>) -> FruityResult<ScriptValue>>),
}

/// Informations about a field of an introspect object
#[derive(Clone)]
pub struct MethodInfo {
    /// The name of the method
    pub name: String,

    /// Call for the method with any field
    pub call: MethodCaller,
}

/// Getter and setter for a field of an introspect object
pub struct IntrospectField<'s> {
    /// Function to get one of the entry field value as FruityAny
    pub get: Box<dyn Fn() -> FruityResult<ScriptValue> + 's>,

    /// Function to set one of the entry field
    ///
    /// # Arguments
    /// * `value` - The new field value as FruityAny
    ///
    pub set: Box<dyn Fn(ScriptValue) -> FruityResult<()> + 's>,
}

/// Method of an introspect object
pub type IntrospectMethod<'s> = Box<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue> + 's>;

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
