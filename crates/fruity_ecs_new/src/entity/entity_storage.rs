use super::{Archetype, ArchetypeComponentTypes, ArchetypeId, EntityId, EntityLocation};
use crate::component::{Component, ExtensionComponentService};
use fruity_game_engine::{signal::Signal, FruityError, FruityResult};
use sorted_vec::SortedVec;
use std::collections::HashMap;

/// The entity storage
pub struct EntityStorage {
    entity_locations: HashMap<EntityId, EntityLocation>,
    archetype_types: HashMap<ArchetypeComponentTypes, ArchetypeId>,
    archetypes: SortedVec<Archetype>,

    /// Signal notified when an entity is created
    pub on_entity_created: Signal<(EntityId, EntityLocation)>,

    /// Signal notified when an entity is deleted
    pub on_entity_deleted: Signal<(EntityId, EntityLocation)>,

    /// Signal notified when an entity is deleted
    pub on_archetype_created: Signal<(ArchetypeComponentTypes, ArchetypeId)>,
}

impl EntityStorage {
    /// Create a new entity storage
    pub fn new() -> Self {
        Self {
            entity_locations: HashMap::new(),
            archetype_types: HashMap::new(),
            archetypes: SortedVec::new(),
            on_entity_created: Signal::new(),
            on_entity_deleted: Signal::new(),
            on_archetype_created: Signal::new(),
        }
    }

    /// Add an entity to the storage
    pub fn create_entity(
        &mut self,
        entity_id: EntityId,
        components: Vec<Box<dyn Component>>,
        extension_component_service: Option<&ExtensionComponentService>,
        default_components: Option<Vec<Box<dyn Component>>>,
    ) -> FruityResult<()> {
        if self.entity_locations.contains_key(&entity_id) {
            return Err(FruityError::GenericFailure(
                format!("Entity with id {:?} already exists", entity_id).into(),
            ));
        }

        // Generate the archetype component types
        let component_types = ArchetypeComponentTypes::from_boxed_components(&components)?;

        // Insert the entity into the archetype, create the archetype if needed
        let entity_location = match self.archetype_types.get(&component_types) {
            Some(archetype_index) => {
                // Safe cause Archetype::component_types never change
                let archetype =
                    unsafe { &mut self.archetypes.get_unchecked_mut_vec()[archetype_index.0] };
                let entity_index = archetype.len();
                archetype.add_entity(
                    entity_id,
                    components,
                    extension_component_service,
                    default_components,
                )?;

                EntityLocation {
                    archetype: *archetype_index,
                    index: entity_index,
                }
            }
            None => {
                let archetype = Archetype::new(
                    entity_id,
                    components,
                    extension_component_service,
                    default_components,
                )?;

                let archetype_index = self.archetypes.push(archetype);
                self.archetype_types
                    .insert(component_types, ArchetypeId(archetype_index));

                EntityLocation {
                    archetype: ArchetypeId(archetype_index),
                    index: 0,
                }
            }
        };

        self.entity_locations
            .insert(entity_id, entity_location.clone());

        // Notify that entity is created
        self.on_entity_created
            .notify((entity_id, entity_location))?;

        Ok(())
    }

    /// Remove an entity from the storage
    pub fn remove_entity(
        &mut self,
        entity_id: EntityId,
    ) -> FruityResult<Option<Vec<Box<dyn Component>>>> {
        let entity_location =
            if let Some(entity_location) = self.entity_locations.remove(&entity_id) {
                entity_location
            } else {
                return Ok(None);
            };

        // Safe cause Archetype::component_types never change
        let archetype =
            unsafe { &mut self.archetypes.get_unchecked_mut_vec()[entity_location.archetype.0] };
        let result = archetype.remove_entity(entity_location.index)?;

        // Notify that entity is deleted
        self.on_entity_deleted
            .notify((entity_id, entity_location))?;

        Ok(Some(result))
    }

    /// Iterate over all entities
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (EntityId, Box<dyn Iterator<Item = &dyn Component> + '_>)> {
        self.archetypes
            .iter()
            .flat_map(|archetype| archetype.iter())
    }

    /// Iterate over all entities
    pub fn iter_ids(&self) -> impl Iterator<Item = EntityId> {
        self.entity_locations
            .keys()
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Clear the storage
    pub fn clear(&mut self) -> FruityResult<()> {
        // Notify that the entities are deleted
        for (entity_id, entity_location) in self.entity_locations.iter() {
            self.on_entity_deleted
                .notify((*entity_id, entity_location.clone()))?;
        }

        self.entity_locations.clear();
        unsafe {
            self.archetypes
                .get_unchecked_mut_vec()
                .iter_mut()
                .try_for_each(|archetype| archetype.clear())
        }?;

        Ok(())
    }

    /// Append the entities of another storage to this one
    pub fn append(&mut self, other: &mut Self) -> FruityResult<()> {
        other.archetypes.drain(..).try_for_each(|mut archetype| {
            let component_types = archetype.get_component_types().clone();

            let (archetype_index, begin_entity_index) =
                match self.archetype_types.get(&component_types) {
                    Some(archetype_index) => {
                        let begin_entity_index = self.archetypes[archetype_index.0].len();

                        // Safe cause Archetype::component_types never change
                        let self_archetype = unsafe {
                            &mut self.archetypes.get_unchecked_mut_vec()[archetype_index.0]
                        };
                        self_archetype.append(&mut archetype)?;

                        (*archetype_index, begin_entity_index)
                    }
                    None => {
                        let archetype_index = self.archetypes.push(archetype);
                        self.archetype_types
                            .insert(component_types.clone(), ArchetypeId(archetype_index));
                        (ArchetypeId(archetype_index), 0)
                    }
                };

            other
                .entity_locations
                .drain()
                .try_for_each(|(entity_id, location)| {
                    let entity_location = EntityLocation {
                        archetype: archetype_index,
                        index: begin_entity_index + location.index,
                    };

                    self.entity_locations
                        .insert(entity_id.clone(), entity_location);

                    Ok(())
                })
        })
    }
}
