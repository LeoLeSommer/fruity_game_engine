use super::BidirectionalIterator;
use super::NoneBidirectionalIterator;
use super::SingleBidirectionalIterator;
use crate::component::Component;
use crate::component::StaticComponent;
use crate::entity::archetype::Archetype;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_reference::EntityReference;
use crate::entity::entity_reference::InnerShareableEntityReference;
use crate::entity::Entity;
use crate::entity::EntityId;
use crate::entity::EntityMut;
use either::Either;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

/// The entity reference
pub struct WithEntityReference;

/// An iterator over entity references
pub struct WithEntityReferenceIterator<'a> {
    current_entity_index: usize,
    end_entity_index: usize,
    archetype: &'a Archetype,
}

impl<'a> Iterator for WithEntityReferenceIterator<'a> {
    type Item = EntityReference;

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

impl<'a> BidirectionalIterator for WithEntityReferenceIterator<'a> {
    fn current(&mut self) -> Self::Item {
        let entity_index = self.current_entity_index;
        self.archetype.get_entity_reference(entity_index)
    }

    fn go_back(&mut self, count: usize) {
        self.current_entity_index -= count;
    }
}

impl<'a> QueryParam<'a> for WithEntityReference {
    type Item = EntityReference;
    type Iterator = WithEntityReferenceIterator<'a>;
    type FromEntityReferenceIterator = SingleBidirectionalIterator<Self::Item>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(_archetype: &'a Archetype) -> usize {
        1
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        WithEntityReferenceIterator {
            current_entity_index: 0,
            end_entity_index: archetype.len(),
            archetype,
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        SingleBidirectionalIterator::new(entity_reference.clone())
    }
}

/// The entity
pub struct WithEntity;

/// An iterator over entities
pub struct WithEntityIterator<'a> {
    current_entity_index: usize,
    end_entity_index: usize,
    archetype: &'a Archetype,
}

impl<'a> Iterator for WithEntityIterator<'a> {
    type Item = Entity<'a>;

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

impl<'a> BidirectionalIterator for WithEntityIterator<'a> {
    fn current(&mut self) -> Self::Item {
        let entity_index = self.current_entity_index;
        Entity {
            entity_index,
            archetype: self.archetype,
        }
    }

    fn go_back(&mut self, count: usize) {
        self.current_entity_index -= count;
    }
}

impl<'a> QueryParam<'a> for WithEntity {
    type Item = Entity<'a>;
    type Iterator = WithEntityIterator<'a>;
    type FromEntityReferenceIterator = SingleBidirectionalIterator<Self::Item>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(_archetype: &'a Archetype) -> usize {
        1
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
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            SingleBidirectionalIterator::new(Entity {
                entity_index: *entity_index,
                archetype: unsafe { archetype_ptr.as_ref() }.unwrap(),
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
    type Item = EntityMut<'a>;

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

impl<'a> BidirectionalIterator for WithEntityMutIterator<'a> {
    fn current(&mut self) -> Self::Item {
        let entity_index = self.current_entity_index;
        EntityMut {
            entity_index,
            archetype: self.archetype,
        }
    }

    fn go_back(&mut self, count: usize) {
        self.current_entity_index -= count;
    }
}

impl<'a> QueryParam<'a> for WithEntityMut {
    type Item = EntityMut<'a>;
    type Iterator = WithEntityMutIterator<'a>;
    type FromEntityReferenceIterator = SingleBidirectionalIterator<Self::Item>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        true
    }

    fn items_per_entity(_archetype: &'a Archetype) -> usize {
        1
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
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            SingleBidirectionalIterator::new(EntityMut {
                entity_index: *entity_index,
                archetype: unsafe { archetype_ptr.as_ref() }.unwrap(),
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

impl<'a> BidirectionalIterator for WithIdIterator<'a> {
    fn current(&mut self) -> Self::Item {
        *unsafe { self.current.as_ref() }
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// The entity id
pub struct WithId;

impl<'a> QueryParam<'a> for WithId {
    type Item = EntityId;
    type Iterator = WithIdIterator<'a>;
    type FromEntityReferenceIterator = SingleBidirectionalIterator<Self::Item>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(_archetype: &'a Archetype) -> usize {
        1
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let begin = &archetype.entity_id_array[0] as *const EntityId as *mut EntityId;
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
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            SingleBidirectionalIterator::new(archetype.entity_id_array[*entity_index])
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity names
pub struct WithNameIterator<'a> {
    current: NonNull<String>,
    end: NonNull<String>,
    _marker: PhantomData<&'a String>,
}

impl<'a> Iterator for WithNameIterator<'a> {
    type Item = &'a String;

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

impl<'a> BidirectionalIterator for WithNameIterator<'a> {
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_ref() }
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// The entity name
pub struct WithName;

impl<'a> QueryParam<'a> for WithName {
    type Item = &'a String;
    type Iterator = WithNameIterator<'a>;
    type FromEntityReferenceIterator = SingleBidirectionalIterator<Self::Item>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(_archetype: &'a Archetype) -> usize {
        1
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let begin = &archetype.name_array[0] as *const String as *mut String;
        WithNameIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            SingleBidirectionalIterator::new(&archetype.name_array[*entity_index])
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity enabled state
pub struct WithEnabledIterator<'a> {
    current: NonNull<bool>,
    end: NonNull<bool>,
    _marker: PhantomData<&'a bool>,
}

impl<'a> Iterator for WithEnabledIterator<'a> {
    type Item = bool;

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

impl<'a> BidirectionalIterator for WithEnabledIterator<'a> {
    fn current(&mut self) -> Self::Item {
        *unsafe { self.current.as_ref() }
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// Is entity enabled
pub struct WithEnabled;

impl<'a> QueryParam<'a> for WithEnabled {
    type Item = bool;
    type Iterator = WithEnabledIterator<'a>;
    type FromEntityReferenceIterator = SingleBidirectionalIterator<Self::Item>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(_archetype: &'a Archetype) -> usize {
        1
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let begin = &archetype.enabled_array[0] as *const bool as *mut bool;
        WithEnabledIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            SingleBidirectionalIterator::new(archetype.enabled_array[*entity_index])
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity components with a given type
pub struct WithIterator<'a, T: Component + StaticComponent + 'static> {
    current: NonNull<T>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static> Iterator for WithIterator<'a, T> {
    type Item = &'a T;

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

impl<'a, T: Component + StaticComponent + 'static> BidirectionalIterator for WithIterator<'a, T> {
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_ref() }
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A readable component reference
pub struct With<T: Component + StaticComponent + 'static> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for With<T> {
    type Item = &'a T;
    type Iterator = WithIterator<'a, T>;
    type FromEntityReferenceIterator = WithIterator<'a, T>;

    fn filter_archetype(archetype: &Archetype) -> bool {
        archetype
            .identifier
            .contains(&T::get_component_name().to_string())
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype.component_storages[T::get_component_name()].components_per_entity
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let component_storage =
            &archetype.component_storages[T::get_component_name()].component_storage;
        let begin = component_storage
            .get(0)
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap() as *const T as *mut T;

        WithIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();
            let component_storage = &archetype.component_storages[T::get_component_name()];

            let begin = component_storage
                .component_storage
                .get(entity_index * component_storage.components_per_entity)
                .as_any_ref()
                .downcast_ref::<T>()
                .unwrap() as *const T as *mut T;

            WithIterator {
                current: unsafe { NonNull::new_unchecked(begin) },
                end: unsafe {
                    NonNull::new_unchecked(begin.add(component_storage.components_per_entity))
                },
                _marker: Default::default(),
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity components with a given type with mutability
pub struct WithMutIterator<'a, T: Component + StaticComponent + 'static> {
    current: NonNull<T>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static> Iterator for WithMutIterator<'a, T> {
    type Item = &'a mut T;

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

impl<'a, T: Component + StaticComponent + 'static> BidirectionalIterator
    for WithMutIterator<'a, T>
{
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_mut() }
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A writable component reference
pub struct WithMut<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for WithMut<T> {
    type Item = &'a mut T;
    type Iterator = WithMutIterator<'a, T>;
    type FromEntityReferenceIterator = WithMutIterator<'a, T>;

    fn filter_archetype(archetype: &Archetype) -> bool {
        archetype
            .identifier
            .contains(&T::get_component_name().to_string())
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype.component_storages[T::get_component_name()].components_per_entity
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let component_storage =
            &archetype.component_storages[T::get_component_name()].component_storage;
        let begin = component_storage
            .get(0)
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap() as *const T as *mut T;

        WithMutIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();
            let component_storage = &archetype.component_storages[T::get_component_name()];

            let begin = component_storage
                .component_storage
                .get(entity_index * component_storage.components_per_entity)
                .as_any_ref()
                .downcast_ref::<T>()
                .unwrap() as *const T as *mut T;

            WithMutIterator {
                current: unsafe { NonNull::new_unchecked(begin) },
                end: unsafe {
                    NonNull::new_unchecked(begin.add(component_storage.components_per_entity))
                },
                _marker: Default::default(),
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity optional components with a given type
pub struct WithOptionalIterator<'a, T: Component + StaticComponent + 'static> {
    current: NonNull<T>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static> Iterator for WithOptionalIterator<'a, T> {
    type Item = Option<&'a T>;

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

impl<'a, T: Component + StaticComponent + 'static> BidirectionalIterator
    for WithOptionalIterator<'a, T>
{
    fn current(&mut self) -> Self::Item {
        Some(unsafe { self.current.as_ref() })
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A readable optional component reference
pub struct WithOptional<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for WithOptional<T> {
    type Item = Option<&'a T>;
    type Iterator = Either<WithOptionalIterator<'a, T>, NoneBidirectionalIterator<&'a T>>;
    type FromEntityReferenceIterator =
        Either<WithOptionalIterator<'a, T>, NoneBidirectionalIterator<&'a T>>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype
            .component_storages
            .get(T::get_component_name())
            .map(|storage| storage.components_per_entity)
            .unwrap_or(1)
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        match archetype.component_storages.get(T::get_component_name()) {
            Some(storage) => {
                let begin = storage
                    .component_storage
                    .get(0)
                    .as_any_ref()
                    .downcast_ref::<T>()
                    .unwrap() as *const T as *mut T;

                Either::Left(WithOptionalIterator {
                    current: unsafe { NonNull::new_unchecked(begin) },
                    end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
                    _marker: Default::default(),
                })
            }
            None => Either::Right(NoneBidirectionalIterator::default()),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            match archetype.component_storages.get(T::get_component_name()) {
                Some(storage) => {
                    let begin = storage
                        .component_storage
                        .get(entity_index * storage.components_per_entity)
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap() as *const T as *mut T;

                    Either::Left(WithOptionalIterator {
                        current: unsafe { NonNull::new_unchecked(begin) },
                        end: unsafe {
                            NonNull::new_unchecked(begin.add(storage.components_per_entity))
                        },
                        _marker: Default::default(),
                    })
                }
                None => Either::Right(NoneBidirectionalIterator::default()),
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity optional components with a given type with mutability
pub struct WithOptionalMutIterator<'a, T: Component + StaticComponent + 'static> {
    current: NonNull<T>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static> Iterator for WithOptionalMutIterator<'a, T> {
    type Item = Option<&'a mut T>;

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

impl<'a, T: Component + StaticComponent + 'static> BidirectionalIterator
    for WithOptionalMutIterator<'a, T>
{
    fn current(&mut self) -> Self::Item {
        Some(unsafe { self.current.as_mut() })
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A writable optional component reference
pub struct WithOptionalMut<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for WithOptionalMut<T> {
    type Item = Option<&'a mut T>;
    type Iterator = Either<WithOptionalMutIterator<'a, T>, NoneBidirectionalIterator<&'a mut T>>;
    type FromEntityReferenceIterator =
        Either<WithOptionalMutIterator<'a, T>, NoneBidirectionalIterator<&'a mut T>>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype
            .component_storages
            .get(T::get_component_name())
            .map(|storage| storage.components_per_entity)
            .unwrap_or(1)
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        match archetype.component_storages.get(T::get_component_name()) {
            Some(storage) => {
                let begin = storage
                    .component_storage
                    .get(0)
                    .as_any_ref()
                    .downcast_ref::<T>()
                    .unwrap() as *const T as *mut T;

                Either::Left(WithOptionalMutIterator {
                    current: unsafe { NonNull::new_unchecked(begin) },
                    end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
                    _marker: Default::default(),
                })
            }
            None => Either::Right(NoneBidirectionalIterator::default()),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            match archetype.component_storages.get(T::get_component_name()) {
                Some(storage) => {
                    let begin = storage
                        .component_storage
                        .get(entity_index * storage.components_per_entity)
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap() as *const T as *mut T;

                    Either::Left(WithOptionalMutIterator {
                        current: unsafe { NonNull::new_unchecked(begin) },
                        end: unsafe {
                            NonNull::new_unchecked(begin.add(storage.components_per_entity))
                        },
                        _marker: Default::default(),
                    })
                }
                None => Either::Right(NoneBidirectionalIterator::default()),
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity components with a given type and its associated extension
pub struct WithExtensionIterator<
    'a,
    T: Component + StaticComponent + 'static,
    E: Component + StaticComponent + 'static,
> {
    current: NonNull<T>,
    current_extension: NonNull<E>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    Iterator for WithExtensionIterator<'a, T, E>
{
    type Item = (&'a T, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();

            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    BidirectionalIterator for WithExtensionIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        (unsafe { self.current.as_ref() }, unsafe {
            self.current_extension.as_ref()
        })
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A readable component reference
pub struct WithExtension<T, E> {
    _phantom: PhantomData<T>,
    _phantom_e: PhantomData<E>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    QueryParam<'a> for WithExtension<T, E>
{
    type Item = (&'a T, &'a E);
    type Iterator = WithExtensionIterator<'a, T, E>;
    type FromEntityReferenceIterator = WithExtensionIterator<'a, T, E>;

    fn filter_archetype(archetype: &Archetype) -> bool {
        archetype
            .identifier
            .contains(&T::get_component_name().to_string())
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype.component_storages[T::get_component_name()].components_per_entity
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let component_storage =
            &archetype.component_storages[T::get_component_name()].component_storage;
        let begin = component_storage
            .get(0)
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap() as *const T as *mut T;

        let component_storage_extension =
            &archetype.component_storages[E::get_component_name()].component_storage;
        let begin_extension = component_storage_extension
            .get(0)
            .as_any_ref()
            .downcast_ref::<E>()
            .unwrap() as *const E as *mut E;

        WithExtensionIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            let component_storage = &archetype.component_storages[T::get_component_name()];
            let begin = component_storage
                .component_storage
                .get(entity_index * component_storage.components_per_entity)
                .as_any_ref()
                .downcast_ref::<T>()
                .unwrap() as *const T as *mut T;

            let component_storage_extension =
                &archetype.component_storages[E::get_component_name()];
            let begin_extension = component_storage_extension
                .component_storage
                .get(entity_index * component_storage.components_per_entity)
                .as_any_ref()
                .downcast_ref::<E>()
                .unwrap() as *const E as *mut E;

            WithExtensionIterator {
                current: unsafe { NonNull::new_unchecked(begin) },
                current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
                end: unsafe {
                    NonNull::new_unchecked(begin.add(component_storage.components_per_entity))
                },
                _marker: Default::default(),
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity components with a given type and its associated extension with mutability
pub struct WithExtensionMutIterator<
    'a,
    T: Component + StaticComponent + 'static,
    E: Component + StaticComponent + 'static,
> {
    current: NonNull<T>,
    current_extension: NonNull<E>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    Iterator for WithExtensionMutIterator<'a, T, E>
{
    type Item = (&'a mut T, &'a mut E);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();

            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    BidirectionalIterator for WithExtensionMutIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        (unsafe { self.current.as_mut() }, unsafe {
            self.current_extension.as_mut()
        })
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A writable component reference
pub struct WithExtensionMut<T, E> {
    _phantom: PhantomData<T>,
    _phantom_e: PhantomData<E>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    QueryParam<'a> for WithExtensionMut<T, E>
{
    type Item = (&'a mut T, &'a mut E);
    type Iterator = WithExtensionMutIterator<'a, T, E>;
    type FromEntityReferenceIterator = WithExtensionMutIterator<'a, T, E>;

    fn filter_archetype(archetype: &Archetype) -> bool {
        archetype
            .identifier
            .contains(&T::get_component_name().to_string())
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype.component_storages[T::get_component_name()].components_per_entity
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        let test = archetype.component_storages.keys().collect::<Vec<_>>();
        println!("{:?}", T::get_component_name());
        println!("{:?}", test);

        let component_storage =
            &archetype.component_storages[T::get_component_name()].component_storage;
        let begin = component_storage
            .get(0)
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap() as *const T as *mut T;

        let component_storage_extension =
            &archetype.component_storages[E::get_component_name()].component_storage;

        let begin_extension = component_storage_extension
            .get(0)
            .as_any_ref()
            .downcast_ref::<E>()
            .unwrap() as *const E as *mut E;

        WithExtensionMutIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            let component_storage = &archetype.component_storages[T::get_component_name()];
            let begin = component_storage
                .component_storage
                .get(entity_index * component_storage.components_per_entity)
                .as_any_ref()
                .downcast_ref::<T>()
                .unwrap() as *const T as *mut T;

            let component_storage_extension =
                &archetype.component_storages[E::get_component_name()];
            let begin_extension = component_storage_extension
                .component_storage
                .get(entity_index * component_storage.components_per_entity)
                .as_any_ref()
                .downcast_ref::<E>()
                .unwrap() as *const E as *mut E;

            WithExtensionMutIterator {
                current: unsafe { NonNull::new_unchecked(begin) },
                current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
                end: unsafe {
                    NonNull::new_unchecked(begin.add(component_storage.components_per_entity))
                },
                _marker: Default::default(),
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity optional components with a given type and its associated extension
pub struct WithExtensionOptionalIterator<
    'a,
    T: Component + StaticComponent + 'static,
    E: Component + StaticComponent + 'static,
> {
    current: NonNull<T>,
    current_extension: NonNull<E>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    Iterator for WithExtensionOptionalIterator<'a, T, E>
{
    type Item = Option<(&'a T, &'a E)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();

            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    BidirectionalIterator for WithExtensionOptionalIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        Some((unsafe { self.current.as_ref() }, unsafe {
            self.current_extension.as_ref()
        }))
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A readable optional component reference
pub struct WithExtensionOptional<T, E> {
    _phantom: PhantomData<T>,
    _phantom_e: PhantomData<E>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    QueryParam<'a> for WithExtensionOptional<T, E>
{
    type Item = Option<(&'a T, &'a E)>;
    type Iterator =
        Either<WithExtensionOptionalIterator<'a, T, E>, NoneBidirectionalIterator<(&'a T, &'a E)>>;
    type FromEntityReferenceIterator =
        Either<WithExtensionOptionalIterator<'a, T, E>, NoneBidirectionalIterator<(&'a T, &'a E)>>;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype.component_storages[T::get_component_name()].components_per_entity
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        match archetype.component_storages.get(T::get_component_name()) {
            Some(storage) => {
                let begin = storage
                    .component_storage
                    .get(0)
                    .as_any_ref()
                    .downcast_ref::<T>()
                    .unwrap() as *const T as *mut T;

                let component_storage_extension =
                    &archetype.component_storages[E::get_component_name()].component_storage;
                let begin_extension = component_storage_extension
                    .get(0)
                    .as_any_ref()
                    .downcast_ref::<E>()
                    .unwrap() as *const E as *mut E;

                Either::Left(WithExtensionOptionalIterator {
                    current: unsafe { NonNull::new_unchecked(begin) },
                    current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
                    end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
                    _marker: Default::default(),
                })
            }
            None => Either::Right(NoneBidirectionalIterator::default()),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            match archetype.component_storages.get(T::get_component_name()) {
                Some(storage) => {
                    let begin = storage
                        .component_storage
                        .get(entity_index * storage.components_per_entity)
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap() as *const T as *mut T;

                    let component_storage_extension =
                        &archetype.component_storages[E::get_component_name()];
                    let begin_extension = component_storage_extension
                        .component_storage
                        .get(entity_index * storage.components_per_entity)
                        .as_any_ref()
                        .downcast_ref::<E>()
                        .unwrap() as *const E as *mut E;

                    Either::Left(WithExtensionOptionalIterator {
                        current: unsafe { NonNull::new_unchecked(begin) },
                        current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
                        end: unsafe {
                            NonNull::new_unchecked(begin.add(storage.components_per_entity))
                        },
                        _marker: Default::default(),
                    })
                }
                None => Either::Right(NoneBidirectionalIterator::default()),
            }
        } else {
            unreachable!()
        }
    }
}

/// An iterator over entity optional components with a given type and its associated extension with mutability
pub struct WithExtensionOptionalMutIterator<
    'a,
    T: Component + StaticComponent + 'static,
    E: Component + StaticComponent + 'static,
> {
    current: NonNull<T>,
    current_extension: NonNull<E>,
    end: NonNull<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    Iterator for WithExtensionOptionalMutIterator<'a, T, E>
{
    type Item = Option<(&'a mut T, &'a mut E)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();

            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };
            self.current_extension =
                unsafe { NonNull::new_unchecked(self.current_extension.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    BidirectionalIterator for WithExtensionOptionalMutIterator<'a, T, E>
{
    fn current(&mut self) -> Self::Item {
        Some((unsafe { self.current.as_mut() }, unsafe {
            self.current_extension.as_mut()
        }))
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

/// A writable optional component reference
pub struct WithExtensionOptionalMut<T, E> {
    _phantom: PhantomData<T>,
    _phantom_e: PhantomData<E>,
}

impl<'a, T: Component + StaticComponent + 'static, E: Component + StaticComponent + 'static>
    QueryParam<'a> for WithExtensionOptionalMut<T, E>
{
    type Item = Option<(&'a mut T, &'a mut E)>;
    type Iterator = Either<
        WithExtensionOptionalMutIterator<'a, T, E>,
        NoneBidirectionalIterator<(&'a mut T, &'a mut E)>,
    >;
    type FromEntityReferenceIterator = Either<
        WithExtensionOptionalMutIterator<'a, T, E>,
        NoneBidirectionalIterator<(&'a mut T, &'a mut E)>,
    >;

    fn filter_archetype(_archetype: &Archetype) -> bool {
        true
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn items_per_entity(archetype: &'a Archetype) -> usize {
        archetype.component_storages[T::get_component_name()].components_per_entity
    }

    fn iter(archetype: &'a Archetype) -> Self::Iterator {
        match archetype.component_storages.get(T::get_component_name()) {
            Some(storage) => {
                let begin = storage
                    .component_storage
                    .get(0)
                    .as_any_ref()
                    .downcast_ref::<T>()
                    .unwrap() as *const T as *mut T;

                let component_storage_extension =
                    &archetype.component_storages[E::get_component_name()].component_storage;
                let begin_extension = component_storage_extension
                    .get(0)
                    .as_any_ref()
                    .downcast_ref::<E>()
                    .unwrap() as *const E as *mut E;

                Either::Left(WithExtensionOptionalMutIterator {
                    current: unsafe { NonNull::new_unchecked(begin) },
                    current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
                    end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
                    _marker: Default::default(),
                })
            }
            None => Either::Right(NoneBidirectionalIterator::default()),
        }
    }

    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        let inner_entity_reference = entity_reference.inner.read();
        if let InnerShareableEntityReference::Archetype {
            entity_index,
            archetype_ptr,
        } = inner_entity_reference.deref()
        {
            let archetype = unsafe { archetype_ptr.as_ref() }.unwrap();

            match archetype.component_storages.get(T::get_component_name()) {
                Some(storage) => {
                    let begin = storage
                        .component_storage
                        .get(entity_index * storage.components_per_entity)
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap() as *const T as *mut T;

                    let component_storage_extension =
                        &archetype.component_storages[E::get_component_name()];
                    let begin_extension = component_storage_extension
                        .component_storage
                        .get(entity_index * storage.components_per_entity)
                        .as_any_ref()
                        .downcast_ref::<E>()
                        .unwrap() as *const E as *mut E;

                    Either::Left(WithExtensionOptionalMutIterator {
                        current: unsafe { NonNull::new_unchecked(begin) },
                        current_extension: unsafe { NonNull::new_unchecked(begin_extension) },
                        end: unsafe {
                            NonNull::new_unchecked(begin.add(storage.components_per_entity))
                        },
                        _marker: Default::default(),
                    })
                }
                None => Either::Right(NoneBidirectionalIterator::default()),
            }
        } else {
            unreachable!()
        }
    }
}
