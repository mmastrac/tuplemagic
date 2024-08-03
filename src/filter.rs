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
/// For types that are generic, use the syntax `~ <T> Option<T>`.
///
/// ## Limitations
///
/// Most trait bounds should be supported, but additional `+` bounds for
/// generics are not at this time.
///
/// ```
/// # use tuplemagic::tuple_filter_predicate;
/// # use std::collections::HashMap;
/// // Private predicate
/// tuple_filter_predicate!(P1 = { include = (u8), exclude = (u16, u32) });
/// // Mark a predicate as public with `pub` (or `pub(crate)`, etc.)
/// tuple_filter_predicate!(pub P2 = { include = (u8), exclude = (u16, u32) });
/// // Generics are supported
/// tuple_filter_predicate!(P3 = { include = (~ <T> Vec<T>), exclude = (~ <T, U> HashMap<T, U>) });
/// // Including constants
/// tuple_filter_predicate!(P4 = { include = (~ <const S: usize> [u8; S]), exclude = (~ <const S: usize> [u16; S]) });
/// // And lifetimes...
/// tuple_filter_predicate!(P5 = { include = (~ <'a> &'a str), exclude = (&'static [u8]) });
/// /// And why not all of the above?
/// tuple_filter_predicate!(P6 = { include = (~ <'a, T: Into<usize>, const S: usize> &'a [T; S]), exclude = () });
/// ```
#[macro_export]
macro_rules! tuple_filter_predicate {
    ($vis:vis $predicate:ident = {
        include = ($($(~ <$($ilt:lifetime),* $(,)? $($igen:ident $($igen2:ident)? $(: $($ibound:path)+)?),*>)? $include:ty),*),
        exclude = ($($(~ <$($elt:lifetime),* $(,)? $($egen:ident $($egen2:ident)? $(: $($ebound:path)+)?),*>)? $exclude:ty),*)
    }) => {
        $vis struct $predicate {}
        $(
            impl $(<$($ilt,)* $($igen $($igen2)? $(: $($ibound)+)?,)*>)? $crate::TypeMap<$predicate> for $include {
                type Mapped = $crate::TupleFilterInclude;
            }
        )*
        $(
            impl $(<$($elt,)*  $($egen $($egen2)? $(: $($ebound)+)?,)*>)? $crate::TypeMap<$predicate> for $exclude {
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
/// # use tuplemagic::{tuple_filter_predicate, tuple_filter};
/// tuple_filter_predicate!(P = { include = (u8, Vec<u8>), exclude = (u16, u32, ~ <T> Option<T>)});
///
/// // You can filter a type definition
/// type T = (u8, u16);
/// type U = tuple_filter!(P::filter_type(T));
/// static_assertions::assert_eq_type!(U, (u8,));
///
/// // The type can also be inline
/// type V = tuple_filter!(P::filter_type((u8, u16)));
/// static_assertions::assert_eq_type!(V, (u8,));
/// ```
///
/// ```
/// # use tuplemagic::{tuple_filter_predicate, tuple_filter};
/// // A predicate for use below
/// tuple_filter_predicate!(P = { include = (u8, Vec<u8>), exclude = (u16, u32, ~ <T> Option<T>)});
///
/// // This type isn't necessary but makes this example a little less noisy
/// type T = (u8, u8, u16, u32, u16, u8, Option<()>, Vec<u8>);
/// let x: T = (0, 1, 2, 3, 4, 5, None, vec![]);
/// let y = tuple_filter!(P::filter(x));
/// assert_eq!(y, (0, 1, 5, vec![]));
///
/// // You can filter an inline tuple as well, assuming all types
/// // can be determined from that tuple.
/// let y = tuple_filter!(P::filter((1_u8, 2_u8, 3_u16)));
/// assert_eq!(y, (1, 2));
/// ```
///
#[macro_export]
macro_rules! tuple_filter {
    ($predicate:ident::filter_type($ty:ty)) => {
        <<$crate::TypePair<
            <$ty as $crate::TupleNest>::Nested,
            <<$ty as $crate::TupleNest>::Nested as $crate::TypeMap<$predicate>>::Mapped,
        > as $crate::__macro_support::TupleFiltered>::Output as $crate::TupleUnnest>::Unnested
    };
    ($predicate:ident::filter($tuple:expr)) => {{
        use $crate::__macro_support::TupleFilterer;
        let tuple = $tuple;
        $crate::__macro_support::TupleFilter::<_, P>::of_ref(&tuple).do_filter(tuple)
    }};
}

/// Include this tuple item in the final result.
#[derive(Default)]
pub struct TupleFilterInclude();
/// Exclude this tuple item from the final result.
#[derive(Default)]
pub struct TupleFilterExclude();
