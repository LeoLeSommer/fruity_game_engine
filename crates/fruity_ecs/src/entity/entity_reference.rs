use super::archetype::Archetype;
use super::Entity;
use super::EntityMut;
use super::entity_service::AddEntityMutation;
use super::entity_service::OnArchetypeAddressMoved;
use super::entity_service::OnComponentAddressMoved;
use super::entity_service::OnEntityAddressAdded;
use super::entity_service::OnEntityLocationMoved;
use super::entity_service::OnEntityLockAddressMoved;
use super::EntityId;
use crate::component::component_reference::AnyComponentReference;
use crate::component::component_reference::ComponentReference;
use crate::component::Component;
use crate::component::StaticComponent;
use crate::entity::entity_guard::EntityReadGuard;use std::ops::DerefMut;
use crate::entity::entity_guard::EntityWriteGuard;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use fruity_game_engine::RwLockReadGuard;
use fruity_game_engine::{export, export_impl, export_struct};
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr::null_mut;
use std::ptr::NonNull;
use std::sync::Arc;

pub(crate) enum InnerShareableEntityReference {
    Archetype {
        entity_index: usize,
        archetype_ptr: *mut Archetype,
    },
    Mutation {
        entity: Arc<RwLock<AddEntityMutation>>,
    },
}

// Safe cause archetypes are updated when an entity is moved trough memory
unsafe impl Send for InnerShareableEntityReference {}

// Safe cause archetypes are updated when an entity is moved trough memory
unsafe impl Sync for InnerShareableEntityReference {}

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
#[export_struct]
pub struct EntityReference {
    pub(crate) inner: Arc<RwLock<InnerShareableEntityReference>>,
    on_entity_lock_address_moved: Signal<OnEntityLockAddressMoved>,
    on_component_address_moved: Signal<OnComponentAddressMoved>,
    on_entity_location_moved_handle: ObserverHandler<OnEntityLocationMoved>,
    on_archetype_address_moved_handle: ObserverHandler<OnArchetypeAddressMoved>,
    on_entity_address_added_handle: ObserverHandler<OnEntityAddressAdded>,
}

#[export_impl]
impl EntityReference {
    pub(crate) fn new(
        inner: InnerShareableEntityReference,
        on_entity_location_moved: &Signal<OnEntityLocationMoved>,
        on_archetype_address_moved: &Signal<OnArchetypeAddressMoved>,
        on_entity_address_added: &Signal<OnEntityAddressAdded>,
        on_entity_lock_address_moved: &Signal<OnEntityLockAddressMoved>,
        on_component_address_moved: &Signal<OnComponentAddressMoved>,
    ) -> Self {
        let inner = Arc::new(RwLock::new(inner));

        // Register memory move observers to update the entity reference inner pointers when the memory is moved
        let (on_entity_location_moved_handle, on_archetype_address_moved_handle, on_entity_address_added_handle) = {
            let inner2 = inner.clone();
            let on_entity_location_moved_handle =
                on_entity_location_moved.add_observer(move |event| {
                    let mut inner_writer = inner2.write();
                    let (entity_index,  archetype_ptr) = if let InnerShareableEntityReference::Archetype { entity_index, archetype_ptr } = inner_writer.deref_mut() {
                        (entity_index, archetype_ptr)
                    } else {
                        return Ok(())
                    };


                    if !archetype_ptr.is_null()
                        && event.old_entity_index == *entity_index
                        && event.old_archetype.as_ptr() == *archetype_ptr
                    {
                        *archetype_ptr = event.new_archetype_ptr;
                        *entity_index = event.new_entity_index;
                    }

                    Ok(())
                });

            let inner2 = inner.clone();
            let on_archetype_address_moved_handle =
                on_archetype_address_moved.add_observer(move |event| {
                    let mut inner_writer = inner2.write();
                    let (entity_index,  archetype_ptr) = if let InnerShareableEntityReference::Archetype { entity_index, archetype_ptr } = inner_writer.deref_mut() {
                        (entity_index, archetype_ptr)
                    } else {
                        return Ok(())
                    };

                    if !archetype_ptr.is_null()
                        && event.old
                            == unsafe { NonNull::new_unchecked(*archetype_ptr) }
                    {
                        if let Some(new) = unsafe { event.new.as_ref() } {
                            *archetype_ptr = new as *const Archetype as *mut Archetype;
                        } else {
                            *archetype_ptr = null_mut();
                            *entity_index = 0;
                        }
                    }

                    Ok(())
                });

            let inner2 = inner.clone();
            let on_entity_address_added_handle =
                on_entity_address_added.add_observer(move |event| {
                    let mut inner_writer = inner2.write();
                    let entity = if let InnerShareableEntityReference::Mutation { entity } = inner_writer.deref_mut() {
                        entity.clone()
                    } else {
                        return Ok(())
                    };


                    let entity_writer = entity.write();
                    if entity_writer.entity_id == event.entity_reference.get_entity_id()?
                    {
                        let new_entity_reference_inner_reader = event.entity_reference.inner.read();
                        let (new_entity_index, new_archetype_ptr) = Self::get_archetype_inner(new_entity_reference_inner_reader)?;

                        *inner_writer = InnerShareableEntityReference::Archetype {
                            entity_index: new_entity_index,
                            archetype_ptr: new_archetype_ptr,
                        };
                    }

                    Ok(())
                });

            (
                on_entity_location_moved_handle,
                on_archetype_address_moved_handle,
                on_entity_address_added_handle,
            )
        };

        Self {
            inner,
            on_entity_lock_address_moved: on_entity_lock_address_moved.clone(),
            on_component_address_moved: on_component_address_moved.clone(),
            on_entity_location_moved_handle: on_entity_location_moved_handle,
            on_archetype_address_moved_handle: on_archetype_address_moved_handle,
            on_entity_address_added_handle: on_entity_address_added_handle,
        }
    }

    /// Get a read access to the entity
    pub fn read(&self) -> FruityResult<EntityReadGuard> {
        let inner = self.read_inner();
        let (entity_index, archetype_ptr) = Self::get_archetype_inner(inner)?;
            if !archetype_ptr.is_null() {
                Ok(EntityReadGuard {
                    _entity_guard: unsafe { archetype_ptr.as_ref() }
                        .unwrap()
                        .lock_array
                        .get(entity_index)
                        .unwrap()
                        .read(),
                    entity: Entity {
                        entity_index: entity_index,
                        archetype: unsafe { archetype_ptr.as_ref().unwrap() },
                    },
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
    }

    /// Get a write access to the entity
    pub fn write(&self) -> FruityResult<EntityWriteGuard> {
        let inner = self.read_inner();
        let (entity_index, archetype_ptr) = Self::get_archetype_inner(inner)?;
            if !archetype_ptr.is_null() {
                Ok(EntityWriteGuard {
                    _entity_guard: unsafe { archetype_ptr.as_ref() }
                        .unwrap()
                        .lock_array
                        .get(entity_index)
                        .unwrap()
                        .write(),
                    entity: EntityMut {
                        entity_index: entity_index,
                        archetype: unsafe { archetype_ptr.as_ref().unwrap() },
                    },
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
    }

    /// Get all components
    #[export]
    pub fn get_all_components(&self) -> FruityResult<Vec<AnyComponentReference>> {
        let inner = self.read_inner();
        let (entity_index, archetype_ptr) = Self::get_archetype_inner(inner)?;
            if !archetype_ptr.is_null() {
                let archetype = unsafe { archetype_ptr.as_mut().unwrap() };
                let entity_lock_ptr =
                    archetype.lock_array.get_mut(entity_index).unwrap() as *mut RwLock<()>;
    
                Ok(archetype
                    .component_storages
                    .iter()
                    .map(|(_, storage)| {
                        storage.get(entity_index).map(|component| {
                            AnyComponentReference::new(
                                &self.on_entity_lock_address_moved,
                                &self.on_component_address_moved,
                                entity_lock_ptr,
                                Some(unsafe {
                                    NonNull::new_unchecked(
                                        component as *const dyn Component as *mut dyn Component,
                                    )
                                }),
                            )
                        })
                    })
                    .flatten()
                    .collect::<Vec<_>>())
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
    }

    /// Get components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    #[export]
    pub fn get_components_by_type_identifier(
        &self,
        component_identifier: String,
    ) -> FruityResult<Vec<AnyComponentReference>> {
        let inner = self.read_inner();
        let (entity_index, archetype_ptr) = Self::get_archetype_inner(inner)?;
            if !archetype_ptr.is_null() {
                let archetype = unsafe { archetype_ptr.as_mut().unwrap() };
                let entity_lock_ptr =
                    archetype.lock_array.get_mut(entity_index).unwrap() as *mut RwLock<()>;
    
                let storage =
                    if let Some(storage) = archetype.component_storages.get(&component_identifier) {
                        storage
                    } else {
                        return Ok(vec![]);
                    };
    
                Ok(storage
                    .get(entity_index)
                    .map(|component| {
                        AnyComponentReference::new(
                            &self.on_entity_lock_address_moved,
                            &self.on_component_address_moved,
                            entity_lock_ptr,
                            Some(unsafe {
                                NonNull::new_unchecked(
                                    component as *const dyn Component as *mut dyn Component,
                                )
                            }),
                        )
                    })
                    .collect::<Vec<_>>())
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
    }

    /// Get components with a given type
    pub fn get_components_by_type<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<Vec<ComponentReference<T>>> {
        let inner = self.read_inner();
        let (entity_index, archetype_ptr) = Self::get_archetype_inner(inner)?;
            if !archetype_ptr.is_null() {
                let archetype = unsafe { archetype_ptr.as_mut().unwrap() };
                let entity_lock_ptr =
                    archetype.lock_array.get_mut(entity_index).unwrap() as *mut RwLock<()>;
    
                let storage =
                    if let Some(storage) = archetype.component_storages.get(T::get_component_name()) {
                        storage
                    } else {
                        return Ok(vec![]);
                    };
    
                storage
                    .get(entity_index)
                    .map(|component| {
                        match component.as_any_ref().downcast_ref::<T>() {
                            Some(component) => {
                                Ok(ComponentReference::new(
                                    &self.on_entity_lock_address_moved,
                                    &self.on_component_address_moved,
                                    entity_lock_ptr,
                                    component as *const T as *mut T,
                                ))
                            },
                            None => Err(FruityError::GenericFailure(format!("You try to access a component with identifier {} in the entity named {} but the component don't match the required type", T::get_component_name(), self.get_name()?)))
                        }
    
                    })
                    .try_collect::<Vec<_>>()
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
    }

        fn get_archetype_inner(guard: RwLockReadGuard<InnerShareableEntityReference>) -> FruityResult<(usize, *mut Archetype)> {
            if let InnerShareableEntityReference::Archetype { entity_index, archetype_ptr } = guard.deref() {
                Ok((*entity_index, *archetype_ptr))
        } else {
            Err(FruityError::GenericFailure(
                "You try to access an entity that is still waiting to be inserted into an archetype".to_string(),
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

    /// Get entity enabled
    #[export]
    pub fn is_enabled(&self) -> FruityResult<bool> {
        Ok(self.read()?.is_enabled())
    }

    fn read_inner(&self) -> RwLockReadGuard<InnerShareableEntityReference> {
        self.inner.read()
    }
}

impl Debug for EntityReference {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
