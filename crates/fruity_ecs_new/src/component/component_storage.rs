use super::Component;
use crate::storage::SlicedVec;
use fruity_game_engine::{any::FruityAny, FruityError, FruityResult};
use std::fmt::Debug;

/// A storage for components
pub trait ComponentStorage: Debug + FruityAny {
    /// Returns a reference to the component at the given index. in a given entity
    fn get(&self, entity_index: usize, component_index: usize) -> Option<&dyn Component>;

    /// Returns a mutable reference to the component at the given index in a given entity
    fn get_mut(
        &mut self,
        entity_index: usize,
        component_index: usize,
    ) -> Option<&mut dyn Component>;

    /// Returns a reference to the component at the given index. in a given entity
    unsafe fn get_unchecked(&self, entity_index: usize, component_index: usize) -> &dyn Component;

    /// Returns a mutable reference to the component at the given index in a given entity
    unsafe fn get_unchecked_mut(
        &mut self,
        entity_index: usize,
        component_index: usize,
    ) -> &mut dyn Component;

    /// Returns an iterator over the components in a given entity
    fn iter_slice(
        &self,
        entity_index: usize,
    ) -> Option<Box<dyn Iterator<Item = &dyn Component> + '_>>;

    /// Inserts a component in a given entity
    fn push_slice(&mut self, components: Vec<Box<dyn Component>>) -> FruityResult<()>;

    /// Removes the component at the given index in a given entity
    fn remove_slice(&mut self, entity_index: usize) -> Vec<Box<dyn Component>>;

    /// Appends the components of another storage to this storage
    fn append(&mut self, other: &mut dyn ComponentStorage) -> FruityResult<()>;

    /// Returns the number of components in the storage in a given entity
    fn len(&self, entity_index: usize) -> usize;

    /// Clear the storage
    fn clear(&mut self);

    /// Reserves capacity for at least `additional` more components to be inserted in the storage
    fn reserve(&mut self, additional: usize);

    /// Returns the capacity of the storage
    fn capacity(&self) -> usize;
}

/// A component storage that uses a sliced vec
#[derive(FruityAny, Debug, Clone, Default)]
pub struct VecComponentStorage<T: Component> {
    data: SlicedVec<T>,
}

impl<T: Component> VecComponentStorage<T> {
    /// Create a new component storage
    pub fn new() -> Self {
        Self {
            data: SlicedVec::new(),
        }
    }
}

impl<T: Component> ComponentStorage for VecComponentStorage<T> {
    fn get(&self, entity_index: usize, component_index: usize) -> Option<&dyn Component> {
        self.data
            .get_slice(entity_index)?
            .get(component_index)
            .map(|component| component as &dyn Component)
    }

    fn get_mut(
        &mut self,
        entity_index: usize,
        component_index: usize,
    ) -> Option<&mut dyn Component> {
        self.data
            .get_slice_mut(entity_index)?
            .get_mut(component_index)
            .map(|component| component as &mut dyn Component)
    }

    unsafe fn get_unchecked(&self, entity_index: usize, component_index: usize) -> &dyn Component {
        let slice = self.data.get_unchecked_slice(entity_index);
        let component = slice.get_unchecked(component_index);

        component as &dyn Component
    }

    unsafe fn get_unchecked_mut(
        &mut self,
        entity_index: usize,
        component_index: usize,
    ) -> &mut dyn Component {
        let slice = self.data.get_unchecked_mut_slice(entity_index);
        let component = slice.get_unchecked_mut(component_index);

        component as &mut dyn Component
    }

    fn iter_slice(
        &self,
        entity_index: usize,
    ) -> Option<Box<dyn Iterator<Item = &dyn Component> + '_>> {
        self.data.get_slice(entity_index).map(|slice| {
            Box::new(slice.iter().map(|component| component as &dyn Component))
                as Box<dyn Iterator<Item = &dyn Component>>
        })
    }

    fn push_slice(&mut self, components: Vec<Box<dyn Component>>) -> FruityResult<()> {
        let components = components
            .into_iter()
            .map(|component| {
                component
                    .as_any_box()
                    .downcast::<T>()
                    .map(|component| *component)
                    .map_err(|component| {
                        FruityError::GenericFailure(format!(
                            "Failed to downcast {:?} to {}",
                            &component,
                            std::any::type_name::<T>()
                        ))
                    })
            })
            .collect::<FruityResult<Vec<_>>>()?;

        self.data.push_slice(components);

        Ok(())
    }

    fn remove_slice(&mut self, entity_index: usize) -> Vec<Box<dyn Component>> {
        self.data
            .remove_slice(entity_index)
            .into_iter()
            .map(|component| Box::new(component) as Box<dyn Component>)
            .collect()
    }

    fn append(&mut self, other: &mut dyn ComponentStorage) -> FruityResult<()> {
        let other =
            other
                .as_any_mut()
                .downcast_mut::<Self>()
                .ok_or(FruityError::GenericFailure(format!(
                    "Failed to downcast to {}",
                    std::any::type_name::<Self>()
                )))?;

        self.data.append(&mut other.data);

        Ok(())
    }

    fn len(&self, entity_index: usize) -> usize {
        self.data.slice_len(entity_index)
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }
}
