use crate::nest::{TupleNest, TupleUnnest};
use crate::TypeMap;

#[doc(hidden)]
pub trait TupleMapper {
    type Output;
}

impl<TUPLE, PREDICATE> TupleMapper for (TUPLE, PREDICATE)
where
    TUPLE: TupleNest,
    <TUPLE as TupleNest>::Nested: TypeMap<PREDICATE>,
    <<TUPLE as TupleNest>::Nested as TypeMap<PREDICATE>>::Mapped: TupleUnnest,
{
    type Output =
        <<<TUPLE as TupleNest>::Nested as TypeMap<PREDICATE>>::Mapped as TupleUnnest>::Unnested;
}

/// Perform a mapping operation on a tuple using the given predicate type (a
/// [`TypeMap`] that returns a mapped type).
///
/// ```
/// # use tuplemagic::{TypeMap, tuple_mapper};
/// type T = (Option<u8>, Option<u16>, Option<()>);
/// struct RemoveOption {}
/// impl<T> TypeMap<RemoveOption> for Option<T> {
///     type Mapped = T;
/// }
///
/// tuple_mapper!(U = T:map(RemoveOption));
/// let _: (u8, u16, ()) = U::default();
/// ```
#[macro_export]
macro_rules! tuple_mapper {
    ($name:ident = $ty:ty:map($predicate:ident)) => {
        type $name = <($ty, $predicate) as $crate::TupleMapper>::Output;
    };
}
