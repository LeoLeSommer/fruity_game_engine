use crate::entity::EntityId;
use crate::entity::EntityReference;
use crate::entity::EntityService;
use fruity_game_engine::inject::Injectable;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityResult;

// /// Queries for scripting languages
// pub mod script;

/// Queries for tuples
pub mod tuple;

/// Queries for with stuffs
pub mod with;

/// Queries for without stuffs
pub mod without;

mod entity_storage_query;
pub use entity_storage_query::*;

/// A query over entities
pub struct Query<T> {
    entity_storage_query: EntityStorageQuery<T>,
    on_created: Signal<EntityReference>,
    on_deleted: Signal<EntityId>,
}

impl<T> Clone for Query<T> {
    fn clone(&self) -> Self {
        Self {
            entity_storage_query: self.entity_storage_query.clone(),
            on_created: self.on_created.clone(),
            on_deleted: self.on_deleted.clone(),
        }
    }
}

impl<'a, T: QueryParam<'a> + 'static> Query<T> {
    /// Create the entity query
    pub fn new(entity_service: &EntityService) -> Self {
        let entity_storage_reader = entity_service.entity_storage.read();

        // Filter existing archetypes
        let entity_storage_query = EntityStorageQuery::new(&entity_storage_reader);

        // Returns the query
        Self {
            entity_storage_query,
            on_created: entity_service.on_created.clone(),
            on_deleted: entity_service.on_deleted.clone(),
        }
    }

    /// Call a function for every entities of an query
    pub fn for_each(
        &self,
        callback: impl Fn(T::Item) -> FruityResult<()> + Send + Sync,
    ) -> FruityResult<()> {
        self.entity_storage_query.for_each(callback)
    }

    /// Call a function for every entities of an query
    pub fn on_created(
        &self,
        callback: impl Fn(T::Item) -> FruityResult<Option<Box<dyn Fn() + Send + Sync>>>
            + Send
            + Sync
            + 'static,
    ) -> ObserverHandler<EntityReference> {
        let on_deleted = self.on_deleted.clone();
        self.on_created.add_observer(move |entity_reference| {
            if let Some(entity_reference_inner) = entity_reference.inner.read().as_ref() {
                let matches_query = {
                    let entity_storage_inner = entity_reference_inner.entity_storage.read();
                    let archetype = &entity_storage_inner.archetypes
                        [entity_reference_inner.location.archetype.0];

                    T::filter_archetype(archetype.get_component_types())
                };

                if matches_query {
                    let entity_id = entity_reference.get_entity_id()?;
                    let mut iterator = T::from_entity_reference(&entity_reference);

                    iterator.try_for_each(|item| {
                        let dispose_callback = callback(item)?;

                        if let Some(dispose_callback) = dispose_callback {
                            on_deleted.add_self_dispose_observer(
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
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
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
