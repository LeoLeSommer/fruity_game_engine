use super::{EntityIterator, QueryParam};
use crate::entity::{Archetype, ArchetypeComponentTypes, EntityReference};

macro_rules! struct_iterator {
    (
        $iterator_ident:ident,
        $($tn:ident),+
    ) => {
        #[allow(non_snake_case)]
        #[allow(missing_docs)]
        pub struct $iterator_ident<
            'a,
            $($tn: QueryParam<'a> + 'static),*
        > {
            $($tn: $tn::Iterator),*
        }
    };
}

macro_rules! struct_from_entity_reference_iterator {
    (
        $iterator_ident:ident,
        $($tn:ident),+
    ) => {
        #[allow(non_snake_case)]
        #[allow(missing_docs)]
        pub struct $iterator_ident<
            'a,
            $($tn: QueryParam<'a> + 'static),*
        > {
            $($tn: $tn::FromEntityReferenceIterator),*
        }
    };
}

macro_rules! next_for_entity_iterators {
    (
        [$($all_tn:ident),*]
        $self:ident,
        $last:ident,
        $($previous_tn:ident),+
    ) => {
        // Next for previous
        next_for_entity_iterators!([$($all_tn),*] $self, $($previous_tn),*);

        // Next if needed for T(x)
        if !$self.$last.has_reach_entity_end() {
            $self.$last.next()?;

            // Reinitialize the left iterators
            $(
                $self.$previous_tn.reset_current_entity();
            )*

            // Returns the current result
            return Some((
                $($self.$all_tn.current(),)*
            ))
        }
    };
    (
        [$($all_tn:ident),*]
        $self:ident,
        $last:ident
    ) => {
        // Next if needed for T(x)
        if !$self.$last.has_reach_entity_end() {
            $self.$last.next()?;

            // Returns the current result
            return Some((
                $($self.$all_tn.current(),)*
            ))
        }
    };
    // This is to reverse the elem order
    ($self:ident, [$($all_tn:ident),*] [] $($reversed:ident,)*) => {
        next_for_entity_iterators!([$($all_tn),*] $self, $($reversed),*) // base case
    };
    ($self:ident, [$($all_tn:ident),*] [$first:ident] $($reversed:ident,)*) => {
        next_for_entity_iterators!($self, [$($all_tn),*] [] $first, $($reversed,)*) // last recursion
    };
    ($self:ident, [$($all_tn:ident),*] [$first:ident, $($rest:ident),*] $($reversed:ident,)*) => {
        next_for_entity_iterators!($self, [$($all_tn),*]  [$($rest),*] $first, $($reversed,)*) // recursion
    };
}

macro_rules! impl_iterator {
    (
        $iterator_ident:ident,
        $($tn:ident),+
    ) => {
        impl<
                'a,
                $($tn: QueryParam<'a> + 'static),*
            > Iterator for $iterator_ident<'a, $($tn),*>
        {
            type Item = ($($tn::Item),*);

            fn next(&mut self) -> Option<Self::Item> {
                // If we arrive at the end of the entity cross product, we just change the entity
                // We do it first cause it is the most common case
                if $(self.$tn.has_reach_entity_end()) && * {
                    return Some((
                        $(self.$tn.next()?,)*
                    ));
                }

                next_for_entity_iterators!(self, [ $($tn),* ] [ $($tn),* ]);

                unreachable!()
            }
        }
    };
}

macro_rules! impl_entity_iterator {
    (
        $iterator_ident:ident,
        $($tn:ident),+
    ) => {
        impl<
                'a,
                $($tn: QueryParam<'a> + 'static),*
            > EntityIterator for $iterator_ident<'a, $($tn),+>
        {
            fn current(&mut self) -> Self::Item {
                ($(self.$tn.current()),*)
            }

            fn has_reach_entity_end(&self) -> bool {
                $(self.$tn.has_reach_entity_end()) && *
            }

            fn reset_current_entity(&mut self) {
                $(self.$tn.reset_current_entity();)*
            }
        }
    };
}

macro_rules! impl_query_param {
    (
        $iterator_ident:ident,
        $from_entity_reference_iterator_ident:ident,
        $($tn:ident),+
    ) => {
        impl<
            'a,
            $($tn: QueryParam<'a> + 'static),*
        > QueryParam<'a> for ($($tn),+)
        {
            type Item = ($($tn::Item),+);
            type Iterator = $iterator_ident<'a, $($tn),+>;
            type FromEntityReferenceIterator = $from_entity_reference_iterator_ident<'a, $($tn),+>;

            fn filter_archetype(component_types: &ArchetypeComponentTypes) -> bool {
                $($tn::filter_archetype(component_types)) && +
            }

            fn iter(archetype: &'a Archetype) -> Self::Iterator {
                $iterator_ident {
                    $(
                        $tn: $tn::iter(archetype),
                    )*
                }
            }

            fn from_entity_reference(
                entity_reference: &EntityReference,
            ) -> Self::FromEntityReferenceIterator {
                $from_entity_reference_iterator_ident {
                    $(
                        $tn: $tn::from_entity_reference(entity_reference),
                    )*
                }
            }
        }
    };
}

macro_rules! tuple_impl_generics {
    (
        $iterator_ident:ident,
        $from_entity_reference_iterator_ident:ident,
        $($tn:ident),+
    ) => {
        struct_iterator!($iterator_ident, $($tn),*);
        impl_iterator!($iterator_ident, $($tn),*);
        impl_entity_iterator!($iterator_ident, $($tn),*);

        struct_from_entity_reference_iterator!($from_entity_reference_iterator_ident, $($tn),*);
        impl_iterator!($from_entity_reference_iterator_ident, $($tn),*);
        impl_entity_iterator!($from_entity_reference_iterator_ident, $($tn),*);

        impl_query_param!($iterator_ident, $from_entity_reference_iterator_ident, $($tn),*);
    }
}

tuple_impl_generics!(T2Iterator, T2FromEntityReferenceIterator, T1, T2);
tuple_impl_generics!(T3Iterator, T3FromEntityReferenceIterator, T1, T2, T3);
tuple_impl_generics!(T4Iterator, T4FromEntityReferenceIterator, T1, T2, T3, T4);
tuple_impl_generics!(
    T5Iterator,
    T5FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5
);
tuple_impl_generics!(
    T6Iterator,
    T6FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6
);
tuple_impl_generics!(
    T7Iterator,
    T7FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7
);
tuple_impl_generics!(
    T8Iterator,
    T8FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8
);
tuple_impl_generics!(
    T9Iterator,
    T9FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9
);
tuple_impl_generics!(
    T10Iterator,
    T10FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10
);
tuple_impl_generics!(
    T11Iterator,
    T11FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11
);
tuple_impl_generics!(
    T12Iterator,
    T12FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12
);
tuple_impl_generics!(
    T13Iterator,
    T13FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13
);
tuple_impl_generics!(
    T14Iterator,
    T14FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14
);
tuple_impl_generics!(
    T15Iterator,
    T15FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15
);
tuple_impl_generics!(
    T16Iterator,
    T16FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16
);
tuple_impl_generics!(
    T17Iterator,
    T17FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17
);
tuple_impl_generics!(
    T18Iterator,
    T18FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18
);
tuple_impl_generics!(
    T19Iterator,
    T19FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19
);
tuple_impl_generics!(
    T20Iterator,
    T20FromEntityReferenceIterator,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19,
    T20
);
