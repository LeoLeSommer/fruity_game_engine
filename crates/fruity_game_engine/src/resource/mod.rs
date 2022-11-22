use crate::{any::FruityAny, introspect::IntrospectObject, RwLock};
use std::{any::Any, fmt::Debug, sync::Arc};

pub use fruity_game_engine_macro::Resource;

/// A reference over a resource that is supposed to be used by components
pub mod resource_reference;

/// The resource manager
pub mod resource_container;

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
