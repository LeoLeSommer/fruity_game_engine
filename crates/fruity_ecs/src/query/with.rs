use super::{EntityIterator, QueryParam, SingleEntityIterator};
use crate::{
    component::{Component, ComponentStorage, ComponentTypeId, Enabled, Name, VecComponentStorage},
    entity::{
        Archetype, ArchetypeComponentTypes, EntityId, EntityLocation, EntityReader,
        EntityReference, EntityWriter, InnerShareableEntityReference,
    },
};
use fruity_game_engine::{RwLockReadGuard, RwLockWriteGuard};
use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

/// The entity
pub struct WithEntity;

/// An iterator over entities
pub struct WithEntityIterator<'a> {
    current_entity_index: usize,
    end_entity_index: usize,
    archetype: &'a Archetype,
}

impl<'a> Iterator for WithEntityIterator<'a> {
    type Item = EntityReader<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entity_index < self.end_entity_index {
            let result = self.current();
            self.current_entity_index += 1;

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> EntityIterator for WithEntityIterator<'a> {
    fn current(&mut self) -> Self::Item {
        let entity_index = self.current_entity_index;
        EntityReader {
            entity_index,
            archetype: NonNull::from(self.archetype),
            phantom: Default::default(),
        }
    }

    fn has_reach_entity_end(&self) -> bool {
        true
    }

    fn reset_current_entity(&mut self) {
        self.current_entity_index -= 1;
    }
}

impl<'a> QueryParam<'a> for WithEntity {
    type Item = EntityReader<'a>;
    type Iterator = WithEntityIterator<'a>;
    type FromEntityReferenceIterator = SingleEntityIterator<Self::Item>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        WithEntityIterator {
            current_entity_index: 0,
            end_entity_index: archetype.len(),
            archetype,
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            SingleEntityIterator::new(EntityReader {
                entity_index: location.index,
                archetype: NonNull::from(&entity_storage.read().archetypes[location.archetype.0]),
                phantom: Default::default(),
            })
        } else {
            unreachable!()
        }
    }
}

/// The entity with mutability
pub struct WithEntityMut;

/// An iterator over entities with mutability
pub struct WithEntityMutIterator<'a> {
    current_entity_index: usize,
    end_entity_index: usize,
    archetype: &'a Archetype,
}

impl<'a> Iterator for WithEntityMutIterator<'a> {
    type Item = EntityWriter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entity_index < self.end_entity_index {
            let result = self.current();
            self.current_entity_index += 1;

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> EntityIterator for WithEntityMutIterator<'a> {
    fn current(&mut self) -> Self::Item {
        let entity_index = self.current_entity_index;
        EntityWriter {
            entity_index,
            archetype: NonNull::from(self.archetype),
            phantom: PhantomData,
        }
    }

    fn has_reach_entity_end(&self) -> bool {
        true
    }

    fn reset_current_entity(&mut self) {
        self.current_entity_index -= 1;
    }
}

impl<'a> QueryParam<'a> for WithEntityMut {
    type Item = EntityWriter<'a>;
    type Iterator = WithEntityMutIterator<'a>;
    type FromEntityReferenceIterator = SingleEntityIterator<Self::Item>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        WithEntityMutIterator {
            current_entity_index: 0,
            end_entity_index: archetype.len(),
            archetype,
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            SingleEntityIterator::new(EntityWriter {
                entity_index: location.index,
                archetype: NonNull::from(&entity_storage.read().archetypes[location.archetype.0]),
                phantom: PhantomData,
            })
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity ids
pub struct WithIdIterator<'a> {
    current: NonNull<EntityId>,
    end: NonNull<EntityId>,
    _marker: PhantomData<&'a EntityId>,
}

impl<'a> Iterator for WithIdIterator<'a> {
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> EntityIterator for WithIdIterator<'a> {
    fn current(&mut self) -> Self::Item {
        *unsafe { self.current.as_ref() }
    }

    fn has_reach_entity_end(&self) -> bool {
        true
    }

    fn reset_current_entity(&mut self) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(1)) };
    }
}

/// The entity id
pub struct WithId;

impl<'a> QueryParam<'a> for WithId {
    type Item = EntityId;
    type Iterator = WithIdIterator<'a>;
    type FromEntityReferenceIterator = SingleEntityIterator<Self::Item>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let begin = &archetype.entity_ids[0] as *const EntityId as *mut EntityId;
        WithIdIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            SingleEntityIterator::new(archetype.entity_ids[location.index])
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity names
pub struct WithNameIterator<'a, T: Iterator<Item = &'a Name> + EntityIterator> {
    with_iterator: T,
}

impl<'a, T: Iterator<Item = &'a Name> + EntityIterator> Iterator for WithNameIterator<'a, T> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator.next().map(|name| &name.0)
    }
}

impl<'a, T: Iterator<Item = &'a Name> + EntityIterator> EntityIterator for WithNameIterator<'a, T> {
    fn current(&mut self) -> Self::Item {
        &self.with_iterator.current().0
    }

    fn has_reach_entity_end(&self) -> bool {
        self.with_iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.with_iterator.reset_current_entity()
    }
}

/// The entity name
pub struct WithName;

impl<'a> QueryParam<'a> for WithName {
    type Item = &'a String;
    type Iterator = WithNameIterator<'a, WithIterator<'a, Name>>;
    type FromEntityReferenceIterator = WithNameIterator<'a, FromEntityWithIterator<'a, Name>>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        WithNameIterator {
            with_iterator: With::iter(archetype),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        WithNameIterator {
            with_iterator: With::from_entity_reference(entity_reference),
        }
    }
}

/// An iterator over entity enabled state
pub struct WithEnabledIterator<'a, T: Iterator<Item = &'a Enabled> + EntityIterator> {
    with_iterator: T,
}

impl<'a, T: Iterator<Item = &'a Enabled> + EntityIterator> Iterator for WithEnabledIterator<'a, T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator.next().map(|enabled| enabled.0)
    }
}

impl<'a, T: Iterator<Item = &'a Enabled> + EntityIterator> EntityIterator
    for WithEnabledIterator<'a, T>
{
    fn current(&mut self) -> Self::Item {
        self.with_iterator.current().0
    }

    fn has_reach_entity_end(&self) -> bool {
        self.with_iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.with_iterator.reset_current_entity()
    }
}

/// The entity enabled state
pub struct WithEnabled;

impl<'a> QueryParam<'a> for WithEnabled {
    type Item = bool;
    type Iterator = WithEnabledIterator<'a, WithIterator<'a, Enabled>>;
    type FromEntityReferenceIterator = WithEnabledIterator<'a, FromEntityWithIterator<'a, Enabled>>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        WithEnabledIterator {
            with_iterator: With::iter(archetype),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        WithEnabledIterator {
            with_iterator: With::from_entity_reference(entity_reference),
        }
    }
}

/// An iterator over entity components with a given type
pub struct WithIterator<'a, T: Component + 'static> {
    _component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    current: NonNull<T>,
    current_entity_length: NonNull<usize>,
    current_entity_index: usize,
    end: NonNull<T>,
}

impl<'a, T: Component + 'static> WithIterator<'a, T> {
    fn new(component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>) -> Self {
        let component_storage = component_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<T>>()
            .unwrap();

        let begin = &component_storage.data.data[0] as *const T as *mut T;
        let begin_entity_length = &component_storage.data.lengths[0] as *const usize as *mut usize;
        let end = unsafe { begin.add(component_storage.data.data.len()) };

        WithIterator {
            _component_storage_lock: component_storage_lock,
            current: unsafe { NonNull::new_unchecked(begin) },
            current_entity_length: unsafe { NonNull::new_unchecked(begin_entity_length) },
            current_entity_index: 0,
            end: unsafe { NonNull::new_unchecked(end) },
        }
    }
}

impl<'a, T: Component + 'static> Iterator for WithIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            if self.current_entity_index == *unsafe { self.current_entity_length.as_ref() } {
                self.current_entity_length =
                    unsafe { NonNull::new_unchecked(self.current_entity_length.as_ptr().add(1)) };
                self.current_entity_index = 0;
            } else {
                self.current_entity_index += 1;
            }

            Some(result)
        } else {
            None
        }
    }
}

impl<'a, T: Component + 'static> EntityIterator for WithIterator<'a, T> {
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_ref() }
    }

    fn has_reach_entity_end(&self) -> bool {
        self.current_entity_index + 1 == *unsafe { self.current_entity_length.as_ref() }
    }

    fn reset_current_entity(&mut self) {
        self.current =
            unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(self.current_entity_index)) };
        self.current_entity_index = 0;
    }
}

/// An iterator over entity components of a single entity
pub struct FromEntityWithIterator<'a, T: Component + 'static> {
    _component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    begin: NonNull<T>,
    current: NonNull<T>,
    end: NonNull<T>,
}

impl<'a, T: Component + 'static> FromEntityWithIterator<'a, T> {
    fn new(
        component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
        location: &EntityLocation,
    ) -> Self {
        let vec_component_storage = component_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<T>>()
            .unwrap();

        let slice_range = vec_component_storage
            .data
            .get_slice(location.index)
            .unwrap()
            .as_ptr_range();

        Self {
            _component_storage_lock: component_storage_lock,
            begin: unsafe { NonNull::new_unchecked(slice_range.start as *mut T) },
            current: unsafe { NonNull::new_unchecked(slice_range.start as *mut T) },
            end: unsafe { NonNull::new_unchecked(slice_range.end as *mut T) },
        }
    }
}

impl<'a, T: Component + 'static> EntityIterator for FromEntityWithIterator<'a, T> {
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_ref() }
    }

    fn has_reach_entity_end(&self) -> bool {
        unsafe { self.current.as_ptr().add(1) == self.end.as_ptr() }
    }

    fn reset_current_entity(&mut self) {
        self.current = self.begin;
    }
}

impl<'a, T: Component + 'static> Iterator for FromEntityWithIterator<'a, T> {
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

/// A readable component reference
pub struct With<T: Component + 'static> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + 'static> QueryParam<'a> for With<T> {
    type Item = &'a T;
    type Iterator = WithIterator<'a, T>;
    type FromEntityReferenceIterator = FromEntityWithIterator<'a, T>;

    fn filter_archetype(component_types: &ArchetypeComponentTypes) -> bool {
        component_types.contains(ComponentTypeId::of::<T>())
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let component_storage_lock =
            archetype.component_storages[&ComponentTypeId::of::<T>()].read();
        WithIterator::new(component_storage_lock)
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            let component_storage_lock =
                archetype.component_storages[&ComponentTypeId::of::<T>()].read();

            FromEntityWithIterator::new(component_storage_lock, location)
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity components with a given type with mutability
pub struct WithMutIterator<'a, T: Component + 'static> {
    _component_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    current: NonNull<T>,
    current_entity_length: NonNull<usize>,
    current_entity_index: usize,
    end: NonNull<T>,
}

impl<'a, T: Component + 'static> WithMutIterator<'a, T> {
    fn new(component_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>) -> Self {
        let component_storage = component_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<T>>()
            .unwrap();

        let begin = &component_storage.data.data[0] as *const T as *mut T;
        let begin_entity_length = &component_storage.data.lengths[0] as *const usize as *mut usize;
        let end = unsafe { begin.add(component_storage.data.data.len()) };

        Self {
            _component_storage_lock: component_storage_lock,
            current: unsafe { NonNull::new_unchecked(begin) },
            current_entity_length: unsafe { NonNull::new_unchecked(begin_entity_length) },
            current_entity_index: 0,
            end: unsafe { NonNull::new_unchecked(end) },
        }
    }
}

impl<'a, T: Component + 'static> Iterator for WithMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = unsafe { self.current.as_mut() };
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            if self.current_entity_index == *unsafe { self.current_entity_length.as_ref() } {
                self.current_entity_length =
                    unsafe { NonNull::new_unchecked(self.current_entity_length.as_ptr().add(1)) };
                self.current_entity_index = 0;
            } else {
                self.current_entity_index += 1;
            }

            Some(result)
        } else {
            None
        }
    }
}

impl<'a, T: Component + 'static> EntityIterator for WithMutIterator<'a, T> {
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_mut() }
    }

    fn has_reach_entity_end(&self) -> bool {
        self.current_entity_index + 1 == *unsafe { self.current_entity_length.as_ref() }
    }

    fn reset_current_entity(&mut self) {
        self.current =
            unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(self.current_entity_index)) };
        self.current_entity_index = 0;
    }
}

/// An iterator over entity components with a given type with mutability
pub struct FromEntityWithMutIterator<'a, T: Component + 'static> {
    _component_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    begin: NonNull<T>,
    current: NonNull<T>,
    end: NonNull<T>,
}

impl<'a, T: Component + 'static> FromEntityWithMutIterator<'a, T> {
    fn new(
        component_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
        location: &EntityLocation,
    ) -> Self {
        let vec_component_storage = component_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<T>>()
            .unwrap();

        let slice_range = vec_component_storage
            .data
            .get_slice(location.index)
            .unwrap()
            .as_ptr_range();

        Self {
            _component_storage_lock: component_storage_lock,
            begin: unsafe { NonNull::new_unchecked(slice_range.start as *mut T) },
            current: unsafe { NonNull::new_unchecked(slice_range.start as *mut T) },
            end: unsafe { NonNull::new_unchecked(slice_range.end as *mut T) },
        }
    }
}

impl<'a, T: Component + 'static> Iterator for FromEntityWithMutIterator<'a, T> {
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

impl<'a, T: Component + 'static> EntityIterator for FromEntityWithMutIterator<'a, T> {
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_mut() }
    }

    fn has_reach_entity_end(&self) -> bool {
        unsafe { self.current.as_ptr().add(1) == self.end.as_ptr() }
    }

    fn reset_current_entity(&mut self) {
        self.current = self.begin;
    }
}

/// A writable component reference
pub struct WithMut<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + 'static> QueryParam<'a> for WithMut<T> {
    type Item = &'a mut T;
    type Iterator = WithMutIterator<'a, T>;
    type FromEntityReferenceIterator = FromEntityWithMutIterator<'a, T>;

    fn filter_archetype(component_types: &ArchetypeComponentTypes) -> bool {
        component_types.contains(ComponentTypeId::of::<T>())
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let component_storage_lock =
            archetype.component_storages[&ComponentTypeId::of::<T>()].write();
        WithMutIterator::new(component_storage_lock)
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            let component_storage_lock =
                archetype.component_storages[&ComponentTypeId::of::<T>()].write();

            FromEntityWithMutIterator::new(component_storage_lock, location)
        } else {
            unreachable!()
        }
    }
}

/// An optional entity iterator
pub struct WithOptionalIterator<T: Iterator + EntityIterator>(Option<T>);

impl<T: Iterator + EntityIterator> WithOptionalIterator<T> {
    fn new(iter: T) -> Self {
        Self(Some(iter))
    }

    fn empty() -> Self {
        Self(None)
    }
}

impl<T: Iterator + EntityIterator> Iterator for WithOptionalIterator<T> {
    type Item = Option<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.0.as_mut() {
            Some(iter.next())
        } else {
            Some(None)
        }
    }
}

impl<T: Iterator + EntityIterator> EntityIterator for WithOptionalIterator<T> {
    fn current(&mut self) -> Self::Item {
        if let Some(iter) = self.0.as_mut() {
            Some(iter.current())
        } else {
            None
        }
    }

    fn has_reach_entity_end(&self) -> bool {
        if let Some(iter) = self.0.as_ref() {
            iter.has_reach_entity_end()
        } else {
            true
        }
    }

    fn reset_current_entity(&mut self) {
        if let Some(iter) = self.0.as_mut() {
            iter.reset_current_entity()
        }
    }
}

/// A readable optional component reference
pub struct WithOptional<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + 'static> QueryParam<'a> for WithOptional<T> {
    type Item = Option<&'a T>;
    type Iterator = WithOptionalIterator<WithIterator<'a, T>>;
    type FromEntityReferenceIterator = WithOptionalIterator<FromEntityWithIterator<'a, T>>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        if let Some(component_storage) = archetype
            .component_storages
            .get(&ComponentTypeId::of::<T>())
        {
            WithOptionalIterator::new(WithIterator::new(component_storage.read()))
        } else {
            WithOptionalIterator::empty()
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            if let Some(component_storage) = archetype
                .component_storages
                .get(&ComponentTypeId::of::<T>())
            {
                let component_storage_lock = component_storage.read();
                WithOptionalIterator::new(FromEntityWithIterator::new(
                    component_storage_lock,
                    location,
                ))
            } else {
                WithOptionalIterator::empty()
            }
        } else {
            unreachable!()
        }
    }
}

/// A writable optional component reference
pub struct WithOptionalMut<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + 'static> QueryParam<'a> for WithOptionalMut<T> {
    type Item = Option<&'a mut T>;
    type Iterator = WithOptionalIterator<WithMutIterator<'a, T>>;
    type FromEntityReferenceIterator = WithOptionalIterator<FromEntityWithMutIterator<'a, T>>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        if let Some(component_storage) = archetype
            .component_storages
            .get(&ComponentTypeId::of::<T>())
        {
            WithOptionalIterator::new(WithMutIterator::new(component_storage.write()))
        } else {
            WithOptionalIterator::empty()
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            if let Some(component_storage) = archetype
                .component_storages
                .get(&ComponentTypeId::of::<T>())
            {
                let component_storage_lock = component_storage.write();
                WithOptionalIterator::new(FromEntityWithMutIterator::new(
                    component_storage_lock,
                    location,
                ))
            } else {
                WithOptionalIterator::empty()
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity components with a given type and its associated extension
pub struct WithExtensionIterator<'a, T: Component + 'static, E: Component + 'static> {
    _extension_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    with_iterator: WithIterator<'a, T>,
    current_extension: NonNull<E>,
}

impl<'a, T: Component + 'static, E: Component + 'static> WithExtensionIterator<'a, T, E> {
    fn new(
        component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
        extension_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    ) -> Self {
        let extension_component_storage = extension_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<E>>()
            .unwrap();

        let begin_extension = &extension_component_storage.data.data[0] as *const E as *mut E;

        Self {
            _extension_storage_lock: extension_storage_lock,
            with_iterator: WithIterator::new(component_storage_lock),
            current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
        }
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> Iterator
    for WithExtensionIterator<'a, T, E>
{
    type Item = (&'a T, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator.next().map(|component| {
            let result = (component, unsafe { self.current_extension.as_ref() });
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            result
        })
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> EntityIterator
    for WithExtensionIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        (unsafe { self.with_iterator.current.as_ref() }, unsafe {
            self.current_extension.as_ref()
        })
    }

    fn has_reach_entity_end(&self) -> bool {
        self.with_iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.current_extension = unsafe {
            NonNull::new_unchecked(
                self.current_extension
                    .as_ptr()
                    .sub(self.with_iterator.current_entity_index),
            )
        };
        self.with_iterator.reset_current_entity();
    }
}

/// An iterator over entity components of a single entity
pub struct FromEntityWithExtensionIterator<'a, T: Component + 'static, E: Component + 'static> {
    _extension_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    with_iterator: FromEntityWithIterator<'a, T>,
    current_extension: NonNull<E>,
    begin_extension: NonNull<E>,
}

impl<'a, T: Component + 'static, E: Component + 'static> FromEntityWithExtensionIterator<'a, T, E> {
    fn new(
        component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
        extension_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
        location: &EntityLocation,
    ) -> Self {
        let extension_component_storage = extension_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<E>>()
            .unwrap();

        let begin_extension = extension_component_storage
            .data
            .get_slice(location.index)
            .unwrap()
            .as_ptr() as *mut E;

        Self {
            _extension_storage_lock: extension_storage_lock,
            with_iterator: FromEntityWithIterator::new(component_storage_lock, location),
            current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
            begin_extension: unsafe { NonNull::new_unchecked(begin_extension) },
        }
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> Iterator
    for FromEntityWithExtensionIterator<'a, T, E>
{
    type Item = (&'a T, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator.next().map(|component| {
            let result = (component, unsafe { self.current_extension.as_ref() });
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            result
        })
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> EntityIterator
    for FromEntityWithExtensionIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        (unsafe { self.with_iterator.current.as_ref() }, unsafe {
            self.current_extension.as_ref()
        })
    }

    fn has_reach_entity_end(&self) -> bool {
        self.with_iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.current_extension = self.begin_extension;
        self.with_iterator.reset_current_entity();
    }
}

/// A readable component reference
pub struct WithExtension<T, E> {
    _phantom: PhantomData<(T, E)>,
}

impl<'a, T: Component + 'static, E: Component + 'static> QueryParam<'a> for WithExtension<T, E> {
    type Item = (&'a T, &'a E);
    type Iterator = WithExtensionIterator<'a, T, E>;
    type FromEntityReferenceIterator = FromEntityWithExtensionIterator<'a, T, E>;

    fn filter_archetype(component_types: &ArchetypeComponentTypes) -> bool {
        component_types.contains(ComponentTypeId::of::<T>())
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let component_storage_lock =
            archetype.component_storages[&ComponentTypeId::of::<T>()].read();

        let extension_component_storage_lock =
            archetype.component_storages[&ComponentTypeId::of::<E>()].read();

        Self::Iterator::new(component_storage_lock, extension_component_storage_lock)
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            let component_storage_lock =
                archetype.component_storages[&ComponentTypeId::of::<T>()].read();

            let extension_component_storage_lock =
                archetype.component_storages[&ComponentTypeId::of::<E>()].read();

            FromEntityWithExtensionIterator::new(
                component_storage_lock,
                extension_component_storage_lock,
                location,
            )
        } else {
            unimplemented!()
        }
    }
}

/// An iterator over entity components with a given type and its associated extension with mutability
pub struct WithExtensionMutIterator<'a, T: Component + 'static, E: Component + 'static> {
    _extension_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    with_iterator: WithMutIterator<'a, T>,
    current_extension: NonNull<E>,
}

impl<'a, T: Component + 'static, E: Component + 'static> WithExtensionMutIterator<'a, T, E> {
    fn new(
        component_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
        extension_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    ) -> Self {
        let extension_component_storage = extension_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<E>>()
            .unwrap();

        let begin_extension = &extension_component_storage.data.data[0] as *const E as *mut E;

        Self {
            _extension_storage_lock: extension_storage_lock,
            with_iterator: WithMutIterator::new(component_storage_lock),
            current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
        }
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> Iterator
    for WithExtensionMutIterator<'a, T, E>
{
    type Item = (&'a mut T, &'a mut E);

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator.next().map(|component| {
            let result = (component, unsafe { self.current_extension.as_mut() });
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            result
        })
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> EntityIterator
    for WithExtensionMutIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        (unsafe { self.with_iterator.current.as_mut() }, unsafe {
            self.current_extension.as_mut()
        })
    }

    fn has_reach_entity_end(&self) -> bool {
        self.with_iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.current_extension = unsafe {
            NonNull::new_unchecked(
                self.current_extension
                    .as_ptr()
                    .sub(self.with_iterator.current_entity_index),
            )
        };
        self.with_iterator.reset_current_entity();
    }
}

/// An iterator over entity components of a single entity
pub struct FromEntityWithExtensionMutIterator<'a, T: Component + 'static, E: Component + 'static> {
    _extension_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
    with_iterator: FromEntityWithMutIterator<'a, T>,
    current_extension: NonNull<E>,
    begin_extension: NonNull<E>,
}

impl<'a, T: Component + 'static, E: Component + 'static>
    FromEntityWithExtensionMutIterator<'a, T, E>
{
    fn new(
        component_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
        extension_storage_lock: RwLockWriteGuard<'a, Box<dyn ComponentStorage>>,
        location: &EntityLocation,
    ) -> Self {
        let extension_component_storage = extension_storage_lock
            .as_any_ref()
            .downcast_ref::<VecComponentStorage<E>>()
            .unwrap();

        let begin_extension = extension_component_storage
            .data
            .get_slice(location.index)
            .unwrap()
            .as_ptr() as *mut E;

        Self {
            _extension_storage_lock: extension_storage_lock,
            with_iterator: FromEntityWithMutIterator::new(component_storage_lock, location),
            current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
            begin_extension: unsafe { NonNull::new_unchecked(begin_extension) },
        }
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> Iterator
    for FromEntityWithExtensionMutIterator<'a, T, E>
{
    type Item = (&'a mut T, &'a mut E);

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator.next().map(|component| {
            let result = (component, unsafe { self.current_extension.as_mut() });
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            result
        })
    }
}

impl<'a, T: Component + 'static, E: Component + 'static> EntityIterator
    for FromEntityWithExtensionMutIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        (unsafe { self.with_iterator.current.as_mut() }, unsafe {
            self.current_extension.as_mut()
        })
    }

    fn has_reach_entity_end(&self) -> bool {
        self.with_iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.current_extension = self.begin_extension;
        self.with_iterator.reset_current_entity();
    }
}

/// A readable component reference
pub struct WithExtensionMut<T, E> {
    _phantom: PhantomData<(T, E)>,
}

impl<'a, T: Component + 'static, E: Component + 'static> QueryParam<'a> for WithExtensionMut<T, E> {
    type Item = (&'a mut T, &'a mut E);
    type Iterator = WithExtensionMutIterator<'a, T, E>;
    type FromEntityReferenceIterator = FromEntityWithExtensionMutIterator<'a, T, E>;

    fn filter_archetype(component_types: &ArchetypeComponentTypes) -> bool {
        component_types.contains(ComponentTypeId::of::<T>())
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let component_storage_lock =
            archetype.component_storages[&ComponentTypeId::of::<T>()].write();

        let extension_component_storage_lock =
            archetype.component_storages[&ComponentTypeId::of::<E>()].write();

        Self::Iterator::new(component_storage_lock, extension_component_storage_lock)
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            let component_storage_lock =
                archetype.component_storages[&ComponentTypeId::of::<T>()].write();

            let extension_component_storage_lock =
                archetype.component_storages[&ComponentTypeId::of::<E>()].write();

            FromEntityWithExtensionMutIterator::new(
                component_storage_lock,
                extension_component_storage_lock,
                location,
            )
        } else {
            unimplemented!()
        }
    }
}

/// A readable optional component reference
pub struct WithExtensionOptional<T, E> {
    _phantom: PhantomData<(T, E)>,
}

impl<'a, T: Component + 'static, E: Component + 'static> QueryParam<'a>
    for WithExtensionOptional<T, E>
{
    type Item = Option<(&'a T, &'a E)>;
    type Iterator = WithOptionalIterator<WithExtensionIterator<'a, T, E>>;
    type FromEntityReferenceIterator =
        WithOptionalIterator<FromEntityWithExtensionIterator<'a, T, E>>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        if let Some(component_storage) = archetype
            .component_storages
            .get(&ComponentTypeId::of::<T>())
        {
            if let Some(extension_storage) = archetype
                .component_storages
                .get(&ComponentTypeId::of::<E>())
            {
                WithOptionalIterator::new(WithExtensionIterator::new(
                    component_storage.read(),
                    extension_storage.read(),
                ))
            } else {
                WithOptionalIterator::empty()
            }
        } else {
            WithOptionalIterator::empty()
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            if let Some(component_storage) = archetype
                .component_storages
                .get(&ComponentTypeId::of::<T>())
            {
                if let Some(extension_storage) = archetype
                    .component_storages
                    .get(&ComponentTypeId::of::<E>())
                {
                    let component_storage_lock = component_storage.read();
                    let extension_storage_lock = extension_storage.read();
                    WithOptionalIterator::new(FromEntityWithExtensionIterator::new(
                        component_storage_lock,
                        extension_storage_lock,
                        location,
                    ))
                } else {
                    WithOptionalIterator::empty()
                }
            } else {
                WithOptionalIterator::empty()
            }
        } else {
            unreachable!()
        }
    }
}

/// A readable optional component reference
pub struct WithExtensionOptionalMut<T, E> {
    _phantom: PhantomData<(T, E)>,
}

impl<'a, T: Component + 'static, E: Component + 'static> QueryParam<'a>
    for WithExtensionOptionalMut<T, E>
{
    type Item = Option<(&'a mut T, &'a mut E)>;
    type Iterator = WithOptionalIterator<WithExtensionMutIterator<'a, T, E>>;
    type FromEntityReferenceIterator =
        WithOptionalIterator<FromEntityWithExtensionMutIterator<'a, T, E>>;

    fn filter_archetype(_component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        if let Some(component_storage) = archetype
            .component_storages
            .get(&ComponentTypeId::of::<T>())
        {
            if let Some(extension_storage) = archetype
                .component_storages
                .get(&ComponentTypeId::of::<E>())
            {
                WithOptionalIterator::new(WithExtensionMutIterator::new(
                    component_storage.write(),
                    extension_storage.write(),
                ))
            } else {
                WithOptionalIterator::empty()
            }
        } else {
            WithOptionalIterator::empty()
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype.0] as *const Archetype,
                )
                .unwrap()
            };

            if let Some(component_storage) = archetype
                .component_storages
                .get(&ComponentTypeId::of::<T>())
            {
                if let Some(extension_storage) = archetype
                    .component_storages
                    .get(&ComponentTypeId::of::<E>())
                {
                    let component_storage_lock = component_storage.write();
                    let extension_storage_lock = extension_storage.write();
                    WithOptionalIterator::new(FromEntityWithExtensionMutIterator::new(
                        component_storage_lock,
                        extension_storage_lock,
                        location,
                    ))
                } else {
                    WithOptionalIterator::empty()
                }
            } else {
                WithOptionalIterator::empty()
            }
        } else {
            unreachable!()
        }
    }
}
