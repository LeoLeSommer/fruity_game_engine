use super::resource_reference::AnyResourceReference;
use crate::any::FruityAny;
use crate::export;
use crate::fruity_export;
use crate::javascript::JsIntrospectObject;
use crate::resource::resource_reference::ResourceReference;
use crate::resource::script_resource::ScriptResource;
use crate::resource::Resource;
use crate::settings::Settings;
use crate::FruityError;
use crate::FruityResult;
use crate::FruityStatus;
use crate::RwLock;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

/// A a function that is used to load a resource
pub type ResourceLoader = fn(&str, &mut dyn Read, Settings, ResourceContainer);

pub(crate) struct InnerResourceContainer {
    resources: HashMap<String, AnyResourceReference>,
    identifier_by_type: HashMap<TypeId, String>,
    resource_loaders: HashMap<String, ResourceLoader>,
}

fruity_export! {
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
                    Some(resource) => {
                        match resource.downcast::<T>() {
                            Some(resource) => {
                                resource
                            }
                            None => {
                                panic!("Failed to get a required resource")
                            }
                        }
                    }
                    None => {
                        panic!("Failed to get a required resource")
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
        #[export(name = "get")]
        pub fn get_untyped(&self, identifier: String) -> Option<AnyResourceReference> {
            let inner = self.inner.read();

            inner
                .resources
                .get(&identifier)
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

        /// Check if a resource identifier has already been registered
        /// Use String as identifier, to match the scripting wrapper requirements
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        ///
        #[export(name = "contains")]
        pub fn contains_by_string(&self, identifier: String) -> bool {
            self.contains(&identifier)
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

        /// Add a resource into the collection with an unknown type
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        /// * `resource` - The resource object
        ///
        #[export(name = "add")]
        pub fn add_script_resource(&self, identifier: String, resource: JsIntrospectObject) {
            let mut inner = self.inner.write();

            let shared = AnyResourceReference::from_native(&identifier, Box::new(ScriptResource::from(resource)));
            inner
                .resources
                .insert(identifier.to_string(), shared);
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
                Err(FruityError::new(
                    FruityStatus::GenericFailure,
                    format!("Resource {} doesn't exists", identifier),
                ))
            }
        }

        /// Remove a resource of the collection
        /// Use String as identifier, to match the scripting wrapper requirements
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        ///
        #[export(name = "remove")]
        pub fn remove_by_string(&self, identifier: String) -> FruityResult<()> {
            self.remove(&identifier)
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

        /// Load an any resource file
        ///
        /// # Arguments
        /// * `path` - The path of the file
        /// * `resource_type` - The resource type
        ///
        pub fn load_resource_file(
            &self,
            path: &str,
            resource_type: &str,
        ) -> FruityResult<()> {
            let mut file = File::open(path).map_err(|_| {
                FruityError::new(
                    FruityStatus::GenericFailure,
                    format!("File couldn't be opened: {:?}", path),
                )
            })?;

            Self::load_resource(self, path, resource_type, &mut file, Settings::default())?;

            Ok(())
        }

        /// Load and add a resource into the collection
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        /// * `resource_type` - The resource type
        /// * `read` - The reader, generaly a file reader
        ///
        pub fn load_resource(
            &self,
            identifier: &str,
            resource_type: &str,
            reader: &mut dyn Read,
            settings: Settings,
        ) -> FruityResult<()> {
            let resource_loader = {
                let inner_reader = self.inner.read();

                if let Some(resource_loader) = inner_reader.resource_loaders.get(resource_type) {
                    Ok(resource_loader.clone())
                } else {
                    Err(FruityError::new(
                        FruityStatus::GenericFailure,
                        format!("Resource type {} is not registered", resource_type),
                    ))
                }?
            };

            resource_loader(identifier, reader, settings, self.clone());
            Ok(())
        }

        /// Load many resources for settings
        ///
        /// # Arguments
        /// * `settings` - The settings of resources
        ///
        pub fn load_resources_settings(&self, settings: Vec<Settings>) {
            settings.into_iter().for_each(|settings| {
                Self::load_resource_settings(self, settings);
            })
        }

        /// Load resources for settings
        ///
        /// # Arguments
        /// * `settings` - The settings of resources
        ///
        pub fn load_resource_settings(&self, settings: Settings) -> Option<()> {
            // Parse settings
            let fields = if let Settings::Object(fields) = settings {
                fields
            } else {
                return None;
            };

            // Get the resource name
            let name = {
                if let Settings::String(name) = fields.get("name")? {
                    name.clone()
                } else {
                    return None;
                }
            };

            // Get the resource path
            let path = {
                if let Settings::String(path) = fields.get("path")? {
                    path.clone()
                } else {
                    return None;
                }
            };

            // Deduce informations about the resource from the path
            let resource_type = Path::new(&path).extension()?;
            let resource_type = resource_type.to_str()?;
            let mut resource_file = File::open(&path).ok()?;

            // Load the resource
            Self::load_resource(
                self,
                &name,
                resource_type,
                &mut resource_file,
                Settings::Object(fields.clone()),
            )
            .ok()?;

            Some(())
        }
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
