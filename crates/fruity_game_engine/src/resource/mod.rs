use crate::{
    introspect::{IntrospectFields, IntrospectMethods},
    sync::Arc,
};
use std::any::Any;

/// A reference over a resource that is supposed to be used by components
mod resource_reference;
pub use resource_reference::*;

/// The resource manager
mod resource_container;
pub use resource_container::*;

/// A resource that can be stored in the resource container
pub trait Resource: IntrospectFields + IntrospectMethods + Send + Sync + 'static {
    /// Convert the resource into an Arc<dyn Any + Send + Sync>
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + 'static> Resource for T {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}
