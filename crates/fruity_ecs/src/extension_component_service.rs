use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component::StaticComponent;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::fruity_export;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::FruityResult;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;

fruity_export! {
    /// A service to store components extensions
    /// When a component is created, if an extension is registered, an other component with a given
    /// type is created, this can be use if ou want to extend already existing components with other
    /// attributes. This is for example used into the physic engine implementations.
    ///
    /// Warning: The same extension type cannot be shared across multiple based component types
    #[derive(FruityAny, Resource)]
    pub struct ExtensionComponentService {
        extension_constructors: HashMap<String, Vec<Box<dyn Fn() -> AnyComponent + Send + Sync>>>,
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
                        .insert(T::get_component_name().to_string(), vec![constructor]);
                }
            }
        }

        /// Create extensions from a component
        pub fn get_component_extension(&self, component: &dyn Component) -> FruityResult<Vec<AnyComponent>> {
            Ok(match self.extension_constructors.get(&component.get_class_name()?) {
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
}

impl Debug for ExtensionComponentService {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
