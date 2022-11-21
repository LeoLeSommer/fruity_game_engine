#![warn(missing_docs)]

//! Any
//!
//! An extended Any trait
//!
//! The difference with the classic Any is that this Any needs to implement converter

use parking_lot::Mutex;
use parking_lot::RwLock;
use std::any::Any;
use std::sync::Arc;

pub use fruity_game_engine_macro::FruityAny;

/// The any trait
pub trait FruityAny: Any + Send + Sync {
  /// Return self as an Any ref
  fn as_any_ref(&self) -> &dyn Any;

  /// Return self as an Any mutable ref
  fn as_any_mut(&mut self) -> &mut dyn Any;

  /// Return self as an Any box
  fn as_any_box(self: Box<Self>) -> Box<dyn Any>;

  /// Return self as an Any arc
  fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: FruityAny + ?Sized> FruityAny for Box<T> {
  fn as_any_ref(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
    self
  }
}

impl<T: FruityAny + ?Sized> FruityAny for Arc<T> {
  fn as_any_ref(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
    self
  }
}

impl<T: FruityAny> FruityAny for Mutex<T> {
  fn as_any_ref(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
    self
  }
}

impl<T: FruityAny> FruityAny for RwLock<T> {
  fn as_any_ref(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
    self
  }
}

impl<T: FruityAny> FruityAny for Option<T> {
  fn as_any_ref(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
    self
  }
}

impl<T: FruityAny> FruityAny for Vec<T> {
  fn as_any_ref(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
    self
  }
}
