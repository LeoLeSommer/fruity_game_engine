use super::{ArchetypePtr, EntityIterator, InnerEntityStorageQuery};
use crate::entity::{Archetype, ArchetypeComponentTypes, EntityId, EntityReference, EntityStorage};
use fruity_game_engine::{
    any::FruityAny,
    export, export_impl, export_struct,
    script_value::ScriptValue,
    signal::{ObserverHandler, Signal},
    sync::{Arc, RwLock},
    FruityError, FruityResult,
};
use sorted_vec::SortedVec;
use std::{fmt::Debug, ptr::NonNull};

mod builder;
pub use builder::*;

mod params;
pub use params::*;

/// A trait that should be implement for everything that can be queried from ['EntityService']
pub trait ScriptQueryParam: Send + Sync {
    /// A filter over the archetypes
    fn filter_archetype(&self, component_types: &ArchetypeComponentTypes) -> bool;

    /// Iter over the queried components into a given archetype
    /// The iterator should not lock the entity guard, the query will take care of it
    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a>;

    /// Iter over the queried components into a given entity
    /// It should not lock the entity guard, the query will take care of it
    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a>;

    /// Duplicate the query param
    fn duplicate(&self) -> Box<dyn ScriptQueryParam>;
}

/// A query over entities
#[derive(FruityAny)]
#[export_struct(typescript = "class ScriptQuery<Args extends any[] = []> {
  forEach(callback: (args: Args) => void);
  onCreated(callback: (args: Args) => undefined | (() => void)): ObserverHandler;
}")]
pub struct ScriptQuery {
    inner: Arc<RwLock<InnerEntityStorageQuery>>,
    on_created: Signal<EntityReference>,
    on_deleted: Signal<EntityId>,
    params: Box<dyn ScriptQueryParam>,
}

#[export_impl]
impl ScriptQuery {
    /// Create the entity query
    pub fn new(
        params: Box<dyn ScriptQueryParam>,
        entity_storage: &EntityStorage,
        on_created: Signal<EntityReference>,
        on_deleted: Signal<EntityId>,
    ) -> Self {
        let params_2 = params.duplicate();

        // Filter existing archetypes
        let inner = Arc::new(RwLock::new(InnerEntityStorageQuery {
            archetypes: SortedVec::from(
                entity_storage
                    .archetypes
                    .iter()
                    .filter(move |archetype| {
                        params_2.filter_archetype(archetype.get_component_types())
                    })
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

        Self::generate_storage_observers(params.duplicate(), &inner, entity_storage);

        // Returns the query
        Self {
            inner,
            on_created: on_created.clone(),
            on_deleted: on_deleted.clone(),
            params,
        }
    }

    /// Call a function for every entities of an query
    #[export]
    pub fn for_each(
        &self,
        callback: Arc<dyn Send + Sync + Fn(ScriptValue) -> FruityResult<ScriptValue>>,
    ) -> FruityResult<()> {
        let inner_reader = self.inner.read();
        let mut iterator = inner_reader.archetypes.iter();

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
        callback: Box<
            dyn Send
                + Sync
                + Fn(
                    ScriptValue,
                )
                    -> FruityResult<Option<Box<dyn Send + Sync + Fn() -> FruityResult<()>>>>,
        >,
    ) -> ObserverHandler<EntityReference> {
        let params = self.params.duplicate();
        let on_deleted = self.on_deleted.clone();
        self.on_created.add_observer(move |entity_reference| {
            if let Some(entity_reference_inner) = entity_reference.inner.read().as_ref() {
                let matches_query = {
                    let entity_storage_inner = entity_reference_inner.entity_storage.read();
                    let archetype = &entity_storage_inner.archetypes
                        [entity_reference_inner.location.archetype_index];

                    params.filter_archetype(archetype.get_component_types())
                };

                if matches_query {
                    let entity_id = entity_reference.get_entity_id()?;
                    let mut iterator = params.from_entity_reference(&entity_reference);

                    iterator.try_for_each(|item| {
                        let dispose_callback = callback(item)?;

                        if let Some(dispose_callback) = dispose_callback {
                            on_deleted.add_self_dispose_observer(
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
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        })
    }

    pub(crate) fn generate_storage_observers(
        params: Box<dyn ScriptQueryParam>,
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
                    if params
                        .filter_archetype(unsafe { archetype_ptr.as_ref().get_component_types() })
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

impl Debug for ScriptQuery {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
