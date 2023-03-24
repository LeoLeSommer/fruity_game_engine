use crate::component::Component;
use fruity_game_engine::FruityError;
use fruity_game_engine::RwLockReadGuard;
use fruity_game_engine::RwLockWriteGuard;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentReference`].
///
/// [`read`]: ComponentReference::read
///
pub struct AnyComponentReadGuard<'a> {
    pub(crate) entity_guard: RwLockReadGuard<'a, ()>,
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
                entity_guard: self.entity_guard,
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
    pub(crate) entity_guard: RwLockWriteGuard<'a, ()>,
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
                entity_guard: self.entity_guard,
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
    pub(crate) entity_guard: RwLockReadGuard<'a, ()>,
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
                entity_guard: self.entity_guard,
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
    pub(crate) entity_guard: RwLockWriteGuard<'a, ()>,
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
                entity_guard: self.entity_guard,
                component_ptr: NonNull::<T>::from(result),
            }),
            None => Err(FruityError::GenericFailure(format!(
                "Couldn't convert {:?} to typed component",
                self
            ))),
        }
    }
}
