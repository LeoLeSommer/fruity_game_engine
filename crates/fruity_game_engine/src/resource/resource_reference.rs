use crate::any::FruityAny;
use crate::introspect::IntrospectObject;
use crate::resource::Resource;
use crate::script_value::ScriptValue;
use crate::FruityResult;
use crate::RwLock;
use crate::RwLockReadGuard;
use crate::RwLockWriteGuard;
use std::ops::Deref;
use std::ops::DerefMut;
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
    pub fn from_native<T: Resource + ?Sized>(name: &str, resource: Box<T>) -> Self {
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
        self.resource
            .clone()
            .as_any_arc()
            .downcast::<RwLock<Box<T>>>()
            .ok()
            .map(|resource| ResourceReference::new(&self.name, resource))
    }
}

impl IntrospectObject for AnyResourceReference {
    fn get_class_name(&self) -> FruityResult<String> {
        self.resource.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.resource.get_field_names()
    }

    fn set_field_value(&mut self, _name: &str, _value: ScriptValue) -> FruityResult<()> {
        unreachable!()
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.resource.get_field_value(name)
    }

    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.resource.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.resource.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_mut_method(
        &mut self,
        _name: &str,
        _args: Vec<ScriptValue>,
    ) -> FruityResult<ScriptValue> {
        unreachable!()
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
            std::mem::transmute::<RwLockReadGuard<Box<T>>, RwLockReadGuard<'static, Box<T>>>(
                inner_guard,
            )
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
