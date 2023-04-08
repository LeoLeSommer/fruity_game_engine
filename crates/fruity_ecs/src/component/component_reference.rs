use super::{
    AnyComponentReadGuard, AnyComponentWriteGuard, Component, ComponentReadGuard, ComponentTypeId,
    ComponentWriteGuard,
};
use crate::entity::EntityReference;
use fruity_game_engine::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    script_value::ScriptValue,
    FruityError, FruityResult,
};
use std::ptr::NonNull;

/// A reference to a component
#[derive(Debug, Clone)]
pub struct ComponentReference<T: Component> {
    entity_reference: EntityReference,
    component_index: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Component> ComponentReference<T> {
    /// Create a new component reference
    pub fn new(entity_reference: EntityReference, component_index: usize) -> Self {
        Self {
            entity_reference,
            component_index,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Read the component with a given type
    pub fn read(&self) -> FruityResult<ComponentReadGuard<'_, T>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&ComponentTypeId::of::<T>())
                    .unwrap();
                let storage_reader = storage.read();

                let component_ptr = NonNull::from(
                    storage_reader
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(ComponentReadGuard {
                    storage_guard: storage_reader,
                    component_ptr: component_ptr.cast(),
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }

    /// Write the component with a given type
    pub fn write(&self) -> FruityResult<ComponentWriteGuard<'_, T>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&ComponentTypeId::of::<T>())
                    .unwrap();
                let storage_writer = storage.write();

                let component_ptr = NonNull::from(
                    storage_writer
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(ComponentWriteGuard {
                    storage_guard: storage_writer,
                    component_ptr: component_ptr.cast(),
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }

    /// Read the component with an any type
    pub fn read_any(&self) -> FruityResult<AnyComponentReadGuard<'_>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&ComponentTypeId::of::<T>())
                    .unwrap();
                let storage_reader = storage.read();

                let component_ptr = NonNull::from(
                    storage_reader
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(AnyComponentReadGuard {
                    storage_guard: storage_reader,
                    component_ptr,
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }

    /// Write the component with an any type
    pub fn write_any(&self) -> FruityResult<AnyComponentWriteGuard<'_>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&ComponentTypeId::of::<T>())
                    .unwrap();
                let storage_writer = storage.write();

                let component_ptr = NonNull::from(
                    storage_writer
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(AnyComponentWriteGuard {
                    storage_guard: storage_writer,
                    component_ptr,
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }
}

/// An any reference to a component
#[derive(Debug, Clone, FruityAny)]
pub struct AnyComponentReference {
    entity_reference: EntityReference,
    component_type_id: ComponentTypeId,
    component_index: usize,
}

impl AnyComponentReference {
    /// Create a new component reference
    pub fn new(
        entity_reference: EntityReference,
        component_type_id: ComponentTypeId,
        component_index: usize,
    ) -> Self {
        Self {
            entity_reference,
            component_type_id,
            component_index,
        }
    }

    /// Read the component with a given type
    pub fn read<T: Component>(&self) -> FruityResult<ComponentReadGuard<'_, T>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&self.component_type_id)
                    .unwrap();
                let storage_reader = storage.read();

                let component_ptr = NonNull::from(
                    storage_reader
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(ComponentReadGuard {
                    storage_guard: storage_reader,
                    component_ptr: component_ptr.cast(),
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }

    /// Write the component with a given type
    pub fn write<T: Component>(&self) -> FruityResult<ComponentWriteGuard<'_, T>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&self.component_type_id)
                    .unwrap();
                let storage_writer = storage.write();

                let component_ptr = NonNull::from(
                    storage_writer
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(ComponentWriteGuard {
                    storage_guard: storage_writer,
                    component_ptr: component_ptr.cast(),
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }

    /// Read the component with an any type
    pub fn read_any(&self) -> FruityResult<AnyComponentReadGuard<'_>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&self.component_type_id)
                    .unwrap();
                let storage_reader = storage.read();

                let component_ptr = NonNull::from(
                    storage_reader
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(AnyComponentReadGuard {
                    storage_guard: storage_reader,
                    component_ptr: component_ptr,
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }

    /// Write the component with an any type
    pub fn write_any(&self) -> FruityResult<AnyComponentWriteGuard<'_>> {
        unsafe {
            let entity_reference_inner = self.entity_reference.inner.read();
            if let Some(entity_reference_inner) = entity_reference_inner.as_ref() {
                let entity_storage_reader = entity_reference_inner.entity_storage.read();

                let archetype = entity_storage_reader
                    .archetypes
                    .get_unchecked(entity_reference_inner.location.archetype.0);
                let archetype = NonNull::from(archetype).as_ref();

                let storage = archetype
                    .component_storages
                    .get(&self.component_type_id)
                    .unwrap();
                let storage_writer = storage.write();

                let component_ptr = NonNull::from(
                    storage_writer
                        .get_unchecked(entity_reference_inner.location.index, self.component_index),
                );

                Ok(AnyComponentWriteGuard {
                    storage_guard: storage_writer,
                    component_ptr: component_ptr,
                })
            } else {
                Err(FruityError::GenericFailure(
                    "You try to access a deleted entity".to_string(),
                ))
            }
        }
    }
}

impl IntrospectFields for AnyComponentReference {
    fn is_static(&self) -> FruityResult<bool> {
        self.read_any()?.is_static()
    }

    fn get_class_name(&self) -> FruityResult<String> {
        self.read_any()?.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.read_any()?.get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.write_any()?.set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.read_any()?.get_field_value(name)
    }
}

impl IntrospectMethods for AnyComponentReference {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.read_any()?.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.read_any()?.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.read_any()?.get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.write_any()?.call_mut_method(name, args)
    }
}
