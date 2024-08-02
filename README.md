# tuplemagic

TupleMagic is a Rust library that provides utilities for manipulating tuples
through various operations like mapping, filtering, nesting, and reducing. This
library supports both value-level and type-level operations on tuples.

## Details

TupleMagic transforms tuples into a nested form which allows for varargs-like
processing of tuple types. An input tuple of type `(A, B, C)` becomes the nested
form `(A, (B, (C, EOT)))`.

Turning tuples into nested types simplifies filtering and mapping operations
because it leverages recursive type structures, which are inherently more
adaptable to recursive traits.

Note that some features of this library are only available in macro form as the
non-macro form of the operation may be unergonomic (ie: `filter` and `map`
operations).

## Features

- **Nesting and Unnesting:** Transform tuples into nested structures and vice versa.

```rust
# use tuplemagic::*;
let a = (1, 2, 3).nest();
```

- **Mapping:** Apply transformations to each element of a tuple based on a type
  mapper.

```rust
# use tuplemagic::*;
type T = (Option<u8>, Option<u16>, Option<()>);
struct RemoveOption {}
impl<T> TypeMap<RemoveOption> for Option<T> {
    type Mapped = T;
}
type U = tuple_mapper!(RemoveOption::map(T));
```

- **Filtering:** Include or exclude elements from a tuple based on a predicate.

```rust
# use tuplemagic::*;
type T = (u8, u16, Option<()>);
tuple_filter_predicate!(P = { include = (u8, Vec<u8>), exclude = (u16, u32, ~ <T> Option<T>)});
type U = tuple_filter!(P::filter_type(T));
```

- **Reducing:** Reduce a tuple to a single value using a reducer.

```rust
# use tuplemagic::*;
struct TupleReducerSum;
impl<T> TupleReducer<usize, T> for TupleReducerSum where T: TryInto<usize>,
{
    fn reduce_one(collect: usize, from: T) -> usize {
        collect + from.try_into().unwrap_or_default()
    }
}
let out = TupleReducerSum::reduce((1_u8, 2_u16, 3_u32), 0);
```

## Operations

Features of this crate may be available as macros, traits or both based on
ergonomic concerns. The goal of the crate will be to eventually move to a pure
trait-based system, but this does not appear possible at this time.

The underlying mechanisms of the macros are subject to change and are not
considered stable at this time.

| Operation          | Value Level | Type Level | APIs |
|--------------------|-------------|------------|------|
| Nesting            | Yes         | Yes        | [`TupleNest`](https://docs.rs/tuplemagic/latest/tuplemagic/trait.TupleNest.html), [`TupleUnnest`](https://docs.rs/tuplemagic/latest/tuplemagic/trait.TupleUnnest.html) [`nest!`](https://docs.rs/tuplemagic/latest/tuplemagic/macro.nest.html) |
| Unnesting          | Yes         | Yes        | see above |
| Mapping            |             | Yes        | [`tuple_mapper!`](https://docs.rs/tuplemagic/latest/tuplemagic/macro.tuple_mapper.html) [`TypeMap`](https://docs.rs/tuplemagic/latest/tuplemagic/trait.TypeMap.html) |
| Filtering          | Yes         | Yes        | [`tuple_filter!`](https://docs.rs/tuplemagic/latest/tuplemagic/macro.tuple_filter.html) [`tuple_filter_predicate!`](https://docs.rs/tuplemagic/latest/tuplemagic/macro.tuple_filter_predicate.html) |
| Reducing           | Yes         |            | [`TupleReducer`](https://docs.rs/tuplemagic/latest/tuplemagic/trait.TupleReducer.html), [`TupleReducerCapable`](https://docs.rs/tuplemagic/latest/tuplemagic/trait.TupleReducerCapable.html) |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tuplemagic = "0.x.y"
```
