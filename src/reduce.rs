/// A trait which gives the ability for a reducer to reduce a given tuple. This
/// trait is available for all reducers, for all tuples containing types
/// supported by that reducer.
///
/// See [`TupleReducer`] for more information.
pub trait TupleReducerCapable<T, U> {
    fn reduce(tuple: U, input: T) -> T;
}

/// Perform a reduce operation given the current accumulator and the input type
/// `F`.
///
/// ```
/// use tuplemagic::{TupleReducer, TupleReducerCapable};
/// type T = (u8, u8, u16, u32, u16, u8, Option<()>, Vec<u8>);
///
/// struct TupleReducerSum {}
/// impl TupleReducer<usize, u8> for TupleReducerSum {
///     fn reduce_one(collect: usize, from: u8) -> usize {
///         collect + from as usize
///     }
/// }
/// impl TupleReducer<usize, u16> for TupleReducerSum {
///     fn reduce_one(collect: usize, from: u16) -> usize {
///         collect + from as usize
///     }
/// }
/// impl TupleReducer<usize, u32> for TupleReducerSum {
///     fn reduce_one(collect: usize, from: u32) -> usize {
///         collect + from as usize
///     }
/// }
/// impl<T> TupleReducer<usize, Option<T>> for TupleReducerSum {
///     fn reduce_one(collect: usize, from: Option<T>) -> usize {
///         collect + from.is_some() as usize
///     }
/// }
/// impl<T> TupleReducer<usize, Vec<T>> for TupleReducerSum {
///     fn reduce_one(collect: usize, from: Vec<T>) -> usize {
///         collect + from.len() as usize
///     }
/// }
///
/// let t: T = (1, 10, 100, 1000, 1, 1, Some(()), vec![1]);
/// let out = TupleReducerSum::reduce(t, 0);
/// assert_eq!(out, 1115);
/// ```
pub trait TupleReducer<I, F> {
    fn reduce_one(collect: I, from: F) -> I;
}

macro_rules! tuple_reduce {
    () => {};
    ($first:ident, $($tail:ident,)*) => {
        impl <REDUCER, TARGET, $first,$($tail),*> $crate::TupleReducerCapable<TARGET, ($first,$($tail,)*)> for REDUCER where
            REDUCER: $crate::TupleReducer<TARGET, $first>,
            $(
                REDUCER: $crate::TupleReducer<TARGET, $tail>,
            )* {
                fn reduce(tuple: ($first,$($tail,)*), input: TARGET) -> TARGET {
                    let this = tuple.nest();
                    let accum = input;
                    let accum = REDUCER::reduce_one(accum, this.0);
                    let this = this.1;
                    $(
                        // Loop on $tail but don't use it
                        stringify!($tail);
                        let accum = REDUCER::reduce_one(accum, this.0);
                        let this = this.1;
                    )*
                    let _: $crate::EOT = this;
                    accum
                }
        }
    };
}
pub(crate) use tuple_reduce;
