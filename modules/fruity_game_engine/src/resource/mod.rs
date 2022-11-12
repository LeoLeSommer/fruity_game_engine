use crate::any::FruityAny;
use napi::bindgen_prelude::ToNapiValue;
use std::fmt::Debug;

pub use fruity_game_engine_derive::*;

/// Errors related with ResourceContainer
pub mod error;

/// A reference over a resource that is supposed to be used by components
pub mod resource_reference;

/// The resource manager
pub mod resource_container;

/// A trait that should be implemented by every resources
pub trait Resource: FruityAny + Debug {
  fn as_resource_box(self: Box<Self>) -> Box<dyn Resource>;
}
