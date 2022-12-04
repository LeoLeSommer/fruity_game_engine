#![feature(prelude_import)]
#![warn(missing_docs)]
#![feature(iterator_try_collect)]
//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::rc::Rc;
use crate::entity::entity_service::EntityService;
use crate::extension_component_service::ExtensionComponentService;
use crate::system_service::SystemService;
pub use fruity_ecs_macro::Component;
use fruity_game_engine::module::Module;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::send_wrapper::SendWrapper;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::world::World;
use fruity_game_engine::{export_function, export_value, lazy_static, FruityResult};
/// All related with components
pub mod component {
    /// Provides an abstraction over a component
    pub mod component {
        use crate::entity::archetype::component_collection::ComponentCollection;
        pub use fruity_ecs_macro::Component;
        use fruity_game_engine::any::FruityAny;
        use fruity_game_engine::introspect::{IntrospectFields, IntrospectMethods};
        use fruity_game_engine::javascript::JsIntrospectObject;
        use fruity_game_engine::script_value::convert::TryFromScriptValue;
        use fruity_game_engine::script_value::ScriptValue;
        use fruity_game_engine::send_wrapper::SendWrapper;
        use fruity_game_engine::{FruityError, FruityResult};
        use std::fmt::Debug;
        use std::ops::Deref;
        use super::script_component::ScriptComponent;
        /// An abstraction over a component, should be implemented for every component
        pub trait StaticComponent {
            /// Return the class type name
            fn get_component_name() -> &'static str;
        }
        /// An abstraction over a component, should be implemented for every component
        pub trait Component: IntrospectFields + IntrospectMethods + Debug + Send + Sync {
            /// Get a collection to store this component in the archetype
            fn get_collection(&self) -> Box<dyn ComponentCollection>;
            /// Create a new component that is a clone of self
            fn duplicate(&self) -> Box<dyn Component>;
        }
        impl Clone for Box<dyn Component> {
            fn clone(&self) -> Self {
                self.duplicate()
            }
        }
        /// An container for a component without knowing the instancied type
        pub struct AnyComponent {
            component: Box<dyn Component>,
        }
        impl ::fruity_game_engine::any::FruityAny for AnyComponent {
            fn get_type_name(&self) -> &'static str {
                "AnyComponent"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AnyComponent {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AnyComponent",
                    "component",
                    &&self.component,
                )
            }
        }
        impl AnyComponent {
            /// Returns an AnyComponent
            pub fn new(component: impl Component) -> AnyComponent {
                AnyComponent {
                    component: Box::new(component),
                }
            }
            /// Returns an AnyComponent
            pub fn into_box(self) -> Box<dyn Component> {
                self.component
            }
        }
        impl From<Box<dyn Component>> for AnyComponent {
            fn from(component: Box<dyn Component>) -> Self {
                Self { component }
            }
        }
        impl From<JsIntrospectObject> for AnyComponent {
            fn from(value: JsIntrospectObject) -> Self {
                Self {
                    component: Box::new(ScriptComponent(SendWrapper::new(value))),
                }
            }
        }
        impl Deref for AnyComponent {
            type Target = dyn Component;
            fn deref(&self) -> &<Self as std::ops::Deref>::Target {
                self.component.as_ref()
            }
        }
        impl TryFromScriptValue for AnyComponent {
            fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
                match value {
                    ScriptValue::Object(value) => {
                        match value.downcast::<Box<dyn Component>>() {
                            Ok(value) => Ok(AnyComponent::from(*value)),
                            Err(value) => {
                                match value.downcast::<JsIntrospectObject>() {
                                    Ok(value) => Ok(AnyComponent::from(*value)),
                                    Err(value) => {
                                        Err(
                                            FruityError::InvalidArg({
                                                let res = ::alloc::fmt::format(
                                                    ::core::fmt::Arguments::new_v1(
                                                        &["Couldn\'t convert a ", " to Component"],
                                                        &[
                                                            ::core::fmt::ArgumentV1::new_display(&value.get_type_name()),
                                                        ],
                                                    ),
                                                );
                                                res
                                            }),
                                        )
                                    }
                                }
                            }
                        }
                    }
                    value => {
                        Err(
                            FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Couldn\'t convert ", " to native object"],
                                        &[::core::fmt::ArgumentV1::new_debug(&value)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
    }
    /// Provides reference over a component
    pub mod component_reference {
        use crate::component::component::Component;
        use crate::component::component::StaticComponent;
        use crate::component::component_guard::ComponentReadGuard;
        use crate::component::component_guard::ComponentWriteGuard;
        use crate::component::component_guard::InternalReadGuard;
        use crate::component::component_guard::TypedComponentReadGuard;
        use crate::component::component_guard::TypedComponentWriteGuard;
        use crate::entity::entity_guard::EntityReadGuard;
        use crate::entity::entity_guard::EntityWriteGuard;
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
        pub struct ComponentReference {
            pub(crate) entity_reference: EntityReference,
            pub(crate) component_identifier: String,
            pub(crate) component_index: usize,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ComponentReference {
            #[inline]
            fn clone(&self) -> ComponentReference {
                ComponentReference {
                    entity_reference: ::core::clone::Clone::clone(
                        &self.entity_reference,
                    ),
                    component_identifier: ::core::clone::Clone::clone(
                        &self.component_identifier,
                    ),
                    component_index: ::core::clone::Clone::clone(&self.component_index),
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for ComponentReference {
            fn get_type_name(&self) -> &'static str {
                "ComponentReference"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl ComponentReference {
            /// Get the index of the component for identification
            pub fn get_index(&self) -> usize {
                self.component_index
            }
            /// Get a read access to the component
            pub fn read(&self) -> ComponentReadGuard<'_> {
                ComponentReadGuard {
                    _guard: InternalReadGuard::Read(
                        self.entity_reference.read()._guard.clone(),
                    ),
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
            /// Get a read access to the entity
            pub fn read_entity(&self) -> EntityReadGuard<'_> {
                self.entity_reference.read()
            }
            /// Get a write access to the entity
            pub fn write_entity(&self) -> EntityWriteGuard<'_> {
                self.entity_reference.write()
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
            fn set_field_value(
                &mut self,
                name: &str,
                value: ScriptValue,
            ) -> FruityResult<()> {
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
            fn call_const_method(
                &self,
                name: &str,
                args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                self.read().call_const_method(name, args)
            }
            fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
                self.read().get_mut_method_names()
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                self.write().call_mut_method(name, args)
            }
        }
    }
    /// Provides guards over a component
    pub mod component_guard {
        use crate::component::component::Component;
        use crate::component::component::StaticComponent;
        use crate::entity::archetype::Archetype;
        use fruity_game_engine::RwLockReadGuard;
        use fruity_game_engine::RwLockWriteGuard;
        use std::fmt::Debug;
        use std::fmt::Formatter;
        use std::marker::PhantomData;
        use std::ops::Deref;
        use std::ops::DerefMut;
        use std::rc::Rc;
        pub(crate) enum InternalReadGuard<'a> {
            Read(Rc<RwLockReadGuard<'a, ()>>),
            Write(Rc<RwLockWriteGuard<'a, ()>>),
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for InternalReadGuard<'a> {
            #[inline]
            fn clone(&self) -> InternalReadGuard<'a> {
                match self {
                    InternalReadGuard::Read(__self_0) => {
                        InternalReadGuard::Read(::core::clone::Clone::clone(__self_0))
                    }
                    InternalReadGuard::Write(__self_0) => {
                        InternalReadGuard::Write(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        /// RAII structure used to release the shared read access of a lock when dropped.
        ///
        /// This structure is created by the [`read`] methods on [`ComponentReference`].
        ///
        /// [`read`]: ComponentReference::read
        ///
        pub struct ComponentReadGuard<'a> {
            pub(crate) _guard: InternalReadGuard<'a>,
            pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
            pub(crate) component_identifier: String,
            pub(crate) component_index: usize,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for ComponentReadGuard<'a> {
            #[inline]
            fn clone(&self) -> ComponentReadGuard<'a> {
                ComponentReadGuard {
                    _guard: ::core::clone::Clone::clone(&self._guard),
                    archetype_reader: ::core::clone::Clone::clone(
                        &self.archetype_reader,
                    ),
                    component_identifier: ::core::clone::Clone::clone(
                        &self.component_identifier,
                    ),
                    component_index: ::core::clone::Clone::clone(&self.component_index),
                }
            }
        }
        impl<'a> ComponentReadGuard<'a> {
            /// Get an extension component reader
            pub fn get_extension(
                &self,
                extension_identifier: &str,
            ) -> Option<ComponentReadGuard<'a>> {
                let has_extension = self
                    .archetype_reader
                    .component_storages
                    .contains_key(extension_identifier);
                if has_extension {
                    Some(ComponentReadGuard {
                        _guard: self._guard.clone(),
                        archetype_reader: self.archetype_reader.clone(),
                        component_identifier: extension_identifier.to_string(),
                        component_index: self.component_index.clone(),
                    })
                } else {
                    None
                }
            }
        }
        impl<'a> Debug for ComponentReadGuard<'a> {
            fn fmt(
                &self,
                _: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                Ok(())
            }
        }
        impl<'a> Deref for ComponentReadGuard<'a> {
            type Target = dyn Component;
            fn deref(&self) -> &Self::Target {
                let storage = self
                    .archetype_reader
                    .component_storages
                    .get(&self.component_identifier)
                    .unwrap();
                storage.collection.get(&self.component_index).unwrap()
            }
        }
        impl<'a, T: Component + StaticComponent> TryInto<TypedComponentReadGuard<'a, T>>
        for ComponentReadGuard<'a> {
            type Error = String;
            fn try_into(self) -> Result<TypedComponentReadGuard<'a, T>, Self::Error> {
                match self.as_any_ref().downcast_ref::<T>() {
                    Some(_result) => {
                        Ok(TypedComponentReadGuard {
                            component_reader: self,
                            phantom: PhantomData::<T> {},
                        })
                    }
                    None => {
                        Err({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to typed component"],
                                    &[::core::fmt::ArgumentV1::new_debug(&self)],
                                ),
                            );
                            res
                        })
                    }
                }
            }
        }
        /// RAII structure used to release the shared write access of a lock when dropped.
        ///
        /// This structure is created by the [`write`] methods on [`ComponentReference`].
        ///
        /// [`write`]: ComponentReference::write
        ///
        pub struct ComponentWriteGuard<'a> {
            pub(crate) _guard: Rc<RwLockWriteGuard<'a, ()>>,
            pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
            pub(crate) component_identifier: String,
            pub(crate) component_index: usize,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for ComponentWriteGuard<'a> {
            #[inline]
            fn clone(&self) -> ComponentWriteGuard<'a> {
                ComponentWriteGuard {
                    _guard: ::core::clone::Clone::clone(&self._guard),
                    archetype_reader: ::core::clone::Clone::clone(
                        &self.archetype_reader,
                    ),
                    component_identifier: ::core::clone::Clone::clone(
                        &self.component_identifier,
                    ),
                    component_index: ::core::clone::Clone::clone(&self.component_index),
                }
            }
        }
        impl<'a> ComponentWriteGuard<'a> {
            /// Get an extension component reader
            pub fn get_extension(
                &self,
                extension_identifier: &str,
            ) -> Option<ComponentReadGuard<'a>> {
                let has_extension = self
                    .archetype_reader
                    .component_storages
                    .contains_key(extension_identifier);
                if has_extension {
                    Some(ComponentReadGuard {
                        _guard: InternalReadGuard::Write(self._guard.clone()),
                        archetype_reader: self.archetype_reader.clone(),
                        component_identifier: extension_identifier.to_string(),
                        component_index: self.component_index.clone(),
                    })
                } else {
                    None
                }
            }
            /// Get an extension component writer
            pub fn get_extension_mut(
                &self,
                extension_identifier: &str,
            ) -> Option<ComponentWriteGuard<'a>> {
                let has_extension = self
                    .archetype_reader
                    .component_storages
                    .contains_key(extension_identifier);
                if has_extension {
                    Some(ComponentWriteGuard {
                        _guard: self._guard.clone(),
                        archetype_reader: self.archetype_reader.clone(),
                        component_identifier: extension_identifier.to_string(),
                        component_index: self.component_index.clone(),
                    })
                } else {
                    None
                }
            }
        }
        impl<'a> Debug for ComponentWriteGuard<'a> {
            fn fmt(
                &self,
                _: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                Ok(())
            }
        }
        impl<'a> Deref for ComponentWriteGuard<'a> {
            type Target = dyn Component;
            fn deref(&self) -> &Self::Target {
                let storage = self
                    .archetype_reader
                    .component_storages
                    .get(&self.component_identifier)
                    .unwrap();
                storage.collection.get(&self.component_index).unwrap()
            }
        }
        impl<'a> DerefMut for ComponentWriteGuard<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                let storage = self
                    .archetype_reader
                    .component_storages
                    .get(&self.component_identifier)
                    .unwrap();
                let component = storage.collection.get(&self.component_index).unwrap();
                #[allow(mutable_transmutes)]
                unsafe {
                    std::mem::transmute::<&dyn Component, &mut dyn Component>(component)
                }
            }
        }
        impl<'a, T: Component + StaticComponent> TryInto<TypedComponentWriteGuard<'a, T>>
        for ComponentWriteGuard<'a> {
            type Error = String;
            fn try_into(self) -> Result<TypedComponentWriteGuard<'a, T>, Self::Error> {
                match self.as_any_ref().downcast_ref::<T>() {
                    Some(_result) => {
                        Ok(TypedComponentWriteGuard {
                            component_writer: self,
                            phantom: PhantomData::<T> {},
                        })
                    }
                    None => {
                        Err({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to typed component"],
                                    &[::core::fmt::ArgumentV1::new_debug(&self)],
                                ),
                            );
                            res
                        })
                    }
                }
            }
        }
        /// RAII structure used to release the shared read access of a lock when dropped.
        ///
        /// This structure is created by the [`read`] methods on [`ComponentReference`].
        ///
        /// [`read`]: ComponentReference::read
        ///
        pub struct TypedComponentReadGuard<'a, T: Component + StaticComponent> {
            pub(crate) component_reader: ComponentReadGuard<'a>,
            pub(crate) phantom: PhantomData<T>,
        }
        impl<'a, T: Component + StaticComponent> TypedComponentReadGuard<'a, T> {
            /// Get an extension component reader
            pub fn get_extension<E: Component + StaticComponent>(
                &self,
            ) -> Option<TypedComponentReadGuard<'a, E>> {
                self.component_reader
                    .get_extension(E::get_component_name())
                    .map(|component_reader| TypedComponentReadGuard::<'a, E> {
                        component_reader,
                        phantom: PhantomData {},
                    })
            }
        }
        impl<'a, T: Component + StaticComponent> Clone
        for TypedComponentReadGuard<'a, T> {
            fn clone(&self) -> Self {
                Self {
                    component_reader: self.component_reader.clone(),
                    phantom: PhantomData {},
                }
            }
        }
        impl<'a, T: Component + StaticComponent> Debug
        for TypedComponentReadGuard<'a, T> {
            fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
                self.deref().fmt(formatter)
            }
        }
        impl<'a, T: Component + StaticComponent> Deref
        for TypedComponentReadGuard<'a, T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                self.component_reader.as_any_ref().downcast_ref::<T>().unwrap()
            }
        }
        /// RAII structure used to release the shared write access of a lock when dropped.
        ///
        /// This structure is created by the [`write`] methods on [`ComponentReference`].
        ///
        /// [`write`]: ComponentReference::write
        ///
        pub struct TypedComponentWriteGuard<'a, T: Component + StaticComponent> {
            pub(crate) component_writer: ComponentWriteGuard<'a>,
            pub(crate) phantom: PhantomData<T>,
        }
        impl<'a, T: Component + StaticComponent> TypedComponentWriteGuard<'a, T> {
            /// Get an extension component reader
            pub fn get_extension<E: Component + StaticComponent>(
                &self,
            ) -> Option<TypedComponentReadGuard<'a, E>> {
                self.component_writer
                    .get_extension(E::get_component_name())
                    .map(|component_reader| TypedComponentReadGuard::<'a, E> {
                        component_reader,
                        phantom: PhantomData {},
                    })
            }
            /// Get an extension component writer
            pub fn get_extension_mut<E: Component + StaticComponent>(
                &self,
            ) -> Option<TypedComponentWriteGuard<'a, E>> {
                self.component_writer
                    .get_extension_mut(E::get_component_name())
                    .map(|component_writer| TypedComponentWriteGuard::<'a, E> {
                        component_writer,
                        phantom: PhantomData {},
                    })
            }
        }
        impl<'a, T: Component + StaticComponent> Clone
        for TypedComponentWriteGuard<'a, T> {
            fn clone(&self) -> Self {
                Self {
                    component_writer: self.component_writer.clone(),
                    phantom: PhantomData {},
                }
            }
        }
        impl<'a, T: Component + StaticComponent> Debug
        for TypedComponentWriteGuard<'a, T> {
            fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
                self.deref().fmt(formatter)
            }
        }
        impl<'a, T: Component + StaticComponent> Deref
        for TypedComponentWriteGuard<'a, T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                self.component_writer.as_any_ref().downcast_ref::<T>().unwrap()
            }
        }
        impl<'a, T: Component + StaticComponent> DerefMut
        for TypedComponentWriteGuard<'a, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.component_writer.as_any_mut().downcast_mut::<T>().unwrap()
            }
        }
    }
    /// Provide a component that contains a script value
    pub mod script_component {
        use super::component::Component;
        use crate::entity::archetype::{
            component_array::ComponentArray, component_collection::ComponentCollection,
        };
        use fruity_game_engine::{
            any::FruityAny, introspect::{IntrospectFields, IntrospectMethods},
            javascript::JsIntrospectObject, script_value::ScriptValue,
            send_wrapper::SendWrapper, FruityResult,
        };
        /// Provide a component that contains a script value
        pub struct ScriptComponent(pub SendWrapper<JsIntrospectObject>);
        impl ::fruity_game_engine::any::FruityAny for ScriptComponent {
            fn get_type_name(&self) -> &'static str {
                "ScriptComponent"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ScriptComponent {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ScriptComponent",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ScriptComponent {
            #[inline]
            fn clone(&self) -> ScriptComponent {
                ScriptComponent(::core::clone::Clone::clone(&self.0))
            }
        }
        impl IntrospectFields for ScriptComponent {
            fn get_class_name(&self) -> FruityResult<String> {
                self.0.get_class_name()
            }
            fn get_field_names(&self) -> FruityResult<Vec<String>> {
                self.0.get_field_names()
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ScriptValue,
            ) -> FruityResult<()> {
                self.0.set_field_value(name, value)
            }
            fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
                self.0.get_field_value(name)
            }
        }
        impl IntrospectMethods for ScriptComponent {
            fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
                self.0.get_const_method_names()
            }
            fn call_const_method(
                &self,
                name: &str,
                args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                self.0.call_const_method(name, args)
            }
            fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
                self.0.get_mut_method_names()
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                args: Vec<ScriptValue>,
            ) -> FruityResult<ScriptValue> {
                self.0.call_mut_method(name, args)
            }
        }
        impl Component for ScriptComponent {
            fn get_collection(&self) -> Box<dyn ComponentCollection> {
                Box::new(ComponentArray::<ScriptComponent>::new())
            }
            fn duplicate(&self) -> Box<dyn Component> {
                Box::new(self.clone())
            }
        }
    }
}
/// All related with entities
pub mod entity {
    /// Provides an abstraction over an entity and collections to store entities
    pub mod entity {
        use fruity_game_engine::FruityResult;
        use crate::component::component::AnyComponent;
        use crate::component::component::Component;
        use std::fmt::Debug;
        use std::hash::Hash;
        /// An identifier to an entity type, is composed be the identifier of the contained components
        pub struct EntityTypeIdentifier(pub Vec<String>);
        #[automatically_derived]
        impl ::core::fmt::Debug for EntityTypeIdentifier {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "EntityTypeIdentifier",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for EntityTypeIdentifier {
            #[inline]
            fn clone(&self) -> EntityTypeIdentifier {
                EntityTypeIdentifier(::core::clone::Clone::clone(&self.0))
            }
        }
        impl PartialEq for EntityTypeIdentifier {
            fn eq(&self, other: &EntityTypeIdentifier) -> bool {
                let matching = self
                    .0
                    .iter()
                    .zip(other.0.iter())
                    .filter(|&(a, b)| a == b)
                    .count();
                matching == self.0.len() && matching == other.0.len()
            }
        }
        impl Eq for EntityTypeIdentifier {}
        impl Hash for EntityTypeIdentifier {
            fn hash<H>(&self, state: &mut H)
            where
                H: std::hash::Hasher,
            {
                self.0.hash(state)
            }
        }
        impl EntityTypeIdentifier {
            /// Check if an entity identifier contains an other one
            /// For example ["c1", "c2", "c3"] contains ["c3", "c2"]
            pub fn contains(&self, other: &String) -> bool {
                self.0.contains(other)
            }
        }
        /// An identifier for an entity
        pub type EntityId = u64;
        /// Get the entity type identifier from a list of components
        pub fn get_type_identifier_by_any(
            components: &[AnyComponent],
        ) -> FruityResult<EntityTypeIdentifier> {
            let identifier = components
                .iter()
                .map(|component| component.get_class_name())
                .try_collect::<Vec<_>>()?;
            Ok(EntityTypeIdentifier(identifier))
        }
        /// Get the entity type identifier from a list of components
        pub fn get_type_identifier(
            components: &[&dyn Component],
        ) -> FruityResult<EntityTypeIdentifier> {
            let identifier = components
                .iter()
                .map(|component| component.get_class_name())
                .try_collect::<Vec<_>>()?;
            Ok(EntityTypeIdentifier(identifier))
        }
    }
    /// Provides a query over entities with given types
    pub mod entity_query {
        use crate::entity::archetype::Archetype;
        use crate::entity::archetype::ArchetypeArcRwLock;
        use crate::entity::entity::EntityId;
        use crate::entity::entity_guard::EntityReadGuard;
        use crate::entity::entity_guard::EntityWriteGuard;
        use crate::entity::entity_reference::EntityReference;
        use crate::EntityService;
        use fruity_game_engine::inject::Injectable;
        use fruity_game_engine::resource::resource_container::ResourceContainer;
        use fruity_game_engine::signal::ObserverHandler;
        use fruity_game_engine::signal::Signal;
        use fruity_game_engine::FruityResult;
        use fruity_game_engine::RwLock;
        use rayon::iter::ParallelBridge;
        use rayon::iter::ParallelIterator;
        use std::marker::PhantomData;
        use std::sync::Arc;
        /// Queries for scripting languages
        pub(crate) mod script {
            use crate::entity::archetype::Archetype;
            use crate::entity::archetype::ArchetypeArcRwLock;
            use crate::entity::entity_query::script::params::With;
            use crate::entity::entity_query::script::params::WithEnabled;
            use crate::entity::entity_query::script::params::WithEntity;
            use crate::entity::entity_query::script::params::WithId;
            use crate::entity::entity_query::script::params::WithName;
            use crate::entity::entity_query::script::params::WithOptional;
            use crate::entity::entity_query::EntityId;
            use crate::entity::entity_reference::EntityReference;
            use fruity_game_engine::any::FruityAny;
            use fruity_game_engine::script_value::ScriptCallback;
            use fruity_game_engine::script_value::ScriptValue;
            use fruity_game_engine::signal::ObserverHandler;
            use fruity_game_engine::signal::Signal;
            use fruity_game_engine::FruityError;
            use fruity_game_engine::FruityResult;
            use fruity_game_engine::RwLock;
            use fruity_game_engine::{export, export_impl, export_struct};
            use itertools::Itertools;
            use std::fmt::Debug;
            use std::rc::Rc;
            use std::sync::Arc;
            pub(crate) mod params {
                use crate::entity::archetype::Archetype;
                use crate::entity::entity_query::script::ScriptQueryParam;
                use crate::entity::entity_reference::EntityReference;
                use fruity_game_engine::any::FruityAny;
                use fruity_game_engine::script_value::convert::TryIntoScriptValue;
                use fruity_game_engine::script_value::ScriptValue;
                use fruity_game_engine::FruityResult;
                pub struct WithEntity {}
                impl ::fruity_game_engine::any::FruityAny for WithEntity {
                    fn get_type_name(&self) -> &'static str {
                        "WithEntity"
                    }
                    fn as_any_ref(&self) -> &dyn std::any::Any {
                        self
                    }
                    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                        self
                    }
                    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                        self
                    }
                    fn as_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                    fn as_fruity_any_ref(
                        &self,
                    ) -> &dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_mut(
                        &mut self,
                    ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_box(
                        self: Box<Self>,
                    ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                    fn as_fruity_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for WithEntity {
                    #[inline]
                    fn clone(&self) -> WithEntity {
                        WithEntity {}
                    }
                }
                impl ScriptQueryParam for WithEntity {
                    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
                        Box::new(self.clone())
                    }
                    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
                        true
                    }
                    fn get_entity_components(
                        &self,
                        entity_reference: EntityReference,
                    ) -> FruityResult<Vec<ScriptValue>> {
                        Ok(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    entity_reference.into_script_value()?,
                                ]),
                            ),
                        )
                    }
                }
                pub struct WithId {}
                impl ::fruity_game_engine::any::FruityAny for WithId {
                    fn get_type_name(&self) -> &'static str {
                        "WithId"
                    }
                    fn as_any_ref(&self) -> &dyn std::any::Any {
                        self
                    }
                    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                        self
                    }
                    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                        self
                    }
                    fn as_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                    fn as_fruity_any_ref(
                        &self,
                    ) -> &dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_mut(
                        &mut self,
                    ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_box(
                        self: Box<Self>,
                    ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                    fn as_fruity_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for WithId {
                    #[inline]
                    fn clone(&self) -> WithId {
                        WithId {}
                    }
                }
                impl ScriptQueryParam for WithId {
                    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
                        Box::new(self.clone())
                    }
                    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
                        true
                    }
                    fn get_entity_components(
                        &self,
                        entity_reference: EntityReference,
                    ) -> FruityResult<Vec<ScriptValue>> {
                        let entity_reader = entity_reference.read();
                        Ok(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    entity_reader.get_entity_id().into_script_value()?,
                                ]),
                            ),
                        )
                    }
                }
                pub struct WithName {}
                impl ::fruity_game_engine::any::FruityAny for WithName {
                    fn get_type_name(&self) -> &'static str {
                        "WithName"
                    }
                    fn as_any_ref(&self) -> &dyn std::any::Any {
                        self
                    }
                    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                        self
                    }
                    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                        self
                    }
                    fn as_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                    fn as_fruity_any_ref(
                        &self,
                    ) -> &dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_mut(
                        &mut self,
                    ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_box(
                        self: Box<Self>,
                    ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                    fn as_fruity_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for WithName {
                    #[inline]
                    fn clone(&self) -> WithName {
                        WithName {}
                    }
                }
                impl ScriptQueryParam for WithName {
                    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
                        Box::new(self.clone())
                    }
                    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
                        true
                    }
                    fn get_entity_components(
                        &self,
                        entity_reference: EntityReference,
                    ) -> FruityResult<Vec<ScriptValue>> {
                        let entity_reader = entity_reference.read();
                        Ok(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    entity_reader.get_name().into_script_value()?,
                                ]),
                            ),
                        )
                    }
                }
                pub struct WithEnabled {}
                impl ::fruity_game_engine::any::FruityAny for WithEnabled {
                    fn get_type_name(&self) -> &'static str {
                        "WithEnabled"
                    }
                    fn as_any_ref(&self) -> &dyn std::any::Any {
                        self
                    }
                    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                        self
                    }
                    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                        self
                    }
                    fn as_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                    fn as_fruity_any_ref(
                        &self,
                    ) -> &dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_mut(
                        &mut self,
                    ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_box(
                        self: Box<Self>,
                    ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                    fn as_fruity_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for WithEnabled {
                    #[inline]
                    fn clone(&self) -> WithEnabled {
                        WithEnabled {}
                    }
                }
                impl ScriptQueryParam for WithEnabled {
                    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
                        Box::new(self.clone())
                    }
                    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
                        true
                    }
                    fn get_entity_components(
                        &self,
                        entity_reference: EntityReference,
                    ) -> FruityResult<Vec<ScriptValue>> {
                        let entity_reader = entity_reference.read();
                        Ok(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    entity_reader.is_enabled().into_script_value()?,
                                ]),
                            ),
                        )
                    }
                }
                pub struct With {
                    pub identifier: String,
                }
                impl ::fruity_game_engine::any::FruityAny for With {
                    fn get_type_name(&self) -> &'static str {
                        "With"
                    }
                    fn as_any_ref(&self) -> &dyn std::any::Any {
                        self
                    }
                    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                        self
                    }
                    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                        self
                    }
                    fn as_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                    fn as_fruity_any_ref(
                        &self,
                    ) -> &dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_mut(
                        &mut self,
                    ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_box(
                        self: Box<Self>,
                    ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                    fn as_fruity_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for With {
                    #[inline]
                    fn clone(&self) -> With {
                        With {
                            identifier: ::core::clone::Clone::clone(&self.identifier),
                        }
                    }
                }
                impl ScriptQueryParam for With {
                    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
                        Box::new(self.clone())
                    }
                    fn filter_archetype(&self, archetype: &Archetype) -> bool {
                        archetype.identifier.contains(&self.identifier)
                    }
                    fn get_entity_components(
                        &self,
                        entity_reference: EntityReference,
                    ) -> FruityResult<Vec<ScriptValue>> {
                        entity_reference
                            .get_components_by_type_identifier(&self.identifier)
                            .into_iter()
                            .map(|component| component.into_script_value())
                            .try_collect::<Vec<_>>()
                    }
                }
                pub struct WithOptional {
                    pub identifier: String,
                }
                impl ::fruity_game_engine::any::FruityAny for WithOptional {
                    fn get_type_name(&self) -> &'static str {
                        "WithOptional"
                    }
                    fn as_any_ref(&self) -> &dyn std::any::Any {
                        self
                    }
                    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                        self
                    }
                    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                        self
                    }
                    fn as_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn std::any::Any> {
                        self
                    }
                    fn as_fruity_any_ref(
                        &self,
                    ) -> &dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_mut(
                        &mut self,
                    ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                        self
                    }
                    fn as_fruity_any_box(
                        self: Box<Self>,
                    ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                    fn as_fruity_any_rc(
                        self: std::rc::Rc<Self>,
                    ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                        self
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for WithOptional {
                    #[inline]
                    fn clone(&self) -> WithOptional {
                        WithOptional {
                            identifier: ::core::clone::Clone::clone(&self.identifier),
                        }
                    }
                }
                impl ScriptQueryParam for WithOptional {
                    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
                        Box::new(self.clone())
                    }
                    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
                        true
                    }
                    fn get_entity_components(
                        &self,
                        entity_reference: EntityReference,
                    ) -> FruityResult<Vec<ScriptValue>> {
                        let components = entity_reference
                            .get_components_by_type_identifier(&self.identifier)
                            .into_iter()
                            .map(|component| component.into_script_value())
                            .try_collect::<Vec<_>>()?;
                        if components.len() > 0 {
                            Ok(components)
                        } else {
                            Ok(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([ScriptValue::Null]),
                                ),
                            )
                        }
                    }
                }
            }
            pub trait ScriptQueryParam: FruityAny + Send + Sync {
                fn duplicate(&self) -> Box<dyn ScriptQueryParam>;
                fn filter_archetype(&self, archetype: &Archetype) -> bool;
                fn get_entity_components(
                    &self,
                    entity_reference: EntityReference,
                ) -> FruityResult<Vec<ScriptValue>>;
            }
            pub struct ScriptQuery {
                pub(crate) archetypes: Arc<RwLock<Vec<ArchetypeArcRwLock>>>,
                pub(crate) on_entity_created: Signal<EntityReference>,
                pub(crate) on_entity_deleted: Signal<EntityId>,
                pub(crate) params: Vec<Box<dyn ScriptQueryParam>>,
            }
            impl ::fruity_game_engine::introspect::IntrospectFields for ScriptQuery {
                fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                    Ok("ScriptQuery".to_string())
                }
                fn get_field_names(
                    &self,
                ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                    Ok(::alloc::vec::Vec::new())
                }
                fn set_field_value(
                    &mut self,
                    name: &str,
                    value: ::fruity_game_engine::script_value::ScriptValue,
                ) -> ::fruity_game_engine::FruityResult<()> {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
                fn get_field_value(
                    &self,
                    name: &str,
                ) -> ::fruity_game_engine::FruityResult<
                    ::fruity_game_engine::script_value::ScriptValue,
                > {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
            impl ::fruity_game_engine::any::FruityAny for ScriptQuery {
                fn get_type_name(&self) -> &'static str {
                    "ScriptQuery"
                }
                fn as_any_ref(&self) -> &dyn std::any::Any {
                    self
                }
                fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                    self
                }
                fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                    self
                }
                fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                    self
                }
                fn as_fruity_any_ref(
                    &self,
                ) -> &dyn ::fruity_game_engine::any::FruityAny {
                    self
                }
                fn as_fruity_any_mut(
                    &mut self,
                ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                    self
                }
                fn as_fruity_any_box(
                    self: Box<Self>,
                ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                    self
                }
                fn as_fruity_any_rc(
                    self: std::rc::Rc<Self>,
                ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                    self
                }
            }
            impl ScriptQuery {
                pub fn with_entity(&self) -> ScriptQuery {
                    let mut query = self.clone();
                    query.params.push(Box::new(WithEntity {}));
                    query
                }
                pub fn with_id(&self) -> ScriptQuery {
                    let mut query = self.clone();
                    query.params.push(Box::new(WithId {}));
                    query
                }
                pub fn with_name(&self) -> ScriptQuery {
                    let mut query = self.clone();
                    query.params.push(Box::new(WithName {}));
                    query
                }
                pub fn with_enabled(&self) -> ScriptQuery {
                    let mut query = self.clone();
                    query.params.push(Box::new(WithEnabled {}));
                    query
                }
                pub fn with(&self, component_identifier: String) -> ScriptQuery {
                    let mut query = self.clone();
                    query
                        .params
                        .push(
                            Box::new(With {
                                identifier: component_identifier,
                            }),
                        );
                    query
                }
                pub fn with_optional(
                    &self,
                    component_identifier: String,
                ) -> ScriptQuery {
                    let mut query = self.clone();
                    query
                        .params
                        .push(
                            Box::new(WithOptional {
                                identifier: component_identifier,
                            }),
                        );
                    query
                }
                pub fn for_each(
                    &self,
                    callback: Rc<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
                ) -> FruityResult<()> {
                    let archetypes = self.archetypes.read();
                    let archetype_filter = self.archetype_filter();
                    let entities = archetypes
                        .iter()
                        .filter(|archetype| archetype_filter(archetype))
                        .map(|archetype| archetype.iter(false))
                        .flatten()
                        .collect::<Vec<_>>();
                    entities
                        .into_iter()
                        .try_for_each(|entity| {
                            let script_params: Vec<Vec<ScriptValue>> = self
                                .params
                                .iter()
                                .map(|param| param.get_entity_components(entity.clone()))
                                .try_collect()?;
                            let mut script_params = script_params
                                .into_iter()
                                .multi_cartesian_product();
                            script_params
                                .try_for_each(|params| {
                                    callback(params)?;
                                    Result::<(), FruityError>::Ok(())
                                })?;
                            Result::<(), FruityError>::Ok(())
                        })
                }
                /// Call a function for every entities of an query
                pub fn on_created(
                    &self,
                    callback: Rc<dyn ScriptCallback>,
                ) -> FruityResult<ObserverHandler<EntityReference>> {
                    let archetype_filter = self.archetype_filter();
                    let params = self
                        .params
                        .iter()
                        .map(|param| param.duplicate())
                        .collect::<Vec<_>>();
                    let callback = callback.create_thread_safe_callback()?;
                    Ok(
                        self
                            .on_entity_created
                            .add_observer(move |entity| {
                                if archetype_filter(&entity.archetype) {
                                    let mut serialized_params = params
                                        .iter()
                                        .map(|param| param.get_entity_components(entity.clone()))
                                        .multi_cartesian_product()
                                        .flatten();
                                    serialized_params
                                        .try_for_each(|params| {
                                            callback(params);
                                            Result::<(), FruityError>::Ok(())
                                        })
                                } else {
                                    Ok(())
                                }
                            }),
                    )
                }
                fn archetype_filter(
                    &self,
                ) -> Box<dyn Fn(&ArchetypeArcRwLock) -> bool + Send + Sync + 'static> {
                    let params = self
                        .clone()
                        .params
                        .into_iter()
                        .map(|param| param)
                        .collect::<Vec<_>>();
                    Box::new(move |archetype| {
                        for param in params.iter() {
                            if !param.filter_archetype(&archetype.read()) {
                                return false;
                            }
                        }
                        true
                    })
                }
            }
            impl ::fruity_game_engine::introspect::IntrospectMethods for ScriptQuery {
                fn get_const_method_names(
                    &self,
                ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                    Ok(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                "with_entity".to_string(),
                                "with_id".to_string(),
                                "with_name".to_string(),
                                "with_enabled".to_string(),
                                "with".to_string(),
                                "with_optional".to_string(),
                                "for_each".to_string(),
                                "on_created".to_string(),
                            ]),
                        ),
                    )
                }
                fn call_const_method(
                    &self,
                    name: &str,
                    __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
                ) -> ::fruity_game_engine::FruityResult<
                    ::fruity_game_engine::script_value::ScriptValue,
                > {
                    use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
                    match name {
                        "with_entity" => self.with_entity().into_script_value(),
                        "with_id" => self.with_id().into_script_value(),
                        "with_name" => self.with_name().into_script_value(),
                        "with_enabled" => self.with_enabled().into_script_value(),
                        "with" => {
                            let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                                __args,
                            );
                            let __arg_0 = __caster.cast_next::<String>()?;
                            self.with(__arg_0).into_script_value()
                        }
                        "with_optional" => {
                            let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                                __args,
                            );
                            let __arg_0 = __caster.cast_next::<String>()?;
                            self.with_optional(__arg_0).into_script_value()
                        }
                        "for_each" => {
                            let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                                __args,
                            );
                            let __arg_0 = __caster
                                .cast_next::<
                                    Rc<dyn Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
                                >()?;
                            self.for_each(__arg_0).into_script_value()
                        }
                        "on_created" => {
                            let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                                __args,
                            );
                            let __arg_0 = __caster
                                .cast_next::<Rc<dyn ScriptCallback>>()?;
                            self.on_created(__arg_0).into_script_value()
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
                fn get_mut_method_names(
                    &self,
                ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                    Ok(::alloc::vec::Vec::new())
                }
                fn call_mut_method(
                    &mut self,
                    name: &str,
                    __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
                ) -> ::fruity_game_engine::FruityResult<
                    ::fruity_game_engine::script_value::ScriptValue,
                > {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
            impl Clone for ScriptQuery {
                fn clone(&self) -> Self {
                    Self {
                        archetypes: self.archetypes.clone(),
                        on_entity_created: self.on_entity_created.clone(),
                        on_entity_deleted: self.on_entity_deleted.clone(),
                        params: self
                            .params
                            .iter()
                            .map(|param| param.duplicate())
                            .collect(),
                    }
                }
            }
            impl Debug for ScriptQuery {
                fn fmt(
                    &self,
                    _formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::result::Result<(), std::fmt::Error> {
                    Ok(())
                }
            }
        }
        /// Queries for tuples
        pub mod tuple {
            use crate::entity::archetype::Archetype;
            use crate::entity::entity_query::QueryParam;
            use crate::entity::entity_query::RequestedEntityGuard;
            use crate::entity::entity_reference::EntityReference;
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2) {
                type Item = (T1::Item, T2::Item);
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(T2)>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (T1, T2.clone()))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3) {
                type Item = (T1::Item, T2::Item, T3::Item);
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (T1, T2.clone(), T3.clone()))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4) {
                type Item = (T1::Item, T2::Item, T3::Item, T4::Item);
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3, T4)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (T1, T2.clone(), T3.clone(), T4.clone()))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5) {
                type Item = (T1::Item, T2::Item, T3::Item, T4::Item, T5::Item);
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3, T4, T5)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5, T6) {
                type Item = (T1::Item, T2::Item, T3::Item, T4::Item, T5::Item, T6::Item);
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3, T4, T5, T6)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5, T6)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5, T6, T7) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3, T4, T5, T6, T7)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5, T6, T7)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5, T6, T7, T8) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3, T4, T5, T6, T7, T8)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5, T6, T7, T8)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3, T4, T5, T6, T7, T8, T9)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5, T6, T7, T8, T9)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(T2, T3, T4, T5, T6, T7, T8, T9, T10)>::filter_archetype(archetype)
                        && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5, T6, T7, T8, T9, T10)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
            > QueryParam<'a> for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)| {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
                T14: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                    T14::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                        || T14::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                        || T14::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                            T14,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                        T14.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
                T14: QueryParam<'a> + 'static,
                T15: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                    T14::Item,
                    T15::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                        || T14::require_read() || T15::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                        || T14::require_write() || T15::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                            T14,
                            T15,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (
                                    T2,
                                    T3,
                                    T4,
                                    T5,
                                    T6,
                                    T7,
                                    T8,
                                    T9,
                                    T10,
                                    T11,
                                    T12,
                                    T13,
                                    T14,
                                    T15,
                                )|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                        T14.clone(),
                                        T15.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
                T14: QueryParam<'a> + 'static,
                T15: QueryParam<'a> + 'static,
                T16: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                    T14::Item,
                    T15::Item,
                    T16::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                        || T14::require_read() || T15::require_read()
                        || T16::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                        || T14::require_write() || T15::require_write()
                        || T16::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                            T14,
                            T15,
                            T16,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (
                                    T2,
                                    T3,
                                    T4,
                                    T5,
                                    T6,
                                    T7,
                                    T8,
                                    T9,
                                    T10,
                                    T11,
                                    T12,
                                    T13,
                                    T14,
                                    T15,
                                    T16,
                                )|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                        T14.clone(),
                                        T15.clone(),
                                        T16.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
                T14: QueryParam<'a> + 'static,
                T15: QueryParam<'a> + 'static,
                T16: QueryParam<'a> + 'static,
                T17: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
            ) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                    T14::Item,
                    T15::Item,
                    T16::Item,
                    T17::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                        || T14::require_read() || T15::require_read()
                        || T16::require_read() || T17::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                        || T14::require_write() || T15::require_write()
                        || T16::require_write() || T17::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                            T14,
                            T15,
                            T16,
                            T17,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (
                                    T2,
                                    T3,
                                    T4,
                                    T5,
                                    T6,
                                    T7,
                                    T8,
                                    T9,
                                    T10,
                                    T11,
                                    T12,
                                    T13,
                                    T14,
                                    T15,
                                    T16,
                                    T17,
                                )|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                        T14.clone(),
                                        T15.clone(),
                                        T16.clone(),
                                        T17.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
                T14: QueryParam<'a> + 'static,
                T15: QueryParam<'a> + 'static,
                T16: QueryParam<'a> + 'static,
                T17: QueryParam<'a> + 'static,
                T18: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
            ) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                    T14::Item,
                    T15::Item,
                    T16::Item,
                    T17::Item,
                    T18::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                        || T14::require_read() || T15::require_read()
                        || T16::require_read() || T17::require_read()
                        || T18::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                        || T14::require_write() || T15::require_write()
                        || T16::require_write() || T17::require_write()
                        || T18::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                            T14,
                            T15,
                            T16,
                            T17,
                            T18,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (
                                    T2,
                                    T3,
                                    T4,
                                    T5,
                                    T6,
                                    T7,
                                    T8,
                                    T9,
                                    T10,
                                    T11,
                                    T12,
                                    T13,
                                    T14,
                                    T15,
                                    T16,
                                    T17,
                                    T18,
                                )|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                        T14.clone(),
                                        T15.clone(),
                                        T16.clone(),
                                        T17.clone(),
                                        T18.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
                T14: QueryParam<'a> + 'static,
                T15: QueryParam<'a> + 'static,
                T16: QueryParam<'a> + 'static,
                T17: QueryParam<'a> + 'static,
                T18: QueryParam<'a> + 'static,
                T19: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
            ) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                    T14::Item,
                    T15::Item,
                    T16::Item,
                    T17::Item,
                    T18::Item,
                    T19::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                        T19,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                        || T14::require_read() || T15::require_read()
                        || T16::require_read() || T17::require_read()
                        || T18::require_read() || T19::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                        || T14::require_write() || T15::require_write()
                        || T16::require_write() || T17::require_write()
                        || T18::require_write() || T19::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                            T14,
                            T15,
                            T16,
                            T17,
                            T18,
                            T19,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (
                                    T2,
                                    T3,
                                    T4,
                                    T5,
                                    T6,
                                    T7,
                                    T8,
                                    T9,
                                    T10,
                                    T11,
                                    T12,
                                    T13,
                                    T14,
                                    T15,
                                    T16,
                                    T17,
                                    T18,
                                    T19,
                                )|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                        T14.clone(),
                                        T15.clone(),
                                        T16.clone(),
                                        T17.clone(),
                                        T18.clone(),
                                        T19.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
            #[allow(unused_parens)]
            #[allow(non_snake_case)]
            impl<
                'a,
                T1: QueryParam<'a> + 'static,
                T2: QueryParam<'a> + 'static,
                T3: QueryParam<'a> + 'static,
                T4: QueryParam<'a> + 'static,
                T5: QueryParam<'a> + 'static,
                T6: QueryParam<'a> + 'static,
                T7: QueryParam<'a> + 'static,
                T8: QueryParam<'a> + 'static,
                T9: QueryParam<'a> + 'static,
                T10: QueryParam<'a> + 'static,
                T11: QueryParam<'a> + 'static,
                T12: QueryParam<'a> + 'static,
                T13: QueryParam<'a> + 'static,
                T14: QueryParam<'a> + 'static,
                T15: QueryParam<'a> + 'static,
                T16: QueryParam<'a> + 'static,
                T17: QueryParam<'a> + 'static,
                T18: QueryParam<'a> + 'static,
                T19: QueryParam<'a> + 'static,
                T20: QueryParam<'a> + 'static,
            > QueryParam<'a>
            for (
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
            ) {
                type Item = (
                    T1::Item,
                    T2::Item,
                    T3::Item,
                    T4::Item,
                    T5::Item,
                    T6::Item,
                    T7::Item,
                    T8::Item,
                    T9::Item,
                    T10::Item,
                    T11::Item,
                    T12::Item,
                    T13::Item,
                    T14::Item,
                    T15::Item,
                    T16::Item,
                    T17::Item,
                    T18::Item,
                    T19::Item,
                    T20::Item,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    <(
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                        T19,
                        T20,
                    )>::filter_archetype(archetype) && T1::filter_archetype(archetype)
                }
                fn require_read() -> bool {
                    T1::require_read() || T2::require_read() || T3::require_read()
                        || T4::require_read() || T5::require_read() || T6::require_read()
                        || T7::require_read() || T8::require_read() || T9::require_read()
                        || T10::require_read() || T11::require_read()
                        || T12::require_read() || T13::require_read()
                        || T14::require_read() || T15::require_read()
                        || T16::require_read() || T17::require_read()
                        || T18::require_read() || T19::require_read()
                        || T20::require_read()
                }
                fn require_write() -> bool {
                    T1::require_write() || T2::require_write() || T3::require_write()
                        || T4::require_write() || T5::require_write()
                        || T6::require_write() || T7::require_write()
                        || T8::require_write() || T9::require_write()
                        || T10::require_write() || T11::require_write()
                        || T12::require_write() || T13::require_write()
                        || T14::require_write() || T15::require_write()
                        || T16::require_write() || T17::require_write()
                        || T18::require_write() || T19::require_write()
                        || T20::require_write()
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <(
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                            T12,
                            T13,
                            T14,
                            T15,
                            T16,
                            T17,
                            T18,
                            T19,
                            T20,
                        )>::iter_entity_components(
                                entity_reference.clone(),
                                entity_guard,
                            )
                            .map(move |
                                (
                                    T2,
                                    T3,
                                    T4,
                                    T5,
                                    T6,
                                    T7,
                                    T8,
                                    T9,
                                    T10,
                                    T11,
                                    T12,
                                    T13,
                                    T14,
                                    T15,
                                    T16,
                                    T17,
                                    T18,
                                    T19,
                                    T20,
                                )|
                            {
                                T1::iter_entity_components(
                                        entity_reference.clone(),
                                        entity_guard,
                                    )
                                    .map(move |T1| (
                                        T1,
                                        T2.clone(),
                                        T3.clone(),
                                        T4.clone(),
                                        T5.clone(),
                                        T6.clone(),
                                        T7.clone(),
                                        T8.clone(),
                                        T9.clone(),
                                        T10.clone(),
                                        T11.clone(),
                                        T12.clone(),
                                        T13.clone(),
                                        T14.clone(),
                                        T15.clone(),
                                        T16.clone(),
                                        T17.clone(),
                                        T18.clone(),
                                        T19.clone(),
                                        T20.clone(),
                                    ))
                            })
                            .flatten(),
                    )
                }
            }
        }
        /// Queries for with stuffs
        pub mod with {
            use crate::component::component::Component;
            use crate::component::component::StaticComponent;
            use crate::component::component_guard::TypedComponentReadGuard;
            use crate::component::component_guard::TypedComponentWriteGuard;
            use crate::entity::archetype::Archetype;
            use crate::entity::entity::EntityId;
            use crate::entity::entity_query::QueryParam;
            use crate::entity::entity_query::RequestedEntityGuard;
            use crate::entity::entity_reference::EntityReference;
            use std::marker::PhantomData;
            /// The entity reference
            pub struct WithEntity;
            impl<'a> QueryParam<'a> for WithEntity {
                type Item = EntityReference;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    _entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([entity_reference.clone()]),
                            )
                            .into_iter(),
                    )
                }
            }
            /// The entity id
            pub struct WithId;
            impl<'a> QueryParam<'a> for WithId {
                type Item = EntityId;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    _entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    let entity_id = {
                        let entity_reader = entity_reference.read();
                        entity_reader.get_entity_id()
                    };
                    Box::new(
                        <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([entity_id]),
                            )
                            .into_iter(),
                    )
                }
            }
            /// The entity name
            pub struct WithName;
            impl<'a> QueryParam<'a> for WithName {
                type Item = String;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    _entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    let name = {
                        let entity_reader = entity_reference.read();
                        entity_reader.get_name()
                    };
                    Box::new(
                        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([name]))
                            .into_iter(),
                    )
                }
            }
            /// Is entity enabled
            pub struct WithEnabled;
            impl<'a> QueryParam<'a> for WithEnabled {
                type Item = bool;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    entity_reference: EntityReference,
                    _entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    let enabled = {
                        let entity_reader = entity_reference.read();
                        entity_reader.is_enabled()
                    };
                    Box::new(
                        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([enabled]))
                            .into_iter(),
                    )
                }
            }
            /// A readable component reference
            pub struct With<T> {
                _phantom: PhantomData<T>,
            }
            impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a>
            for With<T> {
                type Item = TypedComponentReadGuard<'a, T>;
                fn filter_archetype(archetype: &Archetype) -> bool {
                    archetype.identifier.contains(&T::get_component_name().to_string())
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(entity_guard) => {
                            Box::new(entity_guard.iter_components::<T>())
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            Box::new(entity_guard.iter_components::<T>())
                        }
                        RequestedEntityGuard::None => {
                            ::core::panicking::panic("explicit panic")
                        }
                    }
                }
            }
            /// A writable component reference
            pub struct WithMut<T> {
                _phantom: PhantomData<T>,
            }
            impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a>
            for WithMut<T> {
                type Item = TypedComponentWriteGuard<'a, T>;
                fn filter_archetype(archetype: &Archetype) -> bool {
                    archetype.identifier.contains(&T::get_component_name().to_string())
                }
                fn require_read() -> bool {
                    false
                }
                fn require_write() -> bool {
                    true
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(_) => {
                            ::core::panicking::panic("explicit panic")
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            Box::new(entity_guard.iter_components_mut::<T>())
                        }
                        RequestedEntityGuard::None => {
                            ::core::panicking::panic("explicit panic")
                        }
                    }
                }
            }
            /// A readable optional component reference
            pub struct WithOptional<T> {
                _phantom: PhantomData<T>,
            }
            impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a>
            for WithOptional<T> {
                type Item = Option<TypedComponentReadGuard<'a, T>>;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(entity_guard) => {
                            let iter = entity_guard.iter_components::<T>().peekable();
                            let mut iter = iter.peekable();
                            match iter.peek() {
                                Some(_) => Box::new(iter.map(|elem| Some(elem))),
                                None => {
                                    Box::new(
                                        <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([None]),
                                            )
                                            .into_iter(),
                                    )
                                }
                            }
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            let iter = entity_guard.iter_components::<T>().peekable();
                            let mut iter = iter.peekable();
                            match iter.peek() {
                                Some(_) => Box::new(iter.map(|elem| Some(elem))),
                                None => {
                                    Box::new(
                                        <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([None]),
                                            )
                                            .into_iter(),
                                    )
                                }
                            }
                        }
                        RequestedEntityGuard::None => {
                            Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([None]),
                                    )
                                    .into_iter(),
                            )
                        }
                    }
                }
            }
            /// A writable optional component reference
            pub struct WithOptionalMut<T> {
                _phantom: PhantomData<T>,
            }
            impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a>
            for WithOptionalMut<T> {
                type Item = Option<TypedComponentWriteGuard<'a, T>>;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    false
                }
                fn require_write() -> bool {
                    true
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(_) => {
                            Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([None]),
                                    )
                                    .into_iter(),
                            )
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            let iter = entity_guard
                                .iter_components_mut::<T>()
                                .peekable();
                            let mut iter = iter.peekable();
                            match iter.peek() {
                                Some(_) => Box::new(iter.map(|elem| Some(elem))),
                                None => {
                                    Box::new(
                                        <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([None]),
                                            )
                                            .into_iter(),
                                    )
                                }
                            }
                        }
                        RequestedEntityGuard::None => {
                            Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([None]),
                                    )
                                    .into_iter(),
                            )
                        }
                    }
                }
            }
            /// A readable component reference
            pub struct WithExtension<T, E> {
                _phantom: PhantomData<T>,
                _phantom_e: PhantomData<E>,
            }
            impl<
                'a,
                T: Component + StaticComponent + 'static,
                E: Component + StaticComponent + 'static,
            > QueryParam<'a> for WithExtension<T, E> {
                type Item = (
                    TypedComponentReadGuard<'a, T>,
                    TypedComponentReadGuard<'a, E>,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    archetype.identifier.contains(&T::get_component_name().to_string())
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(entity_guard) => {
                            Box::new(
                                entity_guard
                                    .iter_components::<T>()
                                    .map(|component| (
                                        component.clone(),
                                        component.get_extension::<E>().unwrap(),
                                    )),
                            )
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            Box::new(
                                entity_guard
                                    .iter_components::<T>()
                                    .map(|component| (
                                        component.clone(),
                                        component.get_extension::<E>().unwrap(),
                                    )),
                            )
                        }
                        RequestedEntityGuard::None => {
                            ::core::panicking::panic("explicit panic")
                        }
                    }
                }
            }
            /// A writable component reference
            pub struct WithExtensionMut<T, E> {
                _phantom: PhantomData<T>,
                _phantom_e: PhantomData<E>,
            }
            impl<
                'a,
                T: Component + StaticComponent + 'static,
                E: Component + StaticComponent + 'static,
            > QueryParam<'a> for WithExtensionMut<T, E> {
                type Item = (
                    TypedComponentWriteGuard<'a, T>,
                    TypedComponentWriteGuard<'a, E>,
                );
                fn filter_archetype(archetype: &Archetype) -> bool {
                    archetype.identifier.contains(&T::get_component_name().to_string())
                }
                fn require_read() -> bool {
                    false
                }
                fn require_write() -> bool {
                    true
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(_) => {
                            ::core::panicking::panic("explicit panic")
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            Box::new(
                                entity_guard
                                    .iter_components_mut::<T>()
                                    .map(|component| {
                                        (
                                            component.clone(),
                                            component.get_extension_mut::<E>().unwrap(),
                                        )
                                    }),
                            )
                        }
                        RequestedEntityGuard::None => {
                            ::core::panicking::panic("explicit panic")
                        }
                    }
                }
            }
            /// A readable optional component reference
            pub struct WithExtensionOptional<T, E> {
                _phantom: PhantomData<T>,
                _phantom_e: PhantomData<E>,
            }
            impl<
                'a,
                T: Component + StaticComponent + 'static,
                E: Component + StaticComponent + 'static,
            > QueryParam<'a> for WithExtensionOptional<T, E> {
                type Item = Option<
                    (TypedComponentReadGuard<'a, T>, TypedComponentReadGuard<'a, E>),
                >;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    true
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(entity_guard) => {
                            let iter = entity_guard.iter_components::<T>().peekable();
                            let mut iter = iter.peekable();
                            match iter.peek() {
                                Some(_) => {
                                    Box::new(
                                        iter
                                            .map(|component| {
                                                Some((
                                                    component.clone(),
                                                    component.get_extension::<E>().unwrap(),
                                                ))
                                            }),
                                    )
                                }
                                None => {
                                    Box::new(
                                        <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([None]),
                                            )
                                            .into_iter(),
                                    )
                                }
                            }
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            let iter = entity_guard.iter_components::<T>().peekable();
                            let mut iter = iter.peekable();
                            match iter.peek() {
                                Some(_) => {
                                    Box::new(
                                        iter
                                            .map(|component| {
                                                Some((
                                                    component.clone(),
                                                    component.get_extension::<E>().unwrap(),
                                                ))
                                            }),
                                    )
                                }
                                None => {
                                    Box::new(
                                        <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([None]),
                                            )
                                            .into_iter(),
                                    )
                                }
                            }
                        }
                        RequestedEntityGuard::None => {
                            Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([None]),
                                    )
                                    .into_iter(),
                            )
                        }
                    }
                }
            }
            /// A writable optional component reference
            pub struct WithExtensionOptionalMut<T, E> {
                _phantom: PhantomData<T>,
                _phantom_e: PhantomData<E>,
            }
            impl<
                'a,
                T: Component + StaticComponent + 'static,
                E: Component + StaticComponent + 'static,
            > QueryParam<'a> for WithExtensionOptionalMut<T, E> {
                type Item = Option<
                    (TypedComponentWriteGuard<'a, T>, TypedComponentWriteGuard<'a, E>),
                >;
                fn filter_archetype(_archetype: &Archetype) -> bool {
                    true
                }
                fn require_read() -> bool {
                    false
                }
                fn require_write() -> bool {
                    true
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    match entity_guard {
                        RequestedEntityGuard::Read(_) => {
                            Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([None]),
                                    )
                                    .into_iter(),
                            )
                        }
                        RequestedEntityGuard::Write(entity_guard) => {
                            let iter = entity_guard
                                .iter_components_mut::<T>()
                                .peekable();
                            let mut iter = iter.peekable();
                            match iter.peek() {
                                Some(_) => {
                                    Box::new(
                                        iter
                                            .map(|component| {
                                                Some((
                                                    component.clone(),
                                                    component.get_extension_mut::<E>().unwrap(),
                                                ))
                                            }),
                                    )
                                }
                                None => {
                                    Box::new(
                                        <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([None]),
                                            )
                                            .into_iter(),
                                    )
                                }
                            }
                        }
                        RequestedEntityGuard::None => {
                            Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([None]),
                                    )
                                    .into_iter(),
                            )
                        }
                    }
                }
            }
        }
        /// Queries for without stuffs
        pub mod without {
            use crate::component::component::Component;
            use crate::component::component::StaticComponent;
            use crate::entity::archetype::Archetype;
            use crate::entity::entity_query::QueryParam;
            use crate::entity::entity_query::RequestedEntityGuard;
            use crate::entity::entity_reference::EntityReference;
            use std::marker::PhantomData;
            /// Exclude a component from a query
            pub struct Without<T: Component + StaticComponent + 'static> {
                _phantom: PhantomData<T>,
            }
            impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a>
            for Without<T> {
                type Item = ();
                fn filter_archetype(archetype: &Archetype) -> bool {
                    !archetype.identifier.contains(&T::get_component_name().to_string())
                }
                fn require_read() -> bool {
                    false
                }
                fn require_write() -> bool {
                    false
                }
                fn iter_entity_components(
                    _entity_reference: EntityReference,
                    _entity_guard: &'a RequestedEntityGuard<'a>,
                ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(
                        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([()]))
                            .into_iter(),
                    )
                }
            }
        }
        /// An enum to pass a guard into the [’QueryInjectable’]
        pub enum RequestedEntityGuard<'a> {
            /// No guard required
            None,
            /// Read guard required
            Read(EntityReadGuard<'a>),
            /// Write guard required
            Write(EntityWriteGuard<'a>),
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for RequestedEntityGuard<'a> {
            #[inline]
            fn clone(&self) -> RequestedEntityGuard<'a> {
                match self {
                    RequestedEntityGuard::None => RequestedEntityGuard::None,
                    RequestedEntityGuard::Read(__self_0) => {
                        RequestedEntityGuard::Read(::core::clone::Clone::clone(__self_0))
                    }
                    RequestedEntityGuard::Write(__self_0) => {
                        RequestedEntityGuard::Write(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        /// A trait that should be implement for everything that can be queried from ['EntityService']
        pub trait QueryParam<'a> {
            /// The type of the query callback parameter
            type Item: Clone;
            /// A filter over the archetypes
            fn filter_archetype(archetype: &Archetype) -> bool;
            /// Does this require a read guard over the reference
            fn require_read() -> bool;
            /// Does this require a write guard over the reference
            fn require_write() -> bool;
            /// Iter over the queried components into a given entity
            fn iter_entity_components(
                entity_reference: EntityReference,
                entity_guard: &'a RequestedEntityGuard<'a>,
            ) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
        }
        /// A query over entities
        pub struct Query<T> {
            pub(crate) archetypes: Arc<RwLock<Vec<ArchetypeArcRwLock>>>,
            pub(crate) on_entity_created: Signal<EntityReference>,
            pub(crate) on_entity_deleted: Signal<EntityId>,
            pub(crate) _param_phantom: PhantomData<T>,
        }
        impl<T> Clone for Query<T> {
            fn clone(&self) -> Self {
                Query {
                    archetypes: self.archetypes.clone(),
                    on_entity_created: self.on_entity_created.clone(),
                    on_entity_deleted: self.on_entity_deleted.clone(),
                    _param_phantom: PhantomData {},
                }
            }
        }
        unsafe impl<T> Sync for Query<T> {}
        unsafe impl<T> Send for Query<T> {}
        impl<'a, T: QueryParam<'a> + 'static> Query<T> {
            /// Call a function for every entities of an query
            pub fn for_each(
                &self,
                callback: impl Fn(T::Item) -> FruityResult<()> + Send + Sync,
            ) -> FruityResult<()> {
                let archetypes = self.archetypes.read();
                let archetype_iter = archetypes
                    .iter()
                    .filter(|archetype| T::filter_archetype(&archetype.read()));
                let entities = archetype_iter
                    .map(|archetype| archetype.iter(false))
                    .flatten()
                    .collect::<Vec<_>>();
                entities
                    .into_iter()
                    .par_bridge()
                    .try_for_each(|entity| {
                        let entity_guard = if T::require_write() {
                            RequestedEntityGuard::Write(entity.write())
                        } else if T::require_read() {
                            RequestedEntityGuard::Read(entity.read())
                        } else {
                            RequestedEntityGuard::None
                        };
                        let entity_guard = unsafe {
                            std::mem::transmute::<
                                &RequestedEntityGuard,
                                &RequestedEntityGuard,
                            >(&entity_guard)
                        };
                        T::iter_entity_components(entity.clone(), &entity_guard)
                            .try_for_each(|param| callback(param))
                    })
            }
            /// Call a function for every entities of an query
            pub fn on_created(
                &self,
                callback: impl Fn(
                    T::Item,
                ) -> Option<Box<dyn Fn() + Send + Sync>> + Send + Sync + 'static,
            ) -> ObserverHandler<EntityReference> {
                let on_entity_deleted = self.on_entity_deleted.clone();
                self.on_entity_created
                    .add_observer(move |entity| {
                        if T::filter_archetype(&entity.archetype.read()) {
                            let entity_id = {
                                let entity_reader = entity.read();
                                entity_reader.get_entity_id()
                            };
                            let entity_guard = if T::require_write() {
                                RequestedEntityGuard::Write(entity.write())
                            } else if T::require_read() {
                                RequestedEntityGuard::Read(entity.read())
                            } else {
                                RequestedEntityGuard::None
                            };
                            let entity_guard = unsafe {
                                std::mem::transmute::<
                                    &RequestedEntityGuard,
                                    &RequestedEntityGuard,
                                >(&entity_guard)
                            };
                            T::iter_entity_components(entity.clone(), &entity_guard)
                                .try_for_each(|param| {
                                    let dispose_callback = callback(param);
                                    if let Some(dispose_callback) = dispose_callback {
                                        on_entity_deleted
                                            .add_self_dispose_observer(move |signal_entity_id, handler| {
                                                if entity_id == *signal_entity_id {
                                                    dispose_callback();
                                                    handler.dispose_by_ref();
                                                }
                                                Ok(())
                                            })
                                    }
                                    Ok(())
                                })
                        } else {
                            Ok(())
                        }
                    })
            }
        }
        impl<'a, T: QueryParam<'a> + 'static> Injectable for Query<T> {
            fn from_resource_container(resource_container: &ResourceContainer) -> Self {
                let entity_service = resource_container.require::<EntityService>();
                let entity_service = entity_service.read();
                entity_service.query::<T>()
            }
        }
    }
    /// Provides a reference to an entity
    pub mod entity_reference {
        use crate::component::component_reference::ComponentReference;
        use crate::entity::archetype::ArchetypeArcRwLock;
        use crate::entity::entity_guard::EntityReadGuard;
        use crate::entity::entity_guard::EntityWriteGuard;
        use fruity_game_engine::any::FruityAny;
        use fruity_game_engine::RwLockReadGuard;
        use fruity_game_engine::RwLockWriteGuard;
        use fruity_game_engine::{export, export_impl, export_struct};
        use std::fmt::Debug;
        use std::rc::Rc;
        use super::entity::EntityId;
        /// A reference over an entity stored into an Archetype
        pub struct EntityReference {
            pub(crate) entity_id: usize,
            pub(crate) archetype: ArchetypeArcRwLock,
        }
        impl ::fruity_game_engine::introspect::IntrospectFields for EntityReference {
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("EntityReference".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for EntityReference {
            #[inline]
            fn clone(&self) -> EntityReference {
                EntityReference {
                    entity_id: ::core::clone::Clone::clone(&self.entity_id),
                    archetype: ::core::clone::Clone::clone(&self.archetype),
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for EntityReference {
            fn get_type_name(&self) -> &'static str {
                "EntityReference"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl EntityReference {
            /// Get a read access to the entity
            pub fn read(&self) -> EntityReadGuard {
                let archetype_reader = self.archetype.read();
                let guard = archetype_reader
                    .lock_array
                    .get(self.entity_id)
                    .unwrap()
                    .read();
                let guard = unsafe {
                    std::mem::transmute::<
                        RwLockReadGuard<()>,
                        RwLockReadGuard<()>,
                    >(guard)
                };
                EntityReadGuard {
                    entity_id: self.entity_id,
                    _guard: Rc::new(guard),
                    archetype_reader: Rc::new(archetype_reader),
                }
            }
            /// Get a write access to the entity
            pub fn write(&self) -> EntityWriteGuard {
                let archetype_reader = self.archetype.read();
                let guard = archetype_reader
                    .lock_array
                    .get(self.entity_id)
                    .unwrap()
                    .write();
                let guard = unsafe {
                    std::mem::transmute::<
                        RwLockWriteGuard<()>,
                        RwLockWriteGuard<()>,
                    >(guard)
                };
                EntityWriteGuard {
                    entity_id: self.entity_id,
                    _guard: Rc::new(guard),
                    archetype_reader: Rc::new(archetype_reader),
                }
            }
            /// Get all components
            pub fn get_components(&self) -> Vec<ComponentReference> {
                self.archetype.clone().get_entity_components(self.entity_id)
            }
            /// Get components with a given type
            ///
            /// # Arguments
            /// * `component_identifier` - The component identifier
            ///
            pub fn get_components_by_type_identifier(
                &self,
                component_identifier: &str,
            ) -> Vec<ComponentReference> {
                self.archetype
                    .clone()
                    .get_entity_components_from_type(
                        self.entity_id,
                        component_identifier,
                    )
            }
            /// Get entity id
            pub fn get_entity_id(&self) -> EntityId {
                self.read().get_entity_id()
            }
            /// Get entity name
            pub fn get_name(&self) -> String {
                self.read().get_name()
            }
            /// Get entity enabled
            pub fn is_enabled(&self) -> bool {
                self.read().is_enabled()
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for EntityReference {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            "get_entity_id".to_string(),
                            "get_name".to_string(),
                            "is_enabled".to_string(),
                        ]),
                    ),
                )
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
                match name {
                    "get_entity_id" => self.get_entity_id().into_script_value(),
                    "get_name" => self.get_name().into_script_value(),
                    "is_enabled" => self.is_enabled().into_script_value(),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        impl Debug for EntityReference {
            fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                Ok(())
            }
        }
    }
    /// Provides guards for an entity
    pub mod entity_guard {
        use crate::component::component::Component;
        use crate::component::component::StaticComponent;
        use crate::component::component_guard::ComponentReadGuard;
        use crate::component::component_guard::ComponentWriteGuard;
        use crate::component::component_guard::InternalReadGuard;
        use crate::component::component_guard::TypedComponentReadGuard;
        use crate::component::component_guard::TypedComponentWriteGuard;
        use crate::entity::archetype::Archetype;
        use crate::entity::entity::EntityId;
        use fruity_game_engine::RwLockReadGuard;
        use fruity_game_engine::RwLockWriteGuard;
        use std::fmt::Debug;
        use std::rc::Rc;
        /// RAII structure used to release the shared read access of a lock when dropped.
        ///
        /// This structure is created by the [`read`] methods on [`EntityRwLock`].
        ///
        /// [`read`]: EntityRwLock::read
        ///
        pub struct EntityReadGuard<'a> {
            pub(crate) _guard: Rc<RwLockReadGuard<'a, ()>>,
            pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
            pub(crate) entity_id: usize,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for EntityReadGuard<'a> {
            #[inline]
            fn clone(&self) -> EntityReadGuard<'a> {
                EntityReadGuard {
                    _guard: ::core::clone::Clone::clone(&self._guard),
                    archetype_reader: ::core::clone::Clone::clone(
                        &self.archetype_reader,
                    ),
                    entity_id: ::core::clone::Clone::clone(&self.entity_id),
                }
            }
        }
        impl<'a> Debug for EntityReadGuard<'a> {
            fn fmt(
                &self,
                _: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                Ok(())
            }
        }
        impl<'a> EntityReadGuard<'a> {
            /// Get the entity id
            pub fn get_entity_id(&self) -> EntityId {
                *self.archetype_reader.entity_id_array.get(self.entity_id).unwrap()
            }
            /// Get the entity name
            pub fn get_name(&self) -> String {
                self.archetype_reader
                    .name_array
                    .get(self.entity_id)
                    .map(|name| name.clone())
                    .unwrap()
            }
            /// Is the entity enabled
            pub fn is_enabled(&self) -> bool {
                *self.archetype_reader.enabled_array.get(self.entity_id).unwrap()
            }
            /// Read all components of the entity
            pub fn read_all_components(
                &self,
            ) -> impl Iterator<Item = ComponentReadGuard<'_>> {
                self.archetype_reader
                    .component_storages
                    .iter()
                    .map(|(component_identifier, storage)| {
                        let start_index = self.entity_id * storage.components_per_entity;
                        let end_index = start_index + storage.components_per_entity;
                        (start_index..end_index)
                            .map(|component_index| ComponentReadGuard {
                                _guard: InternalReadGuard::Read(self._guard.clone()),
                                archetype_reader: self.archetype_reader.clone(),
                                component_identifier: component_identifier.clone(),
                                component_index,
                            })
                    })
                    .flatten()
            }
            /// Read components with a given type
            ///
            /// # Arguments
            /// * `component_identifier` - The component identifier
            ///
            pub fn iter_components<T: Component + StaticComponent>(
                &self,
            ) -> impl Iterator<Item = TypedComponentReadGuard<'_, T>> {
                self.iter_components_from_type_identifier(T::get_component_name())
                    .into_iter()
                    .map(|guard| guard.try_into())
                    .filter_map(|guard| guard.ok())
            }
            /// Read components with a given type
            ///
            /// # Arguments
            /// * `component_identifier` - The component identifier
            ///
            pub fn iter_components_from_type_identifier(
                &self,
                component_identifier: &str,
            ) -> Box<dyn Iterator<Item = ComponentReadGuard<'_>> + '_> {
                match self.archetype_reader.get_storage_from_type(component_identifier) {
                    Some(storage) => {
                        let start_index = self.entity_id * storage.components_per_entity;
                        let end_index = start_index + storage.components_per_entity;
                        let component_identifier = component_identifier.to_string();
                        Box::new(
                            (start_index..end_index)
                                .map(move |component_index| ComponentReadGuard {
                                    _guard: InternalReadGuard::Read(self._guard.clone()),
                                    archetype_reader: self.archetype_reader.clone(),
                                    component_identifier: component_identifier.clone(),
                                    component_index,
                                }),
                        )
                    }
                    None => Box::new(std::iter::empty()),
                }
            }
            /// Read a single component with a given type
            pub fn read_single_component<T: Component + StaticComponent>(
                &self,
            ) -> Option<TypedComponentReadGuard<'_, T>> {
                self.iter_components().next()
            }
        }
        /// RAII structure used to release the exclusive write access of a lock when dropped.
        ///
        /// This structure is created by the [`write`] methods on [`EntityRwLock`].
        ///
        /// [`write`]: EntityRwLock::write
        ///
        pub struct EntityWriteGuard<'a> {
            pub(crate) entity_id: usize,
            pub(crate) _guard: Rc<RwLockWriteGuard<'a, ()>>,
            pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for EntityWriteGuard<'a> {
            #[inline]
            fn clone(&self) -> EntityWriteGuard<'a> {
                EntityWriteGuard {
                    entity_id: ::core::clone::Clone::clone(&self.entity_id),
                    _guard: ::core::clone::Clone::clone(&self._guard),
                    archetype_reader: ::core::clone::Clone::clone(&self.archetype_reader),
                }
            }
        }
        impl<'a> Debug for EntityWriteGuard<'a> {
            fn fmt(
                &self,
                _: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                Ok(())
            }
        }
        impl<'a> EntityWriteGuard<'a> {
            /// Get the entity id
            pub fn get_entity_id(&self) -> EntityId {
                *self.archetype_reader.entity_id_array.get(self.entity_id).unwrap()
            }
            /// Get the entity name
            pub fn get_name(&self) -> String {
                self.archetype_reader
                    .name_array
                    .get(self.entity_id)
                    .map(|name| name.clone())
                    .unwrap()
            }
            /// Set the entity name
            ///
            /// # Arguments
            /// * `value` - The name value
            ///
            pub fn set_name(&self, value: &str) {
                let name = self.archetype_reader.name_array.get(self.entity_id).unwrap();
                #[allow(mutable_transmutes)]
                let name = unsafe { std::mem::transmute::<&String, &mut String>(name) };
                *name = value.to_string();
            }
            /// Is the entity enabled
            pub fn is_enabled(&self) -> bool {
                *self.archetype_reader.enabled_array.get(self.entity_id).unwrap()
            }
            /// Set the entity enabled state
            ///
            /// # Arguments
            /// * `value` - Is the entity enabled
            ///
            pub fn set_enabled(&self, value: bool) {
                let enabled = self
                    .archetype_reader
                    .enabled_array
                    .get(self.entity_id)
                    .unwrap();
                #[allow(mutable_transmutes)]
                let enabled = unsafe {
                    std::mem::transmute::<&bool, &mut bool>(enabled)
                };
                *enabled = value;
            }
            /// Read components with a given type
            ///
            /// # Arguments
            /// * `component_identifier` - The component identifier
            ///
            pub fn iter_components<T: Component + StaticComponent>(
                &self,
            ) -> impl Iterator<Item = TypedComponentReadGuard<'_, T>> {
                self.iter_components_from_type_identifier(T::get_component_name())
                    .into_iter()
                    .map(|guard| guard.try_into())
                    .filter_map(|guard| guard.ok())
            }
            /// Read components with a given type
            ///
            /// # Arguments
            /// * `component_identifier` - The component identifier
            ///
            pub fn iter_components_from_type_identifier(
                &self,
                component_identifier: &str,
            ) -> Box<dyn Iterator<Item = ComponentReadGuard<'_>> + '_> {
                match self.archetype_reader.get_storage_from_type(component_identifier) {
                    Some(storage) => {
                        let start_index = self.entity_id * storage.components_per_entity;
                        let end_index = start_index + storage.components_per_entity;
                        let component_identifier = component_identifier.to_string();
                        Box::new(
                            (start_index..end_index)
                                .map(move |component_index| ComponentReadGuard {
                                    _guard: InternalReadGuard::Write(self._guard.clone()),
                                    archetype_reader: self.archetype_reader.clone(),
                                    component_identifier: component_identifier.clone(),
                                    component_index,
                                }),
                        )
                    }
                    None => Box::new(std::iter::empty()),
                }
            }
            /// Read a single component with a given type
            pub fn read_single_component<T: Component + StaticComponent>(
                &self,
            ) -> Option<TypedComponentReadGuard<'_, T>> {
                self.iter_components().next()
            }
            /// Write components with a given type
            ///
            /// # Arguments
            /// * `component_identifier` - The component identifier
            ///
            pub fn iter_components_mut<T: Component + StaticComponent>(
                &self,
            ) -> impl Iterator<Item = TypedComponentWriteGuard<'_, T>> {
                self.iter_components_from_type_identifier_mut(T::get_component_name())
                    .into_iter()
                    .map(|guard| guard.try_into())
                    .filter_map(|guard| guard.ok())
            }
            /// Write components with a given type
            ///
            /// # Arguments
            /// * `component_identifier` - The component identifier
            ///
            pub fn iter_components_from_type_identifier_mut(
                &self,
                component_identifier: &str,
            ) -> Box<dyn Iterator<Item = ComponentWriteGuard<'_>> + '_> {
                match self.archetype_reader.get_storage_from_type(component_identifier) {
                    Some(storage) => {
                        let start_index = self.entity_id * storage.components_per_entity;
                        let end_index = start_index + storage.components_per_entity;
                        let component_identifier = component_identifier.to_string();
                        Box::new(
                            (start_index..end_index)
                                .map(move |component_index| ComponentWriteGuard {
                                    _guard: self._guard.clone(),
                                    archetype_reader: self.archetype_reader.clone(),
                                    component_identifier: component_identifier.clone(),
                                    component_index,
                                }),
                        )
                    }
                    None => Box::new(std::iter::empty()),
                }
            }
            /// Write a single component with a given type
            pub fn write_single_component<T: Component + StaticComponent>(
                &self,
            ) -> Option<TypedComponentWriteGuard<'_, T>> {
                self.iter_components_mut().next()
            }
        }
    }
    /// Provides a collections to store archetypes
    pub mod entity_service {
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
        use fruity_game_engine::resource::resource_reference::ResourceReference;
        use fruity_game_engine::resource::Resource;
        use fruity_game_engine::script_value::ScriptValue;
        use fruity_game_engine::signal::Signal;
        use fruity_game_engine::FruityError;
        use fruity_game_engine::FruityResult;
        use fruity_game_engine::Mutex;
        use fruity_game_engine::RwLock;
        use fruity_game_engine::{export, export_impl, export_struct};
        use std::collections::HashMap;
        use std::fmt::Debug;
        use std::marker::PhantomData;
        use std::sync::Arc;
        /// A save for the entities stored in an [’EntityService’]
        pub type EntityServiceSnapshot = ScriptValue;
        /// A storage for every entities, use [’Archetypes’] to store entities of different types
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
        impl ::fruity_game_engine::introspect::IntrospectFields for EntityService {
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("EntityService".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            "on_created".to_string(),
                            "on_deleted".to_string(),
                        ]),
                    ),
                )
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                use ::fruity_game_engine::script_value::convert::TryFromScriptValue;
                match name {
                    "on_created" => {
                        self
                            .on_created = <Signal<
                            EntityReference,
                        >>::from_script_value(value)?;
                    }
                    "on_deleted" => {
                        self.on_deleted = <Signal<EntityId>>::from_script_value(value)?;
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                };
                ::fruity_game_engine::FruityResult::Ok(())
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
                match name {
                    "on_created" => {
                        <Signal<
                            EntityReference,
                        >>::into_script_value(self.on_created.clone())
                    }
                    "on_deleted" => {
                        <Signal<EntityId>>::into_script_value(self.on_deleted.clone())
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for EntityService {
            fn get_type_name(&self) -> &'static str {
                "EntityService"
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
            fn as_fruity_any_rc(
                self: std::rc::Rc<Self>,
            ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl ::fruity_game_engine::resource::Resource for EntityService {
            fn as_resource_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::resource::Resource> {
                self
            }
            fn as_any_arc(
                self: std::sync::Arc<Self>,
            ) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
                self
            }
        }
        impl EntityService {
            /// Returns an EntityService
            pub fn new(resource_container: ResourceContainer) -> EntityService {
                EntityService {
                    id_incrementer: Mutex::new(0),
                    index_map: RwLock::new(HashMap::new()),
                    archetypes: Arc::new(RwLock::new(Vec::new())),
                    extension_component_service: resource_container
                        .require::<ExtensionComponentService>(),
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
            pub fn iter_all_entities(
                &self,
            ) -> impl Iterator<Item = EntityReference> + '_ {
                let archetypes = self.archetypes.read();
                let archetypes = unsafe {
                    std::mem::transmute::<
                        &Vec<ArchetypeArcRwLock>,
                        &Vec<ArchetypeArcRwLock>,
                    >(&archetypes)
                };
                archetypes.iter().map(|archetype| archetype.iter(true)).flatten()
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
            pub fn script_query(&self) -> ScriptQuery {
                ScriptQuery {
                    archetypes: self.archetypes.clone(),
                    on_entity_created: self.on_created.clone(),
                    on_entity_deleted: self.on_deleted.clone(),
                    params: ::alloc::vec::Vec::new(),
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
            pub fn create_with_id(
                &self,
                entity_id: EntityId,
                name: String,
                enabled: bool,
                mut components: Vec<AnyComponent>,
            ) -> FruityResult<EntityId> {
                let entity_id = {
                    let mut id_incrementer = self.id_incrementer.lock();
                    *id_incrementer = u64::max(entity_id + 1, *id_incrementer);
                    entity_id
                };
                components
                    .sort_by(|a, b| {
                        a.get_class_name().unwrap().cmp(&b.get_class_name().unwrap())
                    });
                let archetype_identifier = get_type_identifier_by_any(&components)?;
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
                {
                    let mut index_map = self.index_map.write();
                    index_map.insert(entity_id, indexes);
                }
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
            pub fn remove(&self, entity_id: EntityId) -> FruityResult<()> {
                let indexes = {
                    let mut index_map = self.index_map.write();
                    index_map.remove(&entity_id)
                };
                if let Some(indexes) = indexes {
                    {
                        let archetypes = self.archetypes.read();
                        let archetype = archetypes.get(indexes.0).unwrap();
                        archetype.read().remove(indexes.1);
                    }
                    self.on_deleted.notify(entity_id)?;
                    Ok(())
                } else {
                    Err(
                        FruityError::GenericFailure({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Entity with the id ", " not found"],
                                    &[::core::fmt::ArgumentV1::new_display(&entity_id)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
            /// Add components to an entity
            ///
            /// # Arguments
            /// * `entity_id` - The entity id
            /// * `component_index` - The component index, is based on alphabetical number of the component type name
            ///
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
                            std::mem::transmute::<
                                &Vec<ArchetypeArcRwLock>,
                                &Vec<ArchetypeArcRwLock>,
                            >(&archetypes)
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
                    Err(
                        FruityError::GenericFailure({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Entity with the id ", " not found"],
                                    &[::core::fmt::ArgumentV1::new_display(&entity_id)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
            /// Remove a component from an entity
            ///
            /// # Arguments
            /// * `entity_id` - The entity id
            /// * `component_index` - The component index, is based on alphabetical number of the component type name
            ///
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
                            std::mem::transmute::<
                                &Vec<ArchetypeArcRwLock>,
                                &Vec<ArchetypeArcRwLock>,
                            >(&archetypes)
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
                    Err(
                        FruityError::GenericFailure({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Entity with the id ", " not found"],
                                    &[::core::fmt::ArgumentV1::new_display(&entity_id)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
            fn archetype_by_identifier(
                &self,
                entity_identifier: EntityTypeIdentifier,
            ) -> Option<(usize, &ArchetypeArcRwLock)> {
                let archetypes = self.archetypes.read();
                let archetypes = unsafe {
                    std::mem::transmute::<
                        &Vec<ArchetypeArcRwLock>,
                        &Vec<ArchetypeArcRwLock>,
                    >(&archetypes)
                };
                archetypes
                    .iter()
                    .enumerate()
                    .find(|(_index, archetype)| {
                        *archetype.read().get_type_identifier() == entity_identifier
                    })
            }
            /// Clear all the entities
            pub fn clear(&self) -> FruityResult<()> {
                let entity_ids = {
                    let index_map = self.index_map.read();
                    index_map.iter().map(|(entity_id, _)| *entity_id).collect::<Vec<_>>()
                };
                entity_ids
                    .into_iter()
                    .try_for_each(|entity_id| self.on_deleted.notify(entity_id))?;
                let mut index_map = self.index_map.write();
                let mut id_incrementer = self.id_incrementer.lock();
                let mut archetypes = self.archetypes.write();
                index_map.clear();
                *id_incrementer = 0;
                archetypes.clear();
                Ok(())
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for EntityService {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            "get_entity".to_string(),
                            "query".to_string(),
                            "create".to_string(),
                            "create_with_id".to_string(),
                            "remove".to_string(),
                            "add_component".to_string(),
                            "remove_component".to_string(),
                            "clear".to_string(),
                        ]),
                    ),
                )
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
                match name {
                    "get_entity" => {
                        let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<EntityId>()?;
                        self.get_entity(__arg_0).into_script_value()
                    }
                    "query" => self.script_query().into_script_value(),
                    "create" => {
                        let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<String>()?;
                        let __arg_1 = __caster.cast_next::<bool>()?;
                        let __arg_2 = __caster.cast_next::<Vec<AnyComponent>>()?;
                        self.create(__arg_0, __arg_1, __arg_2).into_script_value()
                    }
                    "create_with_id" => {
                        let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<EntityId>()?;
                        let __arg_1 = __caster.cast_next::<String>()?;
                        let __arg_2 = __caster.cast_next::<bool>()?;
                        let __arg_3 = __caster.cast_next::<Vec<AnyComponent>>()?;
                        self.create_with_id(__arg_0, __arg_1, __arg_2, __arg_3)
                            .into_script_value()
                    }
                    "remove" => {
                        let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<EntityId>()?;
                        self.remove(__arg_0).into_script_value()
                    }
                    "add_component" => {
                        let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<EntityId>()?;
                        let __arg_1 = __caster.cast_next::<Vec<AnyComponent>>()?;
                        self.add_component(__arg_0, __arg_1).into_script_value()
                    }
                    "remove_component" => {
                        let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                            __args,
                        );
                        let __arg_0 = __caster.cast_next::<EntityId>()?;
                        let __arg_1 = __caster.cast_next::<usize>()?;
                        self.remove_component(__arg_0, __arg_1).into_script_value()
                    }
                    "clear" => self.clear().into_script_value(),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        impl Debug for EntityService {
            fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                Ok(())
            }
        }
    }
    /// Provides a collections to store entities
    pub mod archetype {
        use crate::component::component::AnyComponent;
        use crate::component::component::Component;
        use crate::component::component_reference::ComponentReference;
        use crate::entity::archetype::component_storage::ComponentStorage;
        use crate::entity::archetype::entity_properties::EntityProperties;
        use crate::entity::entity::get_type_identifier_by_any;
        use crate::entity::entity::EntityId;
        use crate::entity::entity::EntityTypeIdentifier;
        use crate::entity::entity_reference::EntityReference;
        use crate::ExtensionComponentService;
        use fruity_game_engine::resource::resource_reference::ResourceReference;
        use fruity_game_engine::FruityResult;
        use fruity_game_engine::RwLock;
        use itertools::Itertools;
        use std::collections::BTreeMap;
        use std::collections::HashMap;
        use std::ops::Deref;
        use std::sync::Arc;
        /// This store all the information that are common accross all entities
        pub mod entity_properties {
            use crate::entity::archetype::EntityId;
            use fruity_game_engine::script_value::convert::TryFromScriptValue;
            /// This store all the information that are common accross all entities
            pub struct EntityProperties {
                /// The entity id
                pub entity_id: EntityId,
                /// the entity name
                pub name: String,
                /// If false, the entity will be ignored by the systems
                pub enabled: bool,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for EntityProperties {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "EntityProperties",
                        "entity_id",
                        &&self.entity_id,
                        "name",
                        &&self.name,
                        "enabled",
                        &&self.enabled,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for EntityProperties {
                #[inline]
                fn clone(&self) -> EntityProperties {
                    EntityProperties {
                        entity_id: ::core::clone::Clone::clone(&self.entity_id),
                        name: ::core::clone::Clone::clone(&self.name),
                        enabled: ::core::clone::Clone::clone(&self.enabled),
                    }
                }
            }
            impl ::fruity_game_engine::script_value::convert::TryFromScriptValue
            for EntityProperties {
                fn from_script_value(
                    value: ::fruity_game_engine::script_value::ScriptValue,
                ) -> ::fruity_game_engine::FruityResult<Self> {
                    match value {
                        ::fruity_game_engine::script_value::ScriptValue::Object(
                            value,
                        ) => {
                            match value.downcast::<Self>() {
                                Ok(value) => Ok(*value),
                                Err(value) => {
                                    Ok(Self {
                                        entity_id: <EntityId>::from_script_value(
                                            value.get_field_value("entity_id")?,
                                        )?,
                                        name: <String>::from_script_value(
                                            value.get_field_value("name")?,
                                        )?,
                                        enabled: <bool>::from_script_value(
                                            value.get_field_value("enabled")?,
                                        )?,
                                    })
                                }
                            }
                        }
                        _ => {
                            Err(
                                ::fruity_game_engine::FruityError::InvalidArg({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["Couldn\'t convert ", " to native object"],
                                            &[::core::fmt::ArgumentV1::new_debug(&value)],
                                        ),
                                    );
                                    res
                                }),
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for EntityProperties {
                #[inline]
                fn default() -> EntityProperties {
                    EntityProperties {
                        entity_id: ::core::default::Default::default(),
                        name: ::core::default::Default::default(),
                        enabled: ::core::default::Default::default(),
                    }
                }
            }
        }
        /// An array of component
        pub mod component_array {
            use crate::entity::archetype::component_collection::ComponentCollection;
            use crate::entity::archetype::AnyComponent;
            use crate::entity::archetype::Component;
            /// A collection of entities that share the same component structure
            /// Can store multiple components of the same type
            pub struct ComponentArray<T: Component> {
                components: Vec<Option<T>>,
            }
            impl<T: Component> ComponentArray<T> {
                /// Returns a ComponentArray
                pub fn new() -> Self {
                    Self { components: Vec::new() }
                }
            }
            impl<T: Component> ComponentCollection for ComponentArray<T> {
                fn get(&self, index: &usize) -> Option<&dyn Component> {
                    match self.components.get(*index) {
                        Some(component) => {
                            match component {
                                Some(component) => Some(component),
                                None => None,
                            }
                        }
                        None => None,
                    }
                }
                fn add_many(&mut self, components: Vec<AnyComponent>) {
                    let mut components = components
                        .into_iter()
                        .map(|component| match component
                            .into_box()
                            .as_any_box()
                            .downcast::<T>()
                        {
                            Ok(component) => Some(*component),
                            Err(_) => {
                                ::core::panicking::panic_fmt(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Try to instantiate a component array from a array of components with wrong type",
                                        ],
                                        &[],
                                    ),
                                );
                            }
                        })
                        .collect::<Vec<_>>();
                    self.components.append(&mut components);
                }
                fn remove_many(
                    &mut self,
                    index: usize,
                    count: usize,
                ) -> Vec<AnyComponent> {
                    let end_index = index + count;
                    (index..end_index)
                        .into_iter()
                        .map(|index| std::mem::replace(
                            &mut self.components[index],
                            None,
                        ))
                        .filter_map(|component| {
                            component.map(|component| AnyComponent::new(component))
                        })
                        .collect::<Vec<_>>()
                }
            }
        }
        /// Provides a collection that can store components by taking care of the number of component per entity
        pub mod component_storage {
            use crate::component::component::AnyComponent;
            use crate::entity::archetype::component_collection::ComponentCollection;
            use crate::entity::archetype::Component;
            pub(crate) struct ComponentStorage {
                pub(crate) collection: Box<dyn ComponentCollection>,
                pub(crate) components_per_entity: usize,
            }
            impl ComponentStorage {
                pub(crate) fn new(components: Vec<AnyComponent>) -> Self {
                    let components_per_entity = components.len();
                    let first_component = components.get(0).unwrap();
                    let mut collection = first_component.get_collection();
                    collection.add_many(components);
                    ComponentStorage {
                        collection,
                        components_per_entity,
                    }
                }
                pub(crate) fn add(&mut self, components: Vec<AnyComponent>) {
                    if components.len() != self.components_per_entity {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[
                                    "Try to instantiate a component array from a component array with the wrong size of elements",
                                ],
                                &[],
                            ),
                        );
                    }
                    self.collection.add_many(components);
                }
                pub(crate) fn get(
                    &self,
                    entity_id: usize,
                ) -> impl Iterator<Item = &dyn Component> {
                    let start_index = entity_id * self.components_per_entity;
                    let end_index = start_index + self.components_per_entity;
                    (start_index..end_index)
                        .filter_map(|index| self.collection.get(&index))
                }
            }
        }
        /// An interface that should be implemented by collection of components used into archetypes
        pub mod component_collection {
            use crate::entity::archetype::AnyComponent;
            use crate::entity::archetype::Component;
            /// A abstraction of a collection over components
            pub trait ComponentCollection: Sync + Send {
                /// Get a single component by index
                fn get(&self, index: &usize) -> Option<&dyn Component>;
                /// Add components to the collection
                ///
                /// # Arguments
                /// * `components` - The components that will be added
                ///
                fn add_many(&mut self, components: Vec<AnyComponent>);
                /// Remove components from the collection
                ///
                /// # Arguments
                /// * `index` - The index of the first component to remove
                /// * `count` - The number of components that will be removed
                ///
                fn remove_many(
                    &mut self,
                    index: usize,
                    count: usize,
                ) -> Vec<AnyComponent>;
            }
        }
        pub(crate) struct ArchetypeArcRwLock(Arc<RwLock<Archetype>>);
        #[automatically_derived]
        impl ::core::clone::Clone for ArchetypeArcRwLock {
            #[inline]
            fn clone(&self) -> ArchetypeArcRwLock {
                ArchetypeArcRwLock(::core::clone::Clone::clone(&self.0))
            }
        }
        /// A collection of entities that share the same component structure
        /// Stored as a Struct Of Array
        pub struct Archetype {
            extension_component_service: ResourceReference<ExtensionComponentService>,
            pub(crate) identifier: EntityTypeIdentifier,
            pub(crate) erased_indexes: RwLock<Vec<usize>>,
            pub(crate) entity_id_array: Vec<EntityId>,
            pub(crate) name_array: Vec<String>,
            pub(crate) enabled_array: Vec<bool>,
            pub(crate) lock_array: Vec<RwLock<()>>,
            pub(crate) component_storages: BTreeMap<String, ComponentStorage>,
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
                extension_component_service: ResourceReference<
                    ExtensionComponentService,
                >,
                entity_id: EntityId,
                name: &str,
                enabled: bool,
                mut components: Vec<AnyComponent>,
            ) -> FruityResult<Archetype> {
                let identifier = get_type_identifier_by_any(&components)?;
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
                let grouped_components = Self::group_components_by_type(components);
                let mut component_storages = BTreeMap::new();
                for (class_name, components) in grouped_components {
                    component_storages
                        .insert(class_name, ComponentStorage::new(components));
                }
                Ok(Archetype {
                    extension_component_service,
                    identifier: identifier,
                    erased_indexes: RwLock::new(::alloc::vec::Vec::new()),
                    entity_id_array: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([entity_id]),
                    ),
                    name_array: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([name.to_string()]),
                    ),
                    enabled_array: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([enabled]),
                    ),
                    lock_array: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([RwLock::new(())]),
                    ),
                    component_storages,
                })
            }
            /// Returns the entity type identifier of the archetype
            pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
                &self.identifier
            }
            /// Get components of a specified type from an entity by index
            ///
            /// # Arguments
            /// * `entity_id` - The entity id
            /// * `component_identifier` - The components type identifier
            ///
            pub(crate) fn get_storage_from_type(
                &self,
                component_type_identifier: &str,
            ) -> Option<&ComponentStorage> {
                self.component_storages.get(component_type_identifier)
            }
            /// Get entity count
            pub fn len(&self) -> usize {
                self.entity_id_array.len()
            }
            /// Add an entity into the archetype
            ///
            /// # Arguments
            /// * `entity_id` - The first entity id
            /// * `name` - The first entity name
            /// * `components` - The first entity components
            ///
            pub fn add(
                &mut self,
                entity_id: EntityId,
                name: &str,
                enabled: bool,
                mut components: Vec<AnyComponent>,
            ) -> FruityResult<()> {
                self.entity_id_array.push(entity_id);
                self.name_array.push(name.to_string());
                self.enabled_array.push(enabled);
                self.lock_array.push(RwLock::new(()));
                let mut extensions_component = {
                    let extension_component_service = self
                        .extension_component_service
                        .read();
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
                let grouped_components = Self::group_components_by_type(components);
                for (class_name, components) in grouped_components {
                    let component_array = self.component_storages.get_mut(&class_name);
                    if let Some(component_array) = component_array {
                        component_array.add(components);
                    }
                }
                Ok(())
            }
            /// Remove an entity based on its id
            ///
            /// # Arguments
            /// * `index` - The entity index
            ///
            pub fn remove(&self, index: usize) -> (EntityProperties, Vec<AnyComponent>) {
                {
                    let mut erased_indexes_writer = self.erased_indexes.write();
                    erased_indexes_writer.push(index);
                }
                let entity_id = *self.entity_id_array.get(index).unwrap();
                let name = self.name_array.get(index).unwrap().clone();
                let enabled = *self.enabled_array.get(index).unwrap();
                let components = {
                    self.component_storages
                        .iter()
                        .map(|(_, storage)| storage.get(index))
                        .flatten()
                        .map(|component| AnyComponent::from(
                            Component::duplicate(component),
                        ))
                        .collect::<Vec<_>>()
                };
                (
                    EntityProperties {
                        entity_id,
                        name,
                        enabled,
                    },
                    components,
                )
            }
            fn group_components_by_type(
                components: Vec<AnyComponent>,
            ) -> HashMap<String, Vec<AnyComponent>> {
                components
                    .into_iter()
                    .group_by(|component| component.get_class_name().unwrap())
                    .into_iter()
                    .map(|(class_name, component)| (
                        class_name,
                        component.collect::<Vec<_>>(),
                    ))
                    .collect::<HashMap<_, _>>()
            }
        }
        impl ArchetypeArcRwLock {
            /// Returns an ArchetypeArcRwLock
            pub fn new(archetype: Archetype) -> Self {
                Self(Arc::new(RwLock::new(archetype)))
            }
            /// Get a reference to an entity by index
            ///
            /// # Arguments
            /// * `entity_id` - The entity id
            ///
            pub fn get(&self, entity_id: usize) -> EntityReference {
                EntityReference {
                    entity_id,
                    archetype: self.clone(),
                }
            }
            /// Get components of a specified type from an entity by index
            ///
            /// # Arguments
            /// * `entity_id` - The entity id
            /// * `component_identifier` - The components type identifier
            ///
            pub fn get_entity_components_from_type(
                &self,
                entity_id: usize,
                component_identifier: &str,
            ) -> Vec<ComponentReference> {
                let archetype_reader = self.0.read();
                archetype_reader
                    .component_storages
                    .get(component_identifier)
                    .map(|storage| {
                        let start_index = entity_id * storage.components_per_entity;
                        let end_index = start_index + storage.components_per_entity;
                        (start_index..end_index)
                            .into_iter()
                            .map(move |index| ComponentReference {
                                entity_reference: EntityReference {
                                    entity_id,
                                    archetype: self.clone(),
                                },
                                component_identifier: component_identifier.to_string(),
                                component_index: index,
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default()
            }
            /// Get an iterator over all the components of all the entities
            pub fn iter(
                &self,
                ignore_enabled: bool,
            ) -> impl Iterator<Item = EntityReference> + '_ {
                let archetype_len = self.0.read().len();
                (0..archetype_len)
                    .filter(move |entity_id| {
                        let is_deleted = {
                            let archetype_reader = self.0.read();
                            let erased_indexes_reader = archetype_reader
                                .erased_indexes
                                .read();
                            erased_indexes_reader.contains(entity_id)
                        };
                        if is_deleted {
                            return false;
                        }
                        if !ignore_enabled {
                            let archetype_reader = self.0.read();
                            *archetype_reader.enabled_array.get(*entity_id).unwrap()
                        } else {
                            true
                        }
                    })
                    .map(move |entity_id| EntityReference {
                        entity_id,
                        archetype: self.clone(),
                    })
            }
            /// Get components from an entity by index
            ///
            /// # Arguments
            /// * `entity_id` - The entity id
            ///
            pub fn get_entity_components(
                &self,
                entity_id: usize,
            ) -> Vec<ComponentReference> {
                let archetype_reader = self.0.read();
                archetype_reader
                    .component_storages
                    .iter()
                    .map(|(component_identifier, storage)| {
                        let start_index = entity_id * storage.components_per_entity;
                        let end_index = start_index + storage.components_per_entity;
                        (start_index..end_index)
                            .into_iter()
                            .map(move |index| ComponentReference {
                                entity_reference: EntityReference {
                                    entity_id,
                                    archetype: self.clone(),
                                },
                                component_identifier: component_identifier.clone(),
                                component_index: index,
                            })
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            }
        }
        impl Deref for ArchetypeArcRwLock {
            type Target = Arc<RwLock<Archetype>>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}
/// Provides collection for systems
pub mod system_service {
    use crate::ResourceContainer;
    use fruity_game_engine::any::FruityAny;
    use fruity_game_engine::inject::Inject;
    use fruity_game_engine::profile::profile_scope;
    use fruity_game_engine::resource::Resource;
    use fruity_game_engine::script_value::convert::TryFromScriptValue;
    use fruity_game_engine::script_value::convert::TryIntoScriptValue;
    use fruity_game_engine::script_value::ScriptCallback;
    use fruity_game_engine::script_value::ScriptValue;
    use fruity_game_engine::send_wrapper::SendWrapper;
    use fruity_game_engine::world::World;
    use fruity_game_engine::FruityResult;
    use fruity_game_engine::Mutex;
    use fruity_game_engine::{export, export_impl, export_struct};
    use rayon::prelude::*;
    use std::collections::BTreeMap;
    use std::fmt::Debug;
    use std::rc::Rc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::thread;
    /// A callback for a system called every frame
    pub type SystemCallback = dyn Fn(ResourceContainer) + Sync + Send + 'static;
    /// A callback for a startup system dispose callback
    pub type StartupDisposeSystemCallback = Option<
        Box<dyn FnOnce() + Sync + Send + 'static>,
    >;
    /// A callback for a startup system
    pub type StartupSystemCallback = dyn Fn(
        ResourceContainer,
    ) -> StartupDisposeSystemCallback + Sync + Send + 'static;
    /// Params for a system
    pub struct SystemParams {
        /// The pool index
        pub pool_index: usize,
        /// If true, the system is still running while pause
        pub ignore_pause: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SystemParams {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SystemParams",
                "pool_index",
                &&self.pool_index,
                "ignore_pause",
                &&self.ignore_pause,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SystemParams {
        #[inline]
        fn clone(&self) -> SystemParams {
            SystemParams {
                pool_index: ::core::clone::Clone::clone(&self.pool_index),
                ignore_pause: ::core::clone::Clone::clone(&self.ignore_pause),
            }
        }
    }
    impl ::fruity_game_engine::script_value::convert::TryFromScriptValue
    for SystemParams {
        fn from_script_value(
            value: ::fruity_game_engine::script_value::ScriptValue,
        ) -> ::fruity_game_engine::FruityResult<Self> {
            match value {
                ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                    match value.downcast::<Self>() {
                        Ok(value) => Ok(*value),
                        Err(value) => {
                            Ok(Self {
                                pool_index: <usize>::from_script_value(
                                    value.get_field_value("pool_index")?,
                                )?,
                                ignore_pause: <bool>::from_script_value(
                                    value.get_field_value("ignore_pause")?,
                                )?,
                            })
                        }
                    }
                }
                _ => {
                    Err(
                        ::fruity_game_engine::FruityError::InvalidArg({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to native object"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl Default for SystemParams {
        fn default() -> Self {
            Self {
                pool_index: 50,
                ignore_pause: false,
            }
        }
    }
    /// Params for a system
    pub struct StartupSystemParams {
        /// If true, the system is still running while pause
        pub ignore_pause: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StartupSystemParams {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "StartupSystemParams",
                "ignore_pause",
                &&self.ignore_pause,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StartupSystemParams {
        #[inline]
        fn clone(&self) -> StartupSystemParams {
            StartupSystemParams {
                ignore_pause: ::core::clone::Clone::clone(&self.ignore_pause),
            }
        }
    }
    impl ::fruity_game_engine::script_value::convert::TryFromScriptValue
    for StartupSystemParams {
        fn from_script_value(
            value: ::fruity_game_engine::script_value::ScriptValue,
        ) -> ::fruity_game_engine::FruityResult<Self> {
            match value {
                ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                    match value.downcast::<Self>() {
                        Ok(value) => Ok(*value),
                        Err(value) => {
                            Ok(Self {
                                ignore_pause: <bool>::from_script_value(
                                    value.get_field_value("ignore_pause")?,
                                )?,
                            })
                        }
                    }
                }
                _ => {
                    Err(
                        ::fruity_game_engine::FruityError::InvalidArg({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t convert ", " to native object"],
                                    &[::core::fmt::ArgumentV1::new_debug(&value)],
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    impl Default for StartupSystemParams {
        fn default() -> Self {
            Self { ignore_pause: false }
        }
    }
    struct StartupSystem {
        identifier: String,
        callback: Arc<StartupSystemCallback>,
        ignore_pause: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StartupSystem {
        #[inline]
        fn clone(&self) -> StartupSystem {
            StartupSystem {
                identifier: ::core::clone::Clone::clone(&self.identifier),
                callback: ::core::clone::Clone::clone(&self.callback),
                ignore_pause: ::core::clone::Clone::clone(&self.ignore_pause),
            }
        }
    }
    struct StartupDisposeSystem {
        identifier: String,
        callback: Box<dyn FnOnce() + Sync + Send + 'static>,
    }
    struct ScriptFrameSystem {
        identifier: String,
        callback: Rc<dyn ScriptCallback>,
        ignore_pause: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ScriptFrameSystem {
        #[inline]
        fn clone(&self) -> ScriptFrameSystem {
            ScriptFrameSystem {
                identifier: ::core::clone::Clone::clone(&self.identifier),
                callback: ::core::clone::Clone::clone(&self.callback),
                ignore_pause: ::core::clone::Clone::clone(&self.ignore_pause),
            }
        }
    }
    struct ScriptStartupSystem {
        identifier: String,
        callback: Rc<dyn ScriptCallback>,
        ignore_pause: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ScriptStartupSystem {
        #[inline]
        fn clone(&self) -> ScriptStartupSystem {
            ScriptStartupSystem {
                identifier: ::core::clone::Clone::clone(&self.identifier),
                callback: ::core::clone::Clone::clone(&self.callback),
                ignore_pause: ::core::clone::Clone::clone(&self.ignore_pause),
            }
        }
    }
    pub(crate) struct ScriptStartupDisposeSystem {
        identifier: String,
        callback: Rc<dyn ScriptCallback>,
    }
    struct FrameSystem {
        identifier: String,
        callback: Arc<SystemCallback>,
        ignore_pause: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FrameSystem {
        #[inline]
        fn clone(&self) -> FrameSystem {
            FrameSystem {
                identifier: ::core::clone::Clone::clone(&self.identifier),
                callback: ::core::clone::Clone::clone(&self.callback),
                ignore_pause: ::core::clone::Clone::clone(&self.ignore_pause),
            }
        }
    }
    /// A system pool, see [‘SystemService‘] for more informations
    pub struct FrameSystemPool {
        /// Systems of the pool
        systems: Vec<FrameSystem>,
        /// Script systems of the pool
        script_systems: SendWrapper<Vec<ScriptFrameSystem>>,
        /// Is the pool enabled
        enabled: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FrameSystemPool {
        #[inline]
        fn clone(&self) -> FrameSystemPool {
            FrameSystemPool {
                systems: ::core::clone::Clone::clone(&self.systems),
                script_systems: ::core::clone::Clone::clone(&self.script_systems),
                enabled: ::core::clone::Clone::clone(&self.enabled),
            }
        }
    }
    /// A systems collection
    ///
    /// There is three type of systems:
    /// - begin_systems are called just before the rendering but after the resources allocations, it's perfect for initiliazing your entities
    /// - end systems is called before closing the software
    /// - systems are called every frame
    ///
    /// There is a pool system, when you add a system, you can provide a pool, every systems of the same pool will be executed in parallel
    /// Try to use it realy rarely, cause parallel execution is realy usefull
    /// Pools from 0 to 10 and from 90 to 100 are reservec by the engine, you should avoid to create pool outside this range
    /// Pool 98 is for drawing
    /// Pool 99 is for camera
    ///
    pub struct SystemService {
        pause: AtomicBool,
        system_pools: BTreeMap<usize, FrameSystemPool>,
        startup_systems: Vec<StartupSystem>,
        startup_dispose_callbacks: Mutex<Vec<StartupDisposeSystem>>,
        startup_pause_dispose_callbacks: Mutex<Vec<StartupDisposeSystem>>,
        script_startup_systems: SendWrapper<Vec<ScriptStartupSystem>>,
        script_startup_dispose_callbacks: SendWrapper<Vec<ScriptStartupDisposeSystem>>,
        resource_container: ResourceContainer,
    }
    impl ::fruity_game_engine::introspect::IntrospectFields for SystemService {
        fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
            Ok("SystemService".to_string())
        }
        fn get_field_names(&self) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: ::fruity_game_engine::script_value::ScriptValue,
        ) -> ::fruity_game_engine::FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(
            &self,
            name: &str,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl ::fruity_game_engine::any::FruityAny for SystemService {
        fn get_type_name(&self) -> &'static str {
            "SystemService"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(
            &mut self,
        ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
            self
        }
        fn as_fruity_any_box(
            self: Box<Self>,
        ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
            self
        }
    }
    impl ::fruity_game_engine::resource::Resource for SystemService {
        fn as_resource_box(
            self: Box<Self>,
        ) -> Box<dyn ::fruity_game_engine::resource::Resource> {
            self
        }
        fn as_any_arc(
            self: std::sync::Arc<Self>,
        ) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
            self
        }
    }
    impl Debug for SystemService {
        fn fmt(
            &self,
            _: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            Ok(())
        }
    }
    impl SystemService {
        /// Returns a SystemService
        pub fn new(resource_container: ResourceContainer) -> SystemService {
            SystemService {
                pause: AtomicBool::new(false),
                system_pools: BTreeMap::new(),
                startup_systems: Vec::new(),
                startup_dispose_callbacks: Mutex::new(Vec::new()),
                startup_pause_dispose_callbacks: Mutex::new(Vec::new()),
                script_startup_systems: SendWrapper::new(Vec::new()),
                script_startup_dispose_callbacks: SendWrapper::new(Vec::new()),
                resource_container,
            }
        }
        /// Add a system to the collection
        ///
        /// # Arguments
        /// * `system` - A function that will compute the world
        /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
        ///
        pub fn add_system<T: Inject<()>>(
            &mut self,
            identifier: &str,
            callback: T,
            params: Option<SystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = FrameSystem {
                identifier: identifier.to_string(),
                callback: callback.inject().into(),
                ignore_pause: params.ignore_pause,
            };
            if let Some(pool) = self.system_pools.get_mut(&params.pool_index) {
                pool.systems.push(system)
            } else {
                let systems = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([system]),
                );
                self.system_pools
                    .insert(
                        params.pool_index,
                        FrameSystemPool {
                            systems,
                            script_systems: SendWrapper::new(::alloc::vec::Vec::new()),
                            enabled: true,
                        },
                    );
            };
        }
        /// Add a startup system
        ///
        /// # Arguments
        /// * `system` - A function that will compute the world
        /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
        ///
        pub fn add_startup_system<T: Inject<StartupDisposeSystemCallback>>(
            &mut self,
            identifier: &str,
            callback: T,
            params: Option<StartupSystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = StartupSystem {
                identifier: identifier.to_string(),
                callback: callback.inject().into(),
                ignore_pause: params.ignore_pause,
            };
            self.startup_systems.push(system);
        }
        /// Add a system to the collection
        ///
        /// # Arguments
        /// * `system` - A function that will compute the world
        /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
        ///
        pub fn add_script_system(
            &mut self,
            identifier: String,
            callback: Rc<dyn ScriptCallback>,
            params: Option<SystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = ScriptFrameSystem {
                identifier: identifier.to_string(),
                callback: callback,
                ignore_pause: params.ignore_pause,
            };
            if let Some(pool) = self.system_pools.get_mut(&params.pool_index) {
                pool.script_systems.push(system)
            } else {
                let script_systems = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([system]),
                );
                self.system_pools
                    .insert(
                        params.pool_index,
                        FrameSystemPool {
                            systems: ::alloc::vec::Vec::new(),
                            script_systems: SendWrapper::new(script_systems),
                            enabled: true,
                        },
                    );
            };
        }
        /// Add a startup system
        ///
        /// # Arguments
        /// * `system` - A function that will compute the world
        /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
        ///
        pub fn add_script_startup_system(
            &mut self,
            identifier: String,
            callback: Rc<dyn ScriptCallback>,
            params: Option<StartupSystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = ScriptStartupSystem {
                identifier: identifier.to_string(),
                callback: callback,
                ignore_pause: params.ignore_pause,
            };
            self.script_startup_systems.push(system);
        }
        /// Iter over all the systems pools
        fn iter_system_pools(&self) -> impl Iterator<Item = &FrameSystemPool> {
            self.system_pools.iter().map(|pool| pool.1)
        }
        /// Run all the stored systems
        pub(crate) fn run_frame(&self, world: &World) -> FruityResult<()> {
            let is_paused = self.is_paused();
            self.iter_system_pools()
                .map(|pool| pool.clone())
                .try_for_each(|pool| {
                    if pool.enabled {
                        let resource_container = world.get_resource_container();
                        let handler = thread::spawn(move || {
                            pool.systems
                                .iter()
                                .par_bridge()
                                .for_each(|system| {
                                    if !is_paused || system.ignore_pause {
                                        let _profiler_scope = profile_scope(&system.identifier);
                                        (system.callback)(resource_container.clone())
                                    }
                                });
                        });
                        let script_resource_container = world
                            .get_script_resource_container();
                        pool.script_systems
                            .iter()
                            .try_for_each(|system| {
                                if !is_paused || system.ignore_pause {
                                    let _profiler_scope = profile_scope(&system.identifier);
                                    system
                                        .callback
                                        .call(
                                            <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([
                                                    script_resource_container.clone().into_script_value()?,
                                                ]),
                                            ),
                                        )?;
                                }
                                FruityResult::Ok(())
                            })?;
                        handler.join().unwrap();
                    }
                    FruityResult::Ok(())
                })
        }
        /// Run all the startup systems
        pub(crate) fn run_start(&mut self, world: &World) -> FruityResult<()> {
            let resource_container = world.get_resource_container();
            self.startup_systems
                .par_iter()
                .filter(|system| system.ignore_pause)
                .for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);
                    let dispose_callback = (system.callback)(resource_container.clone());
                    if let Some(dispose_callback) = dispose_callback {
                        let mut startup_dispose_callbacks = self
                            .startup_dispose_callbacks
                            .lock();
                        startup_dispose_callbacks
                            .push(StartupDisposeSystem {
                                identifier: system.identifier.clone(),
                                callback: dispose_callback,
                            });
                    }
                });
            let script_resource_container = world.get_script_resource_container();
            self.script_startup_systems
                .iter()
                .filter(|system| system.ignore_pause)
                .try_for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);
                    let dispose_callback = system
                        .callback
                        .call(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    script_resource_container.clone().into_script_value()?,
                                ]),
                            ),
                        )?;
                    if let ScriptValue::Callback(dispose_callback) = dispose_callback {
                        self.script_startup_dispose_callbacks
                            .push(ScriptStartupDisposeSystem {
                                identifier: system.identifier.clone(),
                                callback: dispose_callback,
                            });
                    }
                    FruityResult::Ok(())
                })?;
            if !self.is_paused() {
                self.run_unpause_start();
            }
            Result::Ok(())
        }
        /// Run all startup dispose callbacks
        pub(crate) fn run_end(&mut self, _world: &World) -> FruityResult<()> {
            if !self.is_paused() {
                self.run_unpause_end();
            }
            let mut startup_dispose_callbacks = self.startup_dispose_callbacks.lock();
            startup_dispose_callbacks
                .drain(..)
                .par_bridge()
                .for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);
                    (system.callback)()
                });
            self.script_startup_dispose_callbacks
                .drain(..)
                .try_for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);
                    system.callback.call(::alloc::vec::Vec::new()).map(|_| ())
                })?;
            FruityResult::Ok(())
        }
        /// Run all the startup systems that start when pause is stopped
        fn run_unpause_start(&self) {
            self.startup_systems
                .iter()
                .filter(|system| !system.ignore_pause)
                .for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);
                    let dispose_callback = (system
                        .callback)(self.resource_container.clone());
                    if let Some(dispose_callback) = dispose_callback {
                        let mut startup_dispose_callbacks = self
                            .startup_pause_dispose_callbacks
                            .lock();
                        startup_dispose_callbacks
                            .push(StartupDisposeSystem {
                                identifier: system.identifier.clone(),
                                callback: dispose_callback,
                            });
                    }
                });
        }
        /// Run all the startup dispose callbacks of systems that start when pause is stopped
        fn run_unpause_end(&self) {
            let mut startup_dispose_callbacks = self
                .startup_pause_dispose_callbacks
                .lock();
            startup_dispose_callbacks
                .drain(..)
                .for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);
                    (system.callback)()
                });
        }
        /// Enable a pool
        pub fn enable_pool(&mut self, index: usize) {
            if let Some(pool) = self.system_pools.get_mut(&index) {
                pool.enabled = true;
            }
        }
        /// Disable a pool
        pub fn disable_pool(&mut self, index: usize) {
            if let Some(pool) = self.system_pools.get_mut(&index) {
                pool.enabled = false;
            }
        }
        /// Is systems paused
        pub fn is_paused(&self) -> bool {
            self.pause.load(Ordering::Relaxed)
        }
        /// Set if systems are paused, only systems that ignore pause will be executed
        ///
        /// # Arguments
        /// * `paused` - The paused value
        ///
        pub fn set_paused(&self, paused: bool) {
            if !paused && self.is_paused() {
                self.run_unpause_start();
            }
            if paused && !self.is_paused() {
                self.run_unpause_end();
            }
            self.pause.store(paused, Ordering::Relaxed);
        }
    }
    impl ::fruity_game_engine::introspect::IntrospectMethods for SystemService {
        fn get_const_method_names(
            &self,
        ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        "is_paused".to_string(),
                        "set_paused".to_string(),
                    ]),
                ),
            )
        }
        fn call_const_method(
            &self,
            name: &str,
            __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
            match name {
                "is_paused" => self.is_paused().into_script_value(),
                "set_paused" => {
                    let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<bool>()?;
                    self.set_paused(__arg_0).into_script_value()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_mut_method_names(
            &self,
        ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        "add_system".to_string(),
                        "add_startup_system".to_string(),
                    ]),
                ),
            )
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
            match name {
                "add_system" => {
                    let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<String>()?;
                    let __arg_1 = __caster.cast_next::<Rc<dyn ScriptCallback>>()?;
                    let __arg_2 = __caster.cast_next::<Option<SystemParams>>()?;
                    self.add_script_system(__arg_0, __arg_1, __arg_2).into_script_value()
                }
                "add_startup_system" => {
                    let mut __caster = ::fruity_game_engine::utils::introspect::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<String>()?;
                    let __arg_1 = __caster.cast_next::<Rc<dyn ScriptCallback>>()?;
                    let __arg_2 = __caster.cast_next::<Option<StartupSystemParams>>()?;
                    self.add_script_startup_system(__arg_0, __arg_1, __arg_2)
                        .into_script_value()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
}
/// A service to store components extensions
pub mod extension_component_service {
    use crate::component::component::AnyComponent;
    use crate::component::component::Component;
    use crate::component::component::StaticComponent;
    use fruity_game_engine::any::FruityAny;
    use fruity_game_engine::resource::resource_container::ResourceContainer;
    use fruity_game_engine::resource::Resource;
    use fruity_game_engine::FruityResult;
    use fruity_game_engine::{export_impl, export_struct};
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fmt::Formatter;
    /// A service to store components extensions
    /// When a component is created, if an extension is registered, an other component with a given
    /// type is created, this can be use if ou want to extend already existing components with other
    /// attributes. This is for example used into the physic engine implementations.
    ///
    /// Warning: The same extension type cannot be shared across multiple based component types
    pub struct ExtensionComponentService {
        extension_constructors: HashMap<
            String,
            Vec<Box<dyn Fn() -> AnyComponent + Send + Sync>>,
        >,
    }
    impl ::fruity_game_engine::introspect::IntrospectFields
    for ExtensionComponentService {
        fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
            Ok("ExtensionComponentService".to_string())
        }
        fn get_field_names(&self) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: ::fruity_game_engine::script_value::ScriptValue,
        ) -> ::fruity_game_engine::FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(
            &self,
            name: &str,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl ::fruity_game_engine::any::FruityAny for ExtensionComponentService {
        fn get_type_name(&self) -> &'static str {
            "ExtensionComponentService"
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(
            &mut self,
        ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
            self
        }
        fn as_fruity_any_box(
            self: Box<Self>,
        ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
            self
        }
        fn as_fruity_any_rc(
            self: std::rc::Rc<Self>,
        ) -> std::rc::Rc<dyn ::fruity_game_engine::any::FruityAny> {
            self
        }
    }
    impl ::fruity_game_engine::resource::Resource for ExtensionComponentService {
        fn as_resource_box(
            self: Box<Self>,
        ) -> Box<dyn ::fruity_game_engine::resource::Resource> {
            self
        }
        fn as_any_arc(
            self: std::sync::Arc<Self>,
        ) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
            self
        }
    }
    impl ExtensionComponentService {
        /// Returns an ExtensionComponentService
        pub fn new(_resource_container: ResourceContainer) -> Self {
            Self {
                extension_constructors: HashMap::new(),
            }
        }
        /// Register a component extension
        pub fn register<T: StaticComponent, E: Component + Default>(&mut self) {
            let constructor = Box::new(|| AnyComponent::new(E::default()));
            match self.extension_constructors.get_mut(T::get_component_name()) {
                Some(constructors) => {
                    constructors.push(constructor);
                }
                None => {
                    self.extension_constructors
                        .insert(
                            T::get_component_name().to_string(),
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([constructor]),
                            ),
                        );
                }
            }
        }
        /// Create extensions from a component
        pub fn get_component_extension(
            &self,
            component: &dyn Component,
        ) -> FruityResult<Vec<AnyComponent>> {
            Ok(
                match self.extension_constructors.get(&component.get_class_name()?) {
                    Some(constructors) => {
                        constructors
                            .iter()
                            .map(|constructor| constructor())
                            .collect::<Vec<_>>()
                    }
                    None => ::alloc::vec::Vec::new(),
                },
            )
        }
    }
    impl ::fruity_game_engine::introspect::IntrospectMethods
    for ExtensionComponentService {
        fn get_const_method_names(
            &self,
        ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_const_method(
            &self,
            name: &str,
            __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_mut_method_names(
            &self,
        ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl Debug for ExtensionComponentService {
        fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
            Ok(())
        }
    }
}
/// Name of the module
pub fn name() -> String {
    "fruity_ecs".to_string()
}
#[allow(missing_docs)]
pub fn __wasm_name() -> Result<
    ::fruity_game_engine::wasm_bindgen::JsValue,
    ::fruity_game_engine::wasm_bindgen::JsError,
> {
    use ::fruity_game_engine::script_value::convert::TryFromScriptValue;
    use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
    let _ret = {
        let _ret = name();
        <String>::into_script_value(_ret).unwrap()
    };
    ::fruity_game_engine::javascript::wasm::script_value_to_js_value(_ret)
        .map_err(|err| ::fruity_game_engine::wasm_bindgen::JsError::from(err))
}
#[automatically_derived]
const __wasm_bindgen_generated_name__const: () = {
    #[allow(missing_docs)]
    pub unsafe extern "C" fn __wasm_bindgen_generated_name() -> <Result<
        ::fruity_game_engine::wasm_bindgen::JsValue,
        ::fruity_game_engine::wasm_bindgen::JsError,
    > as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let _ret = __wasm_name();
            _ret
        };
        <Result<
            ::fruity_game_engine::wasm_bindgen::JsValue,
            ::fruity_game_engine::wasm_bindgen::JsError,
        > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
/// Dependencies of the module
pub fn dependencies() -> Vec<String> {
    ::alloc::vec::Vec::new()
}
#[allow(missing_docs)]
pub fn __wasm_dependencies() -> Result<
    ::fruity_game_engine::wasm_bindgen::JsValue,
    ::fruity_game_engine::wasm_bindgen::JsError,
> {
    use ::fruity_game_engine::script_value::convert::TryFromScriptValue;
    use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
    let _ret = {
        let _ret = dependencies();
        <Vec<String>>::into_script_value(_ret).unwrap()
    };
    ::fruity_game_engine::javascript::wasm::script_value_to_js_value(_ret)
        .map_err(|err| ::fruity_game_engine::wasm_bindgen::JsError::from(err))
}
#[automatically_derived]
const __wasm_bindgen_generated_dependencies__const: () = {
    #[allow(missing_docs)]
    pub unsafe extern "C" fn __wasm_bindgen_generated_dependencies() -> <Result<
        ::fruity_game_engine::wasm_bindgen::JsValue,
        ::fruity_game_engine::wasm_bindgen::JsError,
    > as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let _ret = __wasm_dependencies();
            _ret
        };
        <Result<
            ::fruity_game_engine::wasm_bindgen::JsValue,
            ::fruity_game_engine::wasm_bindgen::JsError,
        > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
/// Setup the module
pub fn setup(world: World, _settings: Settings) -> FruityResult<()> {
    let resource_container = world.get_resource_container();
    let system_service = SystemService::new(resource_container.clone());
    resource_container.add::<SystemService>("system_service", Box::new(system_service));
    let system_service = resource_container.require::<SystemService>();
    world
        .add_run_start_middleware(move |next, world| {
            let mut system_service_writer = system_service.write();
            system_service_writer.run_start(world)?;
            next(world)
        });
    let system_service = resource_container.require::<SystemService>();
    world
        .add_run_frame_middleware(move |next, world| {
            let system_service_reader = system_service.read();
            system_service_reader.run_frame(world)?;
            next(world)
        });
    let system_service = resource_container.require::<SystemService>();
    world
        .add_run_end_middleware(move |next, world| {
            let mut system_service_writer = system_service.write();
            system_service_writer.run_end(world)?;
            next(world)
        });
    let extension_component_service = ExtensionComponentService::new(
        resource_container.clone(),
    );
    resource_container
        .add::<
            ExtensionComponentService,
        >("extension_component_service", Box::new(extension_component_service));
    let entity_service = EntityService::new(resource_container.clone());
    resource_container.add::<EntityService>("entity_service", Box::new(entity_service));
    Ok(())
}
#[allow(missing_docs)]
pub fn __wasm_setup(
    arg_0: ::fruity_game_engine::wasm_bindgen::JsValue,
    arg_1: ::fruity_game_engine::wasm_bindgen::JsValue,
) -> Result<
    ::fruity_game_engine::wasm_bindgen::JsValue,
    ::fruity_game_engine::wasm_bindgen::JsError,
> {
    use ::fruity_game_engine::script_value::convert::TryFromScriptValue;
    use ::fruity_game_engine::script_value::convert::TryIntoScriptValue;
    let _ret = {
        let arg_0 = {
            let arg = ::fruity_game_engine::javascript::wasm::js_value_to_script_value(
                    arg_0,
                )
                .unwrap();
            <World>::from_script_value(arg).unwrap()
        };
        let arg_1 = {
            let arg = ::fruity_game_engine::javascript::wasm::js_value_to_script_value(
                    arg_1,
                )
                .unwrap();
            <Settings>::from_script_value(arg).unwrap()
        };
        let _ret = setup(arg_0, arg_1);
        <FruityResult<()>>::into_script_value(_ret).unwrap()
    };
    ::fruity_game_engine::javascript::wasm::script_value_to_js_value(_ret)
        .map_err(|err| ::fruity_game_engine::wasm_bindgen::JsError::from(err))
}
#[automatically_derived]
const __wasm_bindgen_generated_setup__const: () = {
    #[allow(missing_docs)]
    pub unsafe extern "C" fn __wasm_bindgen_generated_setup(
        arg0: <::fruity_game_engine::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi,
        arg1: <::fruity_game_engine::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi,
    ) -> <Result<
        ::fruity_game_engine::wasm_bindgen::JsValue,
        ::fruity_game_engine::wasm_bindgen::JsError,
    > as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let arg0 = unsafe {
                <::fruity_game_engine::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    arg0,
                )
            };
            let arg1 = unsafe {
                <::fruity_game_engine::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    arg1,
                )
            };
            let _ret = __wasm_setup(arg0, arg1);
            _ret
        };
        <Result<
            ::fruity_game_engine::wasm_bindgen::JsValue,
            ::fruity_game_engine::wasm_bindgen::JsError,
        > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
/// The ecs module, ready to be registered into the fruity_game_engine
pub struct FRUITY_ECS_MODULE {
    __private_field: (),
}
#[doc(hidden)]
pub static FRUITY_ECS_MODULE: FRUITY_ECS_MODULE = FRUITY_ECS_MODULE {
    __private_field: (),
};
impl ::lazy_static::__Deref for FRUITY_ECS_MODULE {
    type Target = SendWrapper<Module>;
    fn deref(&self) -> &SendWrapper<Module> {
        #[inline(always)]
        fn __static_ref_initialize() -> SendWrapper<Module> {
            SendWrapper::new(Module {
                name: name(),
                dependencies: dependencies(),
                setup: Some(Rc::new(setup)),
                load_resources: None,
            })
        }
        #[inline(always)]
        fn __stability() -> &'static SendWrapper<Module> {
            static LAZY: ::lazy_static::lazy::Lazy<SendWrapper<Module>> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for FRUITY_ECS_MODULE {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
