use super::entity::SerializedEntity;
use crate::component::component::AnyComponent;
use crate::entity::archetype::Archetype;
use crate::entity::archetype::ArchetypeArcRwLock;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_query::script::ScriptQuery;
use crate::entity::entity_query::Query;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_reference::EntityReference;
use crate::ExtensionComponentService;
use crate::ResourceContainer;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::Mutex;
use fruity_game_engine::RwLock;
use fruity_game_engine::{export, export_impl, export_struct};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

/// A save for the entities stored in an [’EntityService’]
pub type EntityServiceSnapshot = Vec<SerializedEntity>;

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(FruityAny, Resource)]
#[export_struct]
pub struct EntityService {
    id_incrementer: Mutex<u64>,
    index_map: RwLock<HashMap<EntityId, (usize, usize)>>,
    archetypes: Arc<RwLock<Vec<ArchetypeArcRwLock>>>,
    object_factory_service: ResourceReference<ObjectFactoryService>,
    extension_component_service: ResourceReference<ExtensionComponentService>,

    /// Signal notified when an entity is created
    pub on_created: Signal<EntityReference>,

    /// Signal notified when an entity is deleted
    pub on_deleted: Signal<EntityId>,
}

#[export_impl]
impl EntityService {
    /// Returns an EntityService
    pub fn new(resource_container: ResourceContainer) -> EntityService {
        EntityService {
            id_incrementer: Mutex::new(0),
            index_map: RwLock::new(HashMap::new()),
            archetypes: Arc::new(RwLock::new(Vec::new())),
            object_factory_service: resource_container.require::<ObjectFactoryService>(),
            extension_component_service: resource_container.require::<ExtensionComponentService>(),
            on_created: Signal::new(),
            on_deleted: Signal::new(),
        }
    }

    /// Get an entity specific components
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_identifier` - The component identifiers
    ///
    #[export]
    pub fn get_entity(&self, entity_id: EntityId) -> Option<EntityReference> {
        let index_map = self.index_map.read();
        index_map
            .get(&entity_id)
            .map(|(archetype_index, entity_id)| {
                let archetypes = self.archetypes.read();
                archetypes[*archetype_index].clone().get(*entity_id)
            })
    }

    /// Iterate over all entities
    pub fn iter_all_entities(&self) -> impl Iterator<Item = EntityReference> + '_ {
        let archetypes = self.archetypes.read();
        let archetypes = unsafe {
            std::mem::transmute::<&Vec<ArchetypeArcRwLock>, &Vec<ArchetypeArcRwLock>>(&archetypes)
        };

        archetypes
            .iter()
            .map(|archetype| archetype.iter(true))
            .flatten()
    }

    /// Create a query over entities
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    /// * `callback` - The closure to execute
    ///
    pub fn query<'a, T: QueryParam<'a> + 'static>(&self) -> Query<T> {
        Query::<T> {
            archetypes: self.archetypes.clone(),
            on_entity_created: self.on_created.clone(),
            on_entity_deleted: self.on_deleted.clone(),
            _param_phantom: PhantomData {},
        }
    }

    /// Create a query over entities
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    /// * `callback` - The closure to execute
    ///
    #[export(name = "query")]
    pub fn script_query(&self) -> ScriptQuery {
        ScriptQuery {
            archetypes: self.archetypes.clone(),
            on_entity_created: self.on_created.clone(),
            on_entity_deleted: self.on_deleted.clone(),
            params: vec![],
        }
    }

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
    pub fn create(
        &self,
        name: String,
        enabled: bool,
        components: Vec<AnyComponent>,
    ) -> FruityResult<EntityId> {
        let entity_id = {
            let mut id_incrementer = self.id_incrementer.lock();
            *id_incrementer += 1;
            *id_incrementer
        };

        self.create_with_id(entity_id, name, enabled, components)
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `name` - The name of the entity
    /// * `enabled` - Is the entity active
    /// * `components` - The components that will be added
    ///
    #[export]
    pub fn create_with_id(
        &self,
        entity_id: EntityId,
        name: String,
        enabled: bool,
        mut components: Vec<AnyComponent>,
    ) -> FruityResult<EntityId> {
        // Generate an id for the entity
        let entity_id = {
            let mut id_incrementer = self.id_incrementer.lock();
            *id_incrementer = u64::max(entity_id + 1, *id_incrementer);
            entity_id
        };

        // Generate the archetype identifier from the components
        components.sort_by(|a, b| {
            a.get_class_name()
                .unwrap()
                .cmp(&b.get_class_name().unwrap())
        });
        let archetype_identifier = get_type_identifier_by_any(&components)?;

        // Insert the entity into the archetype, create the archetype if needed
        let indexes = match self.archetype_by_identifier(archetype_identifier) {
            Some((archetype_index, archetype)) => {
                let archetype_entity_id = archetype.read().len();
                archetype
                    .write()
                    .add(entity_id, &name, enabled, components)?;

                (archetype_index, archetype_entity_id)
            }
            None => {
                let mut archetypes = self.archetypes.write();
                let archetype_index = archetypes.len();
                let archetype = Archetype::new(
                    self.extension_component_service.clone(),
                    entity_id,
                    &name,
                    enabled,
                    components,
                )?;

                archetypes.push(ArchetypeArcRwLock::new(archetype));
                (archetype_index, 0)
            }
        };

        // Store the entity storage position
        {
            let mut index_map = self.index_map.write();
            index_map.insert(entity_id, indexes);
        }

        // Notify that entity is created
        let entity_reference = EntityReference {
            entity_id: indexes.1,
            archetype: {
                let archetypes = self.archetypes.read();
                archetypes.get(indexes.0).unwrap().clone()
            },
        };

        self.on_created.notify(entity_reference)?;

        Ok(entity_id)
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    #[export]
    pub fn remove(&self, entity_id: EntityId) -> FruityResult<()> {
        let indexes = {
            let mut index_map = self.index_map.write();
            index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            // Delete the entity
            {
                let archetypes = self.archetypes.read();
                let archetype = archetypes.get(indexes.0).unwrap();
                archetype.read().remove(indexes.1);
            }

            // Propagate the deleted signal
            self.on_deleted.notify(entity_id)?;

            Ok(())
        } else {
            Err(FruityError::GenericFailure(format!(
                "Entity with the id {} not found",
                entity_id
            )))
        }
    }

    /// Add components to an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_index` - The component index, is based on alphabetical number of the component type name
    ///
    #[export]
    pub fn add_component(
        &self,
        entity_id: EntityId,
        mut components: Vec<AnyComponent>,
    ) -> FruityResult<()> {
        let indexes = {
            let mut index_map = self.index_map.write();
            index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            let (old_entity, mut old_components) = {
                let archetypes = self.archetypes.read();
                let archetypes = unsafe {
                    std::mem::transmute::<&Vec<ArchetypeArcRwLock>, &Vec<ArchetypeArcRwLock>>(
                        &archetypes,
                    )
                };

                let archetype = archetypes.get(indexes.0).unwrap();
                archetype.write().remove(indexes.1)
            };

            old_components.append(&mut components);

            self.create_with_id(
                entity_id,
                old_entity.name,
                old_entity.enabled,
                old_components,
            )?;

            Ok(())
        } else {
            Err(FruityError::GenericFailure(format!(
                "Entity with the id {} not found",
                entity_id
            )))
        }
    }

    /// Remove a component from an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_index` - The component index, is based on alphabetical number of the component type name
    ///
    #[export]
    pub fn remove_component(
        &self,
        entity_id: EntityId,
        component_index: usize,
    ) -> FruityResult<()> {
        let indexes = {
            let mut index_map = self.index_map.write();
            index_map.remove(&entity_id)
        };

        if let Some(indexes) = indexes {
            let (old_entity, mut old_components) = {
                let archetypes = self.archetypes.read();
                let archetypes = unsafe {
                    std::mem::transmute::<&Vec<ArchetypeArcRwLock>, &Vec<ArchetypeArcRwLock>>(
                        &archetypes,
                    )
                };

                let archetype = archetypes.get(indexes.0).unwrap();
                archetype.write().remove(indexes.1)
            };

            old_components.remove(component_index);

            self.create_with_id(
                entity_id,
                old_entity.name,
                old_entity.enabled,
                old_components,
            )?;

            Ok(())
        } else {
            Err(FruityError::GenericFailure(format!(
                "Entity with the id {} not found",
                entity_id
            )))
        }
    }

    fn archetype_by_identifier(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<(usize, &ArchetypeArcRwLock)> {
        let archetypes = self.archetypes.read();
        let archetypes = unsafe {
            std::mem::transmute::<&Vec<ArchetypeArcRwLock>, &Vec<ArchetypeArcRwLock>>(&archetypes)
        };

        archetypes.iter().enumerate().find(|(_index, archetype)| {
            *archetype.read().get_type_identifier() == entity_identifier
        })
    }

    /// Clear all the entities
    #[export]
    pub fn clear(&self) -> FruityResult<()> {
        // Raise all entity deleted events
        let entity_ids = {
            let index_map = self.index_map.read();
            index_map
                .iter()
                .map(|(entity_id, _)| *entity_id)
                .collect::<Vec<_>>()
        };

        entity_ids
            .into_iter()
            .try_for_each(|entity_id| self.on_deleted.notify(entity_id))?;

        // Get the writers
        let mut index_map = self.index_map.write();
        let mut id_incrementer = self.id_incrementer.lock();
        let mut archetypes = self.archetypes.write();

        // Clear all entities
        index_map.clear();
        *id_incrementer = 0;
        archetypes.clear();

        Ok(())
    }

    /// Create a snapshot over all the entities
    #[export]
    pub fn snapshot(&self) -> FruityResult<EntityServiceSnapshot> {
        self.iter_all_entities()
            .map(|entity| {
                let entity = entity.read();
                let components = entity
                    .read_all_components()
                    .into_iter()
                    .map(|component| {
                        let script_value = AnyComponent::from_box(component.deref().duplicate())
                            .serialize()
                            .map(|component| component.into_script_value())?;

                        script_value
                    })
                    .try_collect::<Vec<_>>()?;

                Ok(SerializedEntity {
                    entity_id: entity.get_entity_id(),
                    name: entity.get_name(),
                    enabled: entity.is_enabled(),
                    components,
                })
            })
            .try_collect::<Vec<_>>()
    }

    /// Restore an entity snapshot
    ///
    /// # Arguments
    /// * `snapshot` - The snapshot
    ///
    #[export]
    pub fn restore(&self, snapshot: EntityServiceSnapshot) -> FruityResult<()> {
        self.clear()?;

        snapshot
            .iter()
            .try_for_each(|serialized_entity| self.restore_entity(serialized_entity))
    }

    fn restore_entity(&self, serialized_entity: &SerializedEntity) -> FruityResult<()> {
        let object_factory_service = self.object_factory_service.read();
        self.create_with_id(
            serialized_entity.entity_id,
            serialized_entity.name.clone(),
            serialized_entity.enabled,
            serialized_entity
                .components
                .clone()
                .into_iter()
                .map(|serialized_component| {
                    AnyComponent::deserialize(serialized_component, &object_factory_service)
                })
                .try_collect::<Vec<_>>()?,
        )?;

        Ok(())
    }
}

impl Debug for EntityService {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
