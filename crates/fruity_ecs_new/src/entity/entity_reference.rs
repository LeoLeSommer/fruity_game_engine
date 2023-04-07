use super::{Archetype, EntityId, EntityLocation, EntityStorage};
use crate::component::{
    AnyComponentReadGuardIterator, AnyComponentReference, AnyComponentWriteGuardIterator,
    Component, ComponentReadGuard, ComponentReadGuardIterator, ComponentTypeId,
    ComponentWriteGuard, ComponentWriteGuardIterator, Enabled, Name,
};
use either::Either;
use fruity_game_engine::{
    any::FruityAny,
    export, export_impl, export_struct,
    signal::{ObserverHandler, Signal},
    Arc, FruityError, FruityResult, RwLock,
};
use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

/// An entity const reference
#[derive(Clone)]
pub struct EntityReader<'a> {
    pub(crate) entity_index: usize,
    pub(crate) archetype: NonNull<Archetype>,
    pub(crate) phantom: PhantomData<&'a Archetype>,
}

impl<'a> Debug for EntityReader<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityReader<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        unsafe {
            self.archetype
                .as_ref()
                .entity_ids
                .get(self.entity_index)
                .map(|entity_id| *entity_id)
                .unwrap()
        }
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.get_component_by_type::<Name>().unwrap().0.clone()
    }

    /// Get the entity enabled state
    pub fn is_enabled(&self) -> bool {
        self.get_component_by_type::<Enabled>().unwrap().0.clone()
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> impl Iterator<Item = &dyn Component> {
        unsafe {
            self.archetype
                .as_ref()
                .component_storages
                .iter()
                .map(|(_, storage)| {
                    let storage_reader = storage.read();
                    let slice_len = storage_reader.slice_len(self.entity_index);
                    let slice_begin =
                        NonNull::from(storage_reader.get_unchecked(self.entity_index, 0));
                    let component_type_size = storage_reader.get_component_type_size();

                    AnyComponentReadGuardIterator::new(
                        storage_reader,
                        component_type_size,
                        slice_begin,
                        slice_len,
                    )
                })
                .flatten()
        }
    }

    /// Read components with a given type
    pub fn iter_components_by_type<T: Component>(&self) -> impl Iterator<Item = &T> {
        unsafe {
            if let Some(storage) = self
                .archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::of::<T>())
            {
                let storage_reader = storage.read();
                let slice_len = storage_reader.slice_len(self.entity_index);
                let slice_begin = NonNull::from(
                    storage_reader
                        .get_unchecked(self.entity_index, 0)
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap(),
                );

                Either::Left(ComponentReadGuardIterator::new(
                    storage_reader,
                    slice_begin,
                    slice_len,
                ))
            } else {
                Either::Right(vec![].into_iter())
            }
        }
    }

    /// Read any components with a given type
    pub fn iter_components_by_type_identifier(
        &self,
        component_identifier: &str,
    ) -> impl Iterator<Item = &dyn Component> {
        unsafe {
            if let Some(storage) = self
                .archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::from_identifier(component_identifier))
            {
                let storage_reader = storage.read();
                let slice_len = storage_reader.slice_len(self.entity_index);
                let slice_begin = NonNull::from(storage_reader.get_unchecked(self.entity_index, 0));
                let component_type_size = storage_reader.get_component_type_size();

                Either::Left(AnyComponentReadGuardIterator::new(
                    storage_reader,
                    component_type_size,
                    slice_begin,
                    slice_len,
                ))
            } else {
                Either::Right(vec![].into_iter())
            }
        }
    }

    /// Read a single component with a given type
    pub fn get_component_by_type<T: Component>(&self) -> Option<ComponentReadGuard<'_, T>> {
        unsafe {
            self.archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::of::<T>())
                .map(|storage| {
                    let storage_reader = storage.read();
                    let component_ptr = NonNull::from(
                        storage_reader
                            .get_unchecked(self.entity_index, 0)
                            .as_any_ref()
                            .downcast_ref::<T>()
                            .unwrap(),
                    );

                    ComponentReadGuard {
                        storage_guard: storage_reader,
                        component_ptr: component_ptr,
                    }
                })
        }
    }
}

/// An entity mut reference
#[derive(Clone)]
pub struct EntityWriter<'a> {
    pub(crate) entity_index: usize,
    pub(crate) archetype: NonNull<Archetype>,
    pub(crate) phantom: PhantomData<&'a Archetype>,
}

impl<'a> Debug for EntityWriter<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityWriter<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        unsafe {
            self.archetype
                .as_ref()
                .entity_ids
                .get(self.entity_index)
                .map(|entity_id| *entity_id)
                .unwrap()
        }
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.get_component_by_type::<Name>().unwrap().0.clone()
    }

    /// Set the entity name
    pub fn set_name(&mut self, name: String) {
        self.get_component_by_type_mut::<Name>().unwrap().0 = name;
    }

    /// Get the entity enabled state
    pub fn is_enabled(&self) -> bool {
        self.get_component_by_type::<Enabled>().unwrap().0.clone()
    }

    /// Set the entity enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.get_component_by_type_mut::<Enabled>().unwrap().0 = enabled;
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> impl Iterator<Item = &dyn Component> {
        unsafe {
            self.archetype
                .as_ref()
                .component_storages
                .iter()
                .map(|(_, storage)| {
                    let storage_reader = storage.read();
                    let slice_len = storage_reader.slice_len(self.entity_index);
                    let slice_begin =
                        NonNull::from(storage_reader.get_unchecked(self.entity_index, 0));
                    let component_type_size = storage_reader.get_component_type_size();

                    AnyComponentReadGuardIterator::new(
                        storage_reader,
                        component_type_size,
                        slice_begin,
                        slice_len,
                    )
                })
                .flatten()
        }
    }

    /// Read components with a given type
    pub fn iter_components_by_type<T: Component>(&self) -> impl Iterator<Item = &T> {
        unsafe {
            if let Some(storage) = self
                .archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::of::<T>())
            {
                let storage_reader = storage.read();
                let slice_len = storage_reader.slice_len(self.entity_index);
                let slice_begin = NonNull::from(
                    storage_reader
                        .get_unchecked(self.entity_index, 0)
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap(),
                );

                Either::Left(ComponentReadGuardIterator::new(
                    storage_reader,
                    slice_begin,
                    slice_len,
                ))
            } else {
                Either::Right(vec![].into_iter())
            }
        }
    }

    /// Write components with a given type
    pub fn iter_components_by_type_mut<T: Component>(&mut self) -> impl Iterator<Item = &mut T> {
        unsafe {
            if let Some(storage) = self
                .archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::of::<T>())
            {
                let storage_writer = storage.write();
                let slice_len = storage_writer.slice_len(self.entity_index);
                let slice_begin = NonNull::from(
                    storage_writer
                        .get_unchecked(self.entity_index, 0)
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap(),
                );

                Either::Left(ComponentWriteGuardIterator::new(
                    storage_writer,
                    slice_begin,
                    slice_len,
                ))
            } else {
                Either::Right(vec![].into_iter())
            }
        }
    }

    /// Read any components with a given type
    pub fn iter_components_by_type_identifier(
        &self,
        component_identifier: &str,
    ) -> impl Iterator<Item = &dyn Component> {
        unsafe {
            if let Some(storage) = self
                .archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::from_identifier(component_identifier))
            {
                let storage_reader = storage.read();
                let slice_len = storage_reader.slice_len(self.entity_index);
                let slice_begin = NonNull::from(storage_reader.get_unchecked(self.entity_index, 0));
                let component_type_size = storage_reader.get_component_type_size();

                Either::Left(AnyComponentReadGuardIterator::new(
                    storage_reader,
                    component_type_size,
                    slice_begin,
                    slice_len,
                ))
            } else {
                Either::Right(vec![].into_iter())
            }
        }
    }

    /// Write any components with a given type
    pub fn iter_components_by_type_identifier_mut(
        &mut self,
        component_identifier: &str,
    ) -> impl Iterator<Item = &mut dyn Component> {
        unsafe {
            if let Some(storage) = self
                .archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::from_identifier(component_identifier))
            {
                let storage_writer = storage.write();
                let slice_len = storage_writer.slice_len(self.entity_index);
                let slice_begin = NonNull::from(storage_writer.get_unchecked(self.entity_index, 0));
                let component_type_size = storage_writer.get_component_type_size();

                Either::Left(AnyComponentWriteGuardIterator::new(
                    storage_writer,
                    component_type_size,
                    slice_begin,
                    slice_len,
                ))
            } else {
                Either::Right(vec![].into_iter())
            }
        }
    }

    /// Read a single component with a given type
    pub fn get_component_by_type<T: Component>(&self) -> Option<ComponentReadGuard<'_, T>> {
        unsafe {
            self.archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::of::<T>())
                .map(|storage| {
                    let storage_reader = storage.read();
                    let component_ptr = NonNull::from(
                        storage_reader
                            .get_unchecked(self.entity_index, 0)
                            .as_any_ref()
                            .downcast_ref::<T>()
                            .unwrap(),
                    );

                    ComponentReadGuard {
                        storage_guard: storage_reader,
                        component_ptr: component_ptr,
                    }
                })
        }
    }

    /// Write a single component with a given type
    pub fn get_component_by_type_mut<T: Component>(
        &mut self,
    ) -> Option<ComponentWriteGuard<'_, T>> {
        unsafe {
            self.archetype
                .as_ref()
                .component_storages
                .get(&ComponentTypeId::of::<T>())
                .map(|storage| {
                    let storage_writer = storage.write();
                    let component_ptr = NonNull::from(
                        storage_writer
                            .get_unchecked(self.entity_index, 0)
                            .as_any_ref()
                            .downcast_ref::<T>()
                            .unwrap(),
                    );

                    ComponentWriteGuard {
                        storage_guard: storage_writer,
                        component_ptr: component_ptr,
                    }
                })
        }
    }
}

#[derive(Debug)]
pub(crate) struct InnerShareableEntityReference {
    pub(crate) entity_storage: Arc<RwLock<EntityStorage>>,
    pub(crate) entity_id: EntityId,
    pub(crate) location: EntityLocation,
}

/// A reference to an entity
/// Update its own state when an entity is moved
#[derive(Debug, FruityAny)]
#[export_struct]
pub struct EntityReference {
    pub(crate) inner: Arc<RwLock<Option<InnerShareableEntityReference>>>,
    on_entity_location_moved: Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
    on_entity_location_moved_handler:
        ObserverHandler<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
}

impl Drop for EntityReference {
    fn drop(&mut self) {
        self.on_entity_location_moved_handler.dispose_by_ref();
    }
}

#[export_impl]
impl EntityReference {
    pub(crate) fn new(
        entity_storage: Arc<RwLock<EntityStorage>>,
        entity_id: EntityId,
        location: EntityLocation,
        on_entity_location_moved: Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
    ) -> Self {
        let inner = Arc::new(RwLock::new(Some(InnerShareableEntityReference {
            entity_storage,
            entity_id,
            location,
        })));

        let inner_2 = inner.clone();
        let on_entity_location_moved_handler =
            on_entity_location_moved.add_observer(move |(entity_id, entity_storage, location)| {
                let mut inner = inner_2.write();
                if let Some(inner) = inner.as_mut() {
                    if inner.entity_id == *entity_id {
                        inner.entity_storage = entity_storage.clone();
                        inner.location = location.clone();
                    }
                }

                Ok(())
            });

        Self {
            inner,
            on_entity_location_moved,
            on_entity_location_moved_handler,
        }
    }

    /// Get a read access to the entity
    pub fn read(&self) -> FruityResult<EntityReader> {
        let inner = self.inner.read();
        if let Some(inner) = inner.as_ref() {
            let entity_storage = inner.entity_storage.read();

            Ok(EntityReader {
                archetype: NonNull::from(unsafe {
                    entity_storage
                        .archetypes
                        .get_unchecked(inner.location.archetype.0)
                }),
                entity_index: inner.location.index,
                phantom: PhantomData,
            })
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted entity".to_string(),
            ))
        }
    }

    /// Get a write access to the entity
    pub fn write(&self) -> FruityResult<EntityWriter> {
        let inner = self.inner.read();
        if let Some(inner) = inner.as_ref() {
            let entity_storage = inner.entity_storage.write();

            Ok(EntityWriter {
                archetype: NonNull::from(unsafe {
                    entity_storage
                        .archetypes
                        .get_unchecked(inner.location.archetype.0)
                }),
                entity_index: inner.location.index,
                phantom: PhantomData,
            })
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted entity".to_string(),
            ))
        }
    }

    /// Get entity id
    #[export]
    pub fn get_entity_id(&self) -> FruityResult<EntityId> {
        Ok(self.read()?.get_entity_id())
    }

    /// Get entity name
    #[export]
    pub fn get_name(&self) -> FruityResult<String> {
        Ok(self.read()?.get_name())
    }

    /// Set entity name
    #[export]
    pub fn set_name(&self, name: String) -> FruityResult<()> {
        self.write()?.set_name(name);
        Ok(())
    }

    /// Get entity enabled
    #[export]
    pub fn is_enabled(&self) -> FruityResult<bool> {
        Ok(self.read()?.is_enabled())
    }

    /// Set entity enabled
    #[export]
    pub fn set_enabled(&self, enabled: bool) -> FruityResult<()> {
        self.write()?.set_enabled(enabled);
        Ok(())
    }

    /// Get all components
    #[export]
    pub fn get_all_components(&self) -> FruityResult<Vec<AnyComponentReference>> {
        let inner = self.inner.read();
        if let Some(inner) = inner.as_ref() {
            let entity_storage = inner.entity_storage.read();

            let archetype = unsafe {
                entity_storage
                    .archetypes
                    .get_unchecked(inner.location.archetype.0)
            };

            Ok(archetype
                .component_storages
                .iter()
                .map(|(component_type_id, component_storage)| {
                    let slice_len = component_storage.read().slice_len(inner.location.index);
                    (0..slice_len).map(|component_index| {
                        AnyComponentReference::new(
                            self.clone(),
                            component_type_id.clone(),
                            component_index,
                        )
                    })
                })
                .flatten()
                .collect())
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted entity".to_string(),
            ))
        }
    }

    /// Get components with a given type
    #[export]
    pub fn get_components_by_type_identifier(
        &self,
        component_identifier: String,
    ) -> FruityResult<Vec<AnyComponentReference>> {
        self.get_components_by_type_id(ComponentTypeId::from_identifier(&component_identifier))
    }

    /// Get components with a given type
    pub fn get_components_by_type<T: Component>(&self) -> FruityResult<Vec<AnyComponentReference>> {
        self.get_components_by_type_id(ComponentTypeId::of::<T>())
    }

    /// Get components with a given component type id
    fn get_components_by_type_id(
        &self,
        component_type_id: ComponentTypeId,
    ) -> FruityResult<Vec<AnyComponentReference>> {
        let inner = self.inner.read();
        if let Some(inner) = inner.as_ref() {
            let entity_storage = inner.entity_storage.read();

            let archetype = unsafe {
                entity_storage
                    .archetypes
                    .get_unchecked(inner.location.archetype.0)
            };

            let component_storage = if let Some(component_storage) =
                archetype.component_storages.get(&component_type_id)
            {
                component_storage
            } else {
                return Ok(vec![]);
            };

            let slice_len = component_storage.read().slice_len(inner.location.index);
            Ok((0..slice_len)
                .map(|component_index| {
                    AnyComponentReference::new(
                        self.clone(),
                        component_type_id.clone(),
                        component_index,
                    )
                })
                .collect())
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted entity".to_string(),
            ))
        }
    }
}

impl Clone for EntityReference {
    fn clone(&self) -> Self {
        Self::new(
            self.inner.read().as_ref().unwrap().entity_storage.clone(),
            self.inner.read().as_ref().unwrap().entity_id,
            self.inner.read().as_ref().unwrap().location.clone(),
            self.on_entity_location_moved.clone(),
        )
    }
}
