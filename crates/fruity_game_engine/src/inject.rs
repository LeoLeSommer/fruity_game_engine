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
    /// Get the object
    fn from_resource_container(resource_container: &ResourceContainer) -> Self;
}

impl Injectable for ResourceContainer {
    fn from_resource_container(resource_container: &ResourceContainer) -> Self {
        resource_container.clone()
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Injectable for Ref<T> {
    fn from_resource_container(resource_container: &ResourceContainer) -> Self {
        resource_container.require::<T>()
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Injectable for Const<T> {
    fn from_resource_container(resource_container: &ResourceContainer) -> Self {
        let reference = Ref::<T>::from_resource_container(resource_container);
        reference.read()
    }
}

impl<T: IntrospectFields + IntrospectMethods + Send + Sync + ?Sized> Injectable for Mut<T> {
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

impl<R: 'static> Inject<R> for &'static (dyn Fn() -> R + Send + Sync) {
    fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
        Box::new(move |_| self())
    }
}

impl<T1: Injectable, R: 'static> Inject<R> for &'static (dyn Fn(T1) -> R + Send + Sync) {
    fn inject(self) -> Box<dyn Fn(ResourceContainer) -> R + Send + Sync> {
        Box::new(move |resource_container| self(T1::from_resource_container(&resource_container)))
    }
}

impl<T1: Injectable, T2: Injectable, R: 'static> Inject<R>
    for &'static (dyn Fn(T1, T2) -> R + Send + Sync)
{
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
    for &'static (dyn Fn(T1, T2, T3) -> R + Send + Sync)
{
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

impl<T1: Injectable, T2: Injectable, T3: Injectable, T4: Injectable, R: 'static> Inject<R>
    for &'static (dyn Fn(T1, T2, T3, T4) -> R + Send + Sync)
{
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
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5) -> R + Send + Sync)
{
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
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5, T6) -> R + Send + Sync)
{
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
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7) -> R + Send + Sync)
{
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
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8) -> R + Send + Sync)
{
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
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R + Send + Sync)
{
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
    > Inject<R> for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R + Send + Sync)
{
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
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R + Send + Sync)
{
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
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R + Send + Sync)
{
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
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R + Send + Sync)
{
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
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R
                  + Sync
                  + Send)
{
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
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R
                  + Sync
                  + Send)
{
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
    for &'static (dyn Fn(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R
                  + Sync
                  + Send)
{
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
    ) -> R
                  + Sync
                  + Send)
{
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
    ) -> R
                  + Sync
                  + Send)
{
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
    ) -> R
                  + Sync
                  + Send)
{
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
    ) -> R
                  + Sync
                  + Send)
{
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
