use super::InfiniteBidirectionalIterator;
use super::SingleBidirectionalIterator;
use crate::component::Component;
use crate::component::StaticComponent;
use crate::entity::archetype::Archetype;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_reference::EntityReference;
use std::marker::PhantomData;

/// Exclude a component from a query
pub struct Without<T: Component + StaticComponent + 'static> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for Without<T> {
    type Item = ();
    type Iterator = InfiniteBidirectionalIterator<Self::Item>;
    type FromEntityReferenceIterator = SingleBidirectionalIterator<Self::Item>;

    fn filter_archetype(archetype: &Archetype) -> bool {
        !archetype
            .identifier
            .contains(&T::get_component_name().to_string())
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        false
    }

    fn items_per_entity(_archetype: &'a Archetype) -> usize {
        1
    }

    fn iter(_archetype: &'a Archetype) -> Self::Iterator {
        InfiniteBidirectionalIterator::default()
    }

    fn from_entity_reference(
        _entity_reference: &EntityReference,
    ) -> Self::FromEntityReferenceIterator {
        SingleBidirectionalIterator::default()
    }
}
