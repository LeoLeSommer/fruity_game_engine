use super::ArchetypeComponentTypes;
use super::ArchetypeId;
use super::EntityId;
use super::EntityStorage;
use super::SerializedEntity;
use crate::component::Component;
use crate::component::ComponentTypeId;
use crate::component::Enabled;
use crate::component::Name;
use crate::entity::archetype::Archetype;
use crate::entity::EntityLocation;
use crate::serialization::{Deserialize, Serialize};
use crate::ExtensionComponentService;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::profile_scope;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::typescript;
use fruity_game_engine::Arc;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::Mutex;
use fruity_game_engine::RwLock;
use fruity_game_engine::{export, export_impl, export_struct};
use sorted_vec::SortedVec;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr::null_mut;
use std::ptr::NonNull;

/// A save for the entities stored in an [’EntityService’]
#[typescript("type EntityServiceSnapshot = SerializedEntity[]")]
pub type EntityServiceSnapshot = Settings;

pub enum EntityServiceEntityLocation {
    /// The entity is in the main entity storage
    Main(EntityLocation),
    /// The entity is in the pending entity storage
    Pending(EntityLocation),
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(FruityAny)]
#[export_struct]
pub struct EntityService {
    id_incrementer: Mutex<u64>,
    entity_storage: EntityStorage,
    pending_entity_storage: Arc<RwLock<EntityStorage>>,
    resource_container: ResourceContainer,
    extension_component_service: ResourceReference<ExtensionComponentService>,

    /// Signal notified when an entity is created
    /* pub on_created: Signal<EntityReference>, */

    /// Signal notified when an entity is deleted
    pub on_deleted: Signal<EntityId>,

    /// Signal notified when an entity archetype or index in archetype is moved
    pub(crate) on_entity_location_moved: Signal<EntityId>,
}

#[export_impl]
impl EntityService {
    /// Returns an EntityService
    pub fn new(resource_container: ResourceContainer) -> EntityService {
        EntityService {
            id_incrementer: Mutex::new(0),
            entity_storage: EntityStorage::new(),
            pending_entity_storage: Arc::new(RwLock::new(EntityStorage::new())),
            resource_container: resource_container.clone(),
            extension_component_service: resource_container.require::<ExtensionComponentService>(),
            // on_created: Signal::new(),
            on_deleted: Signal::new(),
            on_entity_location_moved: Signal::new(),
        }
    }

    /*
    /// Get an entity specific components
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    #[export]
    pub fn get_entity_reference(&self, entity_id: EntityId) -> Option<EntityReference> {}

    /// Iterate over all entities
    pub fn iter_all_entities(&self) -> impl Iterator<Item = EntityReference> + '_ {
        self.archetypes
            .iter()
            .map(|archetype| archetype.iter(true))
            .flatten()
    }

    /// Create a query over entities
    pub fn query<'a, T: QueryParam<'a> + 'static>(&self) -> Query<T> {
        Query::<T>::new(self)
    }

    /// Create a query over entities
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
        // self.on_created.notify(entity_reference);

        Ok(entity_id)
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    #[export]
    pub fn remove_entity(&self, entity_id: EntityId) -> FruityResult<()> {
        if let Some(_) = self.entity_storage.remove_entity(entity_id)? {
            Ok(())
        } else {
            self.pending_entity_storage
                .write()
                .remove_entity(entity_id)
                .map(|_| Ok(()))
                .unwrap_or_else(|_| {
                    Err(FruityError::GenericFailure(
                        format!("Entity with id {:?} does not exist", entity_id).into(),
                    ))
                })
        }?;

        // Notify that the entity has been deleted
        self.on_deleted.notify(entity_id);

        Ok(())
    }

    /// Add components to an entity
    #[export]
    pub fn add_components(
        &self,
        entity_id: EntityId,
        mut new_components: Vec<Box<dyn Component>>,
    ) -> FruityResult<()> {
        let mut components =
            self.entity_storage
                .remove_entity(entity_id)?
                .ok_or(FruityError::GenericFailure(format!(
                    "Entity with id {:?} does not exist",
                    entity_id
                )))?;

        components.append(&mut new_components);

        self.pending_entity_storage
            .write()
            .create_entity(entity_id, components, None, None)
    }

    /// Remove a component from an entity
    /// TODO: Not easy to use, use component type instead of index
    #[export]
    pub fn remove_component(
        &self,
        entity_id: EntityId,
        component_index: usize,
    ) -> FruityResult<()> {
        let components =
            self.entity_storage
                .remove_entity(entity_id)?
                .ok_or(FruityError::GenericFailure(format!(
                    "Entity with id {:?} does not exist",
                    entity_id
                )))?;

        let mut new_components = components
            .into_iter()
            .enumerate()
            .filter(|(index, component)| *index != component_index)
            .map(|(_, component)| component)
            .collect::<Vec<_>>();

        self.pending_entity_storage
            .write()
            .create_entity(entity_id, new_components, None, None)
    }

    /// Clear all the entities
    #[export]
    pub fn clear(&self) -> FruityResult<()> {
        // Notify that the entity has been deleted
        self.entity_storage.iter_ids().for_each(|entity_id| {
            self.on_deleted.notify(entity_id);
        });

        self.pending_entity_storage
            .read()
            .iter_ids()
            .for_each(|entity_id| {
                self.on_deleted.notify(entity_id);
            });

        *self.id_incrementer.lock() = 0;
        self.entity_storage.clear()?;
        self.pending_entity_storage.write().clear()
    }

    /// Create a snapshot over all the entities
    #[export]
    pub fn snapshot(&self) -> FruityResult<EntityServiceSnapshot> {
        self.entity_storage
            .iter()
            .map(|(entity_id, components)| {
                let name = components
                    .filter(|component| {
                        component.get_component_type_id().unwrap() == ComponentTypeId::of::<Name>()
                    })
                    .next()
                    .unwrap()
                    .as_any_ref()
                    .downcast_ref::<Name>()
                    .unwrap()
                    .0
                    .clone();

                let enabled = components
                    .filter(|component| {
                        component.get_component_type_id().unwrap()
                            == ComponentTypeId::of::<Enabled>()
                    })
                    .next()
                    .unwrap()
                    .as_any_ref()
                    .downcast_ref::<Enabled>()
                    .unwrap()
                    .0;

                let serialized_components = components
                    .filter(|component| {
                        component.get_component_type_id().unwrap() != ComponentTypeId::of::<Name>()
                            && component.get_component_type_id().unwrap()
                                != ComponentTypeId::of::<Enabled>()
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

    pub fn apply_pending_mutations(&mut self) -> FruityResult<()> {
        profile_scope!("apply_pending_mutations");

        self.entity_storage
            .append(&mut self.pending_entity_storage.write())
    }
}

impl Debug for EntityService {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
