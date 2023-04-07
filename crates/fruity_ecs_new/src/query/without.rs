use super::{InfiniteEntityIterator, QueryParam, SingleEntityIterator};
use crate::{
    component::{Component, ComponentTypeId},
    entity::{Archetype, ArchetypeComponentTypes, EntityReference},
};
use std::marker::PhantomData;

/// Exclude a component from a query
pub struct Without<T: Component + 'static> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + 'static> QueryParam<'a> for Without<T> {
    type Item = ();
    type Iterator = InfiniteEntityIterator<Self::Item>;
    type FromEntityReferenceIterator = SingleEntityIterator<Self::Item>;

    fn filter_archetype(component_types: &ArchetypeComponentTypes) -> bool {
        !component_types.contains(ComponentTypeId::of::<T>())
    }

    fn iter(_archetype: &'a Archetype) -> Self::Iterator {
        InfiniteEntityIterator::default()
    }

    fn from_entity_reference(
        _entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        SingleEntityIterator::default()
    }
}
