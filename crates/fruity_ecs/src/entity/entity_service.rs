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
use fruity_game_engine::export;
use fruity_game_engine::fruity_export;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::Mutex;
use fruity_game_engine::RwLock;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

/// A save for the entities stored in an [’EntityService’]
pub type EntityServiceSnapshot = ScriptValue;

fruity_export! {
    /// A storage for every entities, use [’Archetypes’] to store entities of different types
    #[derive(FruityAny, Resource)]
    pub struct EntityService {
        id_incrementer: Mutex<u64>,
        index_map: RwLock<HashMap<EntityId, (usize, usize)>>,
        archetypes: Arc<RwLock<Vec<ArchetypeArcRwLock>>>,
        extension_component_service: ResourceReference<ExtensionComponentService>,

        /// Signal notified when an entity is created
        pub on_created: Signal<EntityReference>,

        /// Signal notified when an entity is deleted
        pub on_deleted: Signal<EntityId>,
    }

    impl EntityService {
        /// Returns an EntityService
        pub fn new(resource_container: ResourceContainer) -> EntityService {
            EntityService {
                id_incrementer: Mutex::new(0),
                index_map: RwLock::new(HashMap::new()),
                archetypes: Arc::new(RwLock::new(Vec::new())),
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
        pub fn create(&self, name: String, enabled: bool, components: Vec<AnyComponent>) -> FruityResult<EntityId> {
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
            components.sort_by(|a, b| a.get_class_name().unwrap().cmp(&b.get_class_name().unwrap()));
            let archetype_identifier = get_type_identifier_by_any(&components)?;

            // Insert the entity into the archetype, create the archetype if needed
            let indexes = match self.archetype_by_identifier(archetype_identifier) {
                Some((archetype_index, archetype)) => {
                    let archetype_entity_id = archetype.read().len();
                    archetype.write().add(entity_id, &name, enabled, components)?;

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
                Err(FruityError::GenericFailure(
                    format!("Entity with the id {} not found", entity_id)
                ))
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
                Err(FruityError::GenericFailure(
                    format!("Entity with the id {} not found", entity_id)
                ))
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
                Err(FruityError::GenericFailure(
                    format!("Entity with the id {} not found", entity_id)
                ))
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

        // TODO: Reimplement the save system in a proper way
        /*
        /// Create a snapshot over all the entities
        #[export]
        pub fn snapshot(&self) -> EntityServiceSnapshot {
            let serialized_entities = self
                .iter_all_entities()
                .filter_map(|entity| {
                    let entity = entity.read();
                    let serialized_components = ScriptValue::Array(
                        entity
                            .read_all_components()
                            .into_iter()
                            .filter_map(|component| component.deref().serialize())
                            .collect::<Vec<_>>(),
                    );

                    let serialized_entity = ScriptValue::Object {
                        class_name: "Entity".to_string(),
                        fields: hashmap! {
                            "entity_id".to_string() => ScriptValue::U64(entity.get_entity_id()),
                            "name".to_string() => ScriptValue::String(entity.get_name()),
                            "enabled".to_string() => ScriptValue::Bool(entity.is_enabled()),
                            "components".to_string() => serialized_components,
                        },
                    };

                    Some(serialized_entity)
                })
                .collect::<Vec<_>>();

            ScriptValue::Array(serialized_entities)
        }

        /// Restore an entity snapshot from a file
        ///
        /// # Arguments
        /// * `filepath` - The file path
        ///
        #[export]
        pub fn restore_from_file(&self, filepath: String) -> FruityResult<ScriptValue> {
            let mut reader = File::open(&filepath).map_err(|_| {
                FruityError::new(
                    FruityStatus::GenericFailure,
                    format!("File couldn't be opened: {:?}", filepath),
                )
            })?;

            deserialize_yaml(&mut reader)
        }

        /// Restore an entity snapshot
        ///
        /// # Arguments
        /// * `snapshot` - The snapshot
        ///
        #[export]
        pub fn restore(&self, snapshot: EntityServiceSnapshot) {
            self.clear();

            if let ScriptValue::Array(entities) = &snapshot {
                entities
                    .iter()
                    .for_each(|serialized_entity| self.restore_entity(serialized_entity));
            }
        }

        fn restore_entity(&self, serialized_entity: &ScriptValue) {
            let object_factory_service = self.object_factory_service.read();

            if let ScriptValue::Object { fields, .. } = serialized_entity {
                let entity_id =
                    if let Ok(entity_id) = EntityId::from_script_value(fields.get("entity_id").unwrap().clone()) {
                        entity_id
                    } else {
                        return;
                    };

                let name = if let Ok(name) = String::from_script_value(fields.get("name").unwrap().clone()) {
                    name
                } else {
                    return;
                };

                let enabled = if let Ok(enabled) = bool::from_script_value(fields.get("enabled").unwrap().clone()) {
                    enabled
                } else {
                    return;
                };

                let components = if let Some(ScriptValue::Array(components)) = fields.get("components")
                {
                    components
                        .iter()
                        .filter_map(|serialized_component| {
                            AnyComponent::deserialize(serialized_component, &object_factory_service)
                        })
                        .collect::<Vec<_>>()
                } else {
                    return;
                };

                self.create_with_id(entity_id, name, enabled, components);
            }
        } */
    }
}

impl Debug for EntityService {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
