use super::resource_reference::AnyResourceReference;
use crate::any::FruityAny;
use crate::resource::resource_reference::ResourceReference;
use crate::resource::Resource;
use crate::settings::Settings;
use crate::FruityError;
use crate::FruityResult;
use crate::RwLock;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// A a function that is used to load a resource
pub type ResourceLoader = fn(&str, Settings, ResourceContainer) -> FruityResult<()>;

pub(crate) struct InnerResourceContainer {
    resources: HashMap<String, AnyResourceReference>,
    identifier_by_type: HashMap<TypeId, String>,
    resource_loaders: HashMap<String, ResourceLoader>,
}

/// The resource manager
#[derive(FruityAny, Clone)]
pub struct ResourceContainer {
    pub(crate) inner: Arc<RwLock<InnerResourceContainer>>,
}

impl ResourceContainer {
    /// Returns a ResourceContainer
    pub fn new() -> ResourceContainer {
        ResourceContainer {
            inner: Arc::new(RwLock::new(InnerResourceContainer {
                resources: HashMap::new(),
                identifier_by_type: HashMap::new(),
                resource_loaders: HashMap::new(),
            })),
        }
    }

    /// Get a required resource by it's identifier
    /// Panic if the resource is not known
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    /// # Generic Arguments
    /// * `T` - The resource type
    ///
    pub fn require<T: Resource + ?Sized>(&self) -> ResourceReference<T> {
        let inner = self.inner.read();

        match inner.identifier_by_type.get(&TypeId::of::<T>()) {
            Some(resource_name) => match inner.resources.get(resource_name) {
                Some(resource) => match resource.downcast::<T>() {
                    Some(resource) => resource,
                    None => {
                        panic!("Failed to get a required resource {}", &resource_name)
                    }
                },
                None => {
                    panic!("Failed to get a required resource {}", &resource_name)
                }
            },
            None => {
                panic!("Failed to get a required resource")
            }
        }
    }

    /// Get a resource by it's identifier
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    /// # Generic Arguments
    /// * `T` - The resource type
    ///
    pub fn get<T: Resource + ?Sized>(&self, identifier: &str) -> Option<ResourceReference<T>> {
        let inner = self.inner.read();

        match inner
            .resources
            .get(identifier)
            .map(|resource| resource.clone())
        {
            Some(resource) => resource.downcast::<T>(),
            None => None,
        }
    }

    /// Get a resource by it's identifier without casting it
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn get_untyped(&self, identifier: &str) -> Option<AnyResourceReference> {
        let inner = self.inner.read();

        inner
            .resources
            .get(identifier)
            .map(|resource| resource.clone())
    }

    /// Check if a resource identifier has already been registered
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn contains(&self, identifier: &str) -> bool {
        let inner = self.inner.read();
        inner.resources.contains_key(identifier)
    }

    /// Add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource` - The resource object
    ///
    pub fn add<T: Resource + ?Sized>(&self, identifier: &str, resource: Box<T>) {
        let mut inner = self.inner.write();

        let shared = AnyResourceReference::from_native(identifier, resource);
        inner
            .resources
            .insert(identifier.to_string(), shared.clone());
        inner
            .identifier_by_type
            .insert(TypeId::of::<T>(), identifier.to_string());
    }

    /// Remove a resource of the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    ///
    pub fn remove(&self, identifier: &str) -> FruityResult<()> {
        let mut inner = self.inner.write();

        if inner.resources.contains_key(identifier) {
            inner.resources.remove(identifier);

            Ok(())
        } else {
            Err(FruityError::GenericFailure(format!(
                "Resource {} doesn't exists",
                identifier
            )))
        }
    }

    /// Add a resource loader that will be used to load resources
    ///
    /// # Arguments
    /// * `resource_type` - The resource loader type
    /// * `loader` - The resource loader
    ///
    pub fn add_resource_loader(&self, resource_type: &str, loader: ResourceLoader) {
        let mut inner = self.inner.write();
        inner
            .resource_loaders
            .insert(resource_type.to_string(), loader);
    }

    /// Load and add a resource into the collection
    ///
    /// # Arguments
    /// * `identifier` - The resource identifier
    /// * `resource_type` - The resource type
    ///
    pub fn load_resource(
        &self,
        identifier: &str,
        resource_type: &str,
        settings: Settings,
    ) -> FruityResult<()> {
        let resource_loader = {
            let inner_reader = self.inner.read();

            if let Some(resource_loader) = inner_reader.resource_loaders.get(resource_type) {
                Ok(resource_loader.clone())
            } else {
                Err(FruityError::GenericFailure(format!(
                    "Resource type {} is not registered",
                    resource_type
                )))
            }?
        };

        resource_loader(identifier, settings, self.clone())
    }

    /// Load many resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_resources_settings(&self, settings: Settings) -> FruityResult<()> {
        if let Settings::Object(settings) = settings {
            if let Some(Settings::Array(resources_settings)) = settings.get("resources") {
                resources_settings.into_iter().try_for_each(|settings| {
                    Self::load_single_resource_settings(self, settings.clone())
                })?;
            }
        }

        Ok(())
    }

    /// Load resources for settings
    ///
    /// # Arguments
    /// * `settings` - The settings of resources
    ///
    pub fn load_single_resource_settings(&self, settings: Settings) -> FruityResult<()> {
        // Parse settings
        let fields = if let Settings::Object(fields) = settings {
            fields
        } else {
            return Err(FruityError::GenericFailure(
                "Wrong resource settings, an object is required".to_string(),
            ));
        };

        // Get the resource name
        let name = {
            if let Some(Settings::String(name)) = fields.get("name") {
                name.clone()
            } else {
                return Err(FruityError::GenericFailure(
                    "Wrong resource settings, field name is required".to_string(),
                ));
            }
        };

        // Get the resource type
        let resource_type = {
            if let Some(Settings::String(resource_type)) = fields.get("type") {
                resource_type.clone()
            } else {
                return Err(FruityError::GenericFailure(
                    "Wrong resource settings, field type is required".to_string(),
                ));
            }
        };

        // Load the resource
        Self::load_resource(
            self,
            &name,
            &resource_type,
            Settings::Object(fields.clone()),
        )
    }
}

impl Debug for ResourceContainer {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
