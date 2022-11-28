use super::{
    resource_container::ResourceContainer, resource_reference::AnyResourceReference,
    script_resource::ScriptResource,
};
use crate::{
    any::FruityAny,
    javascript::JsIntrospectObject,
    script_value::{convert::TryIntoScriptValue, ScriptValue},
    FruityError, FruityResult, FruityStatus,
};
use fruity_game_engine_macro::{export, fruity_export};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

fruity_export! {
    /// The resource manager exposed to scripting language
    #[derive(FruityAny, Clone, Debug)]
    pub struct ScriptResourceContainer {
        resource_container: ResourceContainer,
        script_resources: Rc<RefCell<HashMap<String, ScriptResource>>>,
    }

    impl ScriptResourceContainer {
        /// Returns a ResourceContainer
        pub fn new(resource_container: ResourceContainer) -> Self {
            Self {
                resource_container,
                script_resources: Rc::new(RefCell::new(HashMap::new())),
            }
        }

        /// Get a resource by it's identifier without casting it
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        ///
        #[export]
        pub fn get(&self, identifier: String) -> Option<ScriptOrNativeResource> {
            match self.resource_container.get_untyped(&identifier) {
                Some(value) => Some(ScriptOrNativeResource::Native(value)),
                None => self
                    .script_resources
                    .borrow()
                    .get(&identifier)
                    .map(|resource| ScriptOrNativeResource::Script(resource.clone())),
            }
        }

        /// Check if a resource identifier has already been registered
        /// Use String as identifier, to match the scripting wrapper requirements
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        ///
        #[export]
        pub fn contains(&self, identifier: String) -> bool {
            self.resource_container.contains(&identifier)
                || self.script_resources.borrow().contains_key(&identifier)
        }

        /// Add a resource into the collection with an unknown type
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        /// * `resource` - The resource object
        ///
        #[export]
        pub fn add(&self, identifier: String, resource: JsIntrospectObject) {
            let resource = ScriptResource::from(resource);
            self.script_resources
                .borrow_mut()
                .insert(identifier, resource);
        }

        /// Remove a resource of the collection
        /// Use String as identifier, to match the scripting wrapper requirements
        ///
        /// # Arguments
        /// * `identifier` - The resource identifier
        ///
        #[export]
        pub fn remove(&self, identifier: String) -> FruityResult<()> {
            match self.resource_container.remove(&identifier) {
                Ok(()) => Ok(()),
                Err(_) => {
                    if self.script_resources.borrow().contains_key(&identifier) {
                        self.script_resources.borrow_mut().remove(&identifier);

                        Ok(())
                    } else {
                        Err(FruityError::new(
                            FruityStatus::GenericFailure,
                            format!("Resource {} doesn't exists", identifier),
                        ))
                    }
                }
            }
        }
    }
}

/// Neither a script or a native resource
pub enum ScriptOrNativeResource {
    /// A script resource
    Script(ScriptResource),
    /// A native resource
    Native(AnyResourceReference),
}

impl TryIntoScriptValue for ScriptOrNativeResource {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        match self {
            ScriptOrNativeResource::Script(resource) => resource.into_script_value(),
            ScriptOrNativeResource::Native(resource) => resource.into_script_value(),
        }
    }
}
