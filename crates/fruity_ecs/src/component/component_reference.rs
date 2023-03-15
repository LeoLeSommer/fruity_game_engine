use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::component_guard::InternalReadGuard;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::entity::entity_reference::EntityReference;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::IntrospectFields;
use fruity_game_engine::introspect::IntrospectMethods;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityResult;
use std::any::TypeId;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

/// A reference over an entity stored into an Archetype
#[derive(Clone, FruityAny)]
pub struct ComponentReference {
    pub(crate) entity_reference: EntityReference,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
}

impl ComponentReference {
    /// Get the index of the component for identification
    pub fn get_index(&self) -> usize {
        self.component_index
    }

    /// Get a read access to the component
    pub fn read(&self) -> ComponentReadGuard<'_> {
        ComponentReadGuard {
            _guard: InternalReadGuard::Read(self.entity_reference.read()._guard.clone()),
            archetype_reader: Rc::new(self.entity_reference.archetype.read()),
            component_identifier: self.component_identifier.clone(),
            component_index: self.component_index,
        }
    }

    /// Get a write access to the component
    pub fn write(&self) -> ComponentWriteGuard<'_> {
        ComponentWriteGuard {
            _guard: self.entity_reference.write()._guard.clone(),
            archetype_reader: Rc::new(self.entity_reference.archetype.read()),
            component_identifier: self.component_identifier.clone(),
            component_index: self.component_index,
        }
    }

    /// Get a read access to the component
    pub fn read_typed<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'_, T>> {
        let component_reader = self.read();
        let component_type_id = component_reader.as_any_ref().type_id();

        if component_type_id == TypeId::of::<T>() {
            Some(TypedComponentReadGuard {
                component_reader,
                phantom: PhantomData {},
            })
        } else {
            None
        }
    }

    /// Get a write access to the component
    pub fn write_typed<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentWriteGuard<'_, T>> {
        let component_writer = self.write();
        let component_type_id = component_writer.as_any_ref().type_id();

        if component_type_id == TypeId::of::<T>() {
            Some(TypedComponentWriteGuard {
                component_writer,
                phantom: PhantomData {},
            })
        } else {
            None
        }
    }
}

impl Debug for ComponentReference {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectFields for ComponentReference {
    fn get_class_name(&self) -> FruityResult<String> {
        self.read().get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.read().get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.write().set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.read().get_field_value(name)
    }
}

impl IntrospectMethods for ComponentReference {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.read().get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.read().call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.read().get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.write().call_mut_method(name, args)
    }
}
