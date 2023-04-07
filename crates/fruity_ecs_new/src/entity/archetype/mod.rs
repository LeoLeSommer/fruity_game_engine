use super::EntityId;
use crate::component::{Component, ComponentStorage, ComponentTypeId, ExtensionComponentService};
use fruity_game_engine::{FruityResult, RwLock, RwLockReadGuard};
use std::{
    collections::{BTreeMap, HashMap},
    ops::DerefMut,
};

/// An archetype id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeComponentTypes(Vec<ComponentTypeId>);

impl ArchetypeComponentTypes {
    pub fn from_boxed_components(components: &[Box<dyn Component>]) -> FruityResult<Self> {
        let mut component_types = components
            .iter()
            .map(|component| component.get_component_type_id())
            .try_collect::<Vec<_>>()?;

        component_types.sort();

        Ok(Self(component_types))
    }

    /// Returns true if the given component type id is in the archetype
    pub fn contains(&self, component_type_id: ComponentTypeId) -> bool {
        self.0.contains(&component_type_id)
    }
}

/// An archetype is a collection of components that are stored contiguously in memory
#[derive(Debug)]
pub struct Archetype {
    /// The component types
    component_types: ArchetypeComponentTypes,

    /// The entity ids
    pub(crate) entity_ids: Vec<EntityId>,

    /// The component storages
    pub(crate) component_storages: BTreeMap<ComponentTypeId, RwLock<Box<dyn ComponentStorage>>>,
}

impl Archetype {
    /// Create a new archetype
    pub fn new(
        entity_id: EntityId,
        mut components: Vec<Box<dyn Component>>,
        extension_component_service: Option<&ExtensionComponentService>,
        default_components: Option<Vec<Box<dyn Component>>>,
    ) -> FruityResult<Self> {
        let component_types = ArchetypeComponentTypes::from_boxed_components(&components)?;

        // Add extension components
        if let Some(extension_component_service) = extension_component_service {
            for component_type_id in component_types.0.iter() {
                let mut extension_component = extension_component_service
                    .instantiate_component_extension(component_type_id)?;
                components.append(&mut extension_component);
            }
        }

        // Add default components
        if let Some(mut default_components) = default_components {
            components.append(&mut default_components);
        }

        // Create component storages with the first entity component
        let grouped_components = Self::group_components_by_type(components);
        let mut component_storages = BTreeMap::new();
        for (class_name, components) in grouped_components {
            let first_components = components.first().unwrap();
            let mut component_storage = first_components.get_storage();
            component_storage.push_slice(components)?;

            component_storages.insert(class_name, RwLock::new(component_storage));
        }

        Ok(Self {
            component_types,
            entity_ids: vec![entity_id],
            component_storages,
        })
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_component_types(&self) -> &ArchetypeComponentTypes {
        &self.component_types
    }

    /// Returns the number of entities in this archetype
    pub fn len(&self) -> usize {
        self.entity_ids.len()
    }

    /// Iterate over the entities in this archetype
    pub fn iter(&self) -> ArchetypeEntityIterator {
        ArchetypeEntityIterator {
            archetype: self,
            entity_index: 0,
            component_storage_guards: self
                .component_storages
                .iter()
                .map(|(_, storage)| storage.read())
                .collect(),
        }
    }

    /// Clear the archetype
    pub fn clear(&mut self) -> FruityResult<()> {
        // Clear data
        self.entity_ids.clear();
        self.component_storages.clear();

        Ok(())
    }

    pub fn get_entity_components(&self, entity_index: usize) -> Vec<Box<dyn Component>> {
        self.component_storages
            .values()
            .map(|component_storage| {
                let component_storage = component_storage.read();
                component_storage
                    .iter_slice(entity_index)
                    .unwrap()
                    .map(|component| component.duplicate())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<Box<dyn Component>>>()
    }

    /// Add an entity to the archetype
    pub fn add_entity(
        &mut self,
        entity_id: EntityId,
        mut components: Vec<Box<dyn Component>>,
        extension_component_service: Option<&ExtensionComponentService>,
        default_components: Option<Vec<Box<dyn Component>>>,
    ) -> FruityResult<()> {
        // Store the entity id
        self.entity_ids.push(entity_id);

        // Add extension components
        if let Some(extension_component_service) = extension_component_service {
            for component_type_id in self.component_types.0.iter() {
                let mut extension_component = extension_component_service
                    .instantiate_component_extension(component_type_id)?;
                components.append(&mut extension_component);
            }
        }

        // Add default components
        if let Some(mut default_components) = default_components {
            components.append(&mut default_components);
        }

        // Store all the components
        let grouped_components = Self::group_components_by_type(components);
        for (class_name, components) in grouped_components {
            let component_storage = self.component_storages.get_mut(&class_name).unwrap();
            component_storage.write().push_slice(components)?;
        }

        Ok(())
    }

    pub fn remove_entity(&mut self, entity_index: usize) -> FruityResult<Vec<Box<dyn Component>>> {
        // Remove the entity id
        self.entity_ids.remove(entity_index);

        // Remove the components
        let mut components = Vec::new();
        for component_storage in self.component_storages.values_mut() {
            let mut component = component_storage.write().remove_slice(entity_index);
            components.append(&mut component);
        }

        Ok(components)
    }

    /// Merge two archetypes
    pub fn append(&mut self, other: &mut Self) -> FruityResult<()> {
        // Merge entity ids
        self.entity_ids.append(&mut other.entity_ids);

        // Merge component storages
        for (component_type_id, other_component_storage) in other.component_storages.iter_mut() {
            let component_storage = self.component_storages.get(component_type_id).unwrap();
            component_storage
                .write()
                .append(other_component_storage.write().deref_mut().deref_mut())?;
        }

        Ok(())
    }

    fn group_components_by_type(
        components: Vec<Box<dyn Component>>,
    ) -> HashMap<ComponentTypeId, Vec<Box<dyn Component>>> {
        use itertools::Itertools;

        components
            .into_iter()
            .group_by(|component| component.get_component_type_id().unwrap())
            .into_iter()
            .map(|(class_name, component)| (class_name, component.collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>()
    }
}

impl PartialEq for Archetype {
    fn eq(&self, other: &Self) -> bool {
        self.component_types == other.component_types
    }
}

impl Eq for Archetype {}

impl PartialOrd for Archetype {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.component_types.partial_cmp(&other.component_types)
    }
}

impl Ord for Archetype {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.component_types.cmp(&other.component_types)
    }
}

pub struct ArchetypeEntityIterator<'a> {
    entity_index: usize,
    archetype: &'a Archetype,
    component_storage_guards: Vec<RwLockReadGuard<'a, Box<dyn ComponentStorage>>>,
}

impl<'a> Iterator for ArchetypeEntityIterator<'a> {
    type Item = (EntityId, Box<dyn Iterator<Item = &'a dyn Component> + 'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.entity_index >= self.archetype.len() {
            return None;
        }

        // Disabled lifetime checker for self
        let this = unsafe { &*(self as *const Self) };

        let entity_index = self.entity_index;
        let entity_id = self.archetype.entity_ids[self.entity_index];
        let components = Box::new(
            this.component_storage_guards
                .iter()
                .map(move |component_storage_guard| {
                    component_storage_guard.iter_slice(entity_index).unwrap()
                })
                .flatten(),
        ) as Box<dyn Iterator<Item = &'a dyn Component> + 'a>;

        self.entity_index += 1;

        Some((entity_id, components))
    }
}
