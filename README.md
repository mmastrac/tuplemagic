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

- **Mapping:** Apply transformations to each element of a tuple based on a predicate.

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

| Operation          | Value Level | Type Level |
|--------------------|-------------|------------|
| Nesting            | Yes         | Yes        |
| Unnesting          | Yes         | Yes        |
| Mapping            | Yes         | No         |
| Filtering          | Yes         | Yes        |
| Reducing           | Yes         | No         |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tuplemagic = "0.1.0"
```
