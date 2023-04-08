use super::{EntityId, EntityLocation, EntityStorage, SerializedEntity};
use crate::{
    component::{Component, ComponentTypeId, Enabled, ExtensionComponentService, Name},
    entity::EntityReference,
    query::{Query, QueryParam},
    serialization::{Deserialize, Serialize},
};
use fruity_game_engine::{
    any::FruityAny,
    export, export_impl, export_struct, profile_scope,
    resource::{resource_container::ResourceContainer, resource_reference::ResourceReference},
    settings::Settings,
    signal::Signal,
    typescript, Arc, FruityError, FruityResult, Mutex, RwLock,
};
use std::{collections::HashMap, fmt::Debug, ops::Deref};

/// A save for the entities stored in an [’EntityService’]
#[typescript("type EntityServiceSnapshot = SerializedEntity[]")]
pub type EntityServiceSnapshot = Settings;

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(FruityAny)]
#[export_struct]
pub struct EntityService {
    id_incrementer: Mutex<u64>,
    pub(crate) entity_storage: Arc<RwLock<EntityStorage>>,
    pending_entity_storage: Arc<RwLock<EntityStorage>>,
    pending_entity_to_remove: Arc<RwLock<Vec<EntityId>>>,
    resource_container: ResourceContainer,
    extension_component_service: ResourceReference<ExtensionComponentService>,

    /// Signal notified when an entity is created
    pub on_created: Signal<EntityReference>,

    /// Signal notified when an entity is deleted
    pub on_deleted: Signal<EntityId>,

    /// Signal notified when an entity archetype or index in archetype is moved
    pub(crate) on_entity_location_moved:
        Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
}

#[export_impl]
impl EntityService {
    /// Returns an EntityService
    pub fn new(resource_container: ResourceContainer) -> EntityService {
        EntityService {
            id_incrementer: Mutex::new(0),
            entity_storage: Arc::new(RwLock::new(EntityStorage::new())),
            pending_entity_storage: Arc::new(RwLock::new(EntityStorage::new())),
            pending_entity_to_remove: Arc::new(RwLock::new(Vec::new())),
            resource_container: resource_container.clone(),
            extension_component_service: resource_container.require::<ExtensionComponentService>(),
            on_created: Signal::new(),
            on_deleted: Signal::new(),
            on_entity_location_moved: Signal::new(),
        }
    }

    /// Get an entity specific components
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    #[export]
    pub fn get_entity_reference(&self, entity_id: EntityId) -> Option<EntityReference> {
        let (entity_storage, location) =
            if let Some(location) = self.entity_storage.read().get_entity_location(entity_id) {
                (self.entity_storage.clone(), location)
            } else if let Some(location) = self
                .pending_entity_storage
                .read()
                .get_entity_location(entity_id)
            {
                (self.pending_entity_storage.clone(), location)
            } else {
                return None;
            };

        Some(EntityReference::new(
            entity_storage,
            entity_id,
            location,
            self.on_entity_location_moved.clone(),
        ))
    }

    /// Create a query over entities
    pub fn query<'a, T: QueryParam<'a> + 'static>(&self) -> Query<T> {
        Query::<T>::new(self)
    }

    /*/// Create a query over entities
    #[export(name = "query")]
    pub fn script_query(&self) -> ScriptQueryBuilder {
        ScriptQueryBuilder::new(self.resource_container.require())
    }*/

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `name` - The name of the entity
    /// * `enabled` - Is the entity active
    /// * `components` - The components that will be added
    ///
    #[export]
    pub fn create_entity(
        &self,
        name: String,
        enabled: bool,
        components: Vec<Box<dyn Component>>,
    ) -> FruityResult<EntityId> {
        // Generate an id for the entity
        let entity_id = EntityId({
            let mut id_incrementer = self.id_incrementer.lock();
            *id_incrementer += 1;
            *id_incrementer
        });

        // Add name and enabled as default components
        let default_components = vec![
            Box::new(Name::new(name)) as Box<dyn Component>,
            Box::new(Enabled::new(enabled)) as Box<dyn Component>,
        ];

        // Add the entity to the pending entity storage
        let extension_component_service_reader = self.extension_component_service.read();
        let mut pending_entity_storage_writer = self.pending_entity_storage.write();
        pending_entity_storage_writer.create_entity(
            entity_id,
            components,
            Some(extension_component_service_reader.deref()),
            Some(default_components),
        )?;

        // Notify that the entity has been created
        let entity_reference = self.get_entity_reference(entity_id).unwrap();
        self.on_created.send(entity_reference)?;

        Ok(entity_id)
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    #[export]
    pub fn remove_entity(&self, entity_id: EntityId) -> FruityResult<Vec<Box<dyn Component>>> {
        if let Some(entity_components) = self.entity_storage.read().get_entity_components(entity_id)
        {
            // Add the entity to the pending entities to remove
            self.pending_entity_to_remove.write().push(entity_id);

            Ok(entity_components)
        } else {
            Err(FruityError::GenericFailure(
                format!("Entity with id {:?} does not exist", entity_id).into(),
            ))
        }
    }

    /// Add components to an entity
    #[export]
    pub fn add_components(
        &self,
        entity_id: EntityId,
        mut new_components: Vec<Box<dyn Component>>,
    ) -> FruityResult<()> {
        let mut components = self.remove_entity(entity_id)?;

        components.append(&mut new_components);

        let location = self
            .pending_entity_storage
            .write()
            .create_entity(entity_id, components, None, None)?;

        self.on_entity_location_moved.send((
            entity_id,
            self.pending_entity_storage.clone(),
            location,
        ))
    }

    /// Remove a component from an entity
    /// TODO: Not easy to use, use component type instead of index
    #[export]
    pub fn remove_component(
        &self,
        entity_id: EntityId,
        component_index: usize,
    ) -> FruityResult<()> {
        let components = self.remove_entity(entity_id)?;

        let new_components = components
            .into_iter()
            .enumerate()
            .filter(|(index, _)| *index != component_index)
            .map(|(_, component)| component)
            .collect::<Vec<_>>();

        let location = self.pending_entity_storage.write().create_entity(
            entity_id,
            new_components,
            None,
            None,
        )?;

        self.on_entity_location_moved.send((
            entity_id,
            self.pending_entity_storage.clone(),
            location,
        ))
    }

    /// Clear all the entities
    #[export]
    pub fn clear(&self) -> FruityResult<()> {
        // Notify that the entity has been deleted
        self.entity_storage
            .read()
            .iter_ids()
            .try_for_each(|entity_id| self.on_deleted.send(entity_id))?;

        self.pending_entity_storage
            .read()
            .iter_ids()
            .try_for_each(|entity_id| self.on_deleted.send(entity_id))?;

        *self.id_incrementer.lock() = 0;
        self.entity_storage.write().clear()?;
        self.pending_entity_storage.write().clear()
    }

    /// Create a snapshot over all the entities
    #[export]
    pub fn snapshot(&self) -> FruityResult<EntityServiceSnapshot> {
        self.entity_storage
            .read()
            .iter()
            .map(|(entity_id, components)| {
                let mut name = String::new();
                let mut enabled = false;

                let serialized_components = components
                    .filter(|component| {
                        if component.get_component_type_id().unwrap()
                            == ComponentTypeId::of::<Name>()
                        {
                            name = component
                                .as_any_ref()
                                .downcast_ref::<Name>()
                                .unwrap()
                                .0
                                .clone();

                            false
                        } else if component.get_component_type_id().unwrap()
                            == ComponentTypeId::of::<Enabled>()
                        {
                            enabled = component
                                .as_any_ref()
                                .downcast_ref::<Enabled>()
                                .unwrap()
                                .0
                                .clone();

                            false
                        } else {
                            true
                        }
                    })
                    .map(|component| {
                        component
                            .deref()
                            .duplicate()
                            .serialize(&self.resource_container)
                    })
                    .try_collect::<Vec<_>>()?;

                Ok(SerializedEntity {
                    local_id: entity_id.0,
                    name,
                    enabled,
                    components: serialized_components,
                })
            })
            .try_collect::<Vec<_>>()?
            .serialize(&self.resource_container)
    }

    /// Restore an entity snapshot
    ///
    /// # Arguments
    /// * `snapshot` - The snapshot
    ///
    #[export]
    pub fn restore(&self, clear_before: bool, snapshot: EntityServiceSnapshot) -> FruityResult<()> {
        if clear_before {
            self.clear()?;
        }

        let mut local_id_to_entity_id = HashMap::<u64, EntityId>::new();
        <Vec<SerializedEntity>>::deserialize(
            &snapshot,
            &self.resource_container,
            &local_id_to_entity_id,
        )?
        .iter()
        .try_for_each(|serialized_entity| {
            self.restore_entity(serialized_entity, &mut local_id_to_entity_id)
        })
    }

    fn restore_entity(
        &self,
        serialized_entity: &SerializedEntity,
        local_id_to_entity_id: &mut HashMap<u64, EntityId>,
    ) -> FruityResult<()> {
        let entity_id = self.create_entity(
            serialized_entity.name.clone(),
            serialized_entity.enabled,
            serialized_entity
                .components
                .iter()
                .map(|serialized_component| {
                    <Box<dyn Component>>::deserialize(
                        serialized_component,
                        &self.resource_container,
                        local_id_to_entity_id,
                    )
                })
                .try_collect()?,
        )?;

        local_id_to_entity_id.insert(serialized_entity.local_id, entity_id);

        Ok(())
    }

    /// Apply all the pending mutations, create an entity, add components to an entity, remove components to an entity or delete an entity
    pub unsafe fn apply_pending_mutations(&self) -> FruityResult<()> {
        profile_scope!("apply_pending_mutations");

        let new_locations = self
            .entity_storage
            .write()
            .append(&mut self.pending_entity_storage.write())?;

        new_locations
            .into_iter()
            .try_for_each(|(entity_id, location)| {
                self.on_entity_location_moved.send((
                    entity_id,
                    self.entity_storage.clone(),
                    location,
                ))
            })?;

        self.pending_entity_to_remove
            .write()
            .drain(..)
            .try_for_each(|entity_id| {
                self.entity_storage.write().remove_entity(entity_id)?;
                self.on_deleted.send(entity_id)?;

                Ok(())
            })?;

        Ok(())
    }
}

impl Debug for EntityService {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
