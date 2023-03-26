use super::component_guard::AnyComponentReadGuard;
use super::component_guard::AnyComponentWriteGuard;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::Component;
use crate::component::StaticComponent;
use crate::entity::entity_service::OnComponentAddressMoved;
use crate::entity::entity_service::OnEntityLockAddressMoved;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::IntrospectFields;
use fruity_game_engine::introspect::IntrospectMethods;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::typescript;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use fruity_game_engine::RwLockReadGuard;
use std::fmt::Debug;
use std::ptr::null_mut;
use std::ptr::NonNull;
use std::sync::Arc;

struct InnerShareableAnyComponentReference {
    entity_lock_ptr: *mut RwLock<()>,
    component_ptr: Option<NonNull<dyn Component>>,
}

// Safe cause archetypes are updated when a component is moved trough memory
unsafe impl Send for InnerShareableAnyComponentReference {}

// Safe cause archetypes are updated when a component is moved trough memory
unsafe impl Sync for InnerShareableAnyComponentReference {}

/// A reference over an entity stored into an Archetype
#[derive(FruityAny, Clone)]
#[typescript("type AnyComponentReference<T = unknown> = T")]
pub struct AnyComponentReference {
    inner: Arc<RwLock<InnerShareableAnyComponentReference>>,
    on_entity_lock_address_moved_handle: ObserverHandler<OnEntityLockAddressMoved>,
    on_component_address_moved_handle: ObserverHandler<OnComponentAddressMoved>,
}

impl Drop for AnyComponentReference {
    fn drop(&mut self) {
        self.on_entity_lock_address_moved_handle.dispose_by_ref();
        self.on_component_address_moved_handle.dispose_by_ref();
    }
}

impl AnyComponentReference {
    pub(crate) fn new(
        on_entity_lock_address_moved: &Signal<OnEntityLockAddressMoved>,
        on_component_address_moved: &Signal<OnComponentAddressMoved>,
        entity_lock_ptr: *mut RwLock<()>,
        component_ptr: Option<NonNull<dyn Component>>,
    ) -> Self {
        let inner = Arc::new(RwLock::new(InnerShareableAnyComponentReference {
            entity_lock_ptr,
            component_ptr,
        }));

        let inner2 = inner.clone();
        let inner3 = inner.clone();

        // Register memory move observers to update the entity reference inner pointers when the memory is moved
        let (on_entity_lock_address_moved_handle, on_component_address_moved_handle) = {
            let on_entity_lock_address_moved_handle =
                on_entity_lock_address_moved.add_observer(move |event| {
                    let mut inner_writer = inner2.write();
                    if !inner_writer.entity_lock_ptr.is_null()
                        && unsafe { NonNull::new_unchecked(inner_writer.entity_lock_ptr) }
                            == event.old
                    {
                        inner_writer.entity_lock_ptr = event.new;
                    }

                    Ok(())
                });

            let on_component_address_moved_handle =
                on_component_address_moved.add_observer(move |event| {
                    let mut inner_writer = inner3.write();
                    if let Some(component_ptr) = inner_writer.component_ptr {
                        if component_ptr == event.old {
                            inner_writer.component_ptr = event.new;
                        }
                    }

                    Ok(())
                });

            (
                on_entity_lock_address_moved_handle,
                on_component_address_moved_handle,
            )
        };

        Self {
            inner,
            on_entity_lock_address_moved_handle: on_entity_lock_address_moved_handle,
            on_component_address_moved_handle: on_component_address_moved_handle,
        }
    }

    /// Get a read access to the component
    pub fn read(&self) -> FruityResult<AnyComponentReadGuard<'_>> {
        let inner = self.read_inner();
        if !inner.entity_lock_ptr.is_null() {
            if let Some(component_ptr) = inner.component_ptr {
                Ok(AnyComponentReadGuard {
                    entity_guard: unsafe { inner.entity_lock_ptr.as_ref().unwrap().read() },
                    component_ptr: component_ptr,
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted component".to_string(),
                ))
            }
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted component".to_string(),
            ))
        }
    }

    /// Get a write access to the component
    pub fn write(&self) -> FruityResult<AnyComponentWriteGuard<'_>> {
        let inner = self.read_inner();
        if !inner.entity_lock_ptr.is_null() {
            if let Some(component_ptr) = inner.component_ptr {
                Ok(AnyComponentWriteGuard {
                    entity_guard: unsafe { inner.entity_lock_ptr.as_mut().unwrap().write() },
                    component_ptr: component_ptr,
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted component".to_string(),
                ))
            }
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted component".to_string(),
            ))
        }
    }

    /// Get a read access to the component
    pub fn read_typed<T: Component>(&self) -> FruityResult<ComponentReadGuard<'_, T>> {
        self.read()?.try_into()
    }

    /// Get a write access to the component
    pub fn write_typed<T: Component + StaticComponent>(
        &self,
    ) -> FruityResult<ComponentWriteGuard<'_, T>> {
        self.write()?.try_into()
    }

    fn read_inner(&self) -> RwLockReadGuard<InnerShareableAnyComponentReference> {
        self.inner.read()
    }
}

impl Debug for AnyComponentReference {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectFields for AnyComponentReference {
    fn is_static(&self) -> FruityResult<bool> {
        self.read()?.is_static()
    }

    fn get_class_name(&self) -> FruityResult<String> {
        self.read()?.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.read()?.get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.write()?.set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.read()?.get_field_value(name)
    }
}

impl IntrospectMethods for AnyComponentReference {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.read()?.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.read()?.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.read()?.get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.write()?.call_mut_method(name, args)
    }
}

struct InnerShareableComponentReference<T: Component> {
    entity_lock_ptr: *mut RwLock<()>,
    component_ptr: *mut T,
}

// Safe cause archetypes are updated when a component is moved trough memory
unsafe impl<T: Component> Send for InnerShareableComponentReference<T> {}

// Safe cause archetypes are updated when a component is moved trough memory
unsafe impl<T: Component> Sync for InnerShareableComponentReference<T> {}

/// A reference over an entity stored into an Archetype
/// The pointer are updated with an observer over the EntityService to catch memory updates
#[derive(FruityAny, Clone)]
pub struct ComponentReference<T: Component> {
    inner: Arc<RwLock<InnerShareableComponentReference<T>>>,
    _on_entity_lock_address_moved_handle: ObserverHandler<OnEntityLockAddressMoved>,
    _on_component_address_moved_handle: ObserverHandler<OnComponentAddressMoved>,
}

impl<T: Component> Drop for ComponentReference<T> {
    fn drop(&mut self) {
        self._on_entity_lock_address_moved_handle.dispose_by_ref();
        self._on_component_address_moved_handle.dispose_by_ref();
    }
}

impl<T: Component> ComponentReference<T> {
    pub(crate) fn new(
        on_entity_lock_address_moved: &Signal<OnEntityLockAddressMoved>,
        on_component_address_moved: &Signal<OnComponentAddressMoved>,
        entity_lock_ptr: *mut RwLock<()>,
        component_ptr: *mut T,
    ) -> Self {
        let inner = Arc::new(RwLock::new(InnerShareableComponentReference {
            entity_lock_ptr,
            component_ptr,
        }));

        let inner2 = inner.clone();
        let inner3 = inner.clone();

        // Register memory move observers to update the entity reference inner pointers when the memory is moved
        let (on_entity_lock_address_moved_handle, on_component_address_moved_handle) = {
            let on_entity_lock_address_moved_handle =
                on_entity_lock_address_moved.add_observer(move |event| {
                    let mut inner_writer = inner2.write();
                    if !inner_writer.entity_lock_ptr.is_null()
                        && unsafe { NonNull::new_unchecked(inner_writer.entity_lock_ptr) }
                            == event.old
                    {
                        inner_writer.entity_lock_ptr = event.new;
                    }

                    Ok(())
                });

            let on_component_address_moved_handle =
                on_component_address_moved.add_observer(move |event| {
                    let mut inner_writer = inner3.write();
                    let old = unsafe { event.old.as_ref() }
                        .as_any_ref()
                        .downcast_ref::<T>()
                        .unwrap() as *const T as *mut T;

                    if !inner_writer.component_ptr.is_null() && old == inner_writer.component_ptr {
                        if let Some(mut new) = event.new {
                            inner_writer.component_ptr = unsafe { new.as_mut() }
                                .as_any_mut()
                                .downcast_mut::<T>()
                                .unwrap()
                                as *mut T;
                        } else {
                            inner_writer.component_ptr = null_mut();
                        }
                    }

                    Ok(())
                });

            (
                on_entity_lock_address_moved_handle,
                on_component_address_moved_handle,
            )
        };

        Self {
            inner,
            _on_entity_lock_address_moved_handle: on_entity_lock_address_moved_handle,
            _on_component_address_moved_handle: on_component_address_moved_handle,
        }
    }

    /// Get a read access to the component
    pub fn read(&self) -> FruityResult<ComponentReadGuard<'_, T>> {
        let inner = self.read_inner();
        if !inner.entity_lock_ptr.is_null() && !inner.component_ptr.is_null() {
            Ok(ComponentReadGuard {
                entity_guard: unsafe { inner.entity_lock_ptr.as_ref().unwrap().read() },
                component_ptr: unsafe { NonNull::new_unchecked(inner.component_ptr) },
            })
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted component".to_string(),
            ))
        }
    }

    /// Get a write access to the component
    pub fn write(&self) -> FruityResult<ComponentWriteGuard<'_, T>> {
        let inner = self.read_inner();
        if !inner.entity_lock_ptr.is_null() && !inner.component_ptr.is_null() {
            Ok(ComponentWriteGuard {
                entity_guard: unsafe { inner.entity_lock_ptr.as_mut().unwrap().write() },
                component_ptr: unsafe { NonNull::new_unchecked(inner.component_ptr) },
            })
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted component".to_string(),
            ))
        }
    }

    /// Get a read access to the component
    pub fn read_any(&self) -> FruityResult<AnyComponentReadGuard<'_>> {
        let inner = self.read_inner();
        if !inner.entity_lock_ptr.is_null() && !inner.component_ptr.is_null() {
            Ok(AnyComponentReadGuard {
                entity_guard: unsafe { inner.entity_lock_ptr.as_ref().unwrap().read() },
                component_ptr: unsafe { NonNull::new_unchecked(inner.component_ptr) },
            })
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted component".to_string(),
            ))
        }
    }

    /// Get a write access to the component
    pub fn write_any(&self) -> FruityResult<AnyComponentWriteGuard<'_>> {
        let inner = self.read_inner();
        if !inner.entity_lock_ptr.is_null() && !inner.component_ptr.is_null() {
            Ok(AnyComponentWriteGuard {
                entity_guard: unsafe { inner.entity_lock_ptr.as_mut().unwrap().write() },
                component_ptr: unsafe { NonNull::new_unchecked(inner.component_ptr) },
            })
        } else {
            Err(FruityError::GenericFailure(
                "You try to access a deleted component".to_string(),
            ))
        }
    }

    fn read_inner(&self) -> RwLockReadGuard<InnerShareableComponentReference<T>> {
        self.inner.read()
    }
}

impl<T: Component> Debug for ComponentReference<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<T: Component> IntrospectFields for ComponentReference<T> {
    fn is_static(&self) -> FruityResult<bool> {
        self.read()?.is_static()
    }

    fn get_class_name(&self) -> FruityResult<String> {
        self.read()?.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.read()?.get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.write()?.set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.read()?.get_field_value(name)
    }
}

impl<T: Component> IntrospectMethods for ComponentReference<T> {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.read()?.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.read()?.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.read()?.get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.write()?.call_mut_method(name, args)
    }
}
