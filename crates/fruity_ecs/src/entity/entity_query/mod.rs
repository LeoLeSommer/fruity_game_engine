use super::entity_service::OnArchetypeAddressMoved;
use super::entity_service::OnEntityAddressAdded;
use super::entity_service::OnEntityAddressRemoved;
use super::EntityId;
use crate::entity::archetype::Archetype;
use crate::entity::entity_reference::EntityReference;
use crate::EntityService;
use either::Either;
use fruity_game_engine::inject::Injectable;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use fruity_game_engine::RwLockReadGuard;
use fruity_game_engine::RwLockWriteGuard;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use rayon::iter::ParallelBridge;

#[cfg(not(target_arch = "wasm32"))]
use rayon::iter::ParallelIterator;

/// Queries for scripting languages
pub mod script;

/// Queries for tuples
pub mod tuple;

/// Queries for with stuffs
pub mod with;

/// Queries for without stuffs
pub mod without;

/// An enum to pass a guard into the [’QueryInjectable’]
pub enum RequestedEntityGuard<'a> {
    /// No guard required
    None,
    /// Read guard required
    Read(RwLockReadGuard<'a, ()>),
    /// Write guard required
    Write(RwLockWriteGuard<'a, ()>),
}

/// An iterator that allow you to go back
pub trait BidirectionalIterator: Iterator {
    /// Go back with a given number of move
    fn current(&mut self) -> Self::Item;

    /// Go back with a given number of move
    fn go_back(&mut self, count: usize);
}

impl<
        Item,
        Left: BidirectionalIterator + Iterator<Item = Item>,
        Right: BidirectionalIterator + Iterator<Item = Item>,
    > BidirectionalIterator for Either<Left, Right>
{
    fn current(&mut self) -> Self::Item {
        match self {
            Either::Left(left) => left.current(),
            Either::Right(right) => right.current(),
        }
    }

    fn go_back(&mut self, count: usize) {
        match self {
            Either::Left(left) => left.go_back(count),
            Either::Right(right) => right.go_back(count),
        }
    }
}

/// A simple iterator that never ends returning always the same value
#[derive(Default, Clone)]
pub struct InfiniteBidirectionalIterator<T: Clone> {
    value: T,
}

impl<T: Clone> InfiniteBidirectionalIterator<T> {
    /// Returns a SingleBidirectionalIterator
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone> Iterator for InfiniteBidirectionalIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value.clone())
    }
}

impl<T: Clone> BidirectionalIterator for InfiniteBidirectionalIterator<T> {
    fn current(&mut self) -> Self::Item {
        self.value.clone()
    }

    fn go_back(&mut self, _count: usize) {}
}

/// A simple iterator that contains only one value
#[derive(Default, Clone)]
pub struct SingleBidirectionalIterator<T: Clone> {
    value: Option<T>,
}

impl<T: Clone> SingleBidirectionalIterator<T> {
    /// Returns a SingleBidirectionalIterator
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T: Clone> Iterator for SingleBidirectionalIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.value.take()
    }
}

impl<T: Clone> BidirectionalIterator for SingleBidirectionalIterator<T> {
    fn current(&mut self) -> Self::Item {
        self.value.clone().unwrap()
    }

    fn go_back(&mut self, _count: usize) {}
}

/// A simple iterator that contains only one value
pub struct NoneBidirectionalIterator<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for NoneBidirectionalIterator<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<T> Iterator for NoneBidirectionalIterator<T> {
    type Item = Option<T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(None)
    }
}

impl<T> BidirectionalIterator for NoneBidirectionalIterator<T> {
    fn current(&mut self) -> Self::Item {
        None
    }

    fn go_back(&mut self, _count: usize) {}
}

/// A trait that should be implement for everything that can be queried from ['EntityService']
pub trait QueryParam<'a> {
    /// The type of the query callback parameter
    type Item;

    /// The type of the iterator for iter
    type Iterator: Iterator<Item = Self::Item> + BidirectionalIterator + 'a;

    /// The type of the iterator for from_entity_reference
    type FromEntityReferenceIterator: Iterator<Item = Self::Item> + BidirectionalIterator + 'a;

    /// A filter over the archetypes
    fn filter_archetype(archetype: &Archetype) -> bool;

    /// Does this require a read guard over the reference
    fn require_read() -> bool;

    /// Does this require a write guard over the reference
    fn require_write() -> bool;

    /// How many item are iterated for a given entity
    fn items_per_entity(archetype: &'a Archetype) -> usize;

    /// Iter over the queried components into a given archetype
    /// The iterator should not lock the entity guard, the query will take care of it
    fn iter(archetype: &'a Archetype) -> Self::Iterator;

    /// Iter over the queried components into a given entity
    /// It should not lock the entity guard, the query will take care of it
    fn from_entity_reference(
        entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator;
}

pub(crate) struct ArchetypePtr(NonNull<Archetype>);

// Safe cause archetypes are updated when an archetype is moved trough memory
unsafe impl Sync for ArchetypePtr {}

// Safe cause archetypes are updated when an archetype is moved trough memory
unsafe impl Send for ArchetypePtr {}

/// A query over entities
pub struct Query<T> {
    /// A signal raised when an entity that match the query is created
    pub on_entity_created: Signal<EntityReference>,

    /// A signal raised when an entity that match the query is deleted
    pub on_entity_deleted: Signal<EntityId>,

    archetypes: Arc<RwLock<Vec<ArchetypePtr>>>,
    on_archetype_address_added_handle: ObserverHandler<NonNull<Archetype>>,
    on_archetype_address_moved_handle: ObserverHandler<OnArchetypeAddressMoved>,
    on_entity_address_added_handle: ObserverHandler<OnEntityAddressAdded>,
    on_entity_address_removed_handle: ObserverHandler<OnEntityAddressRemoved>,
    _param_phantom: PhantomData<T>,
}

impl<T> Clone for Query<T> {
    fn clone(&self) -> Self {
        Query {
            archetypes: self.archetypes.clone(),
            on_entity_created: self.on_entity_created.clone(),
            on_entity_deleted: self.on_entity_deleted.clone(),
            on_archetype_address_added_handle: self.on_archetype_address_added_handle.clone(),
            on_archetype_address_moved_handle: self.on_archetype_address_moved_handle.clone(),
            on_entity_address_added_handle: self.on_entity_address_added_handle.clone(),
            on_entity_address_removed_handle: self.on_entity_address_removed_handle.clone(),
            _param_phantom: PhantomData {},
        }
    }
}

// Safe cause all non phantom fields implements ['Sync']
unsafe impl<T> Sync for Query<T> {}

// Safe cause all non phantom fields implements ['Send']
unsafe impl<T> Send for Query<T> {}

impl<'a, T: QueryParam<'a> + 'static> Query<T> {
    /// Create the entity query
    pub fn new(entity_service: &EntityService) -> Self {
        // Filter existing archetypes
        let archetypes = Arc::new(RwLock::new(
            entity_service
                .archetypes
                .iter()
                .filter(|archetype| T::filter_archetype(archetype))
                .map(|archetype| unsafe {
                    ArchetypePtr(NonNull::new_unchecked(
                        archetype as *const Archetype as *mut Archetype,
                    ))
                })
                .collect::<Vec<_>>(),
        ));

        // Listen to entity service archetypes vec reallocations
        // Register memory move observers to update the entity reference inner pointers when the memory is moved
        let (
            on_entity_created,
            on_entity_deleted,
            on_archetype_address_added_handle,
            on_archetype_address_moved_handle,
            on_entity_address_added_handle,
            on_entity_address_removed_handle,
        ) = {
            let archetypes_2 = archetypes.clone();
            let on_archetype_address_added_handle = entity_service
                .on_archetype_address_added
                .add_observer(move |archetype| {
                    if T::filter_archetype(unsafe { archetype.as_ref() }) {
                        let mut archetypes_writer = archetypes_2.write();
                        archetypes_writer.push(ArchetypePtr(*archetype));
                    }

                    Ok(())
                });

            let archetypes_3 = archetypes.clone();
            let on_archetype_address_moved_handle = entity_service
                .on_archetype_address_moved
                .add_observer(move |event| {
                    let mut archetypes_writer = archetypes_3.write();
                    *archetypes_writer = archetypes_writer
                        .drain(..)
                        .filter_map(|archetype| {
                            if archetype.0 == event.old {
                                if let Some(new_archetype) = unsafe { event.new.as_mut() } {
                                    Some(unsafe {
                                        ArchetypePtr(NonNull::new_unchecked(
                                            new_archetype as *mut Archetype,
                                        ))
                                    })
                                } else {
                                    None
                                }
                            } else {
                                Some(archetype)
                            }
                        })
                        .collect::<Vec<_>>();

                    Ok(())
                });

            // Listen the entity create and remove events
            let on_entity_created = Signal::<EntityReference>::new();
            let on_entity_created_2 = on_entity_created.clone();
            let on_entity_address_added_handle = entity_service
                .on_entity_address_added
                .add_observer(move |event| {
                    if T::filter_archetype(unsafe { event.archetype.as_ref() }) {
                        on_entity_created_2.notify(event.entity_reference.clone())?;
                    }

                    Ok(())
                });

            let on_entity_deleted = Signal::<EntityId>::new();
            let on_entity_deleted_2 = on_entity_deleted.clone();
            let on_entity_address_removed_handle = entity_service
                .on_entity_address_removed
                .add_observer(move |event| {
                    if T::filter_archetype(unsafe { event.archetype.as_ref() }) {
                        on_entity_deleted_2.notify(event.entity_id)?;
                    }

                    Ok(())
                });

            (
                on_entity_created,
                on_entity_deleted,
                on_archetype_address_added_handle,
                on_archetype_address_moved_handle,
                on_entity_address_added_handle,
                on_entity_address_removed_handle,
            )
        };

        // Returns the query
        Self {
            archetypes,
            on_entity_created,
            on_entity_deleted,
            on_archetype_address_added_handle,
            on_archetype_address_moved_handle,
            on_entity_address_added_handle,
            on_entity_address_removed_handle,
            _param_phantom: PhantomData {},
        }
    }

    /// Call a function for every entities of an query
    pub fn for_each(
        &self,
        callback: impl Fn(T::Item) -> FruityResult<()> + Send + Sync,
    ) -> FruityResult<()> {
        let archetypes_reader = self.archetypes.read();

        #[cfg(target_arch = "wasm32")]
        let mut iterator = archetypes_reader.iter();

        #[cfg(not(target_arch = "wasm32"))]
        let iterator = archetypes_reader.iter();

        #[cfg(not(target_arch = "wasm32"))]
        let iterator = iterator.par_bridge();

        iterator.try_for_each(|archetype| {
            let archetype = unsafe { archetype.0.as_ref() };
            let test = T::iter(archetype)
                .enumerate()
                .try_for_each(|(index, item)| {
                    let entity_lock = &archetype.lock_array[index / T::items_per_entity(archetype)];

                    // Guard the entity
                    let _entity_guard = if T::require_write() {
                        RequestedEntityGuard::Write(entity_lock.write())
                    } else if T::require_read() {
                        RequestedEntityGuard::Read(entity_lock.read())
                    } else {
                        RequestedEntityGuard::None
                    };

                    callback(item)
                });

            test
        })
    }

    /// Call a function for every entities of an query
    pub fn on_created(
        &self,
        callback: impl Fn(T::Item) -> FruityResult<Option<Box<dyn Fn() + Send + Sync>>>
            + Send
            + Sync
            + 'static,
    ) -> ObserverHandler<EntityReference> {
        let on_entity_deleted = self.on_entity_deleted.clone();
        self.on_entity_created
            .add_observer(move |entity_reference| {
                let entity_id = entity_reference.get_entity_id()?;
                let mut iterator = T::from_entity_reference(&entity_reference);

                iterator.try_for_each(|item| {
                    let dispose_callback = callback(item)?;

                    if let Some(dispose_callback) = dispose_callback {
                        on_entity_deleted.add_self_dispose_observer(
                            move |signal_entity_id, handler| {
                                if entity_id == *signal_entity_id {
                                    dispose_callback();
                                    handler.dispose_by_ref();
                                }

                                Ok(())
                            },
                        )
                    }

                    Ok(())
                })
            })
    }
}

impl<'a, T: QueryParam<'a> + 'static> Injectable for Query<T> {
    type StoredType = Query<T>;

    fn from_resource_container(resource_container: &ResourceContainer) -> Self {
        let entity_service = resource_container.require::<EntityService>();
        let entity_service = entity_service.read();

        entity_service.query::<T>()
    }

    fn finalize(stored: &Self::StoredType) -> Self {
        stored.clone()
    }
}
