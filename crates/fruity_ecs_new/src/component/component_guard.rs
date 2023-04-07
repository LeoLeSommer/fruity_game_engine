use super::{Component, ComponentStorage};
use fruity_game_engine::{FruityError, RwLockReadGuard, RwLockWriteGuard};
use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentReference`].
///
/// [`read`]: ComponentReference::read
///
pub struct AnyComponentReadGuard<'a> {
    pub(crate) storage_guard: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    pub(crate) component_ptr: NonNull<dyn Component>,
}

impl<'a> Debug for AnyComponentReadGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Deref for AnyComponentReadGuard<'a> {
    type Target = dyn Component;

    fn deref(&self) -> &Self::Target {
        unsafe { self.component_ptr.as_ref() }
    }
}

impl<'a, T: Component> TryInto<ComponentReadGuard<'a, T>> for AnyComponentReadGuard<'a> {
    type Error = FruityError;

    fn try_into(self) -> Result<ComponentReadGuard<'a, T>, Self::Error> {
        match unsafe { self.component_ptr.as_ref() }
            .as_any_ref()
            .downcast_ref::<T>()
        {
            Some(result) => Ok(ComponentReadGuard {
                storage_guard: self.storage_guard,
                component_ptr: NonNull::<T>::from(result),
            }),
            None => Err(FruityError::GenericFailure(format!(
                "Couldn't convert {:?} to typed component",
                self
            ))),
        }
    }
}

/// RAII structure used to release the shared write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentReference`].
///
/// [`write`]: ComponentReference::write
///
pub struct AnyComponentWriteGuard<'a> {
    pub(crate) storage_guard: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    pub(crate) component_ptr: NonNull<dyn Component>,
}

impl<'a> Debug for AnyComponentWriteGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Deref for AnyComponentWriteGuard<'a> {
    type Target = dyn Component;

    fn deref(&self) -> &Self::Target {
        unsafe { self.component_ptr.as_ref() }
    }
}

impl<'a> DerefMut for AnyComponentWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.component_ptr.as_mut() }
    }
}

impl<'a, T: Component> TryInto<ComponentWriteGuard<'a, T>> for AnyComponentWriteGuard<'a> {
    type Error = FruityError;

    fn try_into(self) -> Result<ComponentWriteGuard<'a, T>, Self::Error> {
        match unsafe { self.component_ptr.as_ref() }
            .as_any_ref()
            .downcast_ref::<T>()
        {
            Some(result) => Ok(ComponentWriteGuard {
                storage_guard: self.storage_guard,
                component_ptr: NonNull::<T>::from(result),
            }),
            None => Err(FruityError::GenericFailure(format!(
                "Couldn't convert {:?} to typed component",
                self
            ))),
        }
    }
}

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentReference`].
///
/// [`read`]: ComponentReference::read
///
pub struct ComponentReadGuard<'a, T: Component> {
    pub(crate) storage_guard: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    pub(crate) component_ptr: NonNull<T>,
}

impl<'a, T: Component> Debug for ComponentReadGuard<'a, T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.deref().fmt(formatter)
    }
}

impl<'a, T: Component> Deref for ComponentReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.component_ptr.as_ref() }
    }
}

impl<'a, T: Component> TryInto<AnyComponentReadGuard<'a>> for ComponentReadGuard<'a, T> {
    type Error = FruityError;

    fn try_into(self) -> Result<AnyComponentReadGuard<'a>, Self::Error> {
        match unsafe { self.component_ptr.as_ref() }
            .as_any_ref()
            .downcast_ref::<T>()
        {
            Some(result) => Ok(AnyComponentReadGuard {
                storage_guard: self.storage_guard,
                component_ptr: NonNull::<T>::from(result),
            }),
            None => Err(FruityError::GenericFailure(format!(
                "Couldn't convert {:?} to typed component",
                self
            ))),
        }
    }
}

/// RAII structure used to release the shared write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentReference`].
///
/// [`write`]: ComponentReference::write
///
pub struct ComponentWriteGuard<'a, T: Component> {
    pub(crate) storage_guard: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    pub(crate) component_ptr: NonNull<T>,
}

impl<'a, T: Component> Debug for ComponentWriteGuard<'a, T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.deref().fmt(formatter)
    }
}

impl<'a, T: Component> Deref for ComponentWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.component_ptr.as_ref() }
    }
}

impl<'a, T: Component> DerefMut for ComponentWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.component_ptr.as_mut() }
    }
}

impl<'a, T: Component> TryInto<AnyComponentWriteGuard<'a>> for ComponentWriteGuard<'a, T> {
    type Error = FruityError;

    fn try_into(self) -> Result<AnyComponentWriteGuard<'a>, Self::Error> {
        match unsafe { self.component_ptr.as_ref() }
            .as_any_ref()
            .downcast_ref::<T>()
        {
            Some(result) => Ok(AnyComponentWriteGuard {
                storage_guard: self.storage_guard,
                component_ptr: NonNull::<T>::from(result),
            }),
            None => Err(FruityError::GenericFailure(format!(
                "Couldn't convert {:?} to typed component",
                self
            ))),
        }
    }
}

/// An iterator over all components of a specific type.
pub struct ComponentReadGuardIterator<'a, T: Component + 'static> {
    _storage_guard: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    current: NonNull<T>,
    end: NonNull<T>,
}

impl<'a, T: Component + 'static> ComponentReadGuardIterator<'a, T> {
    pub(crate) fn new(
        storage_guard: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
        begin: NonNull<T>,
        count: usize,
    ) -> Self {
        Self {
            _storage_guard: storage_guard,
            current: begin,
            end: unsafe { NonNull::new_unchecked(begin.as_ptr().add(count)) },
        }
    }
}

impl<'a, T: Component + 'static> Iterator for ComponentReadGuardIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = unsafe { self.current.as_ref() };
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

/// A mut iterator over all components of a specific type.
pub struct ComponentWriteGuardIterator<'a, T: Component + 'static> {
    _storage_guard: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    current: NonNull<T>,
    end: NonNull<T>,
}

impl<'a, T: Component + 'static> ComponentWriteGuardIterator<'a, T> {
    pub(crate) fn new(
        storage_guard: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
        begin: NonNull<T>,
        count: usize,
    ) -> Self {
        Self {
            _storage_guard: storage_guard,
            current: begin,
            end: unsafe { NonNull::new_unchecked(begin.as_ptr().add(count)) },
        }
    }
}

impl<'a, T: Component + 'static> Iterator for ComponentWriteGuardIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = unsafe { self.current.as_mut() };
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

/// An iterator over components of an any type.
pub struct AnyComponentReadGuardIterator<'a> {
    _storage_guard: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    component_type_size: usize,
    current: NonNull<dyn Component>,
    end: NonNull<dyn Component>,
}

impl<'a> AnyComponentReadGuardIterator<'a> {
    pub(crate) fn new(
        storage_guard: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
        component_type_size: usize,
        begin: NonNull<dyn Component>,
        count: usize,
    ) -> Self {
        Self {
            _storage_guard: storage_guard,
            component_type_size,
            current: begin,
            end: unsafe {
                NonNull::new_unchecked(begin.as_ptr().byte_add(count * component_type_size))
            },
        }
    }
}

impl<'a> Iterator for AnyComponentReadGuardIterator<'a> {
    type Item = &'a dyn Component;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = unsafe { self.current.as_ref() };
            self.current = unsafe {
                NonNull::new_unchecked(self.current.as_ptr().byte_add(self.component_type_size))
            };

            Some(result)
        } else {
            None
        }
    }
}

/// A mut iterator over components of an any type.
pub struct AnyComponentWriteGuardIterator<'a> {
    _storage_guard: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    component_type_size: usize,
    current: NonNull<dyn Component>,
    end: NonNull<dyn Component>,
}

impl<'a> AnyComponentWriteGuardIterator<'a> {
    pub(crate) fn new(
        storage_guard: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
        component_type_size: usize,
        begin: NonNull<dyn Component>,
        count: usize,
    ) -> Self {
        Self {
            _storage_guard: storage_guard,
            component_type_size,
            current: begin,
            end: unsafe {
                NonNull::new_unchecked(begin.as_ptr().byte_add(count * component_type_size))
            },
        }
    }
}

impl<'a> Iterator for AnyComponentWriteGuardIterator<'a> {
    type Item = &'a mut dyn Component;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = unsafe { self.current.as_mut() };
            self.current = unsafe {
                NonNull::new_unchecked(self.current.as_ptr().byte_add(self.component_type_size))
            };

            Some(result)
        } else {
            None
        }
    }
}
