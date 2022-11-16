use crate::any::FruityAny;
use crate::resource::Resource;
use crate::utils::javascript::get_context_args;
use crate::utils::javascript::js_object_keys;
use crate::ResourceContainer;
use crate::RwLock;
use crate::RwLockReadGuard;
use crate::RwLockWriteGuard;
use neon::context::Context;
use neon::object::Object;
use neon::prelude::FunctionContext;
use neon::prelude::Handle;
use neon::prelude::Value;
use neon::result::JsResult;
use neon::result::NeonResult;
use neon::types::Finalize;
use neon::types::JsBox;
use neon::types::JsFunction;
use neon::types::JsObject;
use neon::types::JsValue;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

/// A reference over an any resource that is supposed to be used by components
#[derive(Debug, Clone, FruityAny)]
pub struct AnyResourceReference {
    /// The name of the resource
    pub name: String,

    /// The resource reference
    pub resource: Arc<RwLock<Box<dyn Resource>>>,

    /// The resource container reference
    pub resource_container: ResourceContainer,
}

impl AnyResourceReference {
    /// Create a resource reference from a resource
    pub fn new(
        name: &str,
        resource: Arc<RwLock<Box<dyn Resource>>>,
        resource_container: ResourceContainer,
    ) -> Self {
        AnyResourceReference {
            name: name.to_string(),
            resource,
            resource_container,
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
            .map(|resource| {
                ResourceReference::new(&self.name, resource, self.resource_container.clone())
            })
    }
}

impl Finalize for AnyResourceReference {}

impl crate::javascript::ToJavascript for AnyResourceReference {
    fn to_js<'a>(self, cx: &'a mut FunctionContext) -> NeonResult<Handle<'a, JsValue>> {
        // Store the reference to the rust object
        let boxed: Handle<JsBox<Self>> = cx.boxed(self.clone());

        // Generate inner javascript object
        let resource = self.resource.read();
        let inner_js: Handle<JsObject> = resource.to_js(cx)?.downcast(cx).unwrap();

        // Get inner object defined keys
        let inner_keys = js_object_keys(cx, &inner_js)?;

        // Generate methods wrapper
        for key in inner_keys.into_iter() {
            let value = inner_js.get_value(cx, key.as_str())?;
            if value.is_a::<JsFunction, FunctionContext>(cx) {
                let key_2 = key.clone();
                let wrapper =
                    JsFunction::new(cx, move |mut cx: FunctionContext| -> JsResult<JsValue> {
                        let inner_method: Handle<JsFunction> =
                            inner_js.get(&mut cx, key_2.as_str())?;

                        let args = get_context_args(&mut cx)?;
                        let result = inner_method.call(&mut cx, inner_js, args)?;

                        Ok(result)
                    })?;

                boxed.set(cx, key.as_str(), wrapper)?;
            }
        }

        Ok(boxed.as_value(cx))

        /*let get_delta = JsFunction::new(cx, |mut cx: cFunctionContext| -> JsResult<JsValue> {
            let this: cHandle<JsBox<RefCell<Self>>> = cx.this().downcast(&mut cx).unwrap();
            let this = this.borrow();

            let result = this.get_delta();

            // TODO: Find a way to remove this
            let cx = unsafe {
                std::mem::transmute::<
                    &mut neon::prelude::FunctionContext,
                    &mut neon::prelude::FunctionContext,
                >(&mut cx)
            };

            Ok(crate::javascript::ToJavascript::to_js(result, cx)?)
        })?;

        boxed.set(cx, "getDelta", get_delta)?;*/
    }
}

/// A reference over a resource that is supposed to be used by components
#[derive(Debug, FruityAny)]
pub struct ResourceReference<T: Resource + ?Sized> {
    /// The name of the resource
    pub name: String,

    /// The resource reference
    pub resource: Arc<RwLock<Box<T>>>,

    /// The resource container reference
    pub resource_container: ResourceContainer,
}

impl<T: Resource + ?Sized> ResourceReference<T> {
    /// Create a resource reference from a resource
    pub fn new(
        name: &str,
        resource: Arc<RwLock<Box<T>>>,
        resource_container: ResourceContainer,
    ) -> Self {
        ResourceReference {
            name: name.to_string(),
            resource,
            resource_container,
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
        ResourceReference::<T>::new(
            &self.name,
            self.resource.clone(),
            self.resource_container.clone(),
        )
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
