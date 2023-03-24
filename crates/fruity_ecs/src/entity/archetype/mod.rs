use super::entity_reference::EntityReference;
use super::entity_service::EntityService;
use super::entity_service::OnArchetypeAddressMoved;
use super::entity_service::OnComponentAddressMoved;
use super::entity_service::OnEntityLocationMoved;
use super::entity_service::OnEntityLockAddressMoved;
use super::EntityId;
use crate::component::component_guard::AnyComponentReadGuard;
use crate::component::component_guard::AnyComponentWriteGuard;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::AnyComponent;
use crate::component::Component;
use crate::component::StaticComponent;
use crate::entity::archetype::component_storage::ComponentStorage;
use crate::entity::archetype::entity_properties::EntityProperties;
use crate::entity::get_type_identifier_by_any;
use crate::entity::EntityTypeIdentifier;
use crate::ExtensionComponentService;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr::null_mut;
use std::ptr::NonNull;

/// This store all the information that are common accross all entities
pub mod entity_properties;

/// Provides a collection that can store components by taking care of the number of component per entity
pub mod component_storage;

/// A collection of entities that share the same component structure
/// Stored as a Struct Of Array
pub struct Archetype {
    test: usize,
    extension_component_service: ResourceReference<ExtensionComponentService>,
    pub(crate) identifier: EntityTypeIdentifier,

    // Store all the component properties into a index persisting storage
    pub(crate) lock_array: Vec<RwLock<()>>,
    pub(crate) entity_id_array: Vec<EntityId>,
    pub(crate) name_array: Vec<String>,
    pub(crate) enabled_array: Vec<bool>,
    pub(crate) component_storages: BTreeMap<String, ArchetypeComponentStorage>,

    // Entity service signals
    pub(crate) on_entity_location_moved: Signal<OnEntityLocationMoved>,
    pub(crate) on_entity_lock_address_moved: Signal<OnEntityLockAddressMoved>,
    pub(crate) on_component_address_moved: Signal<OnComponentAddressMoved>,
    pub(crate) on_archetype_address_moved: Signal<OnArchetypeAddressMoved>,
}

impl Archetype {
    /// Returns an Archetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `name` - The first entity name
    /// * `components` - The first entity components
    ///
    pub fn new(
        entity_service: &EntityService,
        extension_component_service: ResourceReference<ExtensionComponentService>,
        entity_id: EntityId,
        name: &str,
        enabled: bool,
        mut components: Vec<AnyComponent>,
    ) -> FruityResult<Archetype> {
        // Deduce the archetype properties from the first components
        let identifier = get_type_identifier_by_any(&components)?;

        // Inject the extensions
        let mut extensions_component = {
            let extension_component_service = extension_component_service.read();

            components
                .iter()
                .map(|component| {
                    extension_component_service
                        .get_component_extension(component.deref())
                        .unwrap()
                        .into_iter()
                })
                .flatten()
                .collect::<Vec<_>>()
        };
        components.append(&mut extensions_component);

        // Build the archetype component containers
        let grouped_components = Self::group_components_by_type(components);
        let mut component_storages = BTreeMap::new();
        for (class_name, components) in grouped_components {
            component_storages.insert(class_name, ArchetypeComponentStorage::new(components));
        }

        Ok(Archetype {
            test: 42,
            extension_component_service,
            identifier: identifier,
            lock_array: vec![RwLock::new(())],
            entity_id_array: vec![entity_id],
            name_array: vec![name.to_string()],
            enabled_array: vec![enabled],
            component_storages,
            on_entity_location_moved: entity_service.on_entity_location_moved.clone(),
            on_entity_lock_address_moved: entity_service.on_entity_lock_address_moved.clone(),
            on_component_address_moved: entity_service.on_component_address_moved.clone(),
            on_archetype_address_moved: entity_service.on_archetype_address_moved.clone(),
        })
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifier
    }

    /// Get entity count
    pub fn len(&self) -> usize {
        self.lock_array.len()
    }

    /// Add an entity into the archetype
    ///
    /// It is unsafe cause mut reference over Archetype, cause Archetype is widely read everywhere in the ecs code
    ///
    /// # Arguments
    /// * `name` - The first entity name
    /// * `enabled` - Is the entity enabled
    /// * `components` - The first entity components
    ///
    pub unsafe fn add(
        &mut self,
        entity_id: EntityId,
        name: &str,
        enabled: bool,
        mut components: Vec<AnyComponent>,
    ) -> FruityResult<()> {
        // Check if lock array is about to be reallocated
        let is_lock_array_about_to_reallocate =
            self.lock_array.len() + 1 > self.lock_array.capacity();
        let lock_array_old_ptr = self.lock_array.as_ptr();

        // Get component storages are about to be reallocated
        let component_storages_about_to_reallocate = self
            .component_storages
            .iter()
            .filter(|(_, component_storage)| {
                component_storage
                    .component_storage
                    .is_about_to_reallocate_on_next_insert(component_storage.components_per_entity)
            })
            .map(|(name, component_storage)| {
                (
                    name.clone(),
                    component_storage.component_storage.get(0) as *const dyn Component
                        as *mut dyn Component,
                )
            })
            .collect::<Vec<_>>();

        // Store the entity properties
        self.entity_id_array.push(entity_id);
        self.name_array.push(name.to_string());
        self.enabled_array.push(enabled);
        self.lock_array.push(RwLock::new(()));

        // Inject the extensions
        let mut extensions_component = {
            let extension_component_service = self.extension_component_service.read();

            components
                .iter()
                .map(|component| {
                    extension_component_service
                        .get_component_extension(component.deref())
                        .unwrap()
                        .into_iter()
                })
                .flatten()
                .collect::<Vec<_>>()
        };
        components.append(&mut extensions_component);

        // Store all the components
        let grouped_components = Self::group_components_by_type(components);
        for (class_name, components) in grouped_components {
            let component_array = self.component_storages.get_mut(&class_name);
            if let Some(component_array) = component_array {
                component_array.add(components);
            }
        }

        // Notify memory operation
        if is_lock_array_about_to_reallocate {
            let lock_array_new_ptr = self.lock_array.as_ptr();
            let addr_diff = lock_array_old_ptr.byte_offset_from(lock_array_new_ptr);

            self.lock_array.iter().try_for_each(|lock| {
                let new = lock as *const RwLock<()> as *mut RwLock<()>;
                let old = unsafe { NonNull::new_unchecked(new.byte_offset(addr_diff)) };

                self.on_entity_lock_address_moved
                    .notify(OnEntityLockAddressMoved { old, new })
            })?;

            component_storages_about_to_reallocate.iter().try_for_each(
                |(name, old_first_component_ptr)| {
                    let component_storage = self.component_storages.get(name).unwrap();
                    let new_first_component_ptr = component_storage.component_storage.get(0)
                        as *const dyn Component
                        as *mut dyn Component;
                    let addr_diff =
                        old_first_component_ptr.byte_offset_from(new_first_component_ptr);

                    (0..self.len() * component_storage.components_per_entity)
                        .into_iter()
                        .try_for_each(|component_index| {
                            let new_component_ptr =
                                component_storage.component_storage.get(component_index)
                                    as *const dyn Component
                                    as *mut dyn Component;
                            let old_component_ptr = unsafe {
                                NonNull::new_unchecked(new_component_ptr.byte_offset(addr_diff))
                            };

                            self.on_component_address_moved
                                .notify(OnComponentAddressMoved {
                                    old: old_component_ptr,
                                    new: Some(unsafe { NonNull::new_unchecked(new_component_ptr) }),
                                })
                        })
                },
            )?;
        }

        Ok(())
    }

    /// Get a reference to an entity by index
    ///
    /// # Arguments
    /// * `entity_index` - The entity index in the archetype
    ///

    pub fn get_entity_reference(&self, entity_index: usize) -> EntityReference {
        EntityReference::new(
            &self.on_entity_location_moved,
            &self.on_archetype_address_moved,
            &self.on_entity_lock_address_moved,
            &self.on_component_address_moved,
            entity_index,
            self as *const Archetype as *mut Archetype,
        )
    }

    /// Get an iterator over all the components of all the entities
    pub fn iter(&self, ignore_enabled: bool) -> impl Iterator<Item = EntityReference> + '_ {
        let archetype_len = self.lock_array.len();
        let archetype_ptr = self as *const Archetype as *mut Archetype;
        let on_entity_location_moved = self.on_entity_location_moved.clone();
        let on_archetype_address_moved = self.on_archetype_address_moved.clone();
        let on_entity_lock_address_moved = self.on_entity_lock_address_moved.clone();
        let on_component_address_moved = self.on_component_address_moved.clone();

        (0..archetype_len)
            .filter(move |entity_index| {
                if !ignore_enabled {
                    *self.enabled_array.get(*entity_index).unwrap()
                } else {
                    true
                }
            })
            .map(move |entity_index| {
                EntityReference::new(
                    &on_entity_location_moved,
                    &on_archetype_address_moved,
                    &on_entity_lock_address_moved,
                    &on_component_address_moved,
                    entity_index,
                    archetype_ptr,
                )
            })
    }

    /// Remove an entity based on its id
    ///
    /// It is unsafe cause mut reference over Archetype, cause Archetype is widely read everywhere in the ecs code
    ///
    /// # Arguments
    /// * `index` - The entity index
    ///
    pub unsafe fn remove(
        &mut self,
        index: usize,
        propagate_memory_deleted_signals: bool,
    ) -> FruityResult<(EntityProperties, Vec<AnyComponent>)> {
        // Check if index exists
        if index >= self.lock_array.len() {
            return Err(FruityError::GenericFailure(
                "Index out of bound".to_string(),
            ));
        }

        // Notify memory operation
        {
            self.lock_array.iter().skip(index).try_for_each(|lock| {
                let old = lock as *const RwLock<()> as *mut RwLock<()>;
                let new = old.sub(1);

                self.on_entity_lock_address_moved
                    .notify(OnEntityLockAddressMoved {
                        old: unsafe { NonNull::new_unchecked(old) },
                        new,
                    })
            })?;

            self.component_storages
                .iter()
                .try_for_each(|(_, component_storage)| {
                    (0..self.len() * component_storage.components_per_entity)
                        .skip(index * component_storage.components_per_entity)
                        .into_iter()
                        .try_for_each(|component_index| {
                            let old = component_storage.component_storage.get(component_index)
                                as *const dyn Component
                                as *mut dyn Component;
                            let new = old.byte_sub(
                                component_storage.components_per_entity
                                    * component_storage.component_storage.item_size(),
                            );

                            self.on_component_address_moved
                                .notify(OnComponentAddressMoved {
                                    old: unsafe { NonNull::new_unchecked(old) },
                                    new: Some(unsafe { NonNull::new_unchecked(new) }),
                                })
                        })
                })?;

            if propagate_memory_deleted_signals {
                let old = &self.lock_array[index] as *const RwLock<()> as *mut RwLock<()>;
                self.on_entity_lock_address_moved
                    .notify(OnEntityLockAddressMoved {
                        old: unsafe { NonNull::new_unchecked(old) },
                        new: null_mut(),
                    })?;

                self.component_storages
                    .iter()
                    .try_for_each(|(_, component_storage)| {
                        (0..component_storage.components_per_entity)
                            .into_iter()
                            .try_for_each(|component_sub_index| {
                                let old = component_storage
                                    .component_storage
                                    .get(index + component_sub_index)
                                    as *const dyn Component
                                    as *mut dyn Component;

                                self.on_component_address_moved
                                    .notify(OnComponentAddressMoved {
                                        old: unsafe { NonNull::new_unchecked(old) },
                                        new: None,
                                    })
                            })
                    })?;
            }
        }

        // Remove common entity properties
        self.lock_array.remove(index);
        let entity_id = self.entity_id_array.remove(index);
        let name = self.name_array.remove(index);
        let enabled = self.enabled_array.remove(index);

        // Get the entity components from the storage
        let components = {
            self.component_storages
                .iter_mut()
                .map(|(_, storage)| storage.remove(index))
                .flatten()
                .map(|component| AnyComponent::from(component))
                .collect::<Vec<_>>()
        };

        // Return the deleted components
        Ok((
            EntityProperties {
                entity_id,
                name,
                enabled,
            },
            components,
        ))
    }

    fn group_components_by_type(
        components: Vec<AnyComponent>,
    ) -> HashMap<String, Vec<AnyComponent>> {
        components
            .into_iter()
            .group_by(|component| component.get_class_name().unwrap())
            .into_iter()
            .map(|(class_name, component)| (class_name, component.collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>()
    }
}

pub(crate) struct ArchetypeComponentStorage {
    pub(crate) components_per_entity: usize,
    pub(crate) component_storage: Box<dyn ComponentStorage>,
}

impl ArchetypeComponentStorage {
    pub(crate) fn new(components: Vec<AnyComponent>) -> Self {
        let components_per_entity = components.len();
        let first_component = components.get(0).unwrap();
        let component_storage = first_component.get_storage();

        let mut result = Self {
            components_per_entity,
            component_storage,
        };

        unsafe { result.add(components) };
        result
    }

    /// It is unsafe cause mut reference over Archetype, cause Archetype is widely read everywhere in the ecs code
    pub(crate) unsafe fn add(&mut self, components: Vec<AnyComponent>) {
        // Check the components count
        if components.len() != self.components_per_entity {
            panic!("Try to instantiate a component array from a component array with the wrong size of elements");
        }

        self.component_storage.add_many(components);
    }

    pub(crate) fn get(&self, entity_index: usize) -> impl Iterator<Item = &dyn Component> {
        let start_index = entity_index * self.components_per_entity;
        let end_index = start_index + self.components_per_entity;

        (start_index..end_index).map(|index| self.component_storage.get(index))
    }

    /// It is unsafe cause mut reference over Archetype, cause Archetype is widely read everywhere in the ecs code
    pub(crate) unsafe fn remove(&mut self, entity_index: usize) -> Vec<Box<dyn Component>> {
        self.component_storage
            .remove_many(entity_index, self.components_per_entity)
    }
}

/// A struct that allow you to read an entity
#[derive(Clone)]
pub struct Entity<'a> {
    pub(crate) entity_index: usize,
    pub(crate) archetype: &'a Archetype,
}

impl<'a> Debug for Entity<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Entity<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        self.archetype
            .entity_id_array
            .get(self.entity_index)
            .map(|entity_id| *entity_id)
            .unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.archetype
            .name_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        self.archetype
            .enabled_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> impl Iterator<Item = AnyComponentReadGuard<'_>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        self.archetype
            .component_storages
            .iter()
            .map(|(_, storage)| storage.get(self.entity_index))
            .flatten()
            .map(|component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            })
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<impl Iterator<Item = ComponentReadGuard<'_, T>>> {
        Ok(self
            .iter_components_by_type_identifier(T::get_component_name())?
            .into_iter()
            .map(|component_guard| component_guard.try_into())
            .filter_map(|entity_guard| entity_guard.ok()))
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_identifier(
        &self,
        component_identifier: &str,
    ) -> FruityResult<impl Iterator<Item = AnyComponentReadGuard<'_>>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        let storage = self.archetype
            .component_storages
            .get(component_identifier)
            .ok_or(FruityError::GenericFailure(format!("You try to access a component with identifier {} in the entity named {} but the component don't exists", component_identifier, self.get_name())))?;

        Ok(storage
            .get(self.entity_index)
            .map(|component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            }))
    }

    /// Read a single component with a given type
    pub fn get_component_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<Option<ComponentReadGuard<'_, T>>> {
        Ok(self.iter_components_by_type()?.next())
    }
}

/// A struct that allow you to write an entity
#[derive(Clone)]
pub struct EntityMut<'a> {
    pub(crate) entity_index: usize,
    pub(crate) archetype: &'a Archetype,
}

impl<'a> Debug for EntityMut<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityMut<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        self.archetype
            .entity_id_array
            .get(self.entity_index)
            .map(|entity_id| *entity_id)
            .unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.archetype
            .name_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Set the entity name
    ///
    /// # Arguments
    /// * `value` - The name value
    ///
    pub fn set_name(&self, value: &str) {
        let name = &self.archetype.name_array[self.entity_index];

        // Safe cause it is protected by self.entity_guard
        #[allow(mutable_transmutes)]
        let name = unsafe { std::mem::transmute::<&String, &mut String>(name) };

        *name = value.to_string();
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        self.archetype
            .enabled_array
            .get(self.entity_index)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Set the entity enabled state
    ///
    /// # Arguments
    /// * `value` - Is the entity enabled
    ///
    pub fn set_enabled(&self, value: bool) {
        let enabled = &self.archetype.enabled_array[self.entity_index];

        // Safe cause it is protected by self.entity_guard
        #[allow(mutable_transmutes)]
        let enabled = unsafe { std::mem::transmute::<&bool, &mut bool>(enabled) };

        *enabled = value;
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> impl Iterator<Item = AnyComponentReadGuard<'_>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        self.archetype
            .component_storages
            .iter()
            .map(|(_, storage)| storage.get(self.entity_index))
            .flatten()
            .map(|component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            })
    }

    /// Write all components of the entity
    pub fn write_all_components(&self) -> impl Iterator<Item = AnyComponentWriteGuard<'_>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        self.archetype
            .component_storages
            .iter()
            .map(|(_, storage)| storage.get(self.entity_index))
            .flatten()
            .map(|component| AnyComponentWriteGuard {
                entity_guard: lock.write(),
                component_ptr: NonNull::from(component),
            })
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<impl Iterator<Item = ComponentReadGuard<'_, T>>> {
        Ok(self
            .iter_components_by_type_identifier(T::get_component_name())?
            .into_iter()
            .map(|component_guard| component_guard.try_into())
            .filter_map(|entity_guard| entity_guard.ok()))
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_mut<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<impl Iterator<Item = ComponentWriteGuard<'_, T>>> {
        Ok(self
            .iter_components_by_type_identifier_mut(T::get_component_name())?
            .into_iter()
            .map(|component_guard| component_guard.try_into())
            .filter_map(|entity_guard| entity_guard.ok()))
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_identifier(
        &self,
        component_identifier: &str,
    ) -> FruityResult<impl Iterator<Item = AnyComponentReadGuard<'_>>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        let storage = self.archetype
            .component_storages
            .get(component_identifier)
            .ok_or(FruityError::GenericFailure(format!("You try to access a component with identifier {} in the entity named {} but the component don't exists", component_identifier, self.get_name())))?;

        Ok(storage
            .get(self.entity_index)
            .map(|component| AnyComponentReadGuard {
                entity_guard: lock.read(),
                component_ptr: NonNull::from(component),
            }))
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_by_type_identifier_mut(
        &self,
        component_identifier: &str,
    ) -> FruityResult<impl Iterator<Item = AnyComponentWriteGuard<'_>>> {
        let lock = self.archetype.lock_array.get(self.entity_index).unwrap();

        let storage = self.archetype
            .component_storages
            .get(component_identifier)
            .ok_or(FruityError::GenericFailure(format!("You try to access a component with identifier {} in the entity named {} but the component don't exists", component_identifier, self.get_name())))?;

        Ok(storage
            .get(self.entity_index)
            .map(|component| AnyComponentWriteGuard {
                entity_guard: lock.write(),
                component_ptr: NonNull::from(component),
            }))
    }

    /// Read a single component with a given type
    pub fn get_component_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<Option<ComponentReadGuard<'_, T>>> {
        Ok(self.iter_components_by_type()?.next())
    }

    /// Write a single component with a given type
    pub fn get_component_by_type_mut<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<Option<ComponentWriteGuard<'_, T>>> {
        Ok(self.iter_components_by_type_mut()?.next())
    }
}
