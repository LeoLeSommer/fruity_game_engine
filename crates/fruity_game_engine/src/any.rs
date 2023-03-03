#![warn(missing_docs)]

//! Any
//!
//! An extended Any trait
//!
//! The difference with the classic Any is that this Any needs to implement converter

use crate::Mutex;
use crate::RwLock;
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
