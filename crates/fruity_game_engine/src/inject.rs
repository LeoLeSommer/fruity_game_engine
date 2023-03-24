use crate::introspect::IntrospectFields;
use crate::introspect::IntrospectMethods;
use crate::resource::resource_container::ResourceContainer;
use crate::resource::resource_reference::ResourceReadGuard;
use crate::resource::resource_reference::ResourceReference;
use crate::resource::resource_reference::ResourceWriteGuard;

/// A reference over a resource
pub type Ref<T> = ResourceReference<T>;

/// A read guard over a resource
pub type Const<T> = ResourceReadGuard<T>;

/// A write guard over a resource
pub type Mut<T> = ResourceWriteGuard<T>;

/// A trait for a function that needs injection from resource container
/// A simple implementation of the dependency injection pattern
pub trait Injectable: 'static {
    /// The type stored in the closure
    type StoredType: Send + Sync;

    /// Get the object, is only executed once at the creation of the closure
    fn from_resource_container(resource_container: &ResourceContainer) -> Self::StoredType;

    /// Finalize the conversion, is executed at each execution of the function
    fn finalize(stored: &Self::StoredType) -> Self;
}

impl Injectable for ResourceContainer {
    type StoredType = ResourceContainer;

    fn from_resource_container(resource_container: &ResourceContainer) -> Self::StoredType {
        resource_container.clone()
    }

    fn finalize(stored: &Self::StoredType) -> Self {
        stored.clone()
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Injectable for Ref<T> {
    type StoredType = ResourceReference<T>;

    fn from_resource_container(resource_container: &ResourceContainer) -> Self::StoredType {
        resource_container.require::<T>()
    }

    fn finalize(stored: &Self::StoredType) -> Self {
        stored.clone()
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Injectable for Const<T> {
    type StoredType = ResourceReference<T>;

    fn from_resource_container(resource_container: &ResourceContainer) -> Self::StoredType {
        resource_container.require::<T>()
    }

    fn finalize(stored: &Self::StoredType) -> Self {
        stored.read()
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Injectable for Mut<T> {
    type StoredType = ResourceReference<T>;

    fn from_resource_container(resource_container: &ResourceContainer) -> Self::StoredType {
        resource_container.require::<T>()
    }

    fn finalize(stored: &Self::StoredType) -> Self {
        stored.write()
    }
}

/// A trait that is implemented by functions that supports dependency injection
pub trait Inject<R> {
    /// Get a function that proceed the injection
    fn inject(self, resource_container: &ResourceContainer) -> Box<dyn Fn() -> R + Send + Sync>;
}

impl<R: 'static> Inject<R> for &'static (dyn Fn() -> R + Send + Sync) {
    fn inject(self, _resource_container: &ResourceContainer) -> Box<dyn Fn() -> R + Send + Sync> {
        Box::new(move || self())
    }
}

macro_rules! impl_inject {
    ($($tn:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($tn: Injectable),*, R: 'static> Inject<R> for &'static (dyn Fn($($tn),*) -> R + Send + Sync) {
            fn inject(
                self,
                resource_container: &ResourceContainer,
            ) -> Box<dyn Fn() -> R + Send + Sync> {
                $(let $tn = $tn::from_resource_container(&resource_container);)*

                Box::new(move || self($($tn::finalize(&$tn)),*))
            }
        }
    };
}

impl_inject!(T1);
impl_inject!(T1, T2);
impl_inject!(T1, T2, T3);
impl_inject!(T1, T2, T3, T4);
impl_inject!(T1, T2, T3, T4, T5);
impl_inject!(T1, T2, T3, T4, T5, T6);
impl_inject!(T1, T2, T3, T4, T5, T6, T7);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
impl_inject!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
impl_inject!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
