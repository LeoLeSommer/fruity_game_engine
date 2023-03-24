use crate::component::AnyComponent;
use crate::entity::archetype::Component;

/// A abstraction of a collection over components
/// If you wish to implements this, you should do it very carefully, it is intentionally
/// designed to be implemented by a Vec cause a lot of unsafe operations are done on
/// this collection considering it's a vec for optimization concerns
///
/// All items should be stored in memory in a single memory block, if the memory is moved, there
/// is a method that should warn about it
///
/// Basically do not implements this trait except you know what you'r doing
pub trait ComponentStorage: Send + Sync {
    /// Add components to the collection
    ///
    /// It is unsafe cause mut reference over ComponentStorage, cause ComponentStorage is widely read everywhere in the ecs code
    ///
    /// # Arguments
    /// * `components` - The components that will be added
    ///
    unsafe fn add_many(&mut self, components: Vec<AnyComponent>);

    /// Get a single component by index
    fn get(&self, index: usize) -> &dyn Component;

    /// Remove components from the collection
    ///
    /// It is unsafe cause mut reference over ComponentStorage, cause ComponentStorage is widely read everywhere in the ecs code
    ///
    /// # Arguments
    /// * `index` - The index of the first component to remove
    /// * `count` - The number of components that will be removed
    ///
    unsafe fn remove_many(&mut self, index: usize, count: usize) -> Vec<Box<dyn Component>>;

    /// Get every item memory size in bytes
    fn item_size(&self) -> usize;

    /// Returns true if the memory is about to be reallocated on the next insert
    ///
    /// # Arguments
    /// * `count` - The number of components that will be inserted
    ///
    fn is_about_to_reallocate_on_next_insert(&self, count: usize) -> bool;
}

/// A collection of entities that share the same component structure
/// Can store only multiple components of the same type
pub struct VecComponentStorage<T: Component>(Vec<T>);

impl<T: Component> VecComponentStorage<T> {
    /// Returns a VecComponentStorage
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T: Component> ComponentStorage for VecComponentStorage<T> {
    unsafe fn add_many(&mut self, components: Vec<AnyComponent>) {
        // Check that all the components have the good type and convert it to an array of typed component
        let mut components = components.into_iter().map(|component| match component.into_box().as_any_box().downcast::<T>() {
            Ok(component) => *component,
            Err(_) => {
                panic!("Try to instantiate a component array from a array of components with wrong type");
            }
        }).collect::<Vec<_>>();

        self.0.append(&mut components);
    }

    fn get(&self, index: usize) -> &dyn Component {
        self.0.get(index).unwrap()
    }

    unsafe fn remove_many(&mut self, index: usize, count: usize) -> Vec<Box<dyn Component>> {
        self.0
            .drain(index..(index + count))
            .into_iter()
            .map(|component| Box::new(component) as Box<dyn Component>)
            .collect()
    }

    fn item_size(&self) -> usize {
        std::mem::size_of::<T>()
    }

    fn is_about_to_reallocate_on_next_insert(&self, count: usize) -> bool {
        self.0.len() + count > self.0.capacity()
    }
}
