use crate::entity::archetype::Archetype;
use crate::entity::entity_query::BidirectionalIterator;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_reference::EntityReference;
use crate::entity::entity_reference::InnerShareableEntityReference;
use std::ops::Deref;
use std::ops::Mul;

struct TupleSubIterator<'a, T: QueryParam<'a> + 'static> {
    iterator: T::Iterator,
    local_id: usize,
    items_per_entity: usize,
}

struct TupleSubFromEntityReferenceIterator<'a, T: QueryParam<'a> + 'static> {
    iterator: T::FromEntityReferenceIterator,
    local_id: usize,
    items_per_entity: usize,
}

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
            $($tn: TupleSubIterator<'a, $tn>),*
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
            $($tn: TupleSubFromEntityReferenceIterator<'a, $tn>),*
        }
    };
}

macro_rules! next_for_iterators {
    (
        [$($all_tn:ident),*]
        $self:ident,
        $last:ident,
        $($previous_tn:ident),+
    ) => {
        // Next for previous
        next_for_iterators!([$($all_tn),*] $self, $($previous_tn),*);

        // Next if needed for T(x)
        if $self.$last.local_id + 1 < $self.$last.items_per_entity {
            $self.$last.iterator.next()?;
            $self.$last.local_id += 1;

            // Reinitialize the left iterators
            $(
                $self.$previous_tn.iterator.go_back($self.$previous_tn.items_per_entity - 1);
                $self.$previous_tn.local_id += 0;
            )*

            // Returns the current result
            return Some((
                $($self.$all_tn.iterator.current(),)*
            ))
        }
    };
    (
        [$($all_tn:ident),*]
        $self:ident,
        $last:ident
    ) => {
        // Next if needed for T(x)
        if $self.$last.local_id + 1 < $self.$last.items_per_entity {
            $self.$last.iterator.next()?;
            $self.$last.local_id += 1;

            // Returns the current result
            return Some((
                $($self.$all_tn.iterator.current(),)*
            ))
        }
    };
    // This is to reverse the elem order
    ($self:ident, [$($all_tn:ident),*] [] $($reversed:ident,)*) => {
        next_for_iterators!([$($all_tn),*] $self, $($reversed),*) // base case
    };
    ($self:ident, [$($all_tn:ident),*] [$first:ident] $($reversed:ident,)*) => {
        next_for_iterators!($self, [$($all_tn),*] [] $first, $($reversed,)*) // last recursion
    };
    ($self:ident, [$($all_tn:ident),*] [$first:ident, $($rest:ident),*] $($reversed:ident,)*) => {
        next_for_iterators!($self, [$($all_tn),*]  [$($rest),*] $first, $($reversed,)*) // recursion
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
                if $(self.$tn.local_id + 1 == self.$tn.items_per_entity) && * {
                    $(self.$tn.local_id = 0;)*

                    return Some((
                        $(self.$tn.iterator.next()?,)*
                    ));
                }

                next_for_iterators!(self, [ $($tn),* ] [ $($tn),* ]);

                unreachable!()
            }
        }
    };
}

macro_rules! impl_bidirectional_iterator {
    (
        $iterator_ident:ident,
        $($tn:ident),+
    ) => {
        impl<
                'a,
                $($tn: QueryParam<'a> + 'static),*
            > BidirectionalIterator for $iterator_ident<'a, $($tn),+>
        {
            fn current(&mut self) -> Self::Item {
                (
                    $(self.$tn.iterator.current()),*
                )
            }

            fn go_back(&mut self, count: usize) {
                $(self.$tn.iterator.go_back(self.$tn.items_per_entity * count);)*
            }
        }
    };
}

macro_rules! items_per_entity {
    (
        $archetype:ident,
        $t1:ident,
        $($tn:ident),+
    ) => {
        $t1::items_per_entity($archetype)
        $(.mul($tn::items_per_entity($archetype)))*
    }
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

            fn filter_archetype(archetype: &Archetype) -> bool {
                $($tn::filter_archetype(archetype)) && +
            }

            fn require_read() -> bool {
                $($tn::require_read()) || +
            }

            fn require_write() -> bool {
                $($tn::require_write()) || +
            }

            fn items_per_entity(archetype: &'a Archetype) -> usize {
                items_per_entity!(archetype, $($tn),*)
            }

            fn iter(archetype: &'a Archetype) -> Self::Iterator {
                $iterator_ident {
                    $(
                        $tn: TupleSubIterator {
                            iterator: $tn::iter(archetype),
                            local_id: 0,
                            items_per_entity: $tn::items_per_entity(archetype),
                        },
                    )*
                }
            }

            fn from_entity_reference(
                entity_reference: &EntityReference,
            ) -> Self::FromEntityReferenceIterator {
                let inner_entity_reference = entity_reference.inner.read();
                if let InnerShareableEntityReference::Archetype {
                    archetype_ptr, ..
                } = inner_entity_reference.deref()
                {
                    let archetype = unsafe {
                            archetype_ptr
                            .as_ref()
                            .unwrap()
                    };

                    $from_entity_reference_iterator_ident {
                        $(
                            $tn: TupleSubFromEntityReferenceIterator {
                                iterator: $tn::from_entity_reference(entity_reference),
                                local_id: 0,
                                items_per_entity: $tn::items_per_entity(archetype),
                            },
                        )*
                    }
                } else {
                    unreachable!()
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
        impl_bidirectional_iterator!($iterator_ident, $($tn),*);

        struct_from_entity_reference_iterator!($from_entity_reference_iterator_ident, $($tn),*);
        impl_iterator!($from_entity_reference_iterator_ident, $($tn),*);
        impl_bidirectional_iterator!($from_entity_reference_iterator_ident, $($tn),*);

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
