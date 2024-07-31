/// A marker struct (_end of tuple_) for the end of a nested tuple list. As a
/// marker, `EOT` is both a value and a type.
///
/// ```
/// # use tuplemagic::*;
/// let a: (u8, (u8, EOT)) = (1, (2, EOT));
/// ```
#[derive(Debug, Default)]
pub struct EOT;

/// Convert a tuple type or value from the form `(A, B, C, ...)` to the form
/// `(A, (B, (C, ..., EOT)))`.
///
/// Note that while this macro will correctly nest both types and values, it is
/// recommended that the [`TupleNest`] trait be used for values.
///
/// ```
/// # use tuplemagic::*;
///
/// // Use `nest!` to transform a tuple type
/// let a: (u8, u8, u8) = (1, 2, 3);
/// let b: nest!(u8, u8, u8) = a.nest();
/// assert_eq!(&format!("{b:?}"), "(1, (2, (3, EOT)))");
///
/// // Use `nest!` to transform a tuple value directly
/// let c = nest!(1, 2, 3);
/// assert_eq!(&format!("{c:?}"), "(1, (2, (3, EOT)))");
/// ```
#[macro_export]
macro_rules! nest {
    ($first:tt, $($tail:tt),* $(,)?) => {
        ($first, $crate::nest!($($tail,)*))
    };
    () => { $crate::EOT }
}

macro_rules! tuple_nest {
    ($first:ident, $($tail:ident,)*) => {
        impl <$first,$($tail),*> TupleNest for ($first,$($tail,)*) {
            type Nested = $crate::nest!($first,$($tail,)*);

            #[inline(always)]
            #[allow(non_snake_case)]
            fn nest(self) -> Self::Nested {
                let ($first,$($tail),*) = self;
                $crate::nest!($first, $($tail,)*)
            }
        }
        impl <$first,$($tail),*> TupleUnnest for $crate::nest!($first,$($tail,)*) {
            type Unnested = ($first,$($tail,)*);
            type Head = $first;
            type Tail = $crate::nest!($($tail,)*);

            #[inline(always)]
            #[allow(non_snake_case)]
            fn unnest(self) -> Self::Unnested {
                let $crate::nest!($first,$($tail,)*) = self;
                ($first, $($tail,)*)
            }
            fn from(head: Self::Head, tail: Self::Tail) -> Self {
                (head, tail)
            }
            fn split(self) -> (Self::Head, Self::Tail) {
                (self.0, self.1)
            }
        }
    }
}
pub(crate) use tuple_nest;

/// Perform a nesting operation on a tuple. Given a tuple of type `(A,B,C)`,
/// returns `(A, (B, (C, ()))`.
///
/// ```
/// use tuplemagic::TupleNest;
/// let a = (1, 2, 3).nest();
/// assert_eq!(&format!("{a:?}"), "(1, (2, (3, EOT)))");
/// let a = ().nest();
/// assert_eq!(&format!("{a:?}"), "EOT");
/// ```
pub trait TupleNest {
    type Nested: TupleUnnest;
    fn nest(self) -> Self::Nested;
}

/// Perform an unnesting operation on a tuple. Given a tuple of type `(A, (B,
/// (C, ()))`, returns `(A,B,C)`.
///
/// ```
/// use tuplemagic::{TupleNest, TupleUnnest};
/// let a = (1, 2, 3);
/// let b = a.nest();
/// let c = b.unnest();
/// assert_eq!(a, c);
///
/// let a = ();
/// let b = a.nest();
/// let c = b.unnest();
/// assert_eq!(a, c);
/// ```
pub trait TupleUnnest {
    type Unnested: TupleNest;
    type Head;
    type Tail: TupleUnnest;
    fn unnest(self) -> Self::Unnested;
    fn from(head: Self::Head, tail: Self::Tail) -> Self;
    fn split(self) -> (Self::Head, Self::Tail);
    fn head(self) -> Self::Head
    where
        Self: Sized,
    {
        self.split().0
    }
    fn tail(self) -> Self::Tail
    where
        Self: Sized,
    {
        self.split().1
    }
}

impl TupleUnnest for EOT {
    type Unnested = ();
    type Head = EOT;
    type Tail = EOT;
    fn unnest(self) -> Self::Unnested {}
    fn from(_: Self::Head, _: Self::Tail) -> Self {
        EOT
    }
    fn split(self) -> (Self::Head, Self::Tail) {
        (EOT, EOT)
    }
}

impl TupleNest for () {
    type Nested = EOT;
    fn nest(self) -> Self::Nested {
        EOT
    }
}
