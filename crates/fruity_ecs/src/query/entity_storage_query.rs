use crate::entity::{Archetype, ArchetypeComponentTypes, EntityReference, EntityStorage};
use fruity_game_engine::{
    signal::ObserverHandler,
    sync::{Arc, RwLock},
    FruityResult,
};
use sorted_vec::SortedVec;
use std::{marker::PhantomData, ptr::NonNull};

/// An iterator over entities elements
pub trait EntityIterator: Iterator {
    /// Go back with a given number of move
    fn current(&mut self) -> Self::Item;

    /// Returns true if we are at the last elem of the entity
    fn has_reach_entity_end(&self) -> bool;

    /// Go back to the first elem of the entity
    fn reset_current_entity(&mut self);
}

/// A simple iterator that never ends returning always the same value
#[derive(Default, Clone)]
pub struct InfiniteEntityIterator<T: Clone> {
    value: T,
}

impl<T: Clone> InfiniteEntityIterator<T> {
    /// Returns a SingleEntityIterator
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> Iterator for InfiniteEntityIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value.clone())
    }
}

impl<T: Clone> EntityIterator for InfiniteEntityIterator<T> {
    fn current(&mut self) -> Self::Item {
        self.value.clone()
    }

    fn has_reach_entity_end(&self) -> bool {
        true
    }

    fn reset_current_entity(&mut self) {}
}

/// A simple iterator that contains only one value
#[derive(Default, Clone)]
pub struct SingleEntityIterator<T: Clone> {
    value: Option<T>,
}

impl<T: Clone> SingleEntityIterator<T> {
    /// Returns a SingleEntityIterator
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T: Clone> Iterator for SingleEntityIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.value.take()
    }
}

impl<T: Clone> EntityIterator for SingleEntityIterator<T> {
    fn current(&mut self) -> Self::Item {
        self.value.clone().unwrap()
    }

    fn has_reach_entity_end(&self) -> bool {
        true
    }

    fn reset_current_entity(&mut self) {}
}

/// A trait that should be implement for everything that can be queried from ['EntityService']
pub trait QueryParam<'a> {
    /// The type of the query callback parameter
    type Item;

    /// The type of the iterator for iter
    type Iterator: Iterator<Item = Self::Item> + EntityIterator + 'a;

    /// The type of the iterator for from_entity_reference
    type FromEntityReferenceIterator: Iterator<Item = Self::Item> + EntityIterator + 'a;

    /// A filter over the archetypes
    fn filter_archetype(component_types: &ArchetypeComponentTypes) -> bool;

    /// Iter over the queried components into a given archetype
    /// The iterator should not lock the entity guard, the query will take care of it
    fn iter(archetype: &'a Archetype) -> Self::Iterator;

    /// Iter over the queried components into a given entity
    /// It should not lock the entity guard, the query will take care of it
    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator;
}

#[derive(Clone)]
pub(crate) struct ArchetypePtr(pub(crate) NonNull<Archetype>);

impl PartialEq for ArchetypePtr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.as_ref() == other.0.as_ref() }
    }
}

impl Eq for ArchetypePtr {
    fn assert_receiver_is_total_eq(&self) {
        unsafe { self.0.as_ref().assert_receiver_is_total_eq() }
    }
}

impl PartialOrd for ArchetypePtr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe { self.0.as_ref().partial_cmp(&other.0.as_ref()) }
    }
}

impl Ord for ArchetypePtr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.0.as_ref().cmp(&other.0.as_ref()) }
    }
}

// Safe cause archetypes are updated when an archetype is moved trough memory
unsafe impl Sync for ArchetypePtr {}

// Safe cause archetypes are updated when an archetype is moved trough memory
unsafe impl Send for ArchetypePtr {}

pub(crate) struct InnerEntityStorageQuery {
    pub(crate) archetypes: SortedVec<ArchetypePtr>,
    pub(crate) on_archetype_created_handle: Option<ObserverHandler<NonNull<Archetype>>>,
    pub(crate) on_archetypes_reallocated_handle: Option<ObserverHandler<isize>>,
}

impl Drop for InnerEntityStorageQuery {
    fn drop(&mut self) {
        if let Some(on_archetype_created_handle) = self.on_archetype_created_handle.take() {
            on_archetype_created_handle.dispose_by_ref();
        }

        if let Some(on_archetypes_reallocated_handle) = self.on_archetypes_reallocated_handle.take()
        {
            on_archetypes_reallocated_handle.dispose_by_ref();
        }
    }
}

/// A query over entities
pub struct EntityStorageQuery<T> {
    inner: Arc<RwLock<InnerEntityStorageQuery>>,
    _param_phantom: PhantomData<T>,
}

impl<T> Clone for EntityStorageQuery<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _param_phantom: PhantomData,
        }
    }
}

// Safe cause all non phantom fields implements ['Sync']
unsafe impl<T> Sync for EntityStorageQuery<T> {}

// Safe cause all non phantom fields implements ['Send']
unsafe impl<T> Send for EntityStorageQuery<T> {}

impl<'a, T: QueryParam<'a> + 'static> EntityStorageQuery<T> {
    /// Create the entity query
    pub fn new(entity_storage: &EntityStorage) -> Self {
        // Filter existing archetypes
        let inner = Arc::new(RwLock::new(InnerEntityStorageQuery {
            archetypes: SortedVec::from(
                entity_storage
                    .archetypes
                    .iter()
                    .filter(|archetype| T::filter_archetype(archetype.get_component_types()))
                    .map(|archetype| unsafe {
                        ArchetypePtr(NonNull::new_unchecked(
                            archetype as *const Archetype as *mut Archetype,
                        ))
                    })
                    .collect::<Vec<_>>(),
            ),
            on_archetype_created_handle: None,
            on_archetypes_reallocated_handle: None,
        }));

        Self::generate_storage_observers(&inner, entity_storage);

        // Returns the query
        Self {
            inner,
            _param_phantom: PhantomData {},
        }
    }

    /// Call a function for every entities of an query
    pub fn for_each(
        &self,
        callback: impl Fn(T::Item) -> FruityResult<()> + Send + Sync,
    ) -> FruityResult<()> {
        let inner_reader = self.inner.read();

        #[cfg(target_arch = "wasm32")]
        let mut iterator = inner_reader.archetypes.iter();

        #[cfg(not(target_arch = "wasm32"))]
        let mut iterator = inner_reader.archetypes.iter();

        iterator.try_for_each(|archetype| {
            let archetype = unsafe { archetype.0.as_ref() };
            T::iter(archetype).try_for_each(|item| callback(item))
        })
    }

    pub(crate) fn generate_storage_observers(
        inner: &Arc<RwLock<InnerEntityStorageQuery>>,
        entity_storage: &EntityStorage,
    ) {
        // Listen to entity storage archetype create event
        let on_archetype_created_handle = {
            let inner_2 = inner.clone();
            entity_storage
                .on_archetype_created
                .add_observer(move |archetype_ptr| {
                    let mut inner_writer = inner_2.write();

                    // The archetypes after this one are moved cause archetypes are ordered
                    inner_writer.archetypes = SortedVec::from(
                        inner_writer
                            .archetypes
                            .iter()
                            .map(|archetype| unsafe {
                                if archetype.0.as_ref() <= archetype_ptr.as_ref() {
                                    archetype.clone()
                                } else {
                                    ArchetypePtr(NonNull::new_unchecked(
                                        archetype.0.as_ptr().add(1) as *mut Archetype,
                                    ))
                                }
                            })
                            .collect::<Vec<_>>(),
                    );

                    // Add the archetype to the query list if it match the filter
                    if T::filter_archetype(unsafe { archetype_ptr.as_ref().get_component_types() })
                    {
                        inner_writer.archetypes.push(ArchetypePtr(*archetype_ptr));
                    }

                    Ok(())
                })
        };

        // Listen to entity storage archetype reallocated event
        let on_archetypes_reallocated_handle = {
            let inner_2 = inner.clone();
            entity_storage
                .on_archetypes_reallocated
                .add_observer(move |addr_diff| {
                    let mut inner_writer = inner_2.write();

                    inner_writer.archetypes = SortedVec::from(
                        inner_writer
                            .archetypes
                            .iter()
                            .map(|archetype| unsafe {
                                ArchetypePtr(NonNull::new_unchecked(
                                    archetype.0.as_ptr().byte_offset(*addr_diff) as *mut Archetype,
                                ))
                            })
                            .collect::<Vec<_>>(),
                    );

                    Ok(())
                })
        };

        inner.write().on_archetype_created_handle = Some(on_archetype_created_handle);
        inner.write().on_archetypes_reallocated_handle = Some(on_archetypes_reallocated_handle);
    }
}
