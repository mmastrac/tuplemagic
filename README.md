# tuplemagic

TupleMagic is a Rust library that provides utilities for manipulating tuples through various operations like mapping, filtering, nesting, and reducing. This library supports both value-level and type-level operations on tuples.

## Features

- **Nesting and Unnesting:** Transform tuples into nested structures and vice versa.
- **Mapping:** Apply transformations to each element of a tuple based on a predicate.
- **Filtering:** Include or exclude elements from a tuple based on a predicate.
- **Reducing:** Reduce a tuple to a single value using a reducer.

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
