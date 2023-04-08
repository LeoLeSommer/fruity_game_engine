use super::{Component, ComponentTypeId};
use fruity_game_engine::{
    any::FruityAny, export_impl, export_struct, resource::resource_container::ResourceContainer,
    FruityResult,
};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

/// A service to store components extensions
/// When a component is created, if an extension is registered, an other component with a given
/// type is created, this can be use if ou want to extend already existing components with other
/// attributes. This is for example used into the physic engine implementations.
///
/// Warning: The same extension type cannot be shared across multiple based component types
#[derive(FruityAny)]
#[export_struct]
pub struct ExtensionComponentService {
    extension_constructors:
        HashMap<ComponentTypeId, Vec<Box<dyn Fn() -> Box<dyn Component> + Send + Sync>>>,
}

#[export_impl]
impl ExtensionComponentService {
    /// Returns an ExtensionComponentService
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self {
            extension_constructors: HashMap::new(),
        }
    }

    /// Register a component extension
    pub fn register<T: Component, E: Component + Default>(&mut self) {
        let constructor = Box::new(|| Box::new(E::default()) as Box<dyn Component>);
        match self
            .extension_constructors
            .get_mut(&ComponentTypeId::of::<T>())
        {
            Some(constructors) => {
                constructors.push(constructor);
            }
            None => {
                self.extension_constructors
                    .insert(ComponentTypeId::of::<T>(), vec![constructor]);
            }
        }
    }

    /// Create extensions from a component
    pub fn instantiate_component_extension(
        &self,
        component_type_id: &ComponentTypeId,
    ) -> FruityResult<Vec<Box<dyn Component>>> {
        Ok(match self.extension_constructors.get(component_type_id) {
            Some(constructors) => constructors
                .iter()
                .map(|constructor| constructor())
                .collect::<Vec<_>>(),
            None => {
                vec![]
            }
        })
    }
}

impl Debug for ExtensionComponentService {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
