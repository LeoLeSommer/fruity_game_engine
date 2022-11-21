#![warn(missing_docs)]

//! Introspect
//!
//! Implements traits and macros to make a structure abe to list it's field and to get/set it with any
//!

use crate::any::FruityAny;
use crate::any_value::AnyValue;
use crate::utils::introspect::cast_introspect_mut;
use crate::utils::introspect::cast_introspect_ref;
use crate::FruityResult;
use crate::ResourceContainer;
use crate::RwLock;
use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

/// A setter caller
pub type Constructor =
  Arc<dyn Fn(ResourceContainer, Vec<AnyValue>) -> FruityResult<AnyValue> + Send + Sync>;

/// A setter caller
#[derive(Clone)]
pub enum SetterCaller {
  /// Without mutability
  Const(Arc<dyn Fn(&dyn Any, AnyValue) -> FruityResult<()> + Send + Sync>),

  /// With mutability
  Mut(Arc<dyn Fn(&mut dyn Any, AnyValue) -> FruityResult<()> + Send + Sync>),

  /// No setter
  None,
}

/// Informations about a field of an introspect object
#[derive(Clone)]
pub struct FieldInfo {
  /// The name of the field
  pub name: String,

  /// Function to get one of the entry field value as Any
  ///
  /// # Arguments
  /// * `property` - The field name
  ///
  pub getter: Arc<dyn Fn(&dyn Any) -> FruityResult<AnyValue> + Send + Sync>,

  /// Function to set one of the entry field
  ///
  /// # Arguments
  /// * `property` - The field name
  /// * `value` - The new field value as Any
  ///
  pub setter: SetterCaller,
}

/// A method caller
#[derive(Clone)]
pub enum MethodCaller {
  /// Without mutability
  Const(Arc<dyn Fn(&dyn Any, Vec<AnyValue>) -> FruityResult<AnyValue>>),

  /// With mutability
  Mut(Arc<dyn Fn(&mut dyn Any, Vec<AnyValue>) -> FruityResult<AnyValue>>),
}

/// Informations about a field of an introspect object
#[derive(Clone)]
pub struct MethodInfo {
  /// The name of the method
  pub name: String,

  /// Call for the method with any field
  pub call: MethodCaller,
}

/// Trait to implement static introspection to an object
pub trait InstantiableObject {
  /// Get a constructor to instantiate an introspect object
  fn get_constructor() -> Constructor;
}

/// Trait to implement static introspection to an object
pub trait IntrospectObject: Debug + FruityAny {
  /// Return the class type name
  fn get_class_name(&self) -> String;

  /// Get a list of fields with many informations
  fn get_field_infos(&self) -> Vec<FieldInfo>;

  /// Get a list of fields with many informations
  fn get_method_infos(&self) -> Vec<MethodInfo>;
}

impl<T: IntrospectObject + ?Sized> IntrospectObject for Box<T> {
  fn get_class_name(&self) -> String {
    self.as_ref().get_class_name()
  }

  fn get_field_infos(&self) -> Vec<FieldInfo> {
    self
      .as_ref()
      .get_field_infos()
      .into_iter()
      .map(|field_info| {
        let field_info_2 = field_info.clone();

        FieldInfo {
          name: field_info.name.clone(),
          getter: Arc::new(move |this| {
            let this = cast_introspect_ref::<Box<T>>(this)?;
            (field_info.getter)(this.as_ref().as_any_ref())
          }),
          setter: match field_info_2.setter {
            SetterCaller::Const(call) => SetterCaller::Const(Arc::new(move |this, args| {
              let this = cast_introspect_ref::<Box<T>>(this)?;
              call(this.as_ref().as_any_ref(), args)
            })),
            SetterCaller::Mut(call) => SetterCaller::Mut(Arc::new(move |this, args| {
              let this = cast_introspect_mut::<Box<T>>(this)?;
              call(this.as_mut().as_any_mut(), args)
            })),
            SetterCaller::None => SetterCaller::None,
          },
        }
      })
      .collect::<Vec<_>>()
  }

  fn get_method_infos(&self) -> Vec<MethodInfo> {
    self
      .as_ref()
      .get_method_infos()
      .into_iter()
      .map(|method_info| MethodInfo {
        name: method_info.name,
        call: match method_info.call {
          MethodCaller::Const(call) => MethodCaller::Const(Arc::new(move |this, args| {
            let this = cast_introspect_ref::<Box<T>>(this)?;
            call(this.as_ref().as_any_ref(), args)
          })),
          MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
            let this = cast_introspect_mut::<Box<T>>(this)?;
            call(this.as_mut().as_any_mut(), args)
          })),
        },
      })
      .collect::<Vec<_>>()
  }
}

impl<T: IntrospectObject + ?Sized> IntrospectObject for Arc<T> {
  fn get_class_name(&self) -> String {
    format!("Arc<{}>", self.as_ref().get_class_name())
  }

  fn get_field_infos(&self) -> Vec<FieldInfo> {
    self
      .as_ref()
      .get_field_infos()
      .into_iter()
      .map(|field_info| {
        let field_info_2 = field_info.clone();

        FieldInfo {
          name: field_info.name.clone(),
          getter: Arc::new(move |this| {
            let this = cast_introspect_ref::<Arc<T>>(this)?;
            (field_info.getter)(this.as_ref().as_any_ref())
          }),
          setter: match field_info_2.setter {
            SetterCaller::Const(call) => SetterCaller::Const(Arc::new(move |this, args| {
              let this = cast_introspect_ref::<Arc<T>>(this)?;
              call(this.as_ref().as_any_ref(), args)
            })),
            SetterCaller::Mut(_) => {
              panic!("Cannot call a mutable function from an arc, should be wrap into a lock");
            }
            SetterCaller::None => SetterCaller::None,
          },
        }
      })
      .collect::<Vec<_>>()
  }

  fn get_method_infos(&self) -> Vec<MethodInfo> {
    self
      .as_ref()
      .get_method_infos()
      .into_iter()
      .map(|method_info| MethodInfo {
        name: method_info.name,
        call: match method_info.call {
          MethodCaller::Const(call) => MethodCaller::Const(Arc::new(move |this, args| {
            let this = cast_introspect_ref::<Arc<T>>(this)?;
            call(this.as_ref().as_any_ref(), args)
          })),
          MethodCaller::Mut(_) => {
            panic!("Cannot call a mutable function from an arc, should be wrap into a lock");
          }
        },
      })
      .collect::<Vec<_>>()
  }
}

impl<T: IntrospectObject> IntrospectObject for RwLock<T> {
  fn get_class_name(&self) -> String {
    let reader = self.read();
    format!("RwLock<{}>", reader.get_class_name())
  }

  fn get_field_infos(&self) -> Vec<FieldInfo> {
    let reader = self.read();
    reader
      .get_field_infos()
      .into_iter()
      .map(|field_info| {
        let field_info_2 = field_info.clone();

        FieldInfo {
          name: field_info.name.clone(),
          getter: Arc::new(move |this| {
            let this = cast_introspect_ref::<RwLock<T>>(this)?;
            let reader = this.read();

            (field_info.getter)(reader.deref())
          }),
          setter: match field_info_2.setter {
            SetterCaller::Const(call) => SetterCaller::Const(Arc::new(move |this, args| {
              let this = cast_introspect_ref::<RwLock<T>>(this)?;
              let reader = this.read();

              call(reader.deref(), args)
            })),
            SetterCaller::Mut(call) => SetterCaller::Const(Arc::new(move |this, args| {
              let this = cast_introspect_ref::<RwLock<T>>(this)?;
              let mut writer = this.write();

              call(writer.deref_mut(), args)
            })),
            SetterCaller::None => SetterCaller::None,
          },
        }
      })
      .collect::<Vec<_>>()
  }

  fn get_method_infos(&self) -> Vec<MethodInfo> {
    let reader = self.read();
    reader
      .get_method_infos()
      .into_iter()
      .map(|method_info| MethodInfo {
        name: method_info.name,
        call: match method_info.call {
          MethodCaller::Const(call) => MethodCaller::Const(Arc::new(move |this, args| {
            let this = cast_introspect_ref::<RwLock<T>>(this)?;
            let reader = this.read();

            call(reader.deref(), args)
          })),
          MethodCaller::Mut(call) => MethodCaller::Const(Arc::new(move |this, args| {
            let this = cast_introspect_ref::<RwLock<T>>(this)?;
            let mut writer = this.write();

            call(writer.deref_mut(), args)
          })),
        },
      })
      .collect::<Vec<_>>()
  }
}
