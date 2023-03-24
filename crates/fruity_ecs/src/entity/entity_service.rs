use super::entity_query::script::builder::ScriptQueryBuilder;
use super::entity_query::Query;
use super::entity_query::QueryParam;
use super::EntityId;
use super::SerializedEntity;
use crate::component::AnyComponent;
use crate::component::Component;
use crate::deserialize_service::DeserializeService;
use crate::entity::archetype::Archetype;
use crate::entity::entity_reference::EntityReference;
use crate::entity::get_type_identifier_by_any;
use crate::entity::EntityLocation;
use crate::entity::EntityTypeIdentifier;
use crate::ExtensionComponentService;
use crate::ResourceContainer;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::typescript;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::Mutex;
use fruity_game_engine::RwLock;
use fruity_game_engine::{export, export_impl, export_struct};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr::null_mut;
use std::ptr::NonNull;

/// A save for the entities stored in an [’EntityService’]
#[typescript("type EntityServiceSnapshot = SerializedEntity[]")]
pub type EntityServiceSnapshot = Vec<SerializedEntity>;

pub(crate) struct OnEntityAddressAdded {
    pub(crate) entity_reference: EntityReference,
    pub(crate) archetype: NonNull<Archetype>,
}

pub(crate) struct OnEntityAddressRemoved {
    pub(crate) entity_id: EntityId,
    pub(crate) archetype: NonNull<Archetype>,
}

pub(crate) struct OnEntityLocationMoved {
    pub(crate) old_entity_index: usize,
    pub(crate) old_archetype: NonNull<Archetype>,
    pub(crate) new_entity_index: usize,
    /// Can be null ptr if the entity is removed
    pub(crate) new_archetype_ptr: *mut Archetype,
}

pub(crate) struct OnEntityLockAddressMoved {
    pub(crate) old: NonNull<RwLock<()>>,
    /// Can be null ptr if the entity is removed
    pub(crate) new: *mut RwLock<()>,
}

pub(crate) struct OnComponentAddressMoved {
    pub(crate) old: NonNull<dyn Component>,
    /// Can be null ptr if the component is removed
    pub(crate) new: Option<NonNull<dyn Component>>,
}

pub(crate) struct OnArchetypeAddressMoved {
    pub(crate) old: NonNull<Archetype>,
    /// Can be null ptr if the archetype is removed
    pub(crate) new: *mut Archetype,
}

enum Mutation {
    AddEntityMutation {
        entity_id: EntityId,
        name: String,
        enabled: bool,
        components: Vec<AnyComponent>,
    },
    RemoveEntityMutation {
        entity_id: EntityId,
    },
    AddComponentMutation {
        entity_id: EntityId,
        components: Vec<AnyComponent>,
    },
    RemoveComponentMutation {
        entity_id: EntityId,
        component_index: usize,
    },
    ClearMutation,
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(FruityAny)]
#[export_struct]
pub struct EntityService {
    id_incrementer: Mutex<u64>,
    entity_locations: HashMap<EntityId, EntityLocation>,
    pub(crate) archetypes: Vec<Archetype>,
    resource_container: ResourceContainer,
    deserialize_service: ResourceReference<DeserializeService>,
    extension_component_service: ResourceReference<ExtensionComponentService>,
    pending_mutations: Mutex<VecDeque<Mutation>>,

    /// Signal notified when an entity is created
    pub on_created: Signal<EntityReference>,

    /// Signal notified when an entity is deleted
    pub on_deleted: Signal<EntityId>,

    /// Signal notified when an entity ptr address is added
    pub(crate) on_entity_address_added: Signal<OnEntityAddressAdded>,

    /// Signal notified when an entity ptr address is removed
    pub(crate) on_entity_address_removed: Signal<OnEntityAddressRemoved>,

    /// Signal notified when an entity archetype or index in archetype is moved
    pub(crate) on_entity_location_moved: Signal<OnEntityLocationMoved>,

    /// Signal notified when a component ptr address is moved
    pub(crate) on_entity_lock_address_moved: Signal<OnEntityLockAddressMoved>,

    /// Signal notified when a component ptr address is moved
    pub(crate) on_component_address_moved: Signal<OnComponentAddressMoved>,

    /// Signal notified when an archetype ptr address is added
    pub(crate) on_archetype_address_added: Signal<NonNull<Archetype>>,

    /// Signal notified when an archetype ptr address is moved
    pub(crate) on_archetype_address_moved: Signal<OnArchetypeAddressMoved>,
}

#[export_impl]
impl EntityService {
    /// Returns an EntityService
    pub fn new(resource_container: ResourceContainer) -> EntityService {
        EntityService {
            id_incrementer: Mutex::new(0),
            entity_locations: HashMap::new(),
            archetypes: Vec::new(),
            resource_container: resource_container.clone(),
            deserialize_service: resource_container.require::<DeserializeService>(),
            extension_component_service: resource_container.require::<ExtensionComponentService>(),
            on_created: Signal::new(),
            on_deleted: Signal::new(),
            on_entity_address_added: Signal::new(),
            on_entity_address_removed: Signal::new(),
            on_entity_location_moved: Signal::new(),
            on_entity_lock_address_moved: Signal::new(),
            on_archetype_address_added: Signal::new(),
            on_component_address_moved: Signal::new(),
            on_archetype_address_moved: Signal::new(),
            pending_mutations: Mutex::new(VecDeque::new()),
        }
    }

    /// Get an entity specific components
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    #[export]
    pub fn get_entity_reference(&self, entity_id: EntityId) -> Option<EntityReference> {
        self.entity_locations
            .get(&entity_id)
            .map(|entity_location| {
                self.archetypes
                    .get(entity_location.archetype_index)
                    .unwrap()
                    .get_entity_reference(entity_location.entity_index)
            })
    }

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
        // Generate an id for the entity
        let entity_id = EntityId({
            let mut id_incrementer = self.id_incrementer.lock();
            *id_incrementer += 1;
            *id_incrementer
        });

        self.pending_mutations
            .lock()
            .push_back(Mutation::AddEntityMutation {
                entity_id,
                name,
                enabled,
                components,
            });

        Ok(entity_id)
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    #[export]
    pub fn remove(&self, entity_id: EntityId) -> FruityResult<()> {
        // Check if the entity exists
        if !self.entity_locations.contains_key(&entity_id) {
            return Err(FruityError::GenericFailure(format!(
                "Entity with the id {:?} not found",
                entity_id
            )));
        }

        // Register the mutation for the next frame
        self.pending_mutations
            .lock()
            .push_back(Mutation::RemoveEntityMutation { entity_id });

        Ok(())
    }

    /// Add components to an entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `components` - The components that will be added
    ///
    #[export]
    pub fn add_components(
        &self,
        entity_id: EntityId,
        components: Vec<AnyComponent>,
    ) -> FruityResult<()> {
        // Check if the entity exists
        if !self.entity_locations.contains_key(&entity_id) {
            return Err(FruityError::GenericFailure(format!(
                "Entity with the id {:?} not found",
                entity_id
            )));
        }

        // Register the mutation for the next frame
        self.pending_mutations
            .lock()
            .push_back(Mutation::AddComponentMutation {
                entity_id,
                components,
            });

        Ok(())
    }

    /// Remove a component from an entity
    /// TODO: Should be reworked to be more usable, component index is not an intuitive way to describe a component
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_index` - The component index
    ///
    #[export]
    pub fn remove_component(
        &self,
        entity_id: EntityId,
        component_index: usize,
    ) -> FruityResult<()> {
        // Check if the entity exists
        if !self.entity_locations.contains_key(&entity_id) {
            return Err(FruityError::GenericFailure(format!(
                "Entity with the id {:?} not found",
                entity_id
            )));
        }

        // Register the mutation for the next frame
        self.pending_mutations
            .lock()
            .push_back(Mutation::RemoveComponentMutation {
                entity_id,
                component_index,
            });

        Ok(())
    }

    /// Clear all the entities
    #[export]
    pub fn clear(&self) -> FruityResult<()> {
        self.pending_mutations
            .lock()
            .push_back(Mutation::ClearMutation);

        Ok(())
    }

    /// Create a snapshot over all the entities
    #[export]
    pub fn snapshot(&self) -> FruityResult<EntityServiceSnapshot> {
        self.iter_all_entities()
            .map(|entity| {
                let entity = entity.read()?;
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
                    local_id: entity.get_entity_id().0,
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
    pub fn restore(&self, clear_before: bool, snapshot: EntityServiceSnapshot) -> FruityResult<()> {
        if clear_before {
            self.clear()?;
        }

        snapshot
            .iter()
            .try_for_each(|serialized_entity| self.restore_entity(serialized_entity))
    }

    fn restore_entity(&self, serialized_entity: &SerializedEntity) -> FruityResult<()> {
        let deserialize_service = self.deserialize_service.read();
        let mut local_id_to_entity_id = HashMap::<u64, EntityId>::new();

        let entity_id = self.create(
            serialized_entity.name.clone(),
            serialized_entity.enabled,
            serialized_entity
                .components
                .clone()
                .into_iter()
                .map(|serialized_component| {
                    AnyComponent::deserialize(
                        serialized_component,
                        &deserialize_service,
                        &local_id_to_entity_id,
                    )
                })
                .try_collect::<Vec<_>>()?,
        )?;

        local_id_to_entity_id.insert(serialized_entity.local_id, entity_id);

        Ok(())
    }

    /// It is unsafe cause apply mutations is the only operation that mutate the archetypes vec witch is widely
    /// read in an unsafe way everywhere in the ecs code
    pub(crate) unsafe fn apply_pending_mutations(&mut self) -> FruityResult<()> {
        // Apply add entity mutations
        let mutations = {
            let mut mutations = self.pending_mutations.lock();
            mutations.drain(..).collect::<Vec<_>>()
        };

        mutations
            .into_iter()
            .try_for_each(|mutation| match mutation {
                Mutation::AddEntityMutation {
                    entity_id,
                    name,
                    enabled,
                    components,
                } => self
                    .apply_add_entity_mutation(entity_id, name, enabled, components)
                    .map(|_| ()),
                Mutation::RemoveEntityMutation { entity_id } => {
                    self.apply_remove_entity_mutation(entity_id)
                }
                Mutation::AddComponentMutation {
                    entity_id,
                    components,
                } => self.apply_add_component_mutation(entity_id, components),
                Mutation::RemoveComponentMutation {
                    entity_id,
                    component_index,
                } => self.apply_remove_component_mutation(entity_id, component_index),
                Mutation::ClearMutation => self.apply_clear_mutation(),
            })
    }

    unsafe fn apply_add_entity_mutation(
        &mut self,
        entity_id: EntityId,
        name: String,
        enabled: bool,
        mut components: Vec<AnyComponent>,
    ) -> FruityResult<EntityLocation> {
        // Generate the archetype identifier from the components
        components.sort_by(|a, b| {
            a.get_class_name()
                .unwrap()
                .cmp(&b.get_class_name().unwrap())
        });
        let archetype_identifier = get_type_identifier_by_any(&components)?;

        // Insert the entity into the archetype, create the archetype if needed
        let location = match self.archetype_by_identifier_mut(archetype_identifier) {
            Some((archetype_index, archetype)) => {
                let entity_index = archetype.len();
                archetype.add(entity_id, &name, enabled, components)?;

                EntityLocation {
                    archetype_index: archetype_index,
                    entity_index: entity_index,
                }
            }
            None => {
                let archetype_index = self.archetypes.len();
                let archetype = Archetype::new(
                    self,
                    self.extension_component_service.clone(),
                    entity_id,
                    &name,
                    enabled,
                    components,
                )?;

                // Check if a reallocation will be occurred on next archetype insert
                let is_archetypes_about_to_reallocate =
                    self.archetypes.len() + 1 > self.archetypes.capacity();
                let archetypes_old_ptr = self.archetypes.as_ptr();

                self.archetypes.push(archetype);

                // Notify memory operation
                let archetype = &self.archetypes[archetype_index];
                self.on_archetype_address_added.notify(unsafe {
                    NonNull::new_unchecked(archetype as *const Archetype as *mut Archetype)
                })?;

                if is_archetypes_about_to_reallocate {
                    let archetypes_new_ptr = self.archetypes.as_ptr();
                    let addr_diff = archetypes_old_ptr.byte_offset_from(archetypes_new_ptr);

                    self.archetypes.iter().try_for_each(|archetype| {
                        let new = archetype as *const Archetype as *mut Archetype;
                        let old = unsafe { NonNull::new_unchecked(new.byte_offset(addr_diff)) };

                        self.on_archetype_address_moved
                            .notify(OnArchetypeAddressMoved { old, new })
                    })?;
                }

                EntityLocation {
                    archetype_index: archetype_index,
                    entity_index: 0,
                }
            }
        };

        self.entity_locations.insert(entity_id, location.clone());

        // Notify that entity is created
        let entity_reference = self.get_entity_reference(entity_id).unwrap();
        self.on_created.notify(entity_reference.clone())?;

        // Notify memory operation
        let archetype = &self.archetypes[location.archetype_index];
        self.on_entity_address_added.notify(OnEntityAddressAdded {
            entity_reference,
            archetype: unsafe {
                NonNull::new_unchecked(archetype as *const Archetype as *mut Archetype)
            },
        })?;

        Ok(location)
    }

    unsafe fn apply_remove_entity_mutation(&mut self, entity_id: EntityId) -> FruityResult<()> {
        let location =
            self.entity_locations
                .remove(&entity_id)
                .ok_or(FruityError::GenericFailure(format!(
                    "Entity with the id {:?} not found",
                    entity_id
                )))?;

        let archetype = self.archetypes.get_mut(location.archetype_index).ok_or(
            FruityError::GenericFailure(format!("Entity with the id {:?} not found", entity_id)),
        )?;

        archetype.remove(location.entity_index, true)?;

        // Propagate the deleted signal
        self.on_deleted.notify(entity_id)?;

        // Notify memory operation
        let archetype = &self.archetypes[location.archetype_index];
        self.on_entity_address_removed
            .notify(OnEntityAddressRemoved {
                entity_id,
                archetype: unsafe {
                    NonNull::new_unchecked(archetype as *const Archetype as *mut Archetype)
                },
            })?;

        Ok(())
    }

    unsafe fn apply_add_component_mutation(
        &mut self,
        entity_id: EntityId,
        mut components: Vec<AnyComponent>,
    ) -> FruityResult<()> {
        let old_location =
            self.entity_locations
                .remove(&entity_id)
                .ok_or(FruityError::GenericFailure(format!(
                    "Entity with the id {:?} not found",
                    entity_id
                )))?;

        let (old_entity, mut old_components, old_lock_ptr, old_component_storages_ptr) = {
            let archetype = self
                .archetypes
                .get_mut(old_location.archetype_index)
                .ok_or(FruityError::GenericFailure(format!(
                    "Entity with the id {:?} not found",
                    entity_id
                )))?;

            let old_lock_ptr = &archetype.lock_array[old_location.entity_index] as *const RwLock<()>
                as *mut RwLock<()>;

            let old_component_storages_ptr = archetype
                .component_storages
                .iter()
                .map(|(name, component_storage)| {
                    (
                        name.clone(),
                        component_storage.component_storage.get(0) as *const dyn Component
                            as *mut dyn Component,
                    )
                })
                .collect::<HashMap<String, *mut dyn Component>>();

            let (old_entity, old_components) =
                archetype.remove(old_location.entity_index, false)?;

            (
                old_entity,
                old_components,
                old_lock_ptr,
                old_component_storages_ptr,
            )
        };

        old_components.append(&mut components);

        let new_location = self.apply_add_entity_mutation(
            entity_id,
            old_entity.name,
            old_entity.enabled,
            old_components,
        )?;

        // Notify memory operation
        {
            let archetype = &self.archetypes[new_location.archetype_index];
            let new_lock_ptr = &archetype.lock_array[new_location.entity_index] as *const RwLock<()>
                as *mut RwLock<()>;

            self.on_entity_lock_address_moved
                .notify(OnEntityLockAddressMoved {
                    old: unsafe { NonNull::new_unchecked(old_lock_ptr) },
                    new: new_lock_ptr,
                })?;

            let component_storages_diff = archetype
                .component_storages
                .iter()
                .map(|(name, component_storage)| {
                    let old = old_component_storages_ptr.get(name).unwrap();
                    let new = component_storage.component_storage.get(0) as *const dyn Component
                        as *mut dyn Component;

                    (name.clone(), old.byte_offset_from(new))
                })
                .collect::<HashMap<String, isize>>();

            archetype
                .component_storages
                .iter()
                .map(|(name, component_storage)| {
                    component_storage
                        .get(new_location.entity_index)
                        .map(|new_component_storage| {
                            (
                                name.clone(),
                                new_component_storage as *const dyn Component as *mut dyn Component,
                            )
                        })
                })
                .flatten()
                .try_for_each(|(name, new_component_ptr)| {
                    let addr_diff = component_storages_diff.get(&name).unwrap();
                    let old_component_ptr = unsafe {
                        NonNull::new_unchecked(new_component_ptr.byte_offset(*addr_diff))
                    };

                    self.on_component_address_moved
                        .notify(OnComponentAddressMoved {
                            old: old_component_ptr,
                            new: Some(unsafe { NonNull::new_unchecked(new_component_ptr) }),
                        })
                })?;

            self.on_entity_location_moved
                .notify(OnEntityLocationMoved {
                    old_entity_index: old_location.entity_index,
                    old_archetype: unsafe {
                        NonNull::new_unchecked(
                            &self.archetypes[old_location.archetype_index] as *const Archetype
                                as *mut Archetype,
                        )
                    },
                    new_entity_index: new_location.entity_index,
                    new_archetype_ptr: &self.archetypes[new_location.archetype_index]
                        as *const Archetype
                        as *mut Archetype,
                })?;
        }

        Ok(())
    }

    unsafe fn apply_remove_component_mutation(
        &mut self,
        entity_id: EntityId,
        component_index: usize,
    ) -> FruityResult<()> {
        let old_location =
            self.entity_locations
                .remove(&entity_id)
                .ok_or(FruityError::GenericFailure(format!(
                    "Entity with the id {:?} not found",
                    entity_id
                )))?;

        let (old_entity, mut old_components, old_lock_ptr, old_component_storages_ptr) = {
            let archetype = self
                .archetypes
                .get_mut(old_location.archetype_index)
                .ok_or(FruityError::GenericFailure(format!(
                    "Entity with the id {:?} not found",
                    entity_id
                )))?;

            let old_lock_ptr = &archetype.lock_array[old_location.entity_index] as *const RwLock<()>
                as *mut RwLock<()>;

            let old_component_storages_ptr = archetype
                .component_storages
                .iter()
                .map(|(name, component_storage)| {
                    (
                        name.clone(),
                        component_storage.component_storage.get(0) as *const dyn Component
                            as *mut dyn Component,
                    )
                })
                .collect::<HashMap<String, *mut dyn Component>>();

            let (old_entity, old_components) =
                archetype.remove(old_location.entity_index, false)?;

            (
                old_entity,
                old_components,
                old_lock_ptr,
                old_component_storages_ptr,
            )
        };

        old_components.remove(component_index);

        let new_location = self.apply_add_entity_mutation(
            entity_id,
            old_entity.name,
            old_entity.enabled,
            old_components,
        )?;

        // Notify memory operation
        {
            let archetype = &self.archetypes[new_location.archetype_index];
            let new_lock_ptr = &archetype.lock_array[new_location.entity_index] as *const RwLock<()>
                as *mut RwLock<()>;

            self.on_entity_lock_address_moved
                .notify(OnEntityLockAddressMoved {
                    old: unsafe { NonNull::new_unchecked(old_lock_ptr) },
                    new: new_lock_ptr,
                })?;

            let component_storages_diff = archetype
                .component_storages
                .iter()
                .map(|(name, component_storage)| {
                    let old = old_component_storages_ptr.get(name).unwrap();
                    let new = component_storage.component_storage.get(0) as *const dyn Component
                        as *mut dyn Component;

                    (name.clone(), old.byte_offset_from(new))
                })
                .collect::<HashMap<String, isize>>();

            archetype
                .component_storages
                .iter()
                .map(|(name, component_storage)| {
                    component_storage
                        .get(new_location.entity_index)
                        .map(|new_component_storage| {
                            (
                                name.clone(),
                                new_component_storage as *const dyn Component as *mut dyn Component,
                            )
                        })
                })
                .flatten()
                .try_for_each(|(name, new_component_ptr)| {
                    let addr_diff = component_storages_diff.get(&name).unwrap();
                    let old_component_ptr = unsafe {
                        NonNull::new_unchecked(new_component_ptr.byte_offset(*addr_diff))
                    };

                    self.on_component_address_moved
                        .notify(OnComponentAddressMoved {
                            old: old_component_ptr,
                            new: Some(unsafe { NonNull::new_unchecked(new_component_ptr) }),
                        })
                })?;

            self.on_entity_location_moved
                .notify(OnEntityLocationMoved {
                    old_entity_index: old_location.entity_index,
                    old_archetype: unsafe {
                        NonNull::new_unchecked(
                            &self.archetypes[old_location.archetype_index] as *const Archetype
                                as *mut Archetype,
                        )
                    },
                    new_entity_index: new_location.entity_index,
                    new_archetype_ptr: &self.archetypes[new_location.archetype_index]
                        as *const Archetype
                        as *mut Archetype,
                })?;
        }

        Ok(())
    }

    unsafe fn apply_clear_mutation(&mut self) -> FruityResult<()> {
        // Propagate the deleted signals
        self.entity_locations
            .iter()
            .try_for_each(|(entity_id, _location)| self.on_deleted.notify(*entity_id))?;

        // Notify memory operation
        self.entity_locations
            .iter()
            .try_for_each(|(entity_id, location)| {
                let archetype = &self.archetypes[location.archetype_index];
                self.on_entity_address_removed
                    .notify(OnEntityAddressRemoved {
                        entity_id: *entity_id,
                        archetype: unsafe {
                            NonNull::new_unchecked(archetype as *const Archetype as *mut Archetype)
                        },
                    })
            })?;

        self.archetypes.iter().try_for_each(|archetype| {
            archetype.lock_array.iter().try_for_each(|lock| {
                let old = lock as *const RwLock<()> as *mut RwLock<()>;
                self.on_entity_lock_address_moved
                    .notify(OnEntityLockAddressMoved {
                        old: unsafe { NonNull::new_unchecked(old) },
                        new: null_mut(),
                    })
            })?;

            archetype
                .component_storages
                .iter()
                .map(|(_, component_storage)| {
                    (0..component_storage.components_per_entity * archetype.len())
                        .into_iter()
                        .map(|index| {
                            component_storage.component_storage.get(index) as *const dyn Component
                                as *mut dyn Component
                        })
                })
                .flatten()
                .try_for_each(|old_component_ptr| {
                    self.on_component_address_moved
                        .notify(OnComponentAddressMoved {
                            old: unsafe { NonNull::new_unchecked(old_component_ptr) },
                            new: None,
                        })
                })?;

            FruityResult::Ok(())
        })?;

        self.entity_locations
            .iter()
            .try_for_each(|(_entity_id, location)| {
                self.on_entity_location_moved.notify(OnEntityLocationMoved {
                    old_entity_index: location.entity_index,
                    old_archetype: unsafe {
                        NonNull::new_unchecked(
                            &self.archetypes[location.archetype_index] as *const Archetype
                                as *mut Archetype,
                        )
                    },
                    new_entity_index: 0,
                    new_archetype_ptr: null_mut(),
                })
            })?;

        self.archetypes.iter().try_for_each(|archetype| {
            self.on_archetype_address_moved
                .notify(OnArchetypeAddressMoved {
                    old: NonNull::new_unchecked(archetype as *const Archetype as *mut Archetype),
                    new: null_mut(),
                })
        })?;

        // Get the writers
        let mut id_incrementer = self.id_incrementer.lock();

        // Clear all entities
        self.entity_locations.clear();
        *id_incrementer = 0;
        self.archetypes.clear();

        Ok(())
    }

    unsafe fn archetype_by_identifier_mut(
        &mut self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<(usize, &mut Archetype)> {
        self.archetypes
            .iter_mut()
            .enumerate()
            .find(|(_, archetype)| *archetype.get_type_identifier() == entity_identifier)
    }
}

impl Debug for EntityService {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
