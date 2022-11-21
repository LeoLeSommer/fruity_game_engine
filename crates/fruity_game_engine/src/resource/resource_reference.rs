use crate::any::FruityAny;
use crate::convert::FruityFrom;
use crate::introspect::FieldInfo;
use crate::introspect::IntrospectObject;
use crate::introspect::MethodCaller;
use crate::introspect::MethodInfo;
use crate::introspect::SetterCaller;
use crate::resource::Resource;
use crate::script_value::ScriptValue;
use crate::utils::introspect::cast_introspect_mut;
use crate::utils::introspect::cast_introspect_ref;
use crate::RwLock;
use crate::RwLockReadGuard;
use crate::RwLockWriteGuard;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::Arc;

/// A reference over an any resource that is supposed to be used by components
#[derive(Debug, Clone, FruityAny)]
pub struct AnyResourceReference {
  /// The name of the resource
  pub name: String,

  /// The resource reference
  pub resource: Arc<dyn Resource>,
}

impl AnyResourceReference {
  /// Create a resource reference from a resource
  pub fn from<T: Resource + ?Sized>(name: &str, resource: Box<T>) -> Self {
    AnyResourceReference {
      name: name.to_string(),
      resource: Arc::new(RwLock::new(resource)),
    }
  }

  /// Get the name of the referenced resource
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /// Get the name of the referenced resource
  pub fn downcast<T: Resource + ?Sized>(&self) -> Option<ResourceReference<T>> {
    self
      .resource
      .clone()
      .as_any_arc()
      .downcast::<RwLock<Box<T>>>()
      .ok()
      .map(|resource| ResourceReference::new(&self.name, resource))
  }
}

impl IntrospectObject for AnyResourceReference {
  fn get_class_name(&self) -> String {
    "ResourceReference".to_string()
  }

  fn get_field_infos(&self) -> Vec<FieldInfo> {
    let mut fields_infos = self
      .resource
      .get_field_infos()
      .into_iter()
      .map(|field_info| {
        let field_info_2 = field_info.clone();

        FieldInfo {
          name: field_info.name.clone(),
          getter: Rc::new(move |this| {
            let this = cast_introspect_ref::<AnyResourceReference>(this)?;
            (field_info.getter)(this.resource.deref().as_fruity_any_ref())
          }),
          setter: match field_info_2.setter {
            SetterCaller::Const(call) => SetterCaller::Const(Rc::new(move |this, args| {
              let this = cast_introspect_ref::<AnyResourceReference>(this)?;
              call(this.resource.deref().as_fruity_any_ref(), args)
            })),
            SetterCaller::Mut(_) => SetterCaller::Mut(Rc::new(move |_, _| unimplemented!())),
            SetterCaller::None => SetterCaller::None,
          },
        }
      })
      .collect::<Vec<_>>();

    fields_infos.append(&mut vec![FieldInfo {
      name: "resource_name".to_string(),
      getter: Rc::new(move |this| {
        let this = cast_introspect_ref::<AnyResourceReference>(this)?;
        Ok(ScriptValue::String(this.name.clone()))
      }),
      setter: SetterCaller::Mut(Rc::new(move |this, value| {
        let mut this = cast_introspect_mut::<AnyResourceReference>(this)?;
        let value = String::fruity_try_from(value)?;
        this.name = value;

        Result::Ok(())
      })),
    }]);

    fields_infos
  }

  fn get_method_infos(&self) -> Vec<MethodInfo> {
    self
      .resource
      .get_method_infos()
      .into_iter()
      .map(|method_info| MethodInfo {
        name: method_info.name,
        call: match method_info.call {
          MethodCaller::Const(call) => MethodCaller::Const(Rc::new(move |this, args| {
            let this = cast_introspect_ref::<AnyResourceReference>(this)?;
            call(this.resource.deref().as_fruity_any_ref(), args)
          })),
          MethodCaller::Mut(_) => MethodCaller::Mut(Rc::new(move |_, _| unimplemented!())),
        },
      })
      .collect::<Vec<_>>()
  }
}

/// A reference over a resource that is supposed to be used by components
#[derive(Debug, FruityAny)]
pub struct ResourceReference<T: Resource + ?Sized> {
  /// The name of the resource
  pub name: String,

  /// The resource reference
  pub resource: Arc<RwLock<Box<T>>>,
}

impl<T: Resource + ?Sized> ResourceReference<T> {
  /// Create a resource reference from a resource
  pub fn new(name: &str, resource: Arc<RwLock<Box<T>>>) -> Self {
    ResourceReference {
      name: name.to_string(),
      resource,
    }
  }

  /// Get the name of the referenced resource
  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /// Create a read guard over the resource
  pub fn read(&self) -> ResourceReadGuard<T> {
    let inner_guard = self.resource.read();

    // Safe cause the resource guard contains an arc to the referenced resource so it will
    // not be released until the guard is released
    let inner_guard = unsafe {
      std::mem::transmute::<RwLockReadGuard<Box<T>>, RwLockReadGuard<'static, Box<T>>>(inner_guard)
    };

    ResourceReadGuard::<T> {
      _referenced: self.resource.clone(),
      inner_guard,
    }
  }

  /// Create a write guard over the resource
  pub fn write(&self) -> ResourceWriteGuard<T> {
    let inner_guard = self.resource.write();

    // Safe cause the resource guard contains an arc to the referenced resource so it will
    // not be released until the guard is released
    let inner_guard = unsafe {
      std::mem::transmute::<RwLockWriteGuard<Box<T>>, RwLockWriteGuard<'static, Box<T>>>(
        inner_guard,
      )
    };

    ResourceWriteGuard::<T> {
      _referenced: self.resource.clone(),
      inner_guard,
    }
  }
}

impl<T: Resource + ?Sized> Clone for ResourceReference<T> {
  fn clone(&self) -> Self {
    ResourceReference::<T>::new(&self.name, self.resource.clone())
  }
}

/// A read guard for a resource reference
pub struct ResourceReadGuard<T: Resource + ?Sized> {
  _referenced: Arc<RwLock<Box<T>>>,
  inner_guard: RwLockReadGuard<'static, Box<T>>,
}

impl<'a, T: Resource + ?Sized> ResourceReadGuard<T> {
  /// Downcast to the original sized type that implement the resource trait
  pub fn downcast_ref<U: Resource>(&self) -> &U {
    self.deref().as_any_ref().downcast_ref::<U>().unwrap()
  }
}

impl<'a, T: Resource + ?Sized> Deref for ResourceReadGuard<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.inner_guard.deref()
  }
}

/// A write guard for a resource reference
pub struct ResourceWriteGuard<T: Resource + ?Sized> {
  _referenced: Arc<RwLock<Box<T>>>,
  inner_guard: RwLockWriteGuard<'static, Box<T>>,
}

impl<T: Resource + ?Sized> ResourceWriteGuard<T> {
  /// Downcast to the original sized type that implement the resource trait
  pub fn downcast_ref<U: Resource>(&self) -> &U {
    self.deref().as_any_ref().downcast_ref::<U>().unwrap()
  }
}

impl<T: Resource + ?Sized> ResourceWriteGuard<T> {
  /// Downcast to the original sized type that implement the resource trait
  pub fn downcast_mut<U: Resource>(&mut self) -> &mut U {
    self.deref_mut().as_any_mut().downcast_mut::<U>().unwrap()
  }
}

impl<T: Resource + ?Sized> Deref for ResourceWriteGuard<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.inner_guard.deref()
  }
}

impl<T: Resource + ?Sized> DerefMut for ResourceWriteGuard<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.inner_guard.deref_mut()
  }
}
