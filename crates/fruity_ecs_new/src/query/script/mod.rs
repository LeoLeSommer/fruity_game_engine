use super::ArchetypePtr;
use super::BidirectionalIterator;
use crate::entity::archetype::Archetype;
use crate::entity::entity_reference::EntityReference;
use crate::entity::entity_service::EntityService;
use crate::entity::entity_service::OnArchetypeAddressMoved;
use crate::entity::entity_service::OnEntityAddressAdded;
use crate::entity::entity_service::OnEntityAddressRemoved;
use crate::entity::EntityId;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::Arc;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use fruity_game_engine::{export, export_impl, export_struct};
use sorted_vec::SortedVec;
use std::fmt::Debug;
use std::ptr::NonNull;

pub(crate) mod builder;

pub(crate) mod params;

/// A trait that should be implement for everything that can be queried from ['EntityService']
pub trait ScriptQueryParam: FruityAny + Send + Sync {
    /// Create a new query param that is a clone of self
    fn duplicate(&self) -> Box<dyn ScriptQueryParam>;

    /// A filter over the archetypes
    fn filter_archetype(&self, archetype: &Archetype) -> bool;

    /// How many item are iterated for a given entity
    fn items_per_entity(&self, archetype: &Archetype) -> usize;

    /// Iter over the queried components into a given archetype
    /// The iterator should not lock the entity guard, the query will take care of it
    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a>;

    /// Iter over the queried components into a given entity
    /// It should not lock the entity guard, the query will take care of it
    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a>;
}

/// A query over entities
#[derive(FruityAny)]
#[export_struct(typescript = "interface ScriptQuery<Args extends any[] = []> {
  onEntityCreated: Signal<EntityReference>;
  onEntityDeleted: Signal<EntityId>;
  forEach(callback: (args: Args) => void);
  onCreated(callback: (args: Args) => undefined | (() => void)): ObserverHandler;
}")]
pub struct ScriptQuery {
    /// A signal raised when an entity that match the query is created
    pub on_entity_created: Signal<EntityReference>,

    /// A signal raised when an entity that match the query is deleted
    pub on_entity_deleted: Signal<EntityId>,

    archetypes: Arc<RwLock<SortedVec<ArchetypePtr>>>,
    on_archetype_address_added_handle: ObserverHandler<NonNull<Archetype>>,
    on_archetype_address_moved_handle: ObserverHandler<OnArchetypeAddressMoved>,
    on_entity_address_added_handle: ObserverHandler<OnEntityAddressAdded>,
    on_entity_address_removed_handle: ObserverHandler<OnEntityAddressRemoved>,
    params: Box<dyn ScriptQueryParam>,
}

impl Drop for ScriptQuery {
    fn drop(&mut self) {
        self.on_archetype_address_added_handle.dispose_by_ref();
        self.on_archetype_address_moved_handle.dispose_by_ref();
        self.on_entity_address_added_handle.dispose_by_ref();
        self.on_entity_address_removed_handle.dispose_by_ref();
    }
}

#[export_impl]
impl ScriptQuery {
    /// Create the entity query
    pub fn new(entity_service: &EntityService, params: Box<dyn ScriptQueryParam>) -> Self {
        // Filter existing archetypes
        let archetypes = Arc::new(RwLock::new(SortedVec::from(
            entity_service
                .archetypes
                .iter()
                .map(|archetype| unsafe {
                    ArchetypePtr(NonNull::new_unchecked(
                        archetype as *const Archetype as *mut Archetype,
                    ))
                })
                .collect::<Vec<_>>(),
        )));

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
            let params_2 = params.duplicate();
            let archetypes_2 = archetypes.clone();
            let on_archetype_address_added_handle = entity_service
                .on_archetype_address_added
                .add_observer(move |archetype| {
                    if params_2.filter_archetype(unsafe { archetype.as_ref() }) {
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
                    *archetypes_writer = SortedVec::from(
                        archetypes_writer
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
                            .collect::<Vec<_>>(),
                    );

                    Ok(())
                });

            // Listen the entity create and remove events
            let params_2 = params.duplicate();
            let on_entity_created = Signal::<EntityReference>::new();
            let on_entity_created_2 = on_entity_created.clone();
            let on_entity_address_added_handle = entity_service
                .on_entity_address_added
                .add_observer(move |event| {
                    if params_2.filter_archetype(unsafe { event.archetype.as_ref() }) {
                        on_entity_created_2.notify(event.entity_reference.clone())?;
                    }

                    Ok(())
                });

            let params_2 = params.duplicate();
            let on_entity_deleted = Signal::<EntityId>::new();
            let on_entity_deleted_2 = on_entity_deleted.clone();
            let on_entity_address_removed_handle = entity_service
                .on_entity_address_removed
                .add_observer(move |event| {
                    if params_2.filter_archetype(unsafe { event.archetype.as_ref() }) {
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
            params,
        }
    }

    /// Call a function for every entities of an query
    #[export]
    pub fn for_each(
        &self,
        callback: Arc<dyn Send + Sync + Fn(ScriptValue) -> FruityResult<ScriptValue>>,
    ) -> FruityResult<()> {
        let archetypes_reader = self.archetypes.read();
        let mut iterator = archetypes_reader.iter();

        iterator.try_for_each(|archetype| {
            let archetype = unsafe { archetype.0.as_ref() };
            self.params.iter(archetype).try_for_each(|item| {
                callback(item)?;
                Result::<(), FruityError>::Ok(())
            })
        })
    }

    /// Call a function for every entities of an query
    #[export]
    pub fn on_created(
        &self,
        callback: Arc<
            dyn Send
                + Sync
                + Fn(
                    ScriptValue,
                )
                    -> FruityResult<Option<Arc<dyn Send + Sync + Fn() -> FruityResult<()>>>>,
        >,
    ) -> ObserverHandler<EntityReference> {
        let params = self.params.duplicate();
        let on_entity_deleted = self.on_entity_deleted.clone();
        self.on_entity_created
            .add_observer(move |entity_reference| {
                let entity_id = entity_reference.get_entity_id()?;
                let mut iterator = params.from_entity_reference(&entity_reference);

                iterator.try_for_each(|item| {
                    let dispose_callback = callback(item)?;

                    if let Some(dispose_callback) = dispose_callback {
                        on_entity_deleted.add_self_dispose_observer(
                            move |signal_entity_id, handler| {
                                if entity_id == *signal_entity_id {
                                    dispose_callback()?;
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

impl Clone for ScriptQuery {
    fn clone(&self) -> Self {
        Self {
            on_entity_created: self.on_entity_created.clone(),
            on_entity_deleted: self.on_entity_deleted.clone(),
            archetypes: self.archetypes.clone(),
            on_archetype_address_added_handle: self.on_archetype_address_added_handle.clone(),
            on_archetype_address_moved_handle: self.on_archetype_address_moved_handle.clone(),
            on_entity_address_added_handle: self.on_entity_address_added_handle.clone(),
            on_entity_address_removed_handle: self.on_entity_address_removed_handle.clone(),
            params: self.params.duplicate(),
        }
    }
}

impl Debug for ScriptQuery {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
