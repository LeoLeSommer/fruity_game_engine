use super::{Archetype, ArchetypeComponentTypes, EntityId, EntityLocation};
use crate::{
    component::{Component, ExtensionComponentService},
    query::{EntityStorageQuery, QueryParam},
};
use fruity_game_engine::{signal::Signal, FruityError, FruityResult};
use sorted_vec::SortedVec;
use std::{collections::HashMap, ptr::NonNull};

/// The entity storage
#[derive(Debug)]
pub struct EntityStorage {
    entity_locations: HashMap<EntityId, EntityLocation>,
    archetype_types: HashMap<ArchetypeComponentTypes, usize>,
    pub(crate) archetypes: SortedVec<Archetype>,

    /// Signal notified when an archetype is created
    /// The vec is sorted by archetype component types, so the archetypes after the new one are moved
    pub on_archetype_created: Signal<NonNull<Archetype>>,

    /// Signal notified when all archetypes are moved troughs memory
    /// Is raised when the archetypes vec is reallocated to increase capacity
    /// The parameter is the gap between the old and the new address of the archetypes vec
    pub on_archetypes_reallocated: Signal<isize>,
}

impl EntityStorage {
    /// Create a new entity storage
    pub fn new() -> Self {
        Self {
            entity_locations: HashMap::new(),
            archetype_types: HashMap::new(),
            archetypes: SortedVec::new(),
            on_archetype_created: Signal::new(),
            on_archetypes_reallocated: Signal::new(),
        }
    }

    /// Create a query over entities
    pub fn query<'a, T: QueryParam<'a> + 'static>(&self) -> EntityStorageQuery<T> {
        EntityStorageQuery::<T>::new(self)
    }

    /// Check if an entity id exists
    pub fn has_entity(&self, entity_id: EntityId) -> bool {
        self.entity_locations.contains_key(&entity_id)
    }

    /// Get the entity components, clone them
    pub fn get_entity_components(&self, entity_id: EntityId) -> Option<Vec<Box<dyn Component>>> {
        let entity_location = self.entity_locations.get(&entity_id)?;
        let archetype = &self.archetypes[entity_location.archetype_index];

        Some(archetype.get_entity_components(entity_location.entity_index))
    }

    /// Get entity location
    pub fn get_entity_location(&self, entity_id: EntityId) -> Option<EntityLocation> {
        self.entity_locations.get(&entity_id).cloned()
    }

    /// Add an entity to the storage
    pub fn create_entity(
        &mut self,
        entity_id: EntityId,
        components: Vec<Box<dyn Component>>,
        extension_component_service: Option<&ExtensionComponentService>,
        default_components: Option<Vec<Box<dyn Component>>>,
    ) -> FruityResult<EntityLocation> {
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
                    unsafe { &mut self.archetypes.get_unchecked_mut_vec()[*archetype_index] };
                let entity_index = archetype.len();
                archetype.add_entity(
                    entity_id,
                    components,
                    extension_component_service,
                    default_components,
                )?;

                EntityLocation {
                    archetype_index: *archetype_index,
                    entity_index,
                }
            }
            None => {
                let archetype = Archetype::new(
                    0,
                    entity_id,
                    components,
                    extension_component_service,
                    default_components,
                )?;

                // Check if a reallocation will be occurred on next archetype insert
                let is_archetypes_about_to_reallocate =
                    self.archetypes.len() + 1 > self.archetypes.capacity();
                let archetypes_old_ptr = self.archetypes.as_ptr();

                // Insert the archetype
                let archetype_index = self.archetypes.push(archetype);
                self.archetype_types
                    .insert(component_types, archetype_index);

                // Update the archetypes indexes
                unsafe { self.archetypes.get_unchecked_mut_vec() }
                    .iter_mut()
                    .enumerate()
                    .for_each(|(index, archetype)| {
                        archetype.index = index;
                    });

                // Update the archetype index in entity locations
                self.entity_locations
                    .iter_mut()
                    .for_each(|(_, entity_location)| {
                        if entity_location.archetype_index >= archetype_index {
                            entity_location.archetype_index += 1;
                        }
                    });

                // Notify memory moves
                let archetype = &self.archetypes[archetype_index];
                self.on_archetype_created.send(unsafe {
                    NonNull::new_unchecked(archetype as *const Archetype as *mut Archetype)
                })?;

                if is_archetypes_about_to_reallocate {
                    let archetypes_new_ptr = self.archetypes.as_ptr();
                    let addr_diff =
                        unsafe { archetypes_new_ptr.byte_offset_from(archetypes_old_ptr) };

                    self.on_archetypes_reallocated.send(addr_diff)?;
                }

                EntityLocation {
                    archetype_index,
                    entity_index: 0,
                }
            }
        };

        self.entity_locations
            .insert(entity_id, entity_location.clone());

        Ok(entity_location)
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
        let archetype = unsafe {
            &mut self.archetypes.get_unchecked_mut_vec()[entity_location.archetype_index]
        };
        let result = archetype.remove_entity(entity_location.entity_index)?;

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
    pub fn append(&mut self, other: &mut Self) -> FruityResult<Vec<(EntityId, EntityLocation)>> {
        let mut entity_locations = Vec::with_capacity(other.entity_locations.len());
        other.archetypes.drain(..).try_for_each(|mut archetype| {
            let component_types = archetype.get_component_types().clone();

            let (archetype_index, begin_entity_index) =
                match self.archetype_types.get(&component_types) {
                    Some(archetype_index) => {
                        let begin_entity_index = self.archetypes[*archetype_index].len();

                        // Safe cause Archetype::component_types never change
                        let self_archetype = unsafe {
                            &mut self.archetypes.get_unchecked_mut_vec()[*archetype_index]
                        };
                        self_archetype.append(&mut archetype)?;

                        (*archetype_index, begin_entity_index)
                    }
                    None => {
                        let archetype_index = self.archetypes.push(archetype);
                        self.archetype_types
                            .insert(component_types.clone(), archetype_index);
                        (archetype_index, 0)
                    }
                };

            let locations = other
                .entity_locations
                .drain()
                .map(|(entity_id, location)| {
                    let entity_location = EntityLocation {
                        archetype_index,
                        entity_index: begin_entity_index + location.entity_index,
                    };

                    self.entity_locations
                        .insert(entity_id.clone(), entity_location.clone());

                    Ok((entity_id, entity_location))
                })
                .try_collect::<Vec<_>>()?;

            entity_locations.extend(locations);

            Ok(())
        })?;

        Ok(entity_locations)
    }
}
