use std::marker::PhantomData;

use crate::nest::{TupleNest, TupleUnnest};
use crate::{TypeMap, TypePair, EOT};

#[doc(hidden)]
pub trait TupleFiltered {
    type Input;
    type Output;
}

impl<H1, T1, T2> TupleFiltered for TypePair<(H1, T1), (TupleFilterExclude, T2)>
where
    TypePair<T1, T2>: TupleFiltered,
{
    type Input = (H1, T1);
    type Output = <TypePair<T1, T2> as TupleFiltered>::Output;
}

impl<H1, T1, T2> TupleFiltered for TypePair<(H1, T1), (TupleFilterInclude, T2)>
where
    TypePair<T1, T2>: TupleFiltered,
    // T1: TypeEqual<<TypePair<T1, T2> as TupleFiltered>::Input>,
{
    type Input = (H1, T1);
    type Output = (H1, <TypePair<T1, T2> as TupleFiltered>::Output);
}

impl TupleFiltered for TypePair<EOT, EOT> {
    type Output = EOT;
    type Input = EOT;
}

#[doc(hidden)]
pub trait TupleFilteredValue<I> {
    type Output;
    fn filter(input: I) -> Self::Output;
}

impl<H1, T1, T2> TupleFilteredValue<(H1, T1)> for TypePair<(H1, T1), (TupleFilterInclude, T2)>
where
    TypePair<T1, T2>: TupleFilteredValue<T1>,
    // (H1, <TypePair<T1, T2> as TupleFilteredValue<T1>>::Output): TupleUnnest,
{
    type Output = (H1, <TypePair<T1, T2> as TupleFilteredValue<T1>>::Output);
    fn filter(input: (H1, T1)) -> Self::Output {
        (
            input.0,
            <TypePair<T1, T2> as TupleFilteredValue<T1>>::filter(input.1),
        )
    }
}

impl<H1, T1, T2> TupleFilteredValue<(H1, T1)> for TypePair<(H1, T1), (TupleFilterExclude, T2)>
where
    TypePair<T1, T2>: TupleFilteredValue<T1>,
{
    type Output = <TypePair<T1, T2> as TupleFilteredValue<T1>>::Output;
    fn filter(input: (H1, T1)) -> Self::Output {
        <TypePair<T1, T2> as TupleFilteredValue<T1>>::filter(input.1)
    }
}

impl TupleFilteredValue<EOT> for TypePair<EOT, EOT> {
    type Output = EOT;
    fn filter(_: EOT) -> Self::Output {
        EOT
    }
}

#[doc(hidden)]
pub trait TupleFilterer<TUPLE> {
    type Output;
    fn do_filter(&self, tuple: TUPLE) -> Self::Output;
}

#[doc(hidden)]
pub struct TupleFilter<T, P>(PhantomData<(T, P)>);

impl<T, P> TupleFilter<T, P> {
    pub fn of() -> Self {
        Self(PhantomData)
    }
    pub fn of_ref(_: &T) -> Self {
        Self(PhantomData)
    }
}

/// This is a helper type that makes this slightly less painful to work with.
type Filterer<TUPLE, PREDICATE> = TypePair<
    <TUPLE as TupleNest>::Nested,
    <<TUPLE as TupleNest>::Nested as TypeMap<PREDICATE>>::Mapped,
>;

impl<TUPLE, PREDICATE> TupleFilterer<TUPLE> for TupleFilter<TUPLE, PREDICATE>
where
    TUPLE: TupleNest,
    <TUPLE as TupleNest>::Nested: TypeMap<PREDICATE>,
    Filterer<TUPLE, PREDICATE>: TupleFilteredValue<<TUPLE as TupleNest>::Nested>,
    <Filterer<TUPLE, PREDICATE> as TupleFilteredValue<<TUPLE as TupleNest>::Nested>>::Output:
        TupleUnnest,
{
    type Output = <<Filterer<TUPLE, PREDICATE> as TupleFilteredValue<
        <TUPLE as TupleNest>::Nested,
    >>::Output as TupleUnnest>::Unnested;
    fn do_filter(&self, tuple: TUPLE) -> Self::Output {
        let nested = tuple.nest();

        <Filterer<TUPLE, PREDICATE> as TupleFilteredValue<<TUPLE as TupleNest>::Nested>>::filter(
            nested,
        )
        .unnest()
    }
}

/// Define a tuple filter predicate (a [`TypeMap`] that returns
/// [`TupleFilterInclude`] or [`TupleFilterExclude`]) that includes and excludes
/// the given types. All types that may be potentially encountered in the
/// provided tuples must be specified.
///
/// For types that are generic, use the syntax `~ T Option<T>`.
#[macro_export]
macro_rules! tuple_filter_predicate {
    ($predicate:ident = { include = ($($(~ $include_generics:ident)? $include:ty),*), exclude = ($($(~ $exclude_generics:ident)? $exclude:ty),*) }) => {
        struct $predicate {}
        $(
            impl $(<$include_generics>)? $crate::TypeMap<$predicate> for $include {
                type Mapped = $crate::TupleFilterInclude;
            }
        )*
        $(
            impl $(<$exclude_generics>)? $crate::TypeMap<$predicate> for $exclude {
                type Mapped = $crate::TupleFilterExclude;
            }
        )*
    };
}

/// Perform a filtering operation on a tuple using the given predicate type (a
/// [`TypeMap`] that returns [`TupleFilterInclude`] or [`TupleFilterExclude`]).
///
/// This macro can be used to filter a type (using `filter_type`) or a value
/// (using `filter`).
///
/// ```
/// # use tuplemagic::*;
/// type T = (u8, u16);
/// tuple_filter_predicate!(P = { include = (u8, Vec<u8>), exclude = (u16, u32, ~ T Option<T>)});
/// type U = tuple_filter!(P::filter_type(T));
/// let _: (u8,) = U::default();
/// ```
///
/// ```
/// # use tuplemagic::{tuple_filter_predicate, tuple_filter};
/// type T = (u8, u8, u16, u32, u16, u8, Option<()>, Vec<u8>);
/// let x: T = (0, 1, 2, 3, 4, 5, None, vec![]);
/// tuple_filter_predicate!(P = { include = (u8, Vec<u8>), exclude = (u16, u32, ~ T Option<T>)});
/// let y = tuple_filter!(P::filter(x));
/// assert_eq!(y, (0, 1, 5, vec![]));
/// let y = tuple_filter!(P::filter((1_u8, 2_u8, 3_u16)));
/// assert_eq!(y, (1, 2));
/// ```
///
#[macro_export]
macro_rules! tuple_filter {
    ($predicate:ident::filter_type($ty:ty)) => {
        <<TypePair<
            <$ty as TupleNest>::Nested,
            <<$ty as TupleNest>::Nested as TypeMap<$predicate>>::Mapped,
        > as $crate::TupleFiltered>::Output as TupleUnnest>::Unnested
    };
    ($predicate:ident::filter($tuple:expr)) => {{
        use $crate::TupleFilterer;
        let tuple = $tuple;
        $crate::TupleFilter::<_, P>::of_ref(&tuple).do_filter(tuple)
    }};
}

/// Include this tuple item in the final result.
#[derive(Default)]
pub struct TupleFilterInclude();
/// Exclude this tuple item from the final result.
#[derive(Default)]
pub struct TupleFilterExclude();
