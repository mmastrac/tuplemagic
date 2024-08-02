#![doc = include_str!("../README.md")]

mod filter;
mod map;
mod nest;
mod reduce;

#[doc(hidden)]
pub mod __macro_support {
    pub use crate::filter::{TupleFilter, TupleFiltered, TupleFilteredValue, TupleFilterer};
    pub use crate::map::TupleMapper;
}

pub use filter::{TupleFilterExclude, TupleFilterInclude};
pub use nest::{TupleNest, TupleUnnest, EOT};
pub use reduce::{TupleReducer, TupleReducerCapable};

/// Represents a type mapping operation. The type parameter `P` represents the
/// operation.
pub trait TypeMap<P> {
    type Mapped;
}

impl<P> TypeMap<P> for EOT {
    type Mapped = EOT;
}

impl<PREDICATE, OUTPUT, H: TypeMap<PREDICATE, Mapped = OUTPUT>, T> TypeMap<PREDICATE> for (H, T)
where
    T: TypeMap<PREDICATE>,
{
    type Mapped = (OUTPUT, <T as TypeMap<PREDICATE>>::Mapped);
}

#[doc(hidden)]
pub struct TypePair<T1, T2>(T1, T2);
#[doc(hidden)]
pub trait TypePairExtract {
    type T1;
    type T2;
}
impl<T1, T2> TypePairExtract for TypePair<T1, T2> {
    type T1 = T1;
    type T2 = T2;
}

macro_rules! tuple_repeat {
    ($macro:ident) => {
        tuple_repeat!($macro, (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, ));
    };
    ($macro:ident, ($first:ident, $($tail:ident,)*)) => {
        tuple_repeat!($macro, ($($tail,)*));
        $macro!($first, $($tail,)*);
    };
    ($macro:ident, ()) => {
    };
}

use nest::tuple_nest;
tuple_repeat!(tuple_nest);
use reduce::tuple_reduce;
tuple_repeat!(tuple_reduce);

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn filter_type() {
        // This tuple includes a large number of types
        type T = (u8, u8, u16, u32, u16, u8, Option<()>, Vec<u8>);
        tuple_filter_predicate!(P = { include = (u8, Vec<u8>), exclude = (u16, u32, ~ <T> Option<T>)});
        type U = tuple_filter!(P::filter_type(T));
        static_assertions::assert_eq_type!((u8, u8, u8, Vec<u8>), U);
    }

    #[test]
    fn filter_value() {
        // This tuple includes a large number of types
        type T = (u8, u8, u16, u32, u16, u8, Option<()>, Vec<u8>);
        let x: T = (0, 1, 2, 3, 4, 5, None, vec![]);
        tuple_filter_predicate!(P = { include = (u8, Vec<u8>), exclude = (u16, u32, ~ <T> Option<T>)});
        let y = tuple_filter!(P::filter(x));
        assert_eq!(y, (0, 1, 5, vec![]));
        let y = tuple_filter!(P::filter((1_u8, 2_u8, 3_u16)));
        assert_eq!(y, (1, 2));
    }

    #[test]
    fn reduce_value() {
        type T = (u8, u8, u16, u32, u16, u8, Option<()>, Vec<u8>);

        struct TupleReducerSum {}
        impl TupleReducer<usize, u8> for TupleReducerSum {
            fn reduce_one(collect: usize, from: u8) -> usize {
                collect + from as usize
            }
        }
        impl TupleReducer<usize, u16> for TupleReducerSum {
            fn reduce_one(collect: usize, from: u16) -> usize {
                collect + from as usize
            }
        }
        impl TupleReducer<usize, u32> for TupleReducerSum {
            fn reduce_one(collect: usize, from: u32) -> usize {
                collect + from as usize
            }
        }
        impl<T> TupleReducer<usize, Option<T>> for TupleReducerSum {
            fn reduce_one(collect: usize, from: Option<T>) -> usize {
                collect + from.is_some() as usize
            }
        }
        impl<T> TupleReducer<usize, Vec<T>> for TupleReducerSum {
            fn reduce_one(collect: usize, from: Vec<T>) -> usize {
                collect + from.len() as usize
            }
        }

        let t: T = (1, 10, 100, 1000, 1, 1, Some(()), vec![1]);
        let out = TupleReducerSum::reduce(t, 0);
        assert_eq!(out, 1115);
    }

    #[test]
    fn map_type() {
        type T = (Option<u8>, Option<u16>, Option<()>);
        struct RemoveOption {}
        impl<T> TypeMap<RemoveOption> for Option<T> {
            type Mapped = T;
        }

        type U = tuple_mapper!(RemoveOption::map(T));
        static_assertions::assert_eq_type!((u8, u16, ()), U);
    }
}
