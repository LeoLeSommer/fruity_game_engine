use crate::component::component_reference::AnyComponentReference;
use crate::component::Component;
use crate::entity::archetype::Archetype;
use crate::entity::entity_query::script::ScriptQueryParam;
use crate::entity::entity_query::{
    BidirectionalIterator, InfiniteBidirectionalIterator, SingleBidirectionalIterator,
};
use crate::entity::entity_reference::EntityReference;
use crate::entity::EntityId;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::RwLock;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// An iterator over entity references
pub struct WithEntityIterator<'a> {
    current_entity_index: usize,
    end_entity_index: usize,
    archetype: &'a Archetype,
}

impl<'a> Iterator for WithEntityIterator<'a> {
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entity_index < self.end_entity_index {
            let result = self.current();
            self.current_entity_index += 1;

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> BidirectionalIterator for WithEntityIterator<'a> {
    fn current(&mut self) -> Self::Item {
        let entity_index = self.current_entity_index;
        self.archetype
            .get_entity_reference(entity_index)
            .into_script_value()
            .unwrap()
    }

    fn go_back(&mut self, count: usize) {
        self.current_entity_index -= count;
    }
}

#[derive(FruityAny, Clone)]
pub struct WithEntity {}

impl ScriptQueryParam for WithEntity {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn items_per_entity(&self, _archetype: &Archetype) -> usize {
        1
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        Box::new(WithEntityIterator {
            current_entity_index: 0,
            end_entity_index: archetype.len(),
            archetype,
        })
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        Box::new(SingleBidirectionalIterator::new(
            entity_reference.clone().into_script_value().unwrap(),
        ))
    }
}

/// An iterator over entity ids
pub struct WithIdIterator<'a> {
    current: NonNull<EntityId>,
    end: NonNull<EntityId>,
    _marker: PhantomData<&'a EntityId>,
}

impl<'a> Iterator for WithIdIterator<'a> {
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> BidirectionalIterator for WithIdIterator<'a> {
    fn current(&mut self) -> Self::Item {
        (*unsafe { self.current.as_ref() })
            .into_script_value()
            .unwrap()
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

#[derive(FruityAny, Clone)]
pub struct WithId {}

impl ScriptQueryParam for WithId {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn items_per_entity(&self, _archetype: &Archetype) -> usize {
        1
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let begin = &archetype.entity_id_array[0] as *const EntityId as *mut EntityId;
        Box::new(WithIdIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        })
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let inner_entity_reference = entity_reference.inner.read();
        let archetype = unsafe { inner_entity_reference.archetype_ptr.as_ref() }.unwrap();

        Box::new(SingleBidirectionalIterator::new(
            archetype.entity_id_array[inner_entity_reference.entity_index]
                .into_script_value()
                .unwrap(),
        ))
    }
}

/// An iterator over entity names
pub struct WithNameIterator<'a> {
    current: NonNull<String>,
    end: NonNull<String>,
    _marker: PhantomData<&'a String>,
}

impl<'a> Iterator for WithNameIterator<'a> {
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> BidirectionalIterator for WithNameIterator<'a> {
    fn current(&mut self) -> Self::Item {
        unsafe { self.current.as_ref() }
            .clone()
            .into_script_value()
            .unwrap()
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

#[derive(FruityAny, Clone)]
pub struct WithName {}

impl ScriptQueryParam for WithName {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn items_per_entity(&self, _archetype: &Archetype) -> usize {
        1
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let begin = &archetype.name_array[0] as *const String as *mut String;
        Box::new(WithNameIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        })
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let inner_entity_reference = entity_reference.inner.read();
        let archetype = unsafe { inner_entity_reference.archetype_ptr.as_ref() }.unwrap();

        Box::new(SingleBidirectionalIterator::new(
            archetype.name_array[inner_entity_reference.entity_index]
                .clone()
                .into_script_value()
                .unwrap(),
        ))
    }
}

/// An iterator over entity enabled state
pub struct WithEnabledIterator<'a> {
    current: NonNull<bool>,
    end: NonNull<bool>,
    _marker: PhantomData<&'a bool>,
}

impl<'a> Iterator for WithEnabledIterator<'a> {
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current();
            self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().add(1)) };

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> BidirectionalIterator for WithEnabledIterator<'a> {
    fn current(&mut self) -> Self::Item {
        (*unsafe { self.current.as_ref() })
            .into_script_value()
            .unwrap()
    }

    fn go_back(&mut self, count: usize) {
        self.current = unsafe { NonNull::new_unchecked(self.current.as_ptr().sub(count)) };
    }
}

#[derive(FruityAny, Clone)]
pub struct WithEnabled {}

impl ScriptQueryParam for WithEnabled {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn items_per_entity(&self, _archetype: &Archetype) -> usize {
        1
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let begin = &archetype.enabled_array[0] as *const bool as *mut bool;
        Box::new(WithEnabledIterator {
            current: unsafe { NonNull::new_unchecked(begin) },
            end: unsafe { NonNull::new_unchecked(begin.add(archetype.len())) },
            _marker: Default::default(),
        })
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let inner_entity_reference = entity_reference.inner.read();
        let archetype = unsafe { inner_entity_reference.archetype_ptr.as_ref() }.unwrap();

        Box::new(SingleBidirectionalIterator::new(
            archetype.enabled_array[inner_entity_reference.entity_index]
                .into_script_value()
                .unwrap(),
        ))
    }
}

/// An iterator over entity components with a given type
pub struct WithIterator<'a> {
    archetype: &'a Archetype,
    current_entity_lock: NonNull<RwLock<()>>,
    current_component: NonNull<dyn Component>,
    component_size: usize,
    component_index: usize,
    items_per_entity: usize,
    end: NonNull<RwLock<()>>,
}

impl<'a> Iterator for WithIterator<'a> {
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_entity_lock < self.end {
            let result = self.current();
            self.current_component = unsafe {
                NonNull::new_unchecked(
                    self.current_component
                        .as_ptr()
                        .byte_add(self.component_size),
                )
            };
            self.component_index += 1;

            if self.component_index == self.items_per_entity {
                self.component_index = 0;
                self.current_entity_lock =
                    unsafe { NonNull::new_unchecked(self.current_entity_lock.as_ptr().add(1)) };
            }

            Some(result)
        } else {
            None
        }
    }
}

impl<'a> BidirectionalIterator for WithIterator<'a> {
    fn current(&mut self) -> Self::Item {
        AnyComponentReference::new(
            &self.archetype.on_entity_lock_address_moved,
            &self.archetype.on_component_address_moved,
            self.current_entity_lock.as_ptr(),
            Some(self.current_component),
        )
        .into_script_value()
        .unwrap()
    }

    fn go_back(&mut self, count: usize) {
        self.current_component = unsafe {
            NonNull::new_unchecked(
                self.current_component
                    .as_ptr()
                    .byte_sub(count * self.component_size),
            )
        };
    }
}

#[derive(FruityAny, Clone)]
pub struct With {
    pub identifier: String,
}

impl ScriptQueryParam for With {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, archetype: &Archetype) -> bool {
        archetype.identifier.contains(&self.identifier)
    }

    fn items_per_entity(&self, archetype: &Archetype) -> usize {
        archetype.component_storages[&self.identifier].components_per_entity
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let component_storage = &archetype.component_storages[&self.identifier].component_storage;
        let begin_entity_lock = &archetype.lock_array[0] as *const RwLock<()> as *mut RwLock<()>;
        let begin_component = unsafe {
            NonNull::new_unchecked(
                component_storage.get(0) as *const dyn Component as *mut dyn Component
            )
        };

        Box::new(WithIterator {
            archetype,
            current_entity_lock: unsafe { NonNull::new_unchecked(begin_entity_lock) },
            current_component: begin_component,
            component_size: component_storage.item_size(),
            component_index: 0,
            items_per_entity: self.items_per_entity(archetype),
            end: unsafe { NonNull::new_unchecked(begin_entity_lock.add(archetype.len())) },
        })
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let inner_entity_reference = entity_reference.inner.read();
        let archetype = unsafe { inner_entity_reference.archetype_ptr.as_ref() }.unwrap();
        let component_storage = &archetype.component_storages[&self.identifier];

        let begin_entity_lock = &archetype.lock_array[inner_entity_reference.entity_index]
            as *const RwLock<()> as *mut RwLock<()>;
        let begin_component = unsafe {
            NonNull::new_unchecked(
                component_storage.component_storage.get(
                    inner_entity_reference.entity_index * component_storage.components_per_entity,
                ) as *const dyn Component as *mut dyn Component,
            )
        };

        Box::new(WithIterator {
            archetype,
            current_entity_lock: unsafe { NonNull::new_unchecked(begin_entity_lock) },
            current_component: begin_component,
            component_size: component_storage.component_storage.item_size(),
            component_index: 0,
            items_per_entity: self.items_per_entity(archetype),
            end: unsafe {
                NonNull::new_unchecked(
                    begin_entity_lock.add(component_storage.components_per_entity),
                )
            },
        })
    }
}

#[derive(FruityAny, Clone)]
pub struct WithOptional {
    pub identifier: String,
}

impl ScriptQueryParam for WithOptional {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn items_per_entity(&self, archetype: &Archetype) -> usize {
        archetype
            .component_storages
            .get(&self.identifier)
            .map(|storage| storage.components_per_entity)
            .unwrap_or(1)
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        if let Some(component_storage) = archetype.component_storages.get(&self.identifier) {
            let begin_entity_lock =
                &archetype.lock_array[0] as *const RwLock<()> as *mut RwLock<()>;
            let begin_component = unsafe {
                NonNull::new_unchecked(component_storage.component_storage.get(0)
                    as *const dyn Component
                    as *mut dyn Component)
            };

            Box::new(WithIterator {
                archetype,
                current_entity_lock: unsafe { NonNull::new_unchecked(begin_entity_lock) },
                current_component: begin_component,
                component_size: component_storage.component_storage.item_size(),
                component_index: 0,
                items_per_entity: self.items_per_entity(archetype),
                end: unsafe { NonNull::new_unchecked(begin_entity_lock.add(archetype.len())) },
            })
        } else {
            Box::new(InfiniteBidirectionalIterator::new(ScriptValue::Null))
        }
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        let inner_entity_reference = entity_reference.inner.read();
        let archetype = unsafe { inner_entity_reference.archetype_ptr.as_ref() }.unwrap();
        if let Some(component_storage) = archetype.component_storages.get(&self.identifier) {
            let begin_entity_lock = &archetype.lock_array[inner_entity_reference.entity_index]
                as *const RwLock<()> as *mut RwLock<()>;
            let begin_component = unsafe {
                NonNull::new_unchecked(component_storage.component_storage.get(
                    inner_entity_reference.entity_index * component_storage.components_per_entity,
                ) as *const dyn Component
                    as *mut dyn Component)
            };

            Box::new(WithIterator {
                archetype,
                current_entity_lock: unsafe { NonNull::new_unchecked(begin_entity_lock) },
                current_component: begin_component,
                component_size: component_storage.component_storage.item_size(),
                component_index: 0,
                items_per_entity: self.items_per_entity(archetype),
                end: unsafe {
                    NonNull::new_unchecked(
                        begin_entity_lock.add(component_storage.components_per_entity),
                    )
                },
            })
        } else {
            Box::new(SingleBidirectionalIterator::new(ScriptValue::Null))
        }
    }
}

#[derive(FruityAny, Clone)]
pub struct Without {
    pub identifier: String,
}

impl ScriptQueryParam for Without {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, archetype: &Archetype) -> bool {
        !archetype.identifier.contains(&self.identifier)
    }

    fn items_per_entity(&self, _archetype: &Archetype) -> usize {
        1
    }

    fn iter<'a>(
        &self,
        _archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        Box::new(InfiniteBidirectionalIterator::new(ScriptValue::Null))
    }

    fn from_entity_reference<'a>(
        &self,
        _entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        Box::new(SingleBidirectionalIterator::new(ScriptValue::Null))
    }
}

struct TupleIteratorElem<'a> {
    iterator: Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a>,
    local_id: usize,
    items_per_entity: usize,
}

/// An iterator over entity enabled state
pub struct TupleIterator<'a> {
    iterators: Vec<TupleIteratorElem<'a>>,
}

impl<'a> Iterator for TupleIterator<'a> {
    type Item = ScriptValue;

    fn next(&mut self) -> Option<Self::Item> {
        // If we arrive at the end of the entity cross product, we just change the entity
        // We do it first cause it is the most common case
        if self
            .iterators
            .iter()
            .all(|iterator| iterator.local_id + 1 == iterator.items_per_entity)
        {
            self.iterators
                .iter_mut()
                .for_each(|iterator| iterator.local_id = 0);

            return Some(ScriptValue::Array(
                self.iterators
                    .iter_mut()
                    .map(|iterator| iterator.iterator.next())
                    .try_collect()?,
            ));
        }

        // Find next sub-iterator that should be next
        if let Some((index, sub_iterator)) = self
            .iterators
            .iter_mut()
            .enumerate()
            .find(|(_index, iterator)| iterator.local_id + 1 < iterator.items_per_entity)
        {
            sub_iterator.iterator.next();
            sub_iterator.local_id += 1;

            // Reinitialize the left iterators
            self.iterators.iter_mut().take(index).for_each(|iterator| {
                iterator.iterator.go_back(iterator.items_per_entity - 1);
                iterator.local_id += 0;
            });

            // Returns the current result
            Some(ScriptValue::Array(
                self.iterators
                    .iter_mut()
                    .map(|iterator| iterator.iterator.current())
                    .collect(),
            ))
        } else {
            unreachable!()
        }
    }
}

impl<'a> BidirectionalIterator for TupleIterator<'a> {
    fn current(&mut self) -> Self::Item {
        ScriptValue::Array(
            self.iterators
                .iter_mut()
                .map(|iterator| iterator.iterator.current())
                .collect(),
        )
    }

    fn go_back(&mut self, count: usize) {
        self.iterators
            .iter_mut()
            .for_each(|iterator| iterator.iterator.go_back(count))
    }
}

#[derive(FruityAny)]
pub struct Tuple {
    pub params: Vec<Box<dyn ScriptQueryParam>>,
}

impl ScriptQueryParam for Tuple {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, archetype: &Archetype) -> bool {
        self.params
            .iter()
            .all(|param| param.filter_archetype(archetype))
    }

    fn items_per_entity(&self, archetype: &Archetype) -> usize {
        self.params
            .iter()
            .map(|param| param.items_per_entity(archetype))
            .fold(0, |acc, x| acc * x)
    }

    fn iter<'a>(
        &self,
        archetype: &'a Archetype,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        Box::new(TupleIterator {
            iterators: self
                .params
                .iter()
                .map(|param| TupleIteratorElem {
                    iterator: param.iter(archetype),
                    local_id: 0,
                    items_per_entity: param.items_per_entity(archetype),
                })
                .collect(),
        })
    }

    fn from_entity_reference<'a>(
        &self,
        entity_reference: &'a EntityReference,
    ) -> Box<dyn BidirectionalIterator<Item = ScriptValue> + 'a> {
        Box::new(TupleIterator {
            iterators: self
                .params
                .iter()
                .map(|param| {
                    let archetype = {
                        let entity_reference_inner = entity_reference.inner.read();
                        unsafe { entity_reference_inner.archetype_ptr.as_ref().unwrap() }
                    };

                    TupleIteratorElem {
                        iterator: param.from_entity_reference(entity_reference),
                        local_id: 0,
                        items_per_entity: param.items_per_entity(archetype),
                    }
                })
                .collect(),
        })
    }
}

impl Clone for Tuple {
    fn clone(&self) -> Self {
        Self {
            params: self.params.iter().map(|param| param.duplicate()).collect(),
        }
    }
}
