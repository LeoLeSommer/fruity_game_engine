#![feature(prelude_import)]
#![warn(missing_docs)]
#![feature(iterator_try_collect)]
#![feature(downcast_unchecked)]
//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use crate::module::modules_service::ModulesService;
use crate::resource::resource_container::ResourceContainer;
pub use error::FruityError;
pub use error::FruityResult;
pub use fruity_game_engine_macro::export;
pub use fruity_game_engine_macro::export_function;
pub use fruity_game_engine_macro::fruity_export;
pub use fruity_game_engine_macro::fruity_module_exports;
pub use lazy_static::lazy_static;
pub use parking_lot::*;
pub use send_wrapper;
#[cfg(feature = "napi-module")]
pub use napi;
pub mod error {
    #![allow(missing_docs)]
    /// A generic result that is able to be exposed to the js
    pub type FruityResult<T> = Result<T, FruityError>;
    /// A generic error that is able to be exposed to the js
    pub enum FruityError {
        Ok(String),
        InvalidArg(String),
        ObjectExpected(String),
        StringExpected(String),
        NameExpected(String),
        FunctionExpected(String),
        NumberExpected(String),
        BooleanExpected(String),
        ArrayExpected(String),
        GenericFailure(String),
        PendingException(String),
        Cancelled(String),
        EscapeCalledTwice(String),
        HandleScopeMismatch(String),
        CallbackScopeMismatch(String),
        QueueFull(String),
        Closing(String),
        BigintExpected(String),
        DateExpected(String),
        ArrayBufferExpected(String),
        DetachableArraybufferExpected(String),
        WouldDeadlock(String),
        Unknown(String),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FruityError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                FruityError::Ok(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Ok", &__self_0)
                }
                FruityError::InvalidArg(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidArg",
                        &__self_0,
                    )
                }
                FruityError::ObjectExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ObjectExpected",
                        &__self_0,
                    )
                }
                FruityError::StringExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "StringExpected",
                        &__self_0,
                    )
                }
                FruityError::NameExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NameExpected",
                        &__self_0,
                    )
                }
                FruityError::FunctionExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FunctionExpected",
                        &__self_0,
                    )
                }
                FruityError::NumberExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NumberExpected",
                        &__self_0,
                    )
                }
                FruityError::BooleanExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BooleanExpected",
                        &__self_0,
                    )
                }
                FruityError::ArrayExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ArrayExpected",
                        &__self_0,
                    )
                }
                FruityError::GenericFailure(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GenericFailure",
                        &__self_0,
                    )
                }
                FruityError::PendingException(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PendingException",
                        &__self_0,
                    )
                }
                FruityError::Cancelled(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Cancelled",
                        &__self_0,
                    )
                }
                FruityError::EscapeCalledTwice(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EscapeCalledTwice",
                        &__self_0,
                    )
                }
                FruityError::HandleScopeMismatch(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "HandleScopeMismatch",
                        &__self_0,
                    )
                }
                FruityError::CallbackScopeMismatch(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "CallbackScopeMismatch",
                        &__self_0,
                    )
                }
                FruityError::QueueFull(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "QueueFull",
                        &__self_0,
                    )
                }
                FruityError::Closing(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Closing",
                        &__self_0,
                    )
                }
                FruityError::BigintExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BigintExpected",
                        &__self_0,
                    )
                }
                FruityError::DateExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DateExpected",
                        &__self_0,
                    )
                }
                FruityError::ArrayBufferExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ArrayBufferExpected",
                        &__self_0,
                    )
                }
                FruityError::DetachableArraybufferExpected(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DetachableArraybufferExpected",
                        &__self_0,
                    )
                }
                FruityError::WouldDeadlock(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "WouldDeadlock",
                        &__self_0,
                    )
                }
                FruityError::Unknown(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Unknown",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FruityError {
        #[inline]
        fn clone(&self) -> FruityError {
            match self {
                FruityError::Ok(__self_0) => {
                    FruityError::Ok(::core::clone::Clone::clone(__self_0))
                }
                FruityError::InvalidArg(__self_0) => {
                    FruityError::InvalidArg(::core::clone::Clone::clone(__self_0))
                }
                FruityError::ObjectExpected(__self_0) => {
                    FruityError::ObjectExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::StringExpected(__self_0) => {
                    FruityError::StringExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::NameExpected(__self_0) => {
                    FruityError::NameExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::FunctionExpected(__self_0) => {
                    FruityError::FunctionExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::NumberExpected(__self_0) => {
                    FruityError::NumberExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::BooleanExpected(__self_0) => {
                    FruityError::BooleanExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::ArrayExpected(__self_0) => {
                    FruityError::ArrayExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::GenericFailure(__self_0) => {
                    FruityError::GenericFailure(::core::clone::Clone::clone(__self_0))
                }
                FruityError::PendingException(__self_0) => {
                    FruityError::PendingException(::core::clone::Clone::clone(__self_0))
                }
                FruityError::Cancelled(__self_0) => {
                    FruityError::Cancelled(::core::clone::Clone::clone(__self_0))
                }
                FruityError::EscapeCalledTwice(__self_0) => {
                    FruityError::EscapeCalledTwice(::core::clone::Clone::clone(__self_0))
                }
                FruityError::HandleScopeMismatch(__self_0) => {
                    FruityError::HandleScopeMismatch(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                FruityError::CallbackScopeMismatch(__self_0) => {
                    FruityError::CallbackScopeMismatch(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                FruityError::QueueFull(__self_0) => {
                    FruityError::QueueFull(::core::clone::Clone::clone(__self_0))
                }
                FruityError::Closing(__self_0) => {
                    FruityError::Closing(::core::clone::Clone::clone(__self_0))
                }
                FruityError::BigintExpected(__self_0) => {
                    FruityError::BigintExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::DateExpected(__self_0) => {
                    FruityError::DateExpected(::core::clone::Clone::clone(__self_0))
                }
                FruityError::ArrayBufferExpected(__self_0) => {
                    FruityError::ArrayBufferExpected(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                FruityError::DetachableArraybufferExpected(__self_0) => {
                    FruityError::DetachableArraybufferExpected(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                FruityError::WouldDeadlock(__self_0) => {
                    FruityError::WouldDeadlock(::core::clone::Clone::clone(__self_0))
                }
                FruityError::Unknown(__self_0) => {
                    FruityError::Unknown(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
}
/// The any trait
pub mod any {
    #![warn(missing_docs)]
    //! Any
    //!
    //! An extended Any trait
    //!
    //! The difference with the classic Any is that this Any needs to implement converter
    use parking_lot::Mutex;
    use parking_lot::RwLock;
    use std::any::Any;
    use std::rc::Rc;
    use std::sync::Arc;
    pub use fruity_game_engine_macro::FruityAny;
    /// The any trait
    pub trait FruityAny: Any {
        /// Returns the type name
        fn get_type_name(&self) -> &'static str;
        /// Return self as an Any ref
        fn as_any_ref(&self) -> &dyn Any;
        /// Return self as an Any mutable ref
        fn as_any_mut(&mut self) -> &mut dyn Any;
        /// Return self as an Any box
        fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
        /// Return self as an Any rc
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any>;
        /// Return self as an FruityAny ref
        fn as_fruity_any_ref(&self) -> &dyn FruityAny;
        /// Return self as an FruityAny mutable ref
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny;
        /// Return self as an FruityAny box
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny>;
        /// Return self as an AFruityAnyny arc
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny>;
    }
    impl<T: FruityAny + ?Sized> FruityAny for Box<T> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny> {
            self
        }
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny> {
            self
        }
    }
    impl<T: FruityAny + ?Sized> FruityAny for Rc<T> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny> {
            self
        }
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny> {
            self
        }
    }
    impl<T: FruityAny + ?Sized> FruityAny for Arc<T> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny> {
            self
        }
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny> {
            self
        }
    }
    impl<T: FruityAny> FruityAny for Mutex<T> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny> {
            self
        }
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny> {
            self
        }
    }
    impl<T: FruityAny> FruityAny for RwLock<T> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny> {
            self
        }
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny> {
            self
        }
    }
    impl<T: FruityAny> FruityAny for Option<T> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny> {
            self
        }
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny> {
            self
        }
    }
    impl<T: FruityAny> FruityAny for Vec<T> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn FruityAny> {
            self
        }
        fn as_fruity_any_rc(self: Rc<Self>) -> Rc<dyn FruityAny> {
            self
        }
    }
}
/// Add introspection into the types exported to the scripting
pub mod introspect {
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
        fn set_field_value(
            &mut self,
            name: &str,
            value: ScriptValue,
        ) -> FruityResult<()>;
        /// Return the class type name
        fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue>;
        /// Return the class type name
        fn get_const_method_names(&self) -> FruityResult<Vec<String>>;
        /// Return the class type name
        fn call_const_method(
            &self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue>;
        /// Return the class type name
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>>;
        /// Return the class type name
        fn call_mut_method(
            &mut self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue>;
    }
    impl<T: IntrospectObject + ?Sized> IntrospectObject for Box<T> {
        fn get_class_name(&self) -> FruityResult<String> {
            self.deref().get_class_name()
        }
        fn get_field_names(&self) -> FruityResult<Vec<String>> {
            self.deref().get_field_names()
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: ScriptValue,
        ) -> FruityResult<()> {
            self.deref_mut().set_field_value(name, value)
        }
        fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
            self.deref().get_field_value(name)
        }
        fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
            self.deref().get_const_method_names()
        }
        fn call_const_method(
            &self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            self.deref().call_const_method(name, args)
        }
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
            self.deref().get_mut_method_names()
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
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
        fn set_field_value(
            &mut self,
            _name: &str,
            _value: ScriptValue,
        ) -> FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
            self.deref().get_field_value(name)
        }
        fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
            self.deref().get_const_method_names()
        }
        fn call_const_method(
            &self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            self.deref().call_const_method(name, args)
        }
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            _name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl<T: IntrospectObject> IntrospectObject for RwLock<T> {
        fn get_class_name(&self) -> FruityResult<String> {
            self.read().get_class_name()
        }
        fn get_field_names(&self) -> FruityResult<Vec<String>> {
            self.read().get_field_names()
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: ScriptValue,
        ) -> FruityResult<()> {
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
        fn call_const_method(
            &self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            let reader = self.upgradable_read();
            let const_method_names = reader.get_const_method_names()?;
            let mut_method_names = reader.get_mut_method_names()?;
            if const_method_names.contains(&name.to_string()) {
                reader.call_const_method(name, args)
            } else if mut_method_names.contains(&name.to_string()) {
                let mut writer = RwLockUpgradableReadGuard::<T>::upgrade(reader);
                writer.call_mut_method(name, args)
            } else {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            _name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
}
/// a script value
pub mod script_value {
    #![warn(missing_docs)]
    //! ScriptValue
    //!
    //! Provide a structure that will be used all over the application to send data to scripting
    //! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
    //! data storage
    use crate::introspect::IntrospectObject;
    use crate::script_value::convert::TryFromScriptValue;
    use crate::FruityError;
    use crate::FruityResult;
    use crate::RwLock;
    use lazy_static::__Deref;
    use std::any::Any;
    use std::fmt::Debug;
    use std::rc::Rc;
    use std::sync::Arc;
    /// Traits similar to TryInto and TryFrom for ScriptValue
    pub mod convert {
        use super::ScriptValue;
        use crate::FruityResult;
        pub use fruity_game_engine_macro::TryFromScriptValue;
        /// Traits similar to TryInto for ScriptValue
        pub trait TryIntoScriptValue: Sized {
            /// Performs the conversion.
            fn into_script_value(self) -> FruityResult<ScriptValue>;
        }
        /// Traits similar to TryFrom for ScriptValue
        pub trait TryFromScriptValue: Sized {
            /// Performs the conversion.
            fn from_script_value(value: ScriptValue) -> FruityResult<Self>;
        }
    }
    /// Implementation of script value conversions for primitives
    pub mod impl_primitives {
        use crate::script_value::convert::TryFromScriptValue;
        use crate::script_value::convert::TryIntoScriptValue;
        use crate::script_value::ScriptValue;
        use crate::FruityError;
        use crate::FruityResult;
        impl TryFromScriptValue for ScriptValue {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                Ok(value)
            }
        }
        impl TryIntoScriptValue for ScriptValue {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(self)
            }
        }
        impl TryFromScriptValue for i8 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as i8),
                    ScriptValue::I16(value) => Ok(value as i8),
                    ScriptValue::I32(value) => Ok(value as i8),
                    ScriptValue::I64(value) => Ok(value as i8),
                    ScriptValue::ISize(value) => Ok(value as i8),
                    ScriptValue::U8(value) => Ok(value as i8),
                    ScriptValue::U16(value) => Ok(value as i8),
                    ScriptValue::U32(value) => Ok(value as i8),
                    ScriptValue::U64(value) => Ok(value as i8),
                    ScriptValue::USize(value) => Ok(value as i8),
                    ScriptValue::F32(value) => Ok(value as i8),
                    ScriptValue::F64(value) => Ok(value as i8),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for i16 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as i16),
                    ScriptValue::I16(value) => Ok(value as i16),
                    ScriptValue::I32(value) => Ok(value as i16),
                    ScriptValue::I64(value) => Ok(value as i16),
                    ScriptValue::ISize(value) => Ok(value as i16),
                    ScriptValue::U8(value) => Ok(value as i16),
                    ScriptValue::U16(value) => Ok(value as i16),
                    ScriptValue::U32(value) => Ok(value as i16),
                    ScriptValue::U64(value) => Ok(value as i16),
                    ScriptValue::USize(value) => Ok(value as i16),
                    ScriptValue::F32(value) => Ok(value as i16),
                    ScriptValue::F64(value) => Ok(value as i16),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for i32 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as i32),
                    ScriptValue::I16(value) => Ok(value as i32),
                    ScriptValue::I32(value) => Ok(value as i32),
                    ScriptValue::I64(value) => Ok(value as i32),
                    ScriptValue::ISize(value) => Ok(value as i32),
                    ScriptValue::U8(value) => Ok(value as i32),
                    ScriptValue::U16(value) => Ok(value as i32),
                    ScriptValue::U32(value) => Ok(value as i32),
                    ScriptValue::U64(value) => Ok(value as i32),
                    ScriptValue::USize(value) => Ok(value as i32),
                    ScriptValue::F32(value) => Ok(value as i32),
                    ScriptValue::F64(value) => Ok(value as i32),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for i64 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as i64),
                    ScriptValue::I16(value) => Ok(value as i64),
                    ScriptValue::I32(value) => Ok(value as i64),
                    ScriptValue::I64(value) => Ok(value as i64),
                    ScriptValue::ISize(value) => Ok(value as i64),
                    ScriptValue::U8(value) => Ok(value as i64),
                    ScriptValue::U16(value) => Ok(value as i64),
                    ScriptValue::U32(value) => Ok(value as i64),
                    ScriptValue::U64(value) => Ok(value as i64),
                    ScriptValue::USize(value) => Ok(value as i64),
                    ScriptValue::F32(value) => Ok(value as i64),
                    ScriptValue::F64(value) => Ok(value as i64),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for isize {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as isize),
                    ScriptValue::I16(value) => Ok(value as isize),
                    ScriptValue::I32(value) => Ok(value as isize),
                    ScriptValue::I64(value) => Ok(value as isize),
                    ScriptValue::ISize(value) => Ok(value as isize),
                    ScriptValue::U8(value) => Ok(value as isize),
                    ScriptValue::U16(value) => Ok(value as isize),
                    ScriptValue::U32(value) => Ok(value as isize),
                    ScriptValue::U64(value) => Ok(value as isize),
                    ScriptValue::USize(value) => Ok(value as isize),
                    ScriptValue::F32(value) => Ok(value as isize),
                    ScriptValue::F64(value) => Ok(value as isize),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for u8 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as u8),
                    ScriptValue::I16(value) => Ok(value as u8),
                    ScriptValue::I32(value) => Ok(value as u8),
                    ScriptValue::I64(value) => Ok(value as u8),
                    ScriptValue::ISize(value) => Ok(value as u8),
                    ScriptValue::U8(value) => Ok(value as u8),
                    ScriptValue::U16(value) => Ok(value as u8),
                    ScriptValue::U32(value) => Ok(value as u8),
                    ScriptValue::U64(value) => Ok(value as u8),
                    ScriptValue::USize(value) => Ok(value as u8),
                    ScriptValue::F32(value) => Ok(value as u8),
                    ScriptValue::F64(value) => Ok(value as u8),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for u16 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as u16),
                    ScriptValue::I16(value) => Ok(value as u16),
                    ScriptValue::I32(value) => Ok(value as u16),
                    ScriptValue::I64(value) => Ok(value as u16),
                    ScriptValue::ISize(value) => Ok(value as u16),
                    ScriptValue::U8(value) => Ok(value as u16),
                    ScriptValue::U16(value) => Ok(value as u16),
                    ScriptValue::U32(value) => Ok(value as u16),
                    ScriptValue::U64(value) => Ok(value as u16),
                    ScriptValue::USize(value) => Ok(value as u16),
                    ScriptValue::F32(value) => Ok(value as u16),
                    ScriptValue::F64(value) => Ok(value as u16),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for u32 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as u32),
                    ScriptValue::I16(value) => Ok(value as u32),
                    ScriptValue::I32(value) => Ok(value as u32),
                    ScriptValue::I64(value) => Ok(value as u32),
                    ScriptValue::ISize(value) => Ok(value as u32),
                    ScriptValue::U8(value) => Ok(value as u32),
                    ScriptValue::U16(value) => Ok(value as u32),
                    ScriptValue::U32(value) => Ok(value as u32),
                    ScriptValue::U64(value) => Ok(value as u32),
                    ScriptValue::USize(value) => Ok(value as u32),
                    ScriptValue::F32(value) => Ok(value as u32),
                    ScriptValue::F64(value) => Ok(value as u32),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for u64 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as u64),
                    ScriptValue::I16(value) => Ok(value as u64),
                    ScriptValue::I32(value) => Ok(value as u64),
                    ScriptValue::I64(value) => Ok(value as u64),
                    ScriptValue::ISize(value) => Ok(value as u64),
                    ScriptValue::U8(value) => Ok(value as u64),
                    ScriptValue::U16(value) => Ok(value as u64),
                    ScriptValue::U32(value) => Ok(value as u64),
                    ScriptValue::U64(value) => Ok(value as u64),
                    ScriptValue::USize(value) => Ok(value as u64),
                    ScriptValue::F32(value) => Ok(value as u64),
                    ScriptValue::F64(value) => Ok(value as u64),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for usize {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as usize),
                    ScriptValue::I16(value) => Ok(value as usize),
                    ScriptValue::I32(value) => Ok(value as usize),
                    ScriptValue::I64(value) => Ok(value as usize),
                    ScriptValue::ISize(value) => Ok(value as usize),
                    ScriptValue::U8(value) => Ok(value as usize),
                    ScriptValue::U16(value) => Ok(value as usize),
                    ScriptValue::U32(value) => Ok(value as usize),
                    ScriptValue::U64(value) => Ok(value as usize),
                    ScriptValue::USize(value) => Ok(value as usize),
                    ScriptValue::F32(value) => Ok(value as usize),
                    ScriptValue::F64(value) => Ok(value as usize),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for f32 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as f32),
                    ScriptValue::I16(value) => Ok(value as f32),
                    ScriptValue::I32(value) => Ok(value as f32),
                    ScriptValue::I64(value) => Ok(value as f32),
                    ScriptValue::ISize(value) => Ok(value as f32),
                    ScriptValue::U8(value) => Ok(value as f32),
                    ScriptValue::U16(value) => Ok(value as f32),
                    ScriptValue::U32(value) => Ok(value as f32),
                    ScriptValue::U64(value) => Ok(value as f32),
                    ScriptValue::USize(value) => Ok(value as f32),
                    ScriptValue::F32(value) => Ok(value as f32),
                    ScriptValue::F64(value) => Ok(value as f32),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for f64 {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::I8(value) => Ok(value as f64),
                    ScriptValue::I16(value) => Ok(value as f64),
                    ScriptValue::I32(value) => Ok(value as f64),
                    ScriptValue::I64(value) => Ok(value as f64),
                    ScriptValue::ISize(value) => Ok(value as f64),
                    ScriptValue::U8(value) => Ok(value as f64),
                    ScriptValue::U16(value) => Ok(value as f64),
                    ScriptValue::U32(value) => Ok(value as f64),
                    ScriptValue::U64(value) => Ok(value as f64),
                    ScriptValue::USize(value) => Ok(value as f64),
                    ScriptValue::F32(value) => Ok(value as f64),
                    ScriptValue::F64(value) => Ok(value as f64),
                    _ => {
                        Err(
                            FruityError::NumberExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to "],
                                        &[
                                            ::core::fmt::ArgumentV1::new_debug(&value),
                                            ::core::fmt::ArgumentV1::new_display(&"$type"),
                                        ],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryFromScriptValue for bool {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Bool(value) => Ok(value),
                    _ => {
                        Err(
                            FruityError::BooleanExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to bool"],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryIntoScriptValue for &str {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::String(self.to_string()))
            }
        }
        impl TryIntoScriptValue for String {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::String(self.clone()))
            }
        }
        impl TryFromScriptValue for String {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::String(value) => Ok(value.clone()),
                    _ => {
                        Err(
                            FruityError::StringExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to string"],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl TryIntoScriptValue for i8 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::I8(self))
            }
        }
        impl TryIntoScriptValue for i16 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::I16(self))
            }
        }
        impl TryIntoScriptValue for i32 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::I32(self))
            }
        }
        impl TryIntoScriptValue for i64 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::I64(self))
            }
        }
        impl TryIntoScriptValue for isize {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::ISize(self))
            }
        }
        impl TryIntoScriptValue for u8 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::U8(self))
            }
        }
        impl TryIntoScriptValue for u16 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::U16(self))
            }
        }
        impl TryIntoScriptValue for u32 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::U32(self))
            }
        }
        impl TryIntoScriptValue for u64 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::U64(self))
            }
        }
        impl TryIntoScriptValue for usize {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::USize(self))
            }
        }
        impl TryIntoScriptValue for f32 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::F32(self))
            }
        }
        impl TryIntoScriptValue for f64 {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::F64(self))
            }
        }
        impl TryIntoScriptValue for bool {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Bool(self))
            }
        }
    }
    /// Implementation of script value conversions for functions
    pub mod impl_functions {
        use super::{ScriptCallback, ScriptValue};
        use crate::introspect::IntrospectObject;
        use crate::script_value::convert::{TryFromScriptValue, TryIntoScriptValue};
        use crate::utils::introspect::ArgumentCaster;
        use crate::FruityError;
        use crate::FruityResult;
        use std::rc::Rc;
        impl ScriptCallback
        for Box<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>> {
            fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
                self(args)
            }
            fn create_thread_safe_callback(
                &self,
            ) -> FruityResult<std::sync::Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
                ::core::panicking::panic("not implemented")
            }
        }
        impl TryFromScriptValue
        for Rc<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        Ok(Rc::new(move |args| value.call(args)))
                    }
                    _ => {
                        Err(
                            FruityError::FunctionExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to native callback "],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl<R: TryFromScriptValue> TryFromScriptValue
        for Rc<dyn Fn() -> FruityResult<R>> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        Ok(
                            Rc::new(move || {
                                let args: Vec<ScriptValue> = ::alloc::vec::Vec::new();
                                let result = value.call(args)?;
                                <R>::from_script_value(result)
                            }),
                        )
                    }
                    _ => {
                        Err(
                            FruityError::FunctionExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to native callback "],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl<
            T1: TryIntoScriptValue,
            R: TryFromScriptValue + IntrospectObject,
        > TryFromScriptValue for Rc<dyn Fn(T1) -> FruityResult<R>> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        Ok(
                            Rc::new(move |arg1| {
                                let args: Vec<ScriptValue> = <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([arg1.into_script_value()?]),
                                );
                                let result = value.call(args)?;
                                <R>::from_script_value(result.into_script_value()?)
                            }),
                        )
                    }
                    value => {
                        Err(
                            FruityError::FunctionExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to native callback "],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl<R: TryIntoScriptValue> TryIntoScriptValue for &'static (dyn Fn() -> R) {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(
                    ScriptValue::Callback(
                        Rc::new(
                            Box::new(|_| {
                                let result = self();
                                result.into_script_value()
                            })
                                as Box<
                                    dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>,
                                >,
                        ),
                    ),
                )
            }
        }
        impl<T1: TryFromScriptValue, R: TryIntoScriptValue> TryIntoScriptValue
        for &'static (dyn Fn(T1) -> R) {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(
                    ScriptValue::Callback(
                        Rc::new(
                            Box::new(|args: Vec<ScriptValue>| {
                                let mut caster = ArgumentCaster::new(args);
                                let arg1 = caster.cast_next::<T1>()?;
                                let result = self(arg1);
                                result.into_script_value()
                            })
                                as Box<
                                    dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>,
                                >,
                        ),
                    ),
                )
            }
        }
        impl<T1: TryFromScriptValue, R: TryIntoScriptValue> ScriptCallback
        for &'static (dyn Fn(T1) -> R) {
            fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let result = self(arg1);
                result.into_script_value()
            }
            fn create_thread_safe_callback(
                &self,
            ) -> FruityResult<std::sync::Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
                ::core::panicking::panic("not implemented")
            }
        }
        impl<
            T1: TryIntoScriptValue,
            T2: TryIntoScriptValue,
            R: TryFromScriptValue,
        > TryFromScriptValue for Rc<dyn Fn(T1, T2) -> FruityResult<R>> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Callback(value) => {
                        Ok(
                            Rc::new(move |arg1, arg2| {
                                let args: Vec<ScriptValue> = <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        arg1.into_script_value()?,
                                        arg2.into_script_value()?,
                                    ]),
                                );
                                let result = value.call(args)?;
                                <R>::from_script_value(result)
                            }),
                        )
                    }
                    _ => {
                        Err(
                            FruityError::FunctionExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to native callback "],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl<
            T1: TryFromScriptValue,
            T2: TryFromScriptValue,
            R: TryIntoScriptValue,
        > ScriptCallback for &'static (dyn Fn(T1, T2) -> R) {
            fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T1>()?;
                let arg2 = caster.cast_next::<T2>()?;
                let result = self(arg1, arg2);
                result.into_script_value()
            }
            fn create_thread_safe_callback(
                &self,
            ) -> FruityResult<std::sync::Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
                ::core::panicking::panic("not implemented")
            }
        }
        impl<
            T1: TryFromScriptValue + 'static,
            T2: TryFromScriptValue + 'static,
            R: TryIntoScriptValue + 'static,
        > TryIntoScriptValue for Rc<dyn Fn(T1, T2) -> R> {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(
                    ScriptValue::Callback(
                        Rc::new(
                            Box::new(move |args: Vec<ScriptValue>| {
                                let mut caster = ArgumentCaster::new(args);
                                let arg1 = caster.cast_next::<T1>()?;
                                let arg2 = caster.cast_next::<T2>()?;
                                let result = self(arg1, arg2);
                                result.into_script_value()
                            })
                                as Box<
                                    dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>,
                                >,
                        ),
                    ),
                )
            }
        }
    }
    /// Implementation of script value conversions for containers (like Vec, Box ...)
    pub mod impl_containers {
        use super::ScriptObject;
        use super::ScriptValue;
        use crate::introspect::IntrospectObject;
        use crate::script_value::convert::TryFromScriptValue;
        use crate::script_value::convert::TryIntoScriptValue;
        use crate::FruityError;
        use crate::FruityResult;
        use std::any::type_name;
        use std::collections::HashMap;
        impl<T> TryIntoScriptValue for FruityResult<T>
        where
            T: TryIntoScriptValue,
        {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                match self {
                    Ok(value) => <T as TryIntoScriptValue>::into_script_value(value),
                    Err(err) => Err(err.clone()),
                }
            }
        }
        impl<T> TryFromScriptValue for FruityResult<T>
        where
            T: TryFromScriptValue,
        {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                Ok(<T as TryFromScriptValue>::from_script_value(value))
            }
        }
        impl<T: ScriptObject> TryIntoScriptValue for T {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Object(Box::new(self)))
            }
        }
        impl<T> TryFromScriptValue for T
        where
            T: IntrospectObject,
        {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Object(value) => {
                        match value.downcast::<T>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Err(
                                    FruityError::InvalidArg({
                                        let res = ::alloc::fmt::format(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Couldn\'t convert a ", " to "],
                                                &[
                                                    ::core::fmt::ArgumentV1::new_display(
                                                        &value.get_type_name(),
                                                    ),
                                                    ::core::fmt::ArgumentV1::new_display(&type_name::<T>()),
                                                ],
                                            ),
                                        );
                                        res
                                    }),
                                )
                            }
                        }
                    }
                    value => {
                        Err(
                            FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to native object"],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        impl<T: TryIntoScriptValue + Clone> TryIntoScriptValue for &'static [T] {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(
                    ScriptValue::Array(
                        self
                            .iter()
                            .map(|elem| elem.clone().into_script_value())
                            .try_collect::<Vec<_>>()?,
                    ),
                )
            }
        }
        impl<T: TryIntoScriptValue> TryIntoScriptValue for Vec<T> {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(
                    ScriptValue::Array(
                        self
                            .into_iter()
                            .map(|elem| elem.into_script_value())
                            .try_collect::<Vec<_>>()?,
                    ),
                )
            }
        }
        impl<T: TryIntoScriptValue> TryIntoScriptValue for Option<T> {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                match self {
                    Some(value) => value.into_script_value(),
                    None => Ok(ScriptValue::Null),
                }
            }
        }
        impl<T: TryFromScriptValue> TryFromScriptValue for Option<T> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                Ok(
                    match value {
                        ScriptValue::Null => None,
                        ScriptValue::Undefined => None,
                        _ => T::from_script_value(value).map(|value| Some(value))?,
                    },
                )
            }
        }
        impl<T: TryFromScriptValue> TryFromScriptValue for HashMap<String, T> {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                if let ScriptValue::Object(value) = value {
                    let mut result = HashMap::<String, T>::new();
                    value
                        .get_field_names()?
                        .into_iter()
                        .try_for_each(|name| {
                            let field_value = value.get_field_value(&name)?;
                            result.insert(name, T::from_script_value(field_value)?);
                            FruityResult::Ok(())
                        })?;
                    Ok(result)
                } else {
                    Err(
                        FruityError::ObjectExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to HashMap"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    /// Implementation of script value conversions for tuples
    pub mod impl_tuples {
        use super::ScriptValue;
        use crate::script_value::convert::TryFromScriptValue;
        use crate::script_value::convert::TryIntoScriptValue;
        use crate::utils::introspect::ArgumentCaster;
        use crate::FruityError;
        use crate::FruityResult;
        impl TryIntoScriptValue for () {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(ScriptValue::Undefined)
            }
        }
        impl TryFromScriptValue for () {
            fn from_script_value(_value: ScriptValue) -> FruityResult<Self> {
                Ok(())
            }
        }
        impl<T: TryIntoScriptValue, U: TryIntoScriptValue> TryIntoScriptValue
        for (T, U) {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                Ok(
                    ScriptValue::Array(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                self.0.into_script_value()?,
                                self.1.into_script_value()?,
                            ]),
                        ),
                    ),
                )
            }
        }
        impl<T1: TryFromScriptValue, T2: TryFromScriptValue> TryFromScriptValue
        for (T1, T2) {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Array(args) => {
                        let mut caster = ArgumentCaster::new(args);
                        let arg1 = caster.cast_next::<T1>()?;
                        let arg2 = caster.cast_next::<T2>()?;
                        Ok((arg1, arg2))
                    }
                    value => {
                        Err(
                            FruityError::ArrayExpected({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to tuple"],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
    }
    /// Implementation of script value conversions for tuples
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
        Object(Box<dyn ScriptObject>),
    }
    impl<T: TryFromScriptValue + ?Sized> TryFromScriptValue for Vec<T> {
        fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
            match value {
                ScriptValue::Array(value) => {
                    Ok(
                        value
                            .into_iter()
                            .filter_map(|elem| T::from_script_value(elem).ok())
                            .collect(),
                    )
                }
                _ => {
                    Err(
                        FruityError::ArrayExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to array"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    /// A trait that can be implemented for an object storable in a ScriptValue
    pub trait ScriptObject: IntrospectObject {
        /// Duplicate the script object
        fn duplicate(&self) -> FruityResult<Box<dyn ScriptObject>>;
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
        T: Clone + IntrospectObject,
    {
        fn duplicate(&self) -> FruityResult<Box<dyn ScriptObject>> {
            Ok(Box::new(self.clone()))
        }
    }
    /// A trait that can be implemented for a callback storable in a ScriptValue
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
                _ => {
                    Err(
                        FruityError::InvalidArg({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to callback"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
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
                Self::Object(value) => Self::Object(value.duplicate().unwrap()),
            }
        }
    }
}
/// Tools to export javascript modules
pub mod javascript {
    #![allow(missing_docs)]
    #[cfg(feature = "napi-module")]
    pub mod napi {
        use crate::{
            introspect::IntrospectObject, object_factory_service::ObjectFactory,
            script_value::convert::TryIntoScriptValue, script_value::ScriptObject,
            script_value::{ScriptCallback, ScriptValue},
            FruityError, FruityResult,
        };
        use convert_case::{Case, Casing};
        use fruity_game_engine_macro::FruityAny;
        use napi::{
            bindgen_prelude::CallbackInfo, sys::{napi_env, napi_value},
            threadsafe_function::{
                ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction,
                ThreadsafeFunctionCallMode,
            },
            Env, JsBigInt, JsFunction, JsNumber, JsObject, JsString, JsUnknown,
            PropertyAttributes, Ref, ValueType,
        };
        use napi::{check_status, NapiValue};
        use napi::{JsError, NapiRaw};
        use std::{ffi::CString, marker::PhantomData, ops::Deref};
        use std::{fmt::Debug, vec};
        use std::{rc::Rc, sync::Arc};
        /// Tool to export javascript modules
        pub struct ExportJavascript {
            exports: JsObject,
            env: Env,
        }
        impl ExportJavascript {
            /// Returns an ExportJavascript
            pub fn new(exports: JsObject, env: Env) -> Self {
                Self { exports, env }
            }
            /// Register a class type
            pub fn export_constructor<T>(
                &mut self,
                name: &str,
                value: T,
            ) -> FruityResult<()>
            where
                T: ObjectFactory,
            {
                Ok(())
            }
            /// Register a class type
            pub fn export_function_as_constructor<T>(
                &mut self,
                name: &str,
                value: T,
            ) -> FruityResult<()>
            where
                T: TryIntoScriptValue,
            {
                let js_value = script_value_to_js_value(
                    &self.env,
                    value.into_script_value()?,
                )?;
                self.exports
                    .set_named_property(&name, js_value)
                    .map_err(|e| FruityError::from_napi(e))?;
                Ok(())
            }
            /// Register a class type
            pub fn export_value<T>(&mut self, name: &str, value: T) -> FruityResult<()>
            where
                T: TryIntoScriptValue,
            {
                let js_value = script_value_to_js_value(
                    &self.env,
                    value.into_script_value()?,
                )?;
                self.exports
                    .set_named_property(&name.to_case(Case::Camel), js_value)
                    .map_err(|e| FruityError::from_napi(e))?;
                Ok(())
            }
        }
        /// Create a napi js value from a script value
        pub fn script_value_to_js_value(
            env: &Env,
            value: ScriptValue,
        ) -> FruityResult<JsUnknown> {
            Ok(
                match value.into_script_value()? {
                    ScriptValue::I8(value) => {
                        env
                            .create_int32(value as i32)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::I16(value) => {
                        env
                            .create_int32(value as i32)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::I32(value) => {
                        env
                            .create_int32(value)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::I64(value) => {
                        env
                            .create_int64(value)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::ISize(value) => {
                        env
                            .create_int32(value as i32)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::U8(value) => {
                        env
                            .create_uint32(value as u32)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::U16(value) => {
                        env
                            .create_uint32(value as u32)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::U32(value) => {
                        env
                            .create_uint32(value)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::U64(value) => {
                        env
                            .create_bigint_from_u64(value)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                            .map_err(|e| FruityError::from_napi(e))?
                    }
                    ScriptValue::USize(value) => {
                        env
                            .create_uint32(value as u32)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::F32(value) => {
                        env
                            .create_double(value as f64)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::F64(value) => {
                        env
                            .create_double(value as f64)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::Bool(value) => {
                        env
                            .get_boolean(value)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::String(value) => {
                        env
                            .create_string(&value)
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::Array(value) => {
                        let mut js_array = env
                            .create_empty_array()
                            .map_err(|e| FruityError::from_napi(e))?;
                        for (index, elem) in value.into_iter().enumerate() {
                            js_array
                                .set_element(
                                    index as u32,
                                    script_value_to_js_value(env, elem)?,
                                )
                                .map_err(|e| FruityError::from_napi(e))?;
                        }
                        js_array.into_unknown()
                    }
                    ScriptValue::Null => {
                        env
                            .get_null()
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::Undefined => {
                        env
                            .get_undefined()
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::Iterator(_value) => {
                        ::core::panicking::panic("not yet implemented")
                    }
                    ScriptValue::Callback(callback) => {
                        env
                            .create_function_from_closure(
                                "unknown",
                                move |ctx| {
                                    let args = ctx
                                        .get_all()
                                        .into_iter()
                                        .map(|elem| js_value_to_script_value(ctx.env, elem))
                                        .try_collect::<Vec<_>>()
                                        .map_err(|e| e.into_napi())?;
                                    let result = callback
                                        .call(args)
                                        .map_err(|e| e.into_napi())?;
                                    script_value_to_js_value(ctx.env, result)
                                        .map_err(|e| e.into_napi())
                                },
                            )
                            .map_err(|e| FruityError::from_napi(e))?
                            .into_unknown()
                    }
                    ScriptValue::Object(value) => {
                        match value.downcast::<JsIntrospectObject>() {
                            Ok(value) => {
                                let js_object: JsObject = value.reference.inner();
                                js_object.into_unknown()
                            }
                            Err(value) => {
                                let mut js_object = env
                                    .create_object()
                                    .map_err(|e| FruityError::from_napi(e))?;
                                let field_names = value
                                    .get_field_names()?
                                    .into_iter()
                                    .map(|field_name| CString::new(field_name).unwrap())
                                    .collect::<Vec<_>>();
                                let properties = field_names
                                    .iter()
                                    .map(|field_name| napi_sys::napi_property_descriptor {
                                        utf8name: field_name.as_ptr(),
                                        name: std::ptr::null_mut(),
                                        method: None,
                                        getter: Some(generic_getter),
                                        setter: Some(generic_setter),
                                        value: std::ptr::null_mut(),
                                        attributes: (PropertyAttributes::Default
                                            | PropertyAttributes::Writable
                                            | PropertyAttributes::Enumerable)
                                            .bits(),
                                        data: field_name.as_ptr() as *mut std::ffi::c_void,
                                    })
                                    .collect::<Vec<napi_sys::napi_property_descriptor>>();
                                js_object
                                    .add_finalizer(
                                        (),
                                        (),
                                        |_| {
                                            std::mem::drop(field_names);
                                        },
                                    )
                                    .map_err(|e| FruityError::from_napi(e))?;
                                unsafe {
                                    {
                                        let c = napi_sys::napi_define_properties(
                                            env.raw(),
                                            js_object.raw(),
                                            properties.len(),
                                            properties.as_ptr(),
                                        );
                                        match c {
                                            ::napi::sys::Status::napi_ok => Ok(()),
                                            _ => {
                                                Err(
                                                    ::napi::Error::new(::napi::Status::from(c), "".to_owned()),
                                                )
                                            }
                                        }
                                    }
                                        .map_err(|e| FruityError::from_napi(e))?;
                                }
                                value
                                    .get_const_method_names()?
                                    .into_iter()
                                    .try_for_each(|method_name| {
                                        js_object
                                            .set_named_property(
                                                method_name.clone().to_case(Case::Camel).as_str(),
                                                env
                                                    .create_function_from_closure(
                                                        &method_name.clone(),
                                                        move |ctx| {
                                                            let args = ctx
                                                                .get_all()
                                                                .into_iter()
                                                                .map(|elem| js_value_to_script_value(ctx.env, elem))
                                                                .try_collect::<Vec<_>>()
                                                                .map_err(|e| e.into_napi())?;
                                                            let wrapped = ctx
                                                                .env
                                                                .unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;
                                                            let result = wrapped
                                                                .call_const_method(&method_name, args)
                                                                .map_err(|e| e.into_napi())?;
                                                            script_value_to_js_value(ctx.env, result)
                                                                .map_err(|e| e.into_napi())
                                                        },
                                                    )
                                                    .map_err(|e| FruityError::from_napi(e))?,
                                            )
                                            .map_err(|e| FruityError::from_napi(e))?;
                                        Result::Ok(())
                                    })?;
                                value
                                    .get_mut_method_names()?
                                    .into_iter()
                                    .try_for_each(|method_name| {
                                        js_object
                                            .set_named_property(
                                                method_name.clone().to_case(Case::Camel).as_str(),
                                                env
                                                    .create_function_from_closure(
                                                        &method_name.clone(),
                                                        move |ctx| {
                                                            let args = ctx
                                                                .get_all()
                                                                .into_iter()
                                                                .map(|elem| js_value_to_script_value(ctx.env, elem))
                                                                .try_collect::<Vec<_>>()
                                                                .map_err(|e| e.into_napi())?;
                                                            let wrapped = ctx
                                                                .env
                                                                .unwrap::<Box<dyn ScriptObject>>(&ctx.this()?)?;
                                                            let result = wrapped
                                                                .call_mut_method(&method_name, args)
                                                                .map_err(|e| e.into_napi())?;
                                                            script_value_to_js_value(ctx.env, result)
                                                                .map_err(|e| e.into_napi())
                                                        },
                                                    )
                                                    .map_err(|e| FruityError::from_napi(e))?,
                                            )
                                            .map_err(|e| FruityError::from_napi(e))?;
                                        Result::Ok(())
                                    })?;
                                env.wrap(&mut js_object, value)
                                    .map_err(|e| FruityError::from_napi(e))?;
                                js_object.into_unknown()
                            }
                        }
                    }
                },
            )
        }
        /// Create a script value from a napi js value
        pub fn js_value_to_script_value(
            env: &Env,
            value: JsUnknown,
        ) -> FruityResult<ScriptValue> {
            Ok(
                match value.get_type().map_err(|e| FruityError::from_napi(e))? {
                    ValueType::Undefined => ScriptValue::Undefined,
                    ValueType::Null => ScriptValue::Null,
                    ValueType::Boolean => {
                        ScriptValue::Bool(
                            value
                                .coerce_to_bool()
                                .map_err(|e| FruityError::from_napi(e))?
                                .get_value()
                                .map_err(|e| FruityError::from_napi(e))?,
                        )
                    }
                    ValueType::Number => {
                        ScriptValue::F64(
                            value
                                .coerce_to_number()
                                .map_err(|e| FruityError::from_napi(e))?
                                .get_double()
                                .map_err(|e| FruityError::from_napi(e))?,
                        )
                    }
                    ValueType::String => {
                        ScriptValue::String(
                            value
                                .coerce_to_string()
                                .map_err(|e| FruityError::from_napi(e))?
                                .into_utf8()
                                .map_err(|e| FruityError::from_napi(e))?
                                .as_str()
                                .map_err(|e| FruityError::from_napi(e))?
                                .to_string(),
                        )
                    }
                    ValueType::Symbol => ::core::panicking::panic("not implemented"),
                    ValueType::Object => {
                        let js_object = unsafe { value.cast::<JsObject>() };
                        if js_object.is_array().map_err(|e| FruityError::from_napi(e))? {
                            ScriptValue::Array(
                                (0..js_object
                                    .get_array_length()
                                    .map_err(|e| FruityError::from_napi(e))?)
                                    .map(|index| {
                                        js_value_to_script_value(
                                            env,
                                            js_object
                                                .get_element(index)
                                                .map_err(|e| FruityError::from_napi(e))?,
                                        )
                                    })
                                    .try_collect::<Vec<_>>()?,
                            )
                        } else {
                            match env.unwrap::<Box<dyn ScriptObject>>(&js_object) {
                                Ok(wrapped) => {
                                    ScriptValue::Object(wrapped.deref().duplicate()?)
                                }
                                Err(_) => {
                                    ScriptValue::Object(
                                        Box::new(JsIntrospectObject {
                                            reference: JsSharedRef::new(env, js_object)?,
                                            env: env.clone(),
                                        }),
                                    )
                                }
                            }
                        }
                    }
                    ValueType::Function => {
                        let js_func = JsFunction::try_from(value)
                            .map_err(|e| FruityError::from_napi(e))?;
                        ScriptValue::Callback(
                            Rc::new(JsFunctionCallback {
                                reference: JsSharedRef::new(env, js_func)?,
                                env: env.clone(),
                            }),
                        )
                    }
                    ValueType::External => ::core::panicking::panic("not implemented"),
                    ValueType::BigInt => {
                        ScriptValue::I64(
                            unsafe { value.cast::<JsBigInt>() }
                                .get_i64()
                                .map_err(|e| FruityError::from_napi(e))?
                                .0,
                        )
                    }
                    ValueType::Unknown => ::core::panicking::panic("not implemented"),
                },
            )
        }
        struct RefWrapper {
            reference: Ref<()>,
            env: Env,
        }
        impl Drop for RefWrapper {
            fn drop(&mut self) {
                self.reference.unref(self.env.clone()).unwrap();
            }
        }
        struct JsSharedRef<T>
        where
            T: NapiRaw,
        {
            reference: Rc<RefWrapper>,
            env: Env,
            phantom: PhantomData<T>,
        }
        impl<T> Debug for JsSharedRef<T>
        where
            T: NapiRaw,
        {
            fn fmt(
                &self,
                _: &mut std::fmt::Formatter,
            ) -> std::result::Result<(), std::fmt::Error> {
                Ok(())
            }
        }
        impl<T> Clone for JsSharedRef<T>
        where
            T: NapiRaw,
        {
            fn clone(&self) -> Self {
                Self {
                    reference: self.reference.clone(),
                    env: self.env.clone(),
                    phantom: Default::default(),
                }
            }
        }
        impl<T> JsSharedRef<T>
        where
            T: NapiRaw + NapiValue,
        {
            pub fn new(env: &Env, value: T) -> FruityResult<Self> {
                Ok(Self {
                    reference: Rc::new(RefWrapper {
                        reference: env
                            .create_reference(value)
                            .map_err(|e| FruityError::from_napi(e))?,
                        env: env.clone(),
                    }),
                    env: env.clone(),
                    phantom: Default::default(),
                })
            }
            pub fn inner(&self) -> T {
                self.env.get_reference_value::<T>(&self.reference.reference).unwrap()
            }
        }
        struct JsFunctionCallback {
            reference: JsSharedRef<JsFunction>,
            env: Env,
        }
        impl ScriptCallback for JsFunctionCallback {
            fn call(&self, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
                let js_func = self.reference.inner();
                let args = args
                    .into_iter()
                    .map(|elem| script_value_to_js_value(&self.env, elem))
                    .try_collect::<Vec<_>>()?;
                let result = js_func
                    .call(None, &args)
                    .map_err(|e| FruityError::from_napi(e))?;
                let result = js_value_to_script_value(&self.env, result)?;
                Ok(result)
            }
            fn create_thread_safe_callback(
                &self,
            ) -> FruityResult<Arc<dyn Fn(Vec<ScriptValue>) + Send + Sync>> {
                let js_func = self.reference.inner();
                let thread_safe_func: ThreadsafeFunction<
                    Vec<ScriptValue>,
                    ErrorStrategy::Fatal,
                > = js_func
                    .create_threadsafe_function(
                        0,
                        |ctx: ThreadSafeCallContext<Vec<ScriptValue>>| {
                            let args = ctx
                                .value
                                .into_iter()
                                .map(|elem| script_value_to_js_value(&ctx.env, elem))
                                .try_collect::<Vec<_>>()
                                .map_err(|e| e.into_napi())?;
                            Ok(args)
                        },
                    )
                    .map_err(|e| FruityError::from_napi(e))?;
                Ok(
                    Arc::new(move |args| {
                        thread_safe_func
                            .call(args, ThreadsafeFunctionCallMode::Blocking);
                    }),
                )
            }
        }
        /// A structure to store a javascript object that can be stored in a ScriptValue
        pub struct JsIntrospectObject {
            reference: JsSharedRef<JsObject>,
            env: Env,
        }
        impl crate::any::FruityAny for JsIntrospectObject {
            fn get_type_name(&self) -> &'static str {
                "JsIntrospectObject"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn crate::any::FruityAny> {
                self
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for JsIntrospectObject {
            #[inline]
            fn clone(&self) -> JsIntrospectObject {
                JsIntrospectObject {
                    reference: ::core::clone::Clone::clone(&self.reference),
                    env: ::core::clone::Clone::clone(&self.env),
                }
            }
        }
        impl Debug for JsIntrospectObject {
            fn fmt(
                &self,
                _: &mut std::fmt::Formatter,
            ) -> std::result::Result<(), std::fmt::Error> {
                Ok(())
            }
        }
        impl IntrospectObject for JsIntrospectObject {
            fn get_class_name(&self) -> FruityResult<String> {
                let js_object = self.reference.inner();
                let constructor: JsFunction = js_object
                    .get_named_property("constructor")
                    .map_err(|e| FruityError::from_napi(e))?;
                let constructor = constructor
                    .coerce_to_object()
                    .map_err(|e| FruityError::from_napi(e))?;
                let name: JsString = constructor
                    .get_named_property("name")
                    .map_err(|e| FruityError::from_napi(e))?;
                Ok(
                    name
                        .into_utf8()
                        .map_err(|e| FruityError::from_napi(e))?
                        .as_str()
                        .map_err(|e| FruityError::from_napi(e))?
                        .to_string(),
                )
            }
            fn get_field_names(&self) -> FruityResult<Vec<String>> {
                let js_object = self.reference.inner();
                let properties = js_object
                    .get_property_names()
                    .map_err(|e| FruityError::from_napi(e))?;
                let len = properties
                    .get_named_property::<JsNumber>("length")
                    .map_err(|e| FruityError::from_napi(e))?
                    .get_uint32()
                    .map_err(|e| FruityError::from_napi(e))?;
                (0..len)
                    .map(|index| {
                        let key = properties
                            .get_element::<JsString>(index)
                            .map_err(|e| FruityError::from_napi(e))?;
                        let key = key
                            .into_utf8()
                            .map_err(|e| FruityError::from_napi(e))?
                            .as_str()
                            .map_err(|e| FruityError::from_napi(e))?
                            .to_string();
                        Ok(key.to_case(Case::Snake))
                    })
                    .try_collect()
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ScriptValue,
            ) -> FruityResult<()> {
                let mut js_object = self.reference.inner();
                let value = script_value_to_js_value(&self.env, value)?;
                js_object
                    .set_named_property(&name.to_case(Case::Camel), value)
                    .map_err(|e| FruityError::from_napi(e))?;
                Ok(())
            }
            fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
                let js_object = self.reference.inner();
                let value = js_object
                    .get_named_property(&name.to_case(Case::Camel))
                    .map_err(|e| FruityError::from_napi(e))?;
                js_value_to_script_value(&self.env, value)
            }
            fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_const_method(
                &self,
                _name: &str,
                _args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                _name: &str,
                _args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        unsafe extern "C" fn generic_getter(
            raw_env: napi_env,
            callback_info: napi_sys::napi_callback_info,
        ) -> napi_value {
            unsafe fn generic_getter(
                raw_env: napi_env,
                callback_info: napi_sys::napi_callback_info,
            ) -> napi::Result<napi_value> {
                let field_name = {
                    let mut this = std::ptr::null_mut();
                    let mut args = [std::ptr::null_mut(); 1];
                    let mut argc = 1;
                    let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
                    {
                        let c = napi_sys::napi_get_cb_info(
                            raw_env,
                            callback_info,
                            &mut argc,
                            args.as_mut_ptr(),
                            &mut this,
                            &mut data_ptr,
                        );
                        match c {
                            ::napi::sys::Status::napi_ok => Ok(()),
                            _ => {
                                Err(
                                    ::napi::Error::new(::napi::Status::from(c), "".to_owned()),
                                )
                            }
                        }
                    }?;
                    let data_ptr = std::ffi::CStr::from_ptr(
                        data_ptr as *mut std::ffi::c_char,
                    );
                    data_ptr.to_str().unwrap().to_string()
                };
                let env = Env::from_raw(raw_env);
                let callback_info = CallbackInfo::<
                    3,
                >::new(raw_env, callback_info, None)?;
                let this = JsObject::from_raw(raw_env, callback_info.this())?;
                let wrapped = env.unwrap::<Box<dyn ScriptObject>>(&this)?;
                let result = wrapped
                    .get_field_value(&field_name)
                    .map_err(|e| e.into_napi())?;
                let result = script_value_to_js_value(&env, result)
                    .map_err(|e| e.into_napi())?;
                Ok(result.raw())
            }
            generic_getter(raw_env, callback_info)
                .unwrap_or_else(|e| {
                    unsafe { JsError::from(e).throw_into(raw_env) };
                    std::ptr::null_mut()
                })
        }
        unsafe extern "C" fn generic_setter(
            raw_env: napi_env,
            callback_info: napi_sys::napi_callback_info,
        ) -> napi_value {
            unsafe fn generic_setter(
                raw_env: napi_env,
                callback_info: napi_sys::napi_callback_info,
            ) -> napi::Result<napi_value> {
                let field_name = {
                    let mut this = std::ptr::null_mut();
                    let mut args = [std::ptr::null_mut(); 1];
                    let mut argc = 1;
                    let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
                    {
                        let c = napi_sys::napi_get_cb_info(
                            raw_env,
                            callback_info,
                            &mut argc,
                            args.as_mut_ptr(),
                            &mut this,
                            &mut data_ptr,
                        );
                        match c {
                            ::napi::sys::Status::napi_ok => Ok(()),
                            _ => {
                                Err(
                                    ::napi::Error::new(::napi::Status::from(c), "".to_owned()),
                                )
                            }
                        }
                    }?;
                    let data_ptr = std::ffi::CStr::from_ptr(
                        data_ptr as *mut std::ffi::c_char,
                    );
                    data_ptr.to_str().unwrap().to_string()
                };
                let env = Env::from_raw(raw_env);
                let callback_info = CallbackInfo::<
                    3,
                >::new(raw_env, callback_info, None)?;
                let this = JsObject::from_raw(raw_env, callback_info.this())?;
                let wrapped = env.unwrap::<Box<dyn ScriptObject>>(&this)?;
                let arg = JsUnknown::from_raw(raw_env, callback_info.get_arg(0))?;
                let arg = js_value_to_script_value(&env, arg)
                    .map_err(|e| e.into_napi())?;
                wrapped.set_field_value(&field_name, arg).map_err(|e| e.into_napi())?;
                let result = env.get_undefined()?;
                Ok(result.raw())
            }
            generic_setter(raw_env, callback_info)
                .unwrap_or_else(|e| {
                    unsafe { JsError::from(e).throw_into(raw_env) };
                    std::ptr::null_mut()
                })
        }
        impl FruityError {
            /// Convert a js error to a fruity_game_engine error
            pub fn from_napi(err: napi::Error) -> Self {
                match err.status {
                    napi::Status::Ok => FruityError::Ok(err.reason.to_string()),
                    napi::Status::InvalidArg => {
                        FruityError::InvalidArg(err.reason.to_string())
                    }
                    napi::Status::ObjectExpected => {
                        FruityError::ObjectExpected(err.reason.to_string())
                    }
                    napi::Status::StringExpected => {
                        FruityError::StringExpected(err.reason.to_string())
                    }
                    napi::Status::NameExpected => {
                        FruityError::NameExpected(err.reason.to_string())
                    }
                    napi::Status::FunctionExpected => {
                        FruityError::FunctionExpected(err.reason.to_string())
                    }
                    napi::Status::NumberExpected => {
                        FruityError::NumberExpected(err.reason.to_string())
                    }
                    napi::Status::BooleanExpected => {
                        FruityError::BooleanExpected(err.reason.to_string())
                    }
                    napi::Status::ArrayExpected => {
                        FruityError::ArrayExpected(err.reason.to_string())
                    }
                    napi::Status::GenericFailure => {
                        FruityError::GenericFailure(err.reason.to_string())
                    }
                    napi::Status::PendingException => {
                        FruityError::PendingException(err.reason.to_string())
                    }
                    napi::Status::Cancelled => {
                        FruityError::Cancelled(err.reason.to_string())
                    }
                    napi::Status::EscapeCalledTwice => {
                        FruityError::EscapeCalledTwice(err.reason.to_string())
                    }
                    napi::Status::HandleScopeMismatch => {
                        FruityError::HandleScopeMismatch(err.reason.to_string())
                    }
                    napi::Status::CallbackScopeMismatch => {
                        FruityError::CallbackScopeMismatch(err.reason.to_string())
                    }
                    napi::Status::QueueFull => {
                        FruityError::QueueFull(err.reason.to_string())
                    }
                    napi::Status::Closing => FruityError::Closing(err.reason.to_string()),
                    napi::Status::BigintExpected => {
                        FruityError::BigintExpected(err.reason.to_string())
                    }
                    napi::Status::DateExpected => {
                        FruityError::DateExpected(err.reason.to_string())
                    }
                    napi::Status::ArrayBufferExpected => {
                        FruityError::ArrayBufferExpected(err.reason.to_string())
                    }
                    napi::Status::DetachableArraybufferExpected => {
                        FruityError::DetachableArraybufferExpected(
                            err.reason.to_string(),
                        )
                    }
                    napi::Status::WouldDeadlock => {
                        FruityError::WouldDeadlock(err.reason.to_string())
                    }
                    napi::Status::Unknown => FruityError::Unknown(err.reason.to_string()),
                }
            }
            /// Convert a fruity_game_engine error to a js error
            pub fn into_napi(self) -> napi::Error {
                match self {
                    FruityError::Ok(message) => {
                        napi::Error::new(napi::Status::Ok, message)
                    }
                    FruityError::InvalidArg(message) => {
                        napi::Error::new(napi::Status::InvalidArg, message)
                    }
                    FruityError::ObjectExpected(message) => {
                        napi::Error::new(napi::Status::ObjectExpected, message)
                    }
                    FruityError::StringExpected(message) => {
                        napi::Error::new(napi::Status::StringExpected, message)
                    }
                    FruityError::NameExpected(message) => {
                        napi::Error::new(napi::Status::NameExpected, message)
                    }
                    FruityError::FunctionExpected(message) => {
                        napi::Error::new(napi::Status::FunctionExpected, message)
                    }
                    FruityError::NumberExpected(message) => {
                        napi::Error::new(napi::Status::NumberExpected, message)
                    }
                    FruityError::BooleanExpected(message) => {
                        napi::Error::new(napi::Status::BooleanExpected, message)
                    }
                    FruityError::ArrayExpected(message) => {
                        napi::Error::new(napi::Status::ArrayExpected, message)
                    }
                    FruityError::GenericFailure(message) => {
                        napi::Error::new(napi::Status::GenericFailure, message)
                    }
                    FruityError::PendingException(message) => {
                        napi::Error::new(napi::Status::PendingException, message)
                    }
                    FruityError::Cancelled(message) => {
                        napi::Error::new(napi::Status::Cancelled, message)
                    }
                    FruityError::EscapeCalledTwice(message) => {
                        napi::Error::new(napi::Status::EscapeCalledTwice, message)
                    }
                    FruityError::HandleScopeMismatch(message) => {
                        napi::Error::new(napi::Status::HandleScopeMismatch, message)
                    }
                    FruityError::CallbackScopeMismatch(message) => {
                        napi::Error::new(napi::Status::CallbackScopeMismatch, message)
                    }
                    FruityError::QueueFull(message) => {
                        napi::Error::new(napi::Status::QueueFull, message)
                    }
                    FruityError::Closing(message) => {
                        napi::Error::new(napi::Status::Closing, message)
                    }
                    FruityError::BigintExpected(message) => {
                        napi::Error::new(napi::Status::BigintExpected, message)
                    }
                    FruityError::DateExpected(message) => {
                        napi::Error::new(napi::Status::DateExpected, message)
                    }
                    FruityError::ArrayBufferExpected(message) => {
                        napi::Error::new(napi::Status::ArrayBufferExpected, message)
                    }
                    FruityError::DetachableArraybufferExpected(message) => {
                        napi::Error::new(
                            napi::Status::DetachableArraybufferExpected,
                            message,
                        )
                    }
                    FruityError::WouldDeadlock(message) => {
                        napi::Error::new(napi::Status::WouldDeadlock, message)
                    }
                    FruityError::Unknown(message) => {
                        napi::Error::new(napi::Status::Unknown, message)
                    }
                }
            }
        }
    }
    #[cfg(feature = "napi-module")]
    pub use crate::javascript::napi::*;
}
/// Tools to load dynamicaly modules
pub mod module {
    use crate::script_value::convert::TryFromScriptValue;
    use crate::settings::Settings;
    use crate::world::World;
    use crate::FruityResult;
    use std::rc::Rc;
    /// A service to manage modules loading
    pub mod modules_service {
        use crate::export;
        use crate::module::Module;
        use crate::FruityResult;
        use crate::ResourceContainer;
        /// A service for frame management
        pub struct ModulesService {
            modules: Vec<Module>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ModulesService {
            #[inline]
            fn clone(&self) -> ModulesService {
                ModulesService {
                    modules: ::core::clone::Clone::clone(&self.modules),
                }
            }
        }
        impl ModulesService {
            /// Returns an ModulesService
            pub fn new(_resource_container: ResourceContainer) -> Self {
                Self { modules: Vec::new() }
            }
            /// Register a module
            pub fn register_module(&mut self, module: Module) {
                self.modules.push(module);
            }
            /// Traverse the stored modules, order taking care of dependencies
            pub fn traverse_modules_by_dependencies(
                &self,
                callback: &dyn Fn(Module) -> FruityResult<()>,
            ) -> FruityResult<()> {
                let mut processed_module_identifiers = Vec::<String>::new();
                let mut remaining_modules = self
                    .modules
                    .iter()
                    .map(|module| module.clone())
                    .collect::<Vec<_>>();
                while remaining_modules.len() > 0 {
                    let (with_all_dependencies_loaded, others): (Vec<_>, Vec<_>) = remaining_modules
                        .into_iter()
                        .partition(|loader| {
                            loader
                                .dependencies
                                .iter()
                                .all(|dependency| {
                                    processed_module_identifiers.contains(&dependency)
                                })
                        });
                    if with_all_dependencies_loaded.len() == 0 {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1_formatted(
                                &[
                                    "A problem happened, couldn\'t load the dependencies cause there is a missing dependency, the modules that are still waiting to be initialized are:\n",
                                ],
                                &[
                                    ::core::fmt::ArgumentV1::new_debug(
                                        &&others
                                            .iter()
                                            .map(|module| module.name.clone())
                                            .collect::<Vec<_>>(),
                                    ),
                                ],
                                &[
                                    ::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 4u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ),
                        );
                    }
                    processed_module_identifiers
                        .append(
                            &mut with_all_dependencies_loaded
                                .iter()
                                .map(|module| module.name.clone())
                                .collect::<Vec<_>>(),
                        );
                    with_all_dependencies_loaded
                        .into_iter()
                        .try_for_each(|module| callback(module))?;
                    remaining_modules = others;
                }
                Ok(())
            }
        }
    }
    /// A module for the engine
    pub struct Module {
        /// The name of the module
        pub name: String,
        /// The dependencies of the module
        pub dependencies: Vec<String>,
        /// A function that initialize the module
        pub setup: Option<Rc<dyn Fn(World, Settings) -> FruityResult<()>>>,
        /// A function that initialize the module resources
        pub load_resources: Option<Rc<dyn Fn(World, Settings) -> FruityResult<()>>>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Module {
        #[inline]
        fn clone(&self) -> Module {
            Module {
                name: ::core::clone::Clone::clone(&self.name),
                dependencies: ::core::clone::Clone::clone(&self.dependencies),
                setup: ::core::clone::Clone::clone(&self.setup),
                load_resources: ::core::clone::Clone::clone(&self.load_resources),
            }
        }
    }
    impl crate::script_value::convert::TryFromScriptValue for Module {
        fn from_script_value(
            value: crate::script_value::ScriptValue,
        ) -> crate::FruityResult<Self> {
            match value {
                crate::script_value::ScriptValue::Object(value) => {
                    match value.downcast::<Self>() {
                        Ok(value) => Ok(*value),
                        Err(value) => {
                            Ok(Self {
                                name: <String>::from_script_value(
                                    value.get_field_value("name")?,
                                )?,
                                dependencies: <Vec<
                                    String,
                                >>::from_script_value(
                                    value.get_field_value("dependencies")?,
                                )?,
                                setup: <Option<
                                    Rc<dyn Fn(World, Settings) -> FruityResult<()>>,
                                >>::from_script_value(value.get_field_value("setup")?)?,
                                load_resources: <Option<
                                    Rc<dyn Fn(World, Settings) -> FruityResult<()>>,
                                >>::from_script_value(
                                    value.get_field_value("load_resources")?,
                                )?,
                            })
                        }
                    }
                }
                _ => {
                    Err(
                        crate::FruityError::InvalidArg({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to native object"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Module {
        #[inline]
        fn default() -> Module {
            Module {
                name: ::core::default::Default::default(),
                dependencies: ::core::default::Default::default(),
                setup: ::core::default::Default::default(),
                load_resources: ::core::default::Default::default(),
            }
        }
    }
}
/// All related with resources
pub mod resource {
    use crate::{any::FruityAny, introspect::IntrospectObject, RwLock};
    use std::{any::Any, fmt::Debug, sync::Arc};
    pub use fruity_game_engine_macro::Resource;
    /// A reference over a resource that is supposed to be used by components
    pub mod resource_reference {
        use crate::any::FruityAny;
        use crate::introspect::IntrospectObject;
        use crate::resource::Resource;
        use crate::script_value::ScriptValue;
        use crate::FruityResult;
        use crate::RwLock;
        use crate::RwLockReadGuard;
        use crate::RwLockWriteGuard;
        use std::ops::Deref;
        use std::ops::DerefMut;
        use std::sync::Arc;
        /// A reference over an any resource that is supposed to be used by components
        pub struct AnyResourceReference {
            /// The name of the resource
            pub name: String,
            /// The resource reference
            pub resource: Arc<dyn Resource>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AnyResourceReference {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "AnyResourceReference",
                    "name",
                    &&self.name,
                    "resource",
                    &&self.resource,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AnyResourceReference {
            #[inline]
            fn clone(&self) -> AnyResourceReference {
                AnyResourceReference {
                    name: ::core::clone::Clone::clone(&self.name),
                    resource: ::core::clone::Clone::clone(&self.resource),
                }
            }
        }
        impl crate::any::FruityAny for AnyResourceReference {
            fn get_type_name(&self) -> &'static str {
                "AnyResourceReference"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn crate::any::FruityAny> {
                self
            }
        }
        impl AnyResourceReference {
            /// Create a resource reference from a resource
            pub fn from_native<T: Resource + ?Sized>(
                name: &str,
                resource: Box<T>,
            ) -> Self {
                AnyResourceReference {
                    name: name.to_string(),
                    resource: Arc::new(RwLock::new(resource)),
                }
            }
            /// Get the name of the referenced resource
            pub fn get_name(&self) -> String {
                self.name.clone()
            }
            /// Get the name of the referenced resource
            pub fn downcast<T: Resource + ?Sized>(
                &self,
            ) -> Option<ResourceReference<T>> {
                self.resource
                    .clone()
                    .as_any_arc()
                    .downcast::<RwLock<Box<T>>>()
                    .ok()
                    .map(|resource| ResourceReference::new(&self.name, resource))
            }
        }
        impl IntrospectObject for AnyResourceReference {
            fn get_class_name(&self) -> FruityResult<String> {
                self.resource.get_class_name()
            }
            fn get_field_names(&self) -> FruityResult<Vec<String>> {
                self.resource.get_field_names()
            }
            fn set_field_value(
                &mut self,
                _name: &str,
                _value: ScriptValue,
            ) -> FruityResult<()> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
                self.resource.get_field_value(name)
            }
            fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
                self.resource.get_const_method_names()
            }
            fn call_const_method(
                &self,
                name: &str,
                args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                self.resource.call_const_method(name, args)
            }
            fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                _name: &str,
                _args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        /// A reference over a resource that is supposed to be used by components
        pub struct ResourceReference<T: Resource + ?Sized> {
            /// The name of the resource
            pub name: String,
            /// The resource reference
            pub resource: Arc<RwLock<Box<T>>>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug + Resource + ?Sized> ::core::fmt::Debug
        for ResourceReference<T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ResourceReference",
                    "name",
                    &&self.name,
                    "resource",
                    &&self.resource,
                )
            }
        }
        impl<T: Resource + ?Sized> crate::any::FruityAny for ResourceReference<T> {
            fn get_type_name(&self) -> &'static str {
                "ResourceReference"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn crate::any::FruityAny> {
                self
            }
        }
        impl<T: Resource + ?Sized> ResourceReference<T> {
            /// Create a resource reference from a resource
            pub fn new(name: &str, resource: Arc<RwLock<Box<T>>>) -> Self {
                ResourceReference {
                    name: name.to_string(),
                    resource,
                }
            }
            /// Get the name of the referenced resource
            pub fn get_name(&self) -> String {
                self.name.clone()
            }
            /// Create a read guard over the resource
            pub fn read(&self) -> ResourceReadGuard<T> {
                let inner_guard = self.resource.read();
                let inner_guard = unsafe {
                    std::mem::transmute::<
                        RwLockReadGuard<Box<T>>,
                        RwLockReadGuard<'static, Box<T>>,
                    >(inner_guard)
                };
                ResourceReadGuard::<T> {
                    _referenced: self.resource.clone(),
                    inner_guard,
                }
            }
            /// Create a write guard over the resource
            pub fn write(&self) -> ResourceWriteGuard<T> {
                let inner_guard = self.resource.write();
                let inner_guard = unsafe {
                    std::mem::transmute::<
                        RwLockWriteGuard<Box<T>>,
                        RwLockWriteGuard<'static, Box<T>>,
                    >(inner_guard)
                };
                ResourceWriteGuard::<T> {
                    _referenced: self.resource.clone(),
                    inner_guard,
                }
            }
        }
        impl<T: Resource + ?Sized> Clone for ResourceReference<T> {
            fn clone(&self) -> Self {
                ResourceReference::<T>::new(&self.name, self.resource.clone())
            }
        }
        /// A read guard for a resource reference
        pub struct ResourceReadGuard<T: Resource + ?Sized> {
            _referenced: Arc<RwLock<Box<T>>>,
            inner_guard: RwLockReadGuard<'static, Box<T>>,
        }
        impl<'a, T: Resource + ?Sized> ResourceReadGuard<T> {
            /// Downcast to the original sized type that implement the resource trait
            pub fn downcast_ref<U: Resource>(&self) -> &U {
                self.deref().as_any_ref().downcast_ref::<U>().unwrap()
            }
        }
        impl<'a, T: Resource + ?Sized> Deref for ResourceReadGuard<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                self.inner_guard.deref()
            }
        }
        /// A write guard for a resource reference
        pub struct ResourceWriteGuard<T: Resource + ?Sized> {
            _referenced: Arc<RwLock<Box<T>>>,
            inner_guard: RwLockWriteGuard<'static, Box<T>>,
        }
        impl<T: Resource + ?Sized> ResourceWriteGuard<T> {
            /// Downcast to the original sized type that implement the resource trait
            pub fn downcast_ref<U: Resource>(&self) -> &U {
                self.deref().as_any_ref().downcast_ref::<U>().unwrap()
            }
        }
        impl<T: Resource + ?Sized> ResourceWriteGuard<T> {
            /// Downcast to the original sized type that implement the resource trait
            pub fn downcast_mut<U: Resource>(&mut self) -> &mut U {
                self.deref_mut().as_any_mut().downcast_mut::<U>().unwrap()
            }
        }
        impl<T: Resource + ?Sized> Deref for ResourceWriteGuard<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                self.inner_guard.deref()
            }
        }
        impl<T: Resource + ?Sized> DerefMut for ResourceWriteGuard<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.inner_guard.deref_mut()
            }
        }
    }
    /// The resource manager
    pub mod resource_container {
        use super::resource_reference::AnyResourceReference;
        use crate::any::FruityAny;
        use crate::resource::resource_reference::ResourceReference;
        use crate::resource::Resource;
        use crate::settings::Settings;
        use crate::FruityError;
        use crate::FruityResult;
        use crate::RwLock;
        use std::any::TypeId;
        use std::collections::HashMap;
        use std::fmt::Debug;
        use std::fs::File;
        use std::io::Read;
        use std::path::Path;
        use std::sync::Arc;
        /// A a function that is used to load a resource
        pub type ResourceLoader = fn(&str, &mut dyn Read, Settings, ResourceContainer);
        pub(crate) struct InnerResourceContainer {
            resources: HashMap<String, AnyResourceReference>,
            identifier_by_type: HashMap<TypeId, String>,
            resource_loaders: HashMap<String, ResourceLoader>,
        }
        /// The resource manager
        pub struct ResourceContainer {
            pub(crate) inner: Arc<RwLock<InnerResourceContainer>>,
        }
        impl crate::any::FruityAny for ResourceContainer {
            fn get_type_name(&self) -> &'static str {
                "ResourceContainer"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn crate::any::FruityAny> {
                self
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ResourceContainer {
            #[inline]
            fn clone(&self) -> ResourceContainer {
                ResourceContainer {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        impl ResourceContainer {
            /// Returns a ResourceContainer
            pub fn new() -> ResourceContainer {
                ResourceContainer {
                    inner: Arc::new(
                        RwLock::new(InnerResourceContainer {
                            resources: HashMap::new(),
                            identifier_by_type: HashMap::new(),
                            resource_loaders: HashMap::new(),
                        }),
                    ),
                }
            }
            /// Get a required resource by it's identifier
            /// Panic if the resource is not known
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            /// # Generic Arguments
            /// * `T` - The resource type
            ///
            pub fn require<T: Resource + ?Sized>(&self) -> ResourceReference<T> {
                let inner = self.inner.read();
                match inner.identifier_by_type.get(&TypeId::of::<T>()) {
                    Some(resource_name) => {
                        match inner.resources.get(resource_name) {
                            Some(resource) => {
                                match resource.downcast::<T>() {
                                    Some(resource) => resource,
                                    None => {
                                        ::core::panicking::panic_fmt(
                                            ::core::fmt::Arguments::new_v1(
                                                &["Failed to get a required resource"],
                                                &[],
                                            ),
                                        )
                                    }
                                }
                            }
                            None => {
                                ::core::panicking::panic_fmt(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Failed to get a required resource"],
                                        &[],
                                    ),
                                )
                            }
                        }
                    }
                    None => {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["Failed to get a required resource"],
                                &[],
                            ),
                        )
                    }
                }
            }
            /// Get a resource by it's identifier
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            /// # Generic Arguments
            /// * `T` - The resource type
            ///
            pub fn get<T: Resource + ?Sized>(
                &self,
                identifier: &str,
            ) -> Option<ResourceReference<T>> {
                let inner = self.inner.read();
                match inner.resources.get(identifier).map(|resource| resource.clone()) {
                    Some(resource) => resource.downcast::<T>(),
                    None => None,
                }
            }
            /// Get a resource by it's identifier without casting it
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            pub fn get_untyped(&self, identifier: &str) -> Option<AnyResourceReference> {
                let inner = self.inner.read();
                inner.resources.get(identifier).map(|resource| resource.clone())
            }
            /// Check if a resource identifier has already been registered
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            pub fn contains(&self, identifier: &str) -> bool {
                let inner = self.inner.read();
                inner.resources.contains_key(identifier)
            }
            /// Add a resource into the collection
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            /// * `resource` - The resource object
            ///
            pub fn add<T: Resource + ?Sized>(&self, identifier: &str, resource: Box<T>) {
                let mut inner = self.inner.write();
                let shared = AnyResourceReference::from_native(identifier, resource);
                inner.resources.insert(identifier.to_string(), shared.clone());
                inner
                    .identifier_by_type
                    .insert(TypeId::of::<T>(), identifier.to_string());
            }
            /// Remove a resource of the collection
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            pub fn remove(&self, identifier: &str) -> FruityResult<()> {
                let mut inner = self.inner.write();
                if inner.resources.contains_key(identifier) {
                    inner.resources.remove(identifier);
                    Ok(())
                } else {
                    Err(
                        FruityError::GenericFailure({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Resource ", " doesn\'t exists"],
                                    &[::core::fmt::ArgumentV1::new_display(&identifier)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
            /// Add a resource loader that will be used to load resources
            ///
            /// # Arguments
            /// * `resource_type` - The resource loader type
            /// * `loader` - The resource loader
            ///
            pub fn add_resource_loader(
                &self,
                resource_type: &str,
                loader: ResourceLoader,
            ) {
                let mut inner = self.inner.write();
                inner.resource_loaders.insert(resource_type.to_string(), loader);
            }
            /// Load an any resource file
            ///
            /// # Arguments
            /// * `path` - The path of the file
            /// * `resource_type` - The resource type
            ///
            pub fn load_resource_file(
                &self,
                path: &str,
                resource_type: &str,
            ) -> FruityResult<()> {
                let mut file = File::open(path)
                    .map_err(|_| {
                        FruityError::GenericFailure({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["File couldn\'t be opened: "],
                                    &[::core::fmt::ArgumentV1::new_debug(&path)],
                                ),
                            );
                            res
                        })
                    })?;
                Self::load_resource(
                    self,
                    path,
                    resource_type,
                    &mut file,
                    Settings::default(),
                )?;
                Ok(())
            }
            /// Load and add a resource into the collection
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            /// * `resource_type` - The resource type
            /// * `read` - The reader, generaly a file reader
            ///
            pub fn load_resource(
                &self,
                identifier: &str,
                resource_type: &str,
                reader: &mut dyn Read,
                settings: Settings,
            ) -> FruityResult<()> {
                let resource_loader = {
                    let inner_reader = self.inner.read();
                    if let Some(resource_loader)
                        = inner_reader.resource_loaders.get(resource_type)
                    {
                        Ok(resource_loader.clone())
                    } else {
                        Err(
                            FruityError::GenericFailure({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Resource type ", " is not registered"],
                                        &[::core::fmt::ArgumentV1::new_display(&resource_type)],
                                    ),
                                );
                                res
                            }),
                        )
                    }?
                };
                resource_loader(identifier, reader, settings, self.clone());
                Ok(())
            }
            /// Load many resources for settings
            ///
            /// # Arguments
            /// * `settings` - The settings of resources
            ///
            pub fn load_resources_settings(&self, settings: Vec<Settings>) {
                settings
                    .into_iter()
                    .for_each(|settings| {
                        Self::load_resource_settings(self, settings);
                    })
            }
            /// Load resources for settings
            ///
            /// # Arguments
            /// * `settings` - The settings of resources
            ///
            pub fn load_resource_settings(&self, settings: Settings) -> Option<()> {
                let fields = if let Settings::Object(fields) = settings {
                    fields
                } else {
                    return None;
                };
                let name = {
                    if let Settings::String(name) = fields.get("name")? {
                        name.clone()
                    } else {
                        return None;
                    }
                };
                let path = {
                    if let Settings::String(path) = fields.get("path")? {
                        path.clone()
                    } else {
                        return None;
                    }
                };
                let resource_type = Path::new(&path).extension()?;
                let resource_type = resource_type.to_str()?;
                let mut resource_file = File::open(&path).ok()?;
                Self::load_resource(
                        self,
                        &name,
                        resource_type,
                        &mut resource_file,
                        Settings::Object(fields.clone()),
                    )
                    .ok()?;
                Some(())
            }
        }
        impl Debug for ResourceContainer {
            fn fmt(
                &self,
                _formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                Ok(())
            }
        }
    }
    /// The resource manager for script resources
    /// These resources are not Send + Sync, so this container is intended to be stored
    /// directly into the world, and provide also access to the Send + Sync resources by
    /// referencing the classic ResourceContainer
    pub mod script_resource_container {
        use super::{
            resource_container::ResourceContainer,
            resource_reference::AnyResourceReference,
        };
        use crate::{
            any::FruityAny, javascript::JsIntrospectObject,
            script_value::{convert::TryIntoScriptValue, ScriptValue},
            FruityError, FruityResult,
        };
        use fruity_game_engine_macro::{export, fruity_export};
        use std::{cell::RefCell, collections::HashMap, rc::Rc};
        /// The resource manager exposed to scripting language
        pub struct ScriptResourceContainer {
            resource_container: ResourceContainer,
            script_resources: Rc<RefCell<HashMap<String, JsIntrospectObject>>>,
        }
        impl crate::any::FruityAny for ScriptResourceContainer {
            fn get_type_name(&self) -> &'static str {
                "ScriptResourceContainer"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
                self
            }
            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn crate::any::FruityAny> {
                self
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ScriptResourceContainer {
            #[inline]
            fn clone(&self) -> ScriptResourceContainer {
                ScriptResourceContainer {
                    resource_container: ::core::clone::Clone::clone(
                        &self.resource_container,
                    ),
                    script_resources: ::core::clone::Clone::clone(&self.script_resources),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ScriptResourceContainer {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ScriptResourceContainer",
                    "resource_container",
                    &&self.resource_container,
                    "script_resources",
                    &&self.script_resources,
                )
            }
        }
        impl ScriptResourceContainer {
            /// Returns a ResourceContainer
            pub fn new(resource_container: ResourceContainer) -> Self {
                Self {
                    resource_container,
                    script_resources: Rc::new(RefCell::new(HashMap::new())),
                }
            }
            /// Get a resource by it's identifier without casting it
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            pub fn get(&self, identifier: String) -> Option<ScriptOrNativeResource> {
                match self.resource_container.get_untyped(&identifier) {
                    Some(value) => Some(ScriptOrNativeResource::Native(value)),
                    None => {
                        self
                            .script_resources
                            .borrow()
                            .get(&identifier)
                            .map(|resource| ScriptOrNativeResource::Script(
                                resource.clone(),
                            ))
                    }
                }
            }
            /// Check if a resource identifier has already been registered
            /// Use String as identifier, to match the scripting wrapper requirements
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            pub fn contains(&self, identifier: String) -> bool {
                self.resource_container.contains(&identifier)
                    || self.script_resources.borrow().contains_key(&identifier)
            }
            /// Add a resource into the collection with an unknown type
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            /// * `resource` - The resource object
            ///
            pub fn add(&self, identifier: String, resource: JsIntrospectObject) {
                self.script_resources.borrow_mut().insert(identifier, resource);
            }
            /// Remove a resource of the collection
            /// Use String as identifier, to match the scripting wrapper requirements
            ///
            /// # Arguments
            /// * `identifier` - The resource identifier
            ///
            pub fn remove(&self, identifier: String) -> FruityResult<()> {
                match self.resource_container.remove(&identifier) {
                    Ok(()) => Ok(()),
                    Err(_) => {
                        if self.script_resources.borrow().contains_key(&identifier) {
                            self.script_resources.borrow_mut().remove(&identifier);
                            Ok(())
                        } else {
                            Err(
                                FruityError::GenericFailure({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["Resource ", " doesn\'t exists"],
                                            &[::core::fmt::ArgumentV1::new_display(&identifier)],
                                        ),
                                    );
                                    res
                                }),
                            )
                        }
                    }
                }
            }
        }
        impl crate::introspect::IntrospectObject for ScriptResourceContainer {
            fn get_class_name(&self) -> crate::FruityResult<String> {
                Ok("unknown".to_string())
            }
            fn get_field_names(&self) -> crate::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: crate::script_value::ScriptValue,
            ) -> crate::FruityResult<()> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> crate::FruityResult<crate::script_value::ScriptValue> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_const_method_names(&self) -> crate::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            "get".to_string(),
                            "contains".to_string(),
                            "add".to_string(),
                            "remove".to_string(),
                        ]),
                    ),
                )
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<crate::script_value::ScriptValue>,
            ) -> crate::FruityResult<crate::script_value::ScriptValue> {
                use crate::script_value::convert::TryIntoScriptValue;
                match name {
                    "get" => {
                        let mut __caster = crate::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<String>()?;
                        self.get(__arg_0).into_script_value()
                    }
                    "contains" => {
                        let mut __caster = crate::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<String>()?;
                        self.contains(__arg_0).into_script_value()
                    }
                    "add" => {
                        let mut __caster = crate::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<String>()?;
                        let __arg_1 = __caster.cast_next::<JsIntrospectObject>()?;
                        self.add(__arg_0, __arg_1).into_script_value()
                    }
                    "remove" => {
                        let mut __caster = crate::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<String>()?;
                        self.remove(__arg_0).into_script_value()
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn get_mut_method_names(&self) -> crate::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<crate::script_value::ScriptValue>,
            ) -> crate::FruityResult<crate::script_value::ScriptValue> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        /// Neither a script or a native resource
        pub enum ScriptOrNativeResource {
            /// A script resource
            Script(JsIntrospectObject),
            /// A native resource
            Native(AnyResourceReference),
        }
        impl TryIntoScriptValue for ScriptOrNativeResource {
            fn into_script_value(self) -> FruityResult<ScriptValue> {
                match self {
                    ScriptOrNativeResource::Script(resource) => {
                        resource.into_script_value()
                    }
                    ScriptOrNativeResource::Native(resource) => {
                        resource.into_script_value()
                    }
                }
            }
        }
    }
    /// A trait that should be implemented by every resources
    pub trait Resource: FruityAny + IntrospectObject + Debug + Send + Sync {
        /// Get a box containing a resource as a boxed resource
        fn as_resource_box(self: Box<Self>) -> Box<dyn Resource>;
        /// Return self as an Any arc
        fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
    }
    impl<T> Resource for RwLock<Box<T>>
    where
        T: Resource + ?Sized,
    {
        fn as_resource_box(self: Box<Self>) -> Box<dyn Resource> {
            self
        }
        fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
            self
        }
    }
}
/// Provides a tool to inject resources into functions
pub mod inject {
    use crate::resource::resource_container::ResourceContainer;
    use crate::resource::resource_reference::ResourceReadGuard;
    use crate::resource::resource_reference::ResourceReference;
    use crate::resource::resource_reference::ResourceWriteGuard;
    use crate::resource::Resource;
    /// A reference over a resource
    pub type Ref<T> = ResourceReference<T>;
    /// A read guard over a resource
    pub type Const<T> = ResourceReadGuard<T>;
    /// A write guard over a resource
    pub type Mut<T> = ResourceWriteGuard<T>;
    /// A trait for a function that needs injection from resource container
    /// A simple implementation of the dependency injection pattern
    pub trait Injectable: 'static {
        /// Get the object
        fn from_resource_container(resource_container: &ResourceContainer) -> Self;
    }
    impl Injectable for ResourceContainer {
        fn from_resource_container(resource_container: &ResourceContainer) -> Self {
            resource_container.clone()
        }
    }
    impl<T: Resource + ?Sized> Injectable for Ref<T> {
        fn from_resource_container(resource_container: &ResourceContainer) -> Self {
            resource_container.require::<T>()
        }
    }
    impl<T: Resource + ?Sized> Injectable for Const<T> {
        fn from_resource_container(resource_container: &ResourceContainer) -> Self {
            let reference = Ref::<T>::from_resource_container(resource_container);
            reference.read()
        }
    }
    impl<T: Resource + ?Sized> Injectable for Mut<T> {
        fn from_resource_container(resource_container: &ResourceContainer) -> Self {
            let reference = Ref::<T>::from_resource_container(resource_container);
            reference.write()
        }
    }
    /// A trait that is implemented by functions that supports dependency injection
    pub trait Inject<R> {
        /// Get a function that proceed the injection
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync>;
    }
    impl<R: 'static> Inject<R> for &'static (dyn Fn() -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |_| self())
        }
    }
    impl<T1: Injectable, R: 'static> Inject<R>
    for &'static (dyn Fn(T1) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| self(
                T1::from_resource_container(&resource_container),
            ))
        }
    }
    impl<T1: Injectable, T2: Injectable, R: 'static> Inject<R>
    for &'static (dyn Fn(T1, T2) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<T1: Injectable, T2: Injectable, T3: Injectable, R: 'static> Inject<R>
    for &'static (dyn Fn(T1, T2, T3) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        R: 'static,
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        R: 'static,
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        R: 'static,
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5, T6) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        R: 'static,
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                    T14::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                    T14::from_resource_container(&resource_container),
                    T15::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                    T14::from_resource_container(&resource_container),
                    T15::from_resource_container(&resource_container),
                    T16::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                    T14::from_resource_container(&resource_container),
                    T15::from_resource_container(&resource_container),
                    T16::from_resource_container(&resource_container),
                    T17::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
        T18: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                    T14::from_resource_container(&resource_container),
                    T15::from_resource_container(&resource_container),
                    T16::from_resource_container(&resource_container),
                    T17::from_resource_container(&resource_container),
                    T18::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
        T18: Injectable,
        T19: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                    T14::from_resource_container(&resource_container),
                    T15::from_resource_container(&resource_container),
                    T16::from_resource_container(&resource_container),
                    T17::from_resource_container(&resource_container),
                    T18::from_resource_container(&resource_container),
                    T19::from_resource_container(&resource_container),
                )
            })
        }
    }
    impl<
        T1: Injectable,
        T2: Injectable,
        T3: Injectable,
        T4: Injectable,
        T5: Injectable,
        T6: Injectable,
        T7: Injectable,
        T8: Injectable,
        T9: Injectable,
        T10: Injectable,
        T11: Injectable,
        T12: Injectable,
        T13: Injectable,
        T14: Injectable,
        T15: Injectable,
        T16: Injectable,
        T17: Injectable,
        T18: Injectable,
        T19: Injectable,
        T20: Injectable,
        R: 'static,
    > Inject<R>
    for &'static (dyn Fn(
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
        T20,
    ) -> R + Sync + Send) {
        fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
            Box::new(move |resource_container| {
                self(
                    T1::from_resource_container(&resource_container),
                    T2::from_resource_container(&resource_container),
                    T3::from_resource_container(&resource_container),
                    T4::from_resource_container(&resource_container),
                    T5::from_resource_container(&resource_container),
                    T6::from_resource_container(&resource_container),
                    T7::from_resource_container(&resource_container),
                    T8::from_resource_container(&resource_container),
                    T9::from_resource_container(&resource_container),
                    T10::from_resource_container(&resource_container),
                    T11::from_resource_container(&resource_container),
                    T12::from_resource_container(&resource_container),
                    T13::from_resource_container(&resource_container),
                    T14::from_resource_container(&resource_container),
                    T15::from_resource_container(&resource_container),
                    T16::from_resource_container(&resource_container),
                    T17::from_resource_container(&resource_container),
                    T18::from_resource_container(&resource_container),
                    T19::from_resource_container(&resource_container),
                    T20::from_resource_container(&resource_container),
                )
            })
        }
    }
}
/// Provides a factory for the introspect object
/// This will be used by to do the snapshots
pub mod object_factory_service {
    use crate::{
        any::FruityAny, resource::{resource_container::ResourceContainer, Resource},
        script_value::ScriptValue, FruityResult,
    };
    use fruity_game_engine_macro::{export, fruity_export};
    use std::{collections::HashMap, sync::Arc};
    /// Trait to implement a generic constructor from a ScriptValue
    pub trait ObjectFactory {
        /// Get a constructor to instantiate an object
        fn get_constructor() -> Constructor;
    }
    /// A setter caller
    pub type Constructor = Arc<
        dyn Fn(
            ResourceContainer,
            Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> + Send + Sync,
    >;
    /// Provides a factory for the introspect types
    /// This will be used by to do the snapshots
    pub struct ObjectFactoryService {
        resource_container: ResourceContainer,
        factories: HashMap<String, Constructor>,
    }
    impl crate::any::FruityAny for ObjectFactoryService {
        fn get_type_name(&self) -> &'static str {
            "ObjectFactoryService"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    impl crate::resource::Resource for ObjectFactoryService {
        fn as_resource_box(self: Box<Self>) -> Box<dyn crate::resource::Resource> {
            self
        }
        fn as_any_arc(
            self: std::sync::Arc<Self>,
        ) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
            self
        }
    }
    impl ObjectFactoryService {
        /// Returns an ObjectFactoryService
        pub fn new(resource_container: ResourceContainer) -> ObjectFactoryService {
            ObjectFactoryService {
                resource_container,
                factories: HashMap::new(),
            }
        }
        /// Register a new object factory
        ///
        /// # Arguments
        /// * `object_type` - The object type identifier
        ///
        /// # Generic Arguments
        /// * `T` - The type of the object
        ///
        pub fn register<T>(&mut self, object_type: &str)
        where
            T: ObjectFactory,
        {
            self.factories.insert(object_type.to_string(), T::get_constructor());
        }
        /// Register a new object factory from a function constructor
        ///
        /// # Arguments
        /// * `object_type` - The object type identifier
        /// * `constructor` - The constructor
        ///
        pub fn register_func(
            &mut self,
            object_type: &str,
            constructor: impl Fn(
                ResourceContainer,
                Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> + Send + Sync + 'static,
        ) {
            self.factories.insert(object_type.to_string(), Arc::new(constructor));
        }
        /// Instantiate an object from it's factory
        ///
        /// # Arguments
        /// * `object_type` - The object type identifier
        /// * `serialized` - A serialized value that will populate the new component
        ///
        pub fn instantiate(
            &self,
            object_type: String,
            args: Vec<ScriptValue>,
        ) -> Option<ScriptValue> {
            let factory = self.factories.get(&object_type)?;
            let instantied = factory(self.resource_container.clone(), args).ok()?;
            Some(instantied)
        }
        /// Iterate over all object factories
        pub fn iter(&self) -> impl Iterator<Item = (&String, &Constructor)> {
            self.factories.iter()
        }
    }
    impl crate::introspect::IntrospectObject for ObjectFactoryService {
        fn get_class_name(&self) -> crate::FruityResult<String> {
            Ok("unknown".to_string())
        }
        fn get_field_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: crate::script_value::ScriptValue,
        ) -> crate::FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(
            &self,
            name: &str,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_const_method_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new(["instantiate".to_string()]),
                ),
            )
        }
        fn call_const_method(
            &self,
            name: &str,
            __args: Vec<crate::script_value::ScriptValue>,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            use crate::script_value::convert::TryIntoScriptValue;
            match name {
                "instantiate" => {
                    let mut __caster = crate::utils::introspect::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<String>()?;
                    let __arg_1 = __caster.cast_next::<Vec<ScriptValue>>()?;
                    self.instantiate(__arg_0, __arg_1).into_script_value()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_mut_method_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            __args: Vec<crate::script_value::ScriptValue>,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl std::fmt::Debug for ObjectFactoryService {
        fn fmt(
            &self,
            _: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            Ok(())
        }
    }
}
/// Provides tools to profile functions/blocks
pub mod profile {
    pub use puffin::profile_function;
    use puffin::{are_scopes_on, ProfilerScope};
    /// Profile a scope, you should keep the handle in the scope so it will use the drop
    /// automatic system to profile the duree of the scope
    pub fn profile_scope(identifier: &str) -> Result<Option<ProfilerScope>, ()> {
        let identifier = unsafe { std::mem::transmute::<&str, &str>(identifier) };
        let profiler_scope = if are_scopes_on() {
            Some(ProfilerScope::new(identifier, "system", ""))
        } else {
            None
        };
        Ok(profiler_scope)
    }
}
/// An observer pattern
pub mod signal {
    use crate::any::FruityAny;
    use crate::introspect::IntrospectObject;
    use crate::lazy_static;
    use crate::script_value::convert::TryFromScriptValue;
    use crate::script_value::convert::TryIntoScriptValue;
    use crate::script_value::ScriptCallback;
    use crate::script_value::ScriptValue;
    use crate::utils::introspect::ArgumentCaster;
    use crate::FruityResult;
    use crate::Mutex;
    use crate::RwLock;
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::ops::Deref;
    use std::ops::DerefMut;
    use std::rc::Rc;
    use std::sync::Arc;
    struct IdGenerator {
        incrementer: usize,
    }
    impl IdGenerator {
        pub fn new() -> IdGenerator {
            IdGenerator { incrementer: 0 }
        }
        pub fn generate_id(&mut self) -> usize {
            self.incrementer += 1;
            self.incrementer
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct ID_GENERATOR {
        __private_field: (),
    }
    #[doc(hidden)]
    static ID_GENERATOR: ID_GENERATOR = ID_GENERATOR {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for ID_GENERATOR {
        type Target = Mutex<IdGenerator>;
        fn deref(&self) -> &Mutex<IdGenerator> {
            #[inline(always)]
            fn __static_ref_initialize() -> Mutex<IdGenerator> {
                Mutex::new(IdGenerator::new())
            }
            #[inline(always)]
            fn __stability() -> &'static Mutex<IdGenerator> {
                static LAZY: ::lazy_static::lazy::Lazy<Mutex<IdGenerator>> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for ID_GENERATOR {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    /// An identifier for a signal observer
    pub struct ObserverIdentifier(usize);
    #[automatically_derived]
    impl ::core::fmt::Debug for ObserverIdentifier {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "ObserverIdentifier",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObserverIdentifier {
        #[inline]
        fn clone(&self) -> ObserverIdentifier {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ObserverIdentifier {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObserverIdentifier {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObserverIdentifier {
        #[inline]
        fn eq(&self, other: &ObserverIdentifier) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ObserverIdentifier {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ObserverIdentifier {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ObserverIdentifier {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    struct InternSignal<T: 'static> {
        observers: Vec<
            (ObserverIdentifier, Arc<dyn Fn(&T) -> FruityResult<()> + Sync + Send>),
        >,
    }
    impl<T: 'static> crate::any::FruityAny for InternSignal<T> {
        fn get_type_name(&self) -> &'static str {
            "InternSignal"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    /// An observer pattern
    pub struct Signal<T: 'static> {
        intern: Arc<RwLock<InternSignal<T>>>,
    }
    impl<T: 'static> crate::any::FruityAny for Signal<T> {
        fn get_type_name(&self) -> &'static str {
            "Signal"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone + 'static> ::core::clone::Clone for Signal<T> {
        #[inline]
        fn clone(&self) -> Signal<T> {
            Signal {
                intern: ::core::clone::Clone::clone(&self.intern),
            }
        }
    }
    impl<T> Signal<T> {
        /// Returns a Signal
        pub fn new() -> Signal<T> {
            Signal {
                intern: Arc::new(
                    RwLock::new(InternSignal {
                        observers: Vec::new(),
                    }),
                ),
            }
        }
        /// Add an observer to the signal
        /// An observer is a closure that will be called when the signal will be sent
        pub fn add_observer<F: Fn(&T) -> FruityResult<()> + Sync + Send + 'static>(
            &self,
            observer: F,
        ) -> ObserverHandler<T> {
            let mut intern_writer = self.intern.write();
            let mut id_generator = ID_GENERATOR.lock();
            let observer_id = ObserverIdentifier(id_generator.generate_id());
            intern_writer.observers.push((observer_id, Arc::new(observer)));
            ObserverHandler {
                observer_id,
                intern: self.intern.clone(),
            }
        }
        /// Add an observer to the signal that can dispose itself
        /// An observer is a closure that will be called when the signal will be sent
        pub fn add_self_dispose_observer<
            F: Fn(&T, &ObserverHandler<T>) -> FruityResult<()> + Sync + Send + 'static,
        >(&self, observer: F) {
            let mut intern_writer = self.intern.write();
            let mut id_generator = ID_GENERATOR.lock();
            let observer_id = ObserverIdentifier(id_generator.generate_id());
            let handler = ObserverHandler {
                observer_id,
                intern: self.intern.clone(),
            };
            intern_writer
                .observers
                .push((observer_id, Arc::new(move |data| observer(data, &handler))));
        }
        /// Notify that the event happened
        /// This will launch all the observers that are registered for this signal
        pub fn notify(&self, event: T) -> FruityResult<()> {
            let observers = {
                let intern = self.intern.read();
                intern.observers.clone()
            };
            observers.iter().try_for_each(|(_, observer)| observer(&event))
        }
    }
    impl<T> Default for Signal<T> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T> Debug for Signal<T> {
        fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
            Ok(())
        }
    }
    impl<T> IntrospectObject for Signal<T>
    where
        T: TryFromScriptValue + TryIntoScriptValue + Clone,
    {
        fn get_class_name(&self) -> FruityResult<String> {
            Ok("Signal".to_string())
        }
        fn get_field_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            _name: &str,
            _value: ScriptValue,
        ) -> FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(&self, _name: &str) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new(["notify".to_string()]),
                ),
            )
        }
        fn call_const_method(
            &self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            match name {
                "notify" => {
                    let mut caster = ArgumentCaster::new(args);
                    let arg1 = caster.cast_next::<T>()?;
                    let handle = self.notify(arg1);
                    handle.into_script_value()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new(["add_observer".to_string()]),
                ),
            )
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            match name {
                "add_observer" => {
                    let mut caster = ArgumentCaster::new(args);
                    let arg1 = caster.cast_next::<Rc<dyn ScriptCallback>>()?;
                    let callback = arg1.create_thread_safe_callback()?;
                    let handle = self
                        .add_observer(move |arg| {
                            let arg: ScriptValue = arg.clone().into_script_value()?;
                            callback(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([arg]),
                                ),
                            );
                            Ok(())
                        });
                    handle.into_script_value()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    /// A write guard over a signal property, when it's dropped, the update signal is sent
    pub struct SignalWriteGuard<'a, T: Send + Sync + Clone + 'static> {
        target: &'a mut SignalProperty<T>,
    }
    impl<'a, T: Send + Sync + Clone> Drop for SignalWriteGuard<'a, T> {
        fn drop(&mut self) {
            self.target.on_updated.notify(self.target.value.clone()).unwrap();
        }
    }
    impl<'a, T: Send + Sync + Clone> Deref for SignalWriteGuard<'a, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.target.value
        }
    }
    impl<'a, T: Send + Sync + Clone> DerefMut for SignalWriteGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.target.value
        }
    }
    /// A variable with a signal that is notified on update
    pub struct SignalProperty<T: Send + Sync + Clone + 'static> {
        value: T,
        /// A signal sent when the property is updated
        pub on_updated: Signal<T>,
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone + Send + Sync + Clone + 'static> ::core::clone::Clone
    for SignalProperty<T> {
        #[inline]
        fn clone(&self) -> SignalProperty<T> {
            SignalProperty {
                value: ::core::clone::Clone::clone(&self.value),
                on_updated: ::core::clone::Clone::clone(&self.on_updated),
            }
        }
    }
    impl<T: Send + Sync + Clone + 'static> crate::any::FruityAny for SignalProperty<T> {
        fn get_type_name(&self) -> &'static str {
            "SignalProperty"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    impl<T: Send + Sync + Clone + Default> Default for SignalProperty<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }
    impl<T: Send + Sync + Clone> SignalProperty<T> {
        /// Returns a SignalProperty
        pub fn new(value: T) -> Self {
            Self {
                value,
                on_updated: Signal::new(),
            }
        }
        /// Returns a SignalProperty
        pub fn write(&mut self) -> SignalWriteGuard<T> {
            SignalWriteGuard::<T> {
                target: self,
            }
        }
    }
    impl<T: Send + Sync + Clone> Deref for SignalProperty<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl<T: Send + Sync + Clone + Debug> Debug for SignalProperty<T> {
        fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
            self.value.fmt(formatter)
        }
    }
    impl<T> IntrospectObject for SignalProperty<T>
    where
        T: TryIntoScriptValue + TryFromScriptValue + Send + Sync + Clone + Debug,
    {
        fn get_class_name(&self) -> FruityResult<String> {
            Ok("SignalProperty".to_string())
        }
        fn get_field_names(&self) -> FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        "value".to_string(),
                        "on_updated".to_string(),
                    ]),
                ),
            )
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: ScriptValue,
        ) -> FruityResult<()> {
            match name {
                "value" => self.value = T::from_script_value(value)?,
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            };
            FruityResult::Ok(())
        }
        fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
            match name {
                "value" => self.value.clone().into_script_value(),
                "on_updated" => self.on_updated.clone().into_script_value(),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_const_method(
            &self,
            _name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            _name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    /// A signal subscription handler, can be used to unsubscribe the signal
    pub struct ObserverHandler<T: 'static> {
        observer_id: ObserverIdentifier,
        intern: Arc<RwLock<InternSignal<T>>>,
    }
    impl<T: 'static> crate::any::FruityAny for ObserverHandler<T> {
        fn get_type_name(&self) -> &'static str {
            "ObserverHandler"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    impl<T> ObserverHandler<T> {
        /// Remove an observer from the signal
        pub fn dispose(self) {
            let mut intern = self.intern.write();
            let observer_index = intern
                .observers
                .iter()
                .enumerate()
                .find(|(_index, elem)| elem.0 == self.observer_id)
                .map(|elem| elem.0);
            if let Some(observer_index) = observer_index {
                let _ = intern.observers.remove(observer_index);
            }
        }
        /// Remove an observer from the signal
        pub fn dispose_by_ref(&self) {
            let mut intern = self.intern.write();
            let observer_index = intern
                .observers
                .iter()
                .enumerate()
                .find(|(_index, elem)| elem.0 == self.observer_id)
                .map(|elem| elem.0);
            if let Some(observer_index) = observer_index {
                let _ = intern.observers.remove(observer_index);
            }
        }
    }
    impl<T> IntrospectObject for ObserverHandler<T>
    where
        T: TryFromScriptValue + TryIntoScriptValue,
    {
        fn get_class_name(&self) -> FruityResult<String> {
            Ok("ObserverHandler".to_string())
        }
        fn get_field_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            _name: &str,
            _value: ScriptValue,
        ) -> FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(&self, _name: &str) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new(["dispose".to_string()]),
                ),
            )
        }
        fn call_const_method(
            &self,
            name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            match name {
                "dispose" => self.dispose_by_ref().into_script_value(),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            _name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl<T> Clone for ObserverHandler<T> {
        fn clone(&self) -> Self {
            Self {
                observer_id: self.observer_id.clone(),
                intern: self.intern.clone(),
            }
        }
    }
    impl<T> Debug for ObserverHandler<T> {
        fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
            Ok(())
        }
    }
}
/// Provides a collection for settings
pub mod settings {
    use crate::any::FruityAny;
    use crate::export_function;
    use crate::introspect::IntrospectObject;
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
    #[automatically_derived]
    impl ::core::fmt::Debug for Settings {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Settings::F64(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "F64",
                        &__self_0,
                    )
                }
                Settings::Bool(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bool",
                        &__self_0,
                    )
                }
                Settings::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "String",
                        &__self_0,
                    )
                }
                Settings::Array(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Array",
                        &__self_0,
                    )
                }
                Settings::Object(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Object",
                        &__self_0,
                    )
                }
                Settings::Null => ::core::fmt::Formatter::write_str(f, "Null"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Settings {
        #[inline]
        fn clone(&self) -> Settings {
            match self {
                Settings::F64(__self_0) => {
                    Settings::F64(::core::clone::Clone::clone(__self_0))
                }
                Settings::Bool(__self_0) => {
                    Settings::Bool(::core::clone::Clone::clone(__self_0))
                }
                Settings::String(__self_0) => {
                    Settings::String(::core::clone::Clone::clone(__self_0))
                }
                Settings::Array(__self_0) => {
                    Settings::Array(::core::clone::Clone::clone(__self_0))
                }
                Settings::Object(__self_0) => {
                    Settings::Object(::core::clone::Clone::clone(__self_0))
                }
                Settings::Null => Settings::Null,
            }
        }
    }
    impl crate::any::FruityAny for Settings {
        fn get_type_name(&self) -> &'static str {
            "Settings"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
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
                Settings::Object(fields) => {
                    match fields.get(key) {
                        Some(value) => T::try_from(value.clone()).unwrap_or(default),
                        None => default,
                    }
                }
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
                Settings::Object(fields) => {
                    match fields.get(key) {
                        Some(value) => value.clone(),
                        None => Settings::default(),
                    }
                }
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
    pub fn build_settings_from_yaml(yaml: &Yaml) -> Option<Settings> {
        match yaml {
            Yaml::Real(string) => {
                match string.parse::<f64>() {
                    Ok(value) => Some(Settings::F64(value)),
                    Err(_) => None,
                }
            }
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
    impl TryFrom<Settings> for i8 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as i8),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for i16 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as i16),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for i32 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as i32),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for i64 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as i64),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for isize {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as isize),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for u8 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as u8),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for u16 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as u16),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for u32 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as u32),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for u64 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as u64),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for usize {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as usize),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for f32 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as f32),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for f64 {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::F64(value) => Ok(value as f64),
                _ => {
                    Err(
                        FruityError::NumberExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to "],
                                    &[
                                        ::core::fmt::ArgumentV1::new_debug(&value),
                                        ::core::fmt::ArgumentV1::new_display(&"$type"),
                                    ],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for bool {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::Bool(value) => Ok(value),
                _ => {
                    Err(
                        FruityError::BooleanExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to bool"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryFrom<Settings> for String {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::String(value) => Ok(value),
                _ => {
                    Err(
                        FruityError::StringExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to string"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl<T: TryFrom<Settings> + ?Sized> TryFrom<Settings> for Vec<T> {
        type Error = FruityError;
        fn try_from(value: Settings) -> FruityResult<Self> {
            match value {
                Settings::Array(value) => {
                    Ok(
                        value
                            .into_iter()
                            .filter_map(|elem| T::try_from(elem).ok())
                            .collect(),
                    )
                }
                _ => {
                    Err(
                        FruityError::ArrayExpected({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to array"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl TryIntoScriptValue for Settings {
        fn into_script_value(self) -> FruityResult<ScriptValue> {
            Ok(
                match self {
                    Settings::F64(value) => ScriptValue::F64(value),
                    Settings::Bool(value) => ScriptValue::Bool(value),
                    Settings::String(value) => ScriptValue::String(value.clone()),
                    Settings::Array(value) => {
                        ScriptValue::Array(
                            value
                                .into_iter()
                                .map(|elem| elem.into_script_value())
                                .try_collect::<Vec<_>>()?,
                        )
                    }
                    Settings::Object(value) => {
                        ScriptValue::Object(Box::new(SettingsHashMap(value)))
                    }
                    Settings::Null => ScriptValue::Null,
                },
            )
        }
    }
    impl TryFromScriptValue for Settings {
        fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
            Ok(
                match value {
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
                    ScriptValue::Array(value) => {
                        Settings::Array(
                            value
                                .into_iter()
                                .map(|elem| TryFromScriptValue::from_script_value(elem))
                                .try_collect::<Vec<_>>()?,
                        )
                    }
                    ScriptValue::Null => Settings::Null,
                    ScriptValue::Undefined => Settings::Null,
                    ScriptValue::Iterator(_) => {
                        ::core::panicking::panic("not implemented")
                    }
                    ScriptValue::Callback(_) => {
                        ::core::panicking::panic("not implemented")
                    }
                    ScriptValue::Object(value) => {
                        Settings::Object(
                            value
                                .get_field_names()?
                                .into_iter()
                                .map(|name| {
                                    let field_value = value.get_field_value(&name)?;
                                    TryFromScriptValue::from_script_value(field_value)
                                        .map(|value| (name, value))
                                })
                                .try_collect::<HashMap<_, _>>()?,
                        )
                    }
                },
            )
        }
    }
    struct SettingsHashMap(HashMap<String, Settings>);
    #[automatically_derived]
    impl ::core::fmt::Debug for SettingsHashMap {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "SettingsHashMap",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SettingsHashMap {
        #[inline]
        fn clone(&self) -> SettingsHashMap {
            SettingsHashMap(::core::clone::Clone::clone(&self.0))
        }
    }
    impl crate::any::FruityAny for SettingsHashMap {
        fn get_type_name(&self) -> &'static str {
            "SettingsHashMap"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    impl IntrospectObject for SettingsHashMap {
        fn get_class_name(&self) -> FruityResult<String> {
            Ok("Settings".to_string())
        }
        fn get_field_names(&self) -> FruityResult<Vec<String>> {
            Ok(self.0.keys().map(|key| key.clone()).collect())
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: ScriptValue,
        ) -> FruityResult<()> {
            let value = <Settings>::from_script_value(value)?;
            self.0.entry(name.to_string()).or_insert_with(|| value);
            Ok(())
        }
        fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
            self.0
                .get(name)
                .unwrap_or_else(|| ::core::panicking::panic(
                    "internal error: entered unreachable code",
                ))
                .clone()
                .into_script_value()
        }
        fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_const_method(
            &self,
            _name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            _name: &str,
            _args: Vec<ScriptValue>,
        ) -> FruityResult<ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
}
/// Provides some utils for the game engine
pub mod utils {
    /// Provides utility functions to help the implementation of introspection
    pub mod introspect {
        use crate::script_value::convert::TryFromScriptValue;
        use crate::script_value::ScriptValue;
        use crate::FruityError;
        use crate::FruityResult;
        use std::iter::Enumerate;
        use std::vec::IntoIter as VecIntoIter;
        /// A tool that is used to cast serialized arguments, intended to be used into IntrospectMethod implementations
        pub struct ArgumentCaster {
            args_count: usize,
            iter: Enumerate<VecIntoIter<ScriptValue>>,
            last_index: usize,
        }
        impl ArgumentCaster {
            /// Return an ArgumentCaster
            pub fn new<'a>(args: Vec<ScriptValue>) -> ArgumentCaster {
                ArgumentCaster {
                    args_count: args.len(),
                    iter: args.into_iter().enumerate(),
                    last_index: 1,
                }
            }
            /// Get all the remaining script value arguments from an argument list
            pub fn rest(&mut self) -> Vec<ScriptValue> {
                let mut result = Vec::new();
                while let Some(elem) = self.iter.next() {
                    result.push(elem.1);
                }
                result
            }
            /// Cast a script value argument from an argument list
            ///
            /// # Generic Arguments
            /// * `T` - The type to cast
            ///
            pub fn cast_next<T: TryFromScriptValue + ?Sized>(
                &mut self,
            ) -> FruityResult<T> {
                match self.iter.next() {
                    Some((index, arg)) => {
                        self.last_index = index + 1;
                        T::from_script_value(arg)
                    }
                    None => {
                        T::from_script_value(ScriptValue::Undefined)
                            .map_err(|_| {
                                FruityError::InvalidArg({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &[
                                                "Wrong number of arguments, you provided ",
                                                " and we expect ",
                                            ],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&self.last_index),
                                                ::core::fmt::ArgumentV1::new_display(&(self.args_count + 1)),
                                            ],
                                        ),
                                    );
                                    res
                                })
                            })
                    }
                }
            }
        }
    }
    /// Utility functions related with [’String’]
    pub mod string {
        use std::path::Path;
        /// Extract the file type from a file path
        ///
        /// # Arguments
        /// * `file_path` - The file path
        ///
        pub fn get_file_type_from_path(file_path: &str) -> Option<String> {
            let path = Path::new(file_path);
            Some(path.extension()?.to_str()?.to_string())
        }
    }
    /// Utility functions related with collections
    pub mod collection {
        use std::collections::HashMap;
        use std::hash::Hash;
        /// Insert an element in an hashmap that contains a vec
        ///
        /// # Arguments
        /// * `hashmap` - The hashmap
        /// * `key` - The key of the value that is added
        /// * `value` - The value that will be inserted
        ///
        pub fn insert_in_hashmap_vec<K: Eq + Hash, T>(
            hashmap: &mut HashMap<K, Vec<T>>,
            key: K,
            value: T,
        ) {
            if let Some(vec) = hashmap.get_mut(&key) {
                vec.push(value);
            } else {
                hashmap
                    .insert(
                        key,
                        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([value])),
                    );
            }
        }
        /// A temporary stable implementation for drain_filter
        pub fn drain_filter<T, Pred: Fn(&T) -> bool>(
            source: &mut Vec<T>,
            pred: Pred,
        ) -> Vec<T> {
            let mut drained: Vec<T> = Vec::new();
            while let Some((index, _))
                = source.iter().enumerate().find(|elem| pred(elem.1))
            {
                drained.push(source.remove(index));
            }
            drained
        }
    }
    /// Utility functions related with numbers
    pub mod math {
        use std::f32::consts::PI;
        use std::ops::Range;
        /// Take a radian angle and normalize it between [-PI, PI[
        ///
        /// # Arguments
        /// * `angle` - The input angle
        ///
        pub fn normalize_angle(angle: f32) -> f32 {
            if angle < -PI {
                normalize_angle(angle + 2.0 * PI)
            } else if angle >= PI {
                normalize_angle(angle - 2.0 * PI)
            } else {
                angle
            }
        }
        /// Take a radian angle range and normalize each born between [-PI, PI[
        /// If the range length is 2PI, returns simply -PI..PI
        ///
        /// # Arguments
        /// * `range` - The input range
        ///
        pub fn normalize_angle_range(range: Range<f32>) -> Range<f32> {
            if range.start == range.end {
                return 0.0..0.0;
            }
            let angle1 = normalize_angle(range.start);
            let angle2 = normalize_angle(range.end);
            let start = f32::min(angle1, angle2);
            let end = f32::max(angle1, angle2);
            if start == end { -PI..PI } else { start..end }
        }
    }
}
/// Provides a main object for the game engine
pub mod world {
    use crate::any::FruityAny;
    use crate::export;
    use crate::frame_service::FrameService;
    use crate::module::Module;
    use crate::object_factory_service::ObjectFactoryService;
    use crate::resource::script_resource_container::ScriptResourceContainer;
    use crate::settings::Settings;
    use crate::FruityResult;
    use crate::ModulesService;
    use crate::ResourceContainer;
    use fruity_game_engine_macro::fruity_export;
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::ops::Deref;
    use std::rc::Rc;
    /// A middleware that occurs when entering into the loop
    pub type StartMiddleware = Rc<dyn Fn(&World) -> FruityResult<()>>;
    /// A middleware that occurs when rendering the loop
    pub type FrameMiddleware = Rc<dyn Fn(&World) -> FruityResult<()>>;
    /// A middleware that occurs when leaving the loop
    pub type EndMiddleware = Rc<dyn Fn(&World) -> FruityResult<()>>;
    /// A middleware that occurs when the world runs
    pub type RunMiddleware = Rc<
        dyn Fn(
            &World,
            &(dyn Fn(&World) -> FruityResult<()>),
            &(dyn Fn(&World) -> FruityResult<()>),
            &(dyn Fn(&World) -> FruityResult<()>),
        ) -> FruityResult<()>,
    >;
    struct InnerWorld {
        resource_container: ResourceContainer,
        settings: Settings,
        start_middleware: StartMiddleware,
        frame_middleware: FrameMiddleware,
        end_middleware: EndMiddleware,
        run_middleware: RunMiddleware,
    }
    /// The main container of the ECS
    pub struct World {
        inner: Rc<RefCell<InnerWorld>>,
        module_service: Rc<RefCell<ModulesService>>,
        script_resource_container: ScriptResourceContainer,
    }
    impl crate::any::FruityAny for World {
        fn get_type_name(&self) -> &'static str {
            "World"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for World {
        #[inline]
        fn clone(&self) -> World {
            World {
                inner: ::core::clone::Clone::clone(&self.inner),
                module_service: ::core::clone::Clone::clone(&self.module_service),
                script_resource_container: ::core::clone::Clone::clone(
                    &self.script_resource_container,
                ),
            }
        }
    }
    impl World {
        /// Returns a World
        pub fn new(settings: Settings) -> Self {
            let resource_container = ResourceContainer::new();
            Self::initialize(resource_container.clone(), &settings);
            let module_service = ModulesService::new(resource_container.clone());
            World {
                inner: Rc::new(
                    RefCell::new(InnerWorld {
                        resource_container: resource_container.clone(),
                        settings,
                        start_middleware: Rc::new(|_| Ok(())),
                        frame_middleware: Rc::new(|_| Ok(())),
                        end_middleware: Rc::new(|_| Ok(())),
                        run_middleware: Rc::new(|world, start, frame, end| {
                            start(world)?;
                            frame(world)?;
                            end(world)?;
                            FruityResult::Ok(())
                        }),
                    }),
                ),
                module_service: Rc::new(RefCell::new(module_service)),
                script_resource_container: ScriptResourceContainer::new(
                    resource_container,
                ),
            }
        }
        /// Initialize the world
        pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
            let frame_service = FrameService::new(resource_container.clone());
            resource_container
                .add::<FrameService>("frame_service", Box::new(frame_service));
            let object_factory_service = ObjectFactoryService::new(
                resource_container.clone(),
            );
            resource_container
                .add::<
                    ObjectFactoryService,
                >("object_factory_service", Box::new(object_factory_service));
        }
        /// Register a module
        pub fn register_module(&self, module: Module) -> FruityResult<()> {
            self.module_service.deref().borrow_mut().register_module(module);
            Ok(())
        }
        /// Load the modules
        pub fn setup_modules(&self) -> FruityResult<()> {
            let settings = self.inner.deref().borrow().settings.clone();
            let module_service = self.module_service.deref().borrow();
            module_service
                .traverse_modules_by_dependencies(
                    &Box::new(|module: Module| {
                        if let Some(setup) = module.setup {
                            setup(self.clone(), settings.clone())?;
                        }
                        Ok(())
                    }),
                )
        }
        /// Load the resources
        pub fn load_resources(&self) -> FruityResult<()> {
            let settings = self.inner.deref().borrow().settings.clone();
            let module_service = self.module_service.deref().borrow();
            module_service
                .traverse_modules_by_dependencies(
                    &Box::new(|module: Module| {
                        if let Some(load_resources) = module.load_resources {
                            load_resources(self.clone(), settings.clone())?;
                        }
                        Ok(())
                    }),
                )
        }
        /// Run the world
        pub fn run(&self) -> FruityResult<()> {
            let _profiler_scope = if ::puffin::are_scopes_on() {
                Some(
                    ::puffin::ProfilerScope::new(
                        {
                            fn f() {}
                            let name = ::puffin::type_name_of(f);
                            let name = &name.get(..name.len() - 3).unwrap();
                            ::puffin::clean_function_name(name)
                        },
                        ::puffin::short_file_name(
                            "crates/fruity_game_engine/src/world.rs",
                        ),
                        "",
                    ),
                )
            } else {
                None
            };
            let run_middleware = self.inner.deref().borrow().run_middleware.clone();
            let start_middleware = self.inner.deref().borrow().start_middleware.clone();
            let frame_middleware = self.inner.deref().borrow().frame_middleware.clone();
            let end_middleware = self.inner.deref().borrow().end_middleware.clone();
            run_middleware(
                self,
                start_middleware.deref(),
                frame_middleware.deref(),
                end_middleware.deref(),
            )
        }
        /// Add a run start middleware
        pub fn add_run_start_middleware(
            &self,
            middleware: impl Fn(StartMiddleware, &World) -> FruityResult<()> + 'static,
        ) {
            let mut this = self.inner.deref().borrow_mut();
            let next_middleware = this.start_middleware.clone();
            this
                .start_middleware = Rc::new(move |world| {
                middleware(next_middleware.clone(), world)
            });
        }
        /// Add a run frame middleware
        pub fn add_run_frame_middleware(
            &self,
            middleware: impl Fn(StartMiddleware, &World) -> FruityResult<()> + 'static,
        ) {
            let mut this = self.inner.deref().borrow_mut();
            let next_middleware = this.frame_middleware.clone();
            this
                .frame_middleware = Rc::new(move |world| {
                middleware(next_middleware.clone(), world)
            });
        }
        /// Add a run end middleware
        pub fn add_run_end_middleware(
            &self,
            middleware: impl Fn(StartMiddleware, &World) -> FruityResult<()> + 'static,
        ) {
            let mut this = self.inner.deref().borrow_mut();
            let next_middleware = this.end_middleware.clone();
            this
                .end_middleware = Rc::new(move |world| {
                middleware(next_middleware.clone(), world)
            });
        }
        /// Get resource container
        pub fn get_resource_container(&self) -> ResourceContainer {
            let this = self.inner.deref().borrow();
            this.resource_container.clone()
        }
        /// Get resource container
        pub fn get_script_resource_container(&self) -> ScriptResourceContainer {
            self.script_resource_container.clone()
        }
    }
    impl crate::introspect::IntrospectObject for World {
        fn get_class_name(&self) -> crate::FruityResult<String> {
            Ok("unknown".to_string())
        }
        fn get_field_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: crate::script_value::ScriptValue,
        ) -> crate::FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(
            &self,
            name: &str,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_const_method_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        "register_module".to_string(),
                        "setup_modules".to_string(),
                        "load_resources".to_string(),
                        "run".to_string(),
                        "get_resource_container".to_string(),
                    ]),
                ),
            )
        }
        fn call_const_method(
            &self,
            name: &str,
            __args: Vec<crate::script_value::ScriptValue>,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            use crate::script_value::convert::TryIntoScriptValue;
            match name {
                "register_module" => {
                    let mut __caster = crate::utils::introspect::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<Module>()?;
                    self.register_module(__arg_0).into_script_value()
                }
                "setup_modules" => self.setup_modules().into_script_value(),
                "load_resources" => self.load_resources().into_script_value(),
                "run" => self.run().into_script_value(),
                "get_resource_container" => {
                    self.get_script_resource_container().into_script_value()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_mut_method_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            __args: Vec<crate::script_value::ScriptValue>,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl Debug for World {
        fn fmt(
            &self,
            formatter: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            let this = self.inner.deref().borrow();
            this.resource_container.fmt(formatter)
        }
    }
}
/// A service for frame management
pub mod frame_service {
    use crate::any::FruityAny;
    use crate::fruity_export;
    use crate::resource::resource_container::ResourceContainer;
    use crate::resource::Resource;
    pub use fruity_game_engine_macro::export;
    use std::fmt::Debug;
    use std::time::Instant;
    /// A service for frame management
    pub struct FrameService {
        last_frame_instant: Instant,
        delta: f32,
    }
    impl crate::any::FruityAny for FrameService {
        fn get_type_name(&self) -> &'static str {
            "FrameService"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(&mut self) -> &mut dyn crate::any::FruityAny {
            self
        }
        fn as_fruity_any_box(self: Box<Self>) -> Box<dyn crate::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn crate::any::FruityAny> {
            self
        }
    }
    impl crate::resource::Resource for FrameService {
        fn as_resource_box(self: Box<Self>) -> Box<dyn crate::resource::Resource> {
            self
        }
        fn as_any_arc(
            self: std::sync::Arc<Self>,
        ) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
            self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FrameService {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "FrameService",
                "last_frame_instant",
                &&self.last_frame_instant,
                "delta",
                &&self.delta,
            )
        }
    }
    impl FrameService {
        /// Returns a FrameService
        pub fn new(_resource_container: ResourceContainer) -> FrameService {
            FrameService {
                delta: 0.0,
                last_frame_instant: Instant::now(),
            }
        }
        /// A function that needs to be called on new frame
        /// Intended to be used in the render pipeline
        pub fn begin_frame(&mut self) {
            let now = Instant::now();
            let delta = now.duration_since(self.last_frame_instant);
            self.delta = delta.as_secs_f32();
            self.last_frame_instant = now;
        }
        /// Get the time before the previous frame
        pub fn get_delta(&self) -> f32 {
            self.delta
        }
    }
    impl crate::introspect::IntrospectObject for FrameService {
        fn get_class_name(&self) -> crate::FruityResult<String> {
            Ok("unknown".to_string())
        }
        fn get_field_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: crate::script_value::ScriptValue,
        ) -> crate::FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(
            &self,
            name: &str,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_const_method_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new(["get_delta".to_string()]),
                ),
            )
        }
        fn call_const_method(
            &self,
            name: &str,
            __args: Vec<crate::script_value::ScriptValue>,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            use crate::script_value::convert::TryIntoScriptValue;
            match name {
                "get_delta" => self.get_delta().into_script_value(),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_mut_method_names(&self) -> crate::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            __args: Vec<crate::script_value::ScriptValue>,
        ) -> crate::FruityResult<crate::script_value::ScriptValue> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
}
