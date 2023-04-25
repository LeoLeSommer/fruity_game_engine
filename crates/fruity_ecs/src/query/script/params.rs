use crate::component::AnyComponentReference;
use crate::component::ComponentStorage;
use crate::component::Name;
use crate::entity::Archetype;
use crate::entity::ArchetypeComponentTypes;
use crate::entity::EntityId;
use crate::entity::EntityLocation;
use crate::entity::EntityReference;
use crate::entity::EntityStorage;
use crate::entity::InnerShareableEntityReference;
use crate::query::EntityIterator;
use crate::query::InfiniteEntityIterator;
use crate::query::QueryParam;
use crate::query::SingleEntityIterator;
use crate::query::With;
use crate::query::WithEnabled;
use crate::query::WithId;
use crate::query::WithIdIterator;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::script_value::ScriptObjectType;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::script_value::TryIntoScriptValue;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::sync::Arc;
use fruity_game_engine::sync::RwLock;
use fruity_game_engine::sync::RwLockReadGuard;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

struct ScriptValueIterator<E, I: Iterator<Item = E> + EntityIterator> {
    iterator: I,
    _marker: PhantomData<E>,
}

impl<E, I: Iterator<Item = E> + EntityIterator> ScriptValueIterator<E, I> {
    pub fn new(iterator: I) -> Self {
        Self {
            iterator,
            _marker: PhantomData,
        }
    }
}

impl<E: TryIntoScriptValue, I: Iterator<Item = E> + EntityIterator> Iterator
    for ScriptValueIterator<E, I>
{
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|item| item.into_script_value().unwrap())
    }
}

impl<E: TryIntoScriptValue, I: Iterator<Item = E> + EntityIterator> EntityIterator
    for ScriptValueIterator<E, I>
{
    fn current(&mut self) -> Self::Item {
        self.iterator.current().into_script_value().unwrap()
    }

    fn has_reach_entity_end(&self) -> bool {
        self.iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.iterator.reset_current_entity()
    }
}

use super::ScriptQueryParam;

/// An iterator over entity references
pub struct WithEntityReferenceIterator<'a> {
    id_iterator: WithIdIterator<'a>,
    entity_storage: Arc<RwLock<EntityStorage>>,
    archetype_index: usize,
    current_entity_index: usize,
    on_entity_location_moved: Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
}

impl<'a> Iterator for WithEntityReferenceIterator<'a> {
    type Item = EntityReference;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity_id) = self.id_iterator.next() {
            let result = EntityReference::new(
                self.entity_storage.clone(),
                entity_id,
                EntityLocation {
                    archetype_index: self.archetype_index,
                    entity_index: self.current_entity_index,
                },
                self.on_entity_location_moved.clone(),
            );

            self.current_entity_index += 1;

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> EntityIterator for WithEntityReferenceIterator<'a> {
    fn current(&mut self) -> Self::Item {
        let entity_index = self.current_entity_index;
        let entity_id = self.id_iterator.current();

        EntityReference::new(
            self.entity_storage.clone(),
            entity_id,
            EntityLocation {
                archetype_index: self.archetype_index,
                entity_index,
            },
            self.on_entity_location_moved.clone(),
        )
    }

    fn has_reach_entity_end(&self) -> bool {
        true
    }

    fn reset_current_entity(&mut self) {
        self.current_entity_index -= 1;
    }
}

#[derive(FruityAny, Clone)]
pub(crate) struct ScriptWithEntityReference {
    pub(crate) entity_storage: Arc<RwLock<EntityStorage>>,
    pub(crate) on_entity_location_moved:
        Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
}

impl ScriptQueryParam for ScriptWithEntityReference {
    fn filter_archetype(&self, _component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        let begin = archetype.entity_ids.as_ptr() as *mut EntityId;

        Box::new(ScriptValueIterator::new(WithEntityReferenceIterator {
            id_iterator: WithIdIterator {
                current: unsafe { NonNull::new_unchecked(begin) },
                end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
                _marker: Default::default(),
            },
            entity_storage: self.entity_storage.clone(),
            archetype_index: archetype.index,
            current_entity_index: 0,
            on_entity_location_moved: self.on_entity_location_moved.clone(),
        }))
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(SingleEntityIterator::new(
            entity_reference.clone(),
        )))
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }
}

/// The entity id
#[derive(FruityAny, Clone)]
pub struct ScriptWithId;

impl ScriptQueryParam for ScriptWithId {
    fn filter_archetype(&self, component_types: &ArchetypeComponentTypes) -> bool {
        WithId::filter_archetype(component_types)
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(WithId::iter(archetype)))
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(WithId::from_entity_reference(
            entity_reference,
        )))
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }
}

pub(crate) struct ScriptWithNameIterator<'a, T: Iterator<Item = &'a Name> + EntityIterator> {
    pub(crate) with_iterator: T,
}

impl<'a, T: Iterator<Item = &'a Name> + EntityIterator> Iterator for ScriptWithNameIterator<'a, T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_iterator.next().map(|name| name.0.clone())
    }
}

impl<'a, T: Iterator<Item = &'a Name> + EntityIterator> EntityIterator
    for ScriptWithNameIterator<'a, T>
{
    fn current(&mut self) -> Self::Item {
        self.with_iterator.current().0.clone()
    }

    fn has_reach_entity_end(&self) -> bool {
        self.with_iterator.has_reach_entity_end()
    }

    fn reset_current_entity(&mut self) {
        self.with_iterator.reset_current_entity()
    }
}

/// The entity name
#[derive(FruityAny, Clone)]
pub struct ScriptWithName;

impl ScriptQueryParam for ScriptWithName {
    fn filter_archetype(&self, component_types: &ArchetypeComponentTypes) -> bool {
        WithId::filter_archetype(component_types)
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(ScriptWithNameIterator {
            with_iterator: With::iter(archetype),
        }))
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(ScriptWithNameIterator {
            with_iterator: With::from_entity_reference(entity_reference),
        }))
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }
}

/// The entity enabled
#[derive(FruityAny, Clone)]
pub(crate) struct ScriptWithEnabled;

impl ScriptQueryParam for ScriptWithEnabled {
    fn filter_archetype(&self, component_types: &ArchetypeComponentTypes) -> bool {
        WithId::filter_archetype(component_types)
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(WithEnabled::iter(archetype)))
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(
            WithEnabled::from_entity_reference(entity_reference),
        ))
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }
}

/// An iterator over entity components with a given type
pub struct ScriptWithIterator<'a> {
    _component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
    current_entity_length: NonNull<usize>,
    end_entity_length: NonNull<usize>,
    current_component_index: usize,
    entity_reference_iterator: WithEntityReferenceIterator<'a>,
    script_object_type: ScriptObjectType,
}

impl<'a> ScriptWithIterator<'a> {
    fn new(
        component_storage_lock: RwLockReadGuard<'a, Box<dyn ComponentStorage>>,
        entity_reference_iterator: WithEntityReferenceIterator<'a>,
        script_object_type: ScriptObjectType,
    ) -> Self {
        let begin_entity_length = component_storage_lock.as_slice_lengths_ptr();
        let entity_count = component_storage_lock.slice_count();

        ScriptWithIterator {
            _component_storage_lock: component_storage_lock,
            current_entity_length: unsafe { NonNull::new_unchecked(begin_entity_length) },
            end_entity_length: unsafe {
                NonNull::new_unchecked(begin_entity_length.add(entity_count))
            },
            current_component_index: 0,
            entity_reference_iterator,
            script_object_type,
        }
    }
}

impl<'a> Iterator for ScriptWithIterator<'a> {
    type Item = AnyComponentReference;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entity_length < self.end_entity_length {
            let result = self.current();
            self.current_entity_length =
                unsafe { NonNull::new_unchecked(self.current_entity_length.as_ptr().add(1)) };

            if self.current_component_index == *unsafe { self.current_entity_length.as_ref() } {
                self.entity_reference_iterator.next();
                self.current_entity_length =
                    unsafe { NonNull::new_unchecked(self.current_entity_length.as_ptr().add(1)) };
                self.current_component_index = 0;
            } else {
                self.current_component_index += 1;
            }

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> EntityIterator for ScriptWithIterator<'a> {
    fn current(&mut self) -> Self::Item {
        AnyComponentReference::new(
            self.entity_reference_iterator.current(),
            self.script_object_type.clone(),
            self.current_component_index,
        )
    }

    fn has_reach_entity_end(&self) -> bool {
        self.current_component_index + 1 == *unsafe { self.current_entity_length.as_ref() }
    }

    fn reset_current_entity(&mut self) {
        self.current_component_index = 0;
    }
}

/// An iterator over entity components with a given type
pub struct ScriptFromEntityWithIterator {
    entity_reference: EntityReference,
    current_component_index: usize,
    end_component_index: usize,
    script_object_type: ScriptObjectType,
}

impl ScriptFromEntityWithIterator {
    fn new(
        entity_reference: EntityReference,
        component_count: usize,
        script_object_type: ScriptObjectType,
    ) -> Self {
        ScriptFromEntityWithIterator {
            entity_reference,
            current_component_index: 0,
            end_component_index: component_count,
            script_object_type,
        }
    }
}

impl Iterator for ScriptFromEntityWithIterator {
    type Item = AnyComponentReference;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_component_index < self.end_component_index {
            let result = self.current();
            self.current_component_index += 1;

            Some(result)
        } else {
            None
        }
    }
}

impl EntityIterator for ScriptFromEntityWithIterator {
    fn current(&mut self) -> Self::Item {
        AnyComponentReference::new(
            self.entity_reference.clone(),
            self.script_object_type.clone(),
            self.current_component_index,
        )
    }

    fn has_reach_entity_end(&self) -> bool {
        self.current_component_index + 1 == self.end_component_index
    }

    fn reset_current_entity(&mut self) {
        self.current_component_index = 0;
    }
}

#[derive(FruityAny, Clone)]
pub(crate) struct ScriptWith {
    pub(crate) entity_storage: Arc<RwLock<EntityStorage>>,
    pub(crate) on_entity_location_moved:
        Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
    pub(crate) script_object_type: ScriptObjectType,
}

impl ScriptQueryParam for ScriptWith {
    fn filter_archetype(&self, component_types: &ArchetypeComponentTypes) -> bool {
        component_types.contains(&self.script_object_type)
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        let begin = archetype.entity_ids.as_ptr() as *mut EntityId;
        let entity_reference_iterator = WithEntityReferenceIterator {
            id_iterator: WithIdIterator {
                current: unsafe { NonNull::new_unchecked(begin) },
                end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
                _marker: Default::default(),
            },
            entity_storage: self.entity_storage.clone(),
            archetype_index: archetype.index,
            current_entity_index: 0,
            on_entity_location_moved: self.on_entity_location_moved.clone(),
        };

        Box::new(ScriptValueIterator::new(ScriptWithIterator::new(
            archetype.component_storages[&self.script_object_type].read(),
            entity_reference_iterator,
            self.script_object_type.clone(),
        )))
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype_index] as *const Archetype,
                )
                .unwrap()
            };

            let component_storage_lock =
                archetype.component_storages[&self.script_object_type].read();

            Box::new(ScriptValueIterator::new(ScriptFromEntityWithIterator::new(
                entity_reference.clone(),
                component_storage_lock.slice_len(location.entity_index),
                self.script_object_type.clone(),
            )))
        } else {
            unreachable!()
        }
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }
}

#[derive(FruityAny, Clone)]
pub(crate) struct ScriptWithOptional {
    pub(crate) entity_storage: Arc<RwLock<EntityStorage>>,
    pub(crate) on_entity_location_moved:
        Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
    pub(crate) script_object_type: ScriptObjectType,
}

impl ScriptQueryParam for ScriptWithOptional {
    fn filter_archetype(&self, _component_types: &ArchetypeComponentTypes) -> bool {
        true
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        if let Some(component_storage) = archetype.component_storages.get(&self.script_object_type)
        {
            let begin = archetype.entity_ids.as_ptr() as *mut EntityId;
            let entity_reference_iterator = WithEntityReferenceIterator {
                id_iterator: WithIdIterator {
                    current: unsafe { NonNull::new_unchecked(begin) },
                    end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
                    _marker: Default::default(),
                },
                entity_storage: self.entity_storage.clone(),
                archetype_index: archetype.index,
                current_entity_index: 0,
                on_entity_location_moved: self.on_entity_location_moved.clone(),
            };

            Box::new(ScriptValueIterator::new(ScriptWithIterator::new(
                component_storage.read(),
                entity_reference_iterator,
                self.script_object_type.clone(),
            )))
        } else {
            Box::new(ScriptValueIterator::new(
                InfiniteEntityIterator::<Option<()>>::new(None),
            ))
        }
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        let inner_entity_reference = entity_reference.inner.read();
        if let Some(InnerShareableEntityReference {
            entity_storage,
            location,
            ..
        }) = inner_entity_reference.deref()
        {
            // TODO: Find a way to remove it
            let archetype = unsafe {
                <*const Archetype>::as_ref(
                    &entity_storage.read().archetypes[location.archetype_index] as *const Archetype,
                )
                .unwrap()
            };

            if let Some(component_storage) =
                archetype.component_storages.get(&self.script_object_type)
            {
                let component_storage_lock = component_storage.read();
                Box::new(ScriptValueIterator::new(ScriptFromEntityWithIterator::new(
                    entity_reference.clone(),
                    component_storage_lock.slice_len(location.entity_index),
                    self.script_object_type.clone(),
                )))
            } else {
                Box::new(ScriptValueIterator::new(
                    SingleEntityIterator::<Option<()>>::new(None),
                ))
            }
        } else {
            unreachable!()
        }
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }
}

#[derive(FruityAny, Clone)]
pub(crate) struct ScriptWithout {
    pub(crate) script_object_type: ScriptObjectType,
}

impl ScriptQueryParam for ScriptWithout {
    fn filter_archetype(&self, component_types: &ArchetypeComponentTypes) -> bool {
        !component_types.contains(&self.script_object_type)
    }

    fn iter<'a>(
        &self,
        _archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(SingleEntityIterator::new(
            Option::<()>::None,
        )))
    }

    fn from_entity_reference<'a>(
        &self,
        _entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptValueIterator::new(SingleEntityIterator::new(
            Option::<()>::None,
        )))
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }
}

/// An iterator over entity enabled state
pub struct ScriptTupleIterator<'a> {
    iterators: Vec<Box<dyn EntityIterator<Item = ScriptValue> + 'a>>,
}

impl<'a> Iterator for ScriptTupleIterator<'a> {
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        // If we arrive at the end of the entity cross product, we just change the entity
        // We do it first cause it is the most common case
        if self
            .iterators
            .iter()
            .all(|iterator| iterator.has_reach_entity_end())
        {
            return Some(ScriptValue::Array(
                self.iterators
                    .iter_mut()
                    .map(|iterator| iterator.next())
                    .try_collect()?,
            ));
        }

        // Find next sub-iterator that should be next
        if let Some((index, sub_iterator)) = self
            .iterators
            .iter_mut()
            .enumerate()
            .find(|(_index, iterator)| iterator.has_reach_entity_end())
        {
            sub_iterator.next();

            // Reinitialize the left iterators
            self.iterators.iter_mut().take(index).for_each(|iterator| {
                iterator.reset_current_entity();
            });

            // Returns the current result
            Some(ScriptValue::Array(
                self.iterators
                    .iter_mut()
                    .map(|iterator| iterator.current())
                    .collect(),
            ))
        } else {
            unreachable!()
        }
    }
}

impl<'a> EntityIterator for ScriptTupleIterator<'a> {
    fn current(&mut self) -> Self::Item {
        ScriptValue::Array(
            self.iterators
                .iter_mut()
                .map(|iterator| iterator.current())
                .collect(),
        )
    }

    fn has_reach_entity_end(&self) -> bool {
        self.iterators
            .iter()
            .all(|iterator| iterator.has_reach_entity_end())
    }

    fn reset_current_entity(&mut self) {
        self.iterators
            .iter_mut()
            .for_each(|iterator| iterator.reset_current_entity());
    }
}

#[derive(FruityAny)]
pub(crate) struct ScriptTuple {
    pub(crate) params: Vec<Box<dyn ScriptQueryParam>>,
}

impl ScriptQueryParam for ScriptTuple {
    fn filter_archetype(&self, component_types: &ArchetypeComponentTypes) -> bool {
        self.params
            .iter()
            .all(|param| param.filter_archetype(component_types))
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptTupleIterator {
            iterators: self
                .params
                .iter()
                .map(|param| param.iter(archetype))
                .collect(),
        })
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn EntityIterator<Item = ScriptValue> + 'a> {
        Box::new(ScriptTupleIterator {
            iterators: self
                .params
                .iter()
                .map(|param| param.from_entity_reference(entity_reference))
                .collect(),
        })
    }

    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(Self {
            params: self.params.iter().map(|param| param.duplicate()).collect(),
        })
    }
}
