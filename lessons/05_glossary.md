# 05 - Glossary

Use this when a Rust word appears before it feels natural.

## Attribute

Metadata written above an item with `#[...]`.

```rust
#[test]
fn works() {}
```

## Borrow

To let code use a value through a reference without taking ownership.

```rust
let size = length(&name);
```

## Crate

A Rust compilation unit. A crate can be a library or a binary.

## Enum

A type that can be one of several variants.

```rust
enum Direction {
    Up,
    Down,
}
```

## Expression

Code that produces a value.

```rust
2 + 2
```

## Function

A reusable block of code declared with `fn`.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Generic

A placeholder for a type.

```rust
fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}
```

## Lifetime

The part of the program where a reference is valid. Rust checks lifetimes to prevent dangling references.

## Macro

A code-generating tool called with `!`.

```rust
println!("hello");
```

## Method

A function called on a value with `.`.

```rust
text.len()
```

## Move

Transferring ownership from one variable to another.

```rust
let second = first;
```

If the value is not `Copy`, the old variable cannot be used afterward.

## Ownership

The rule that each value has one owner responsible for dropping it.

## Package

A Cargo project described by a `Cargo.toml`.

## Pattern

A shape used to match or destructure values.

```rust
match value {
    Some(number) => number,
    None => 0,
}
```

## Reference

A borrowed view of a value.

```rust
&value
&mut value
```

## Result

A type for success or failure.

```rust
Result<T, E>
```

It is either `Ok(T)` or `Err(E)`.

## Scope

The region where a variable is valid.

```rust
{
    let value = 1;
}
```

`value` exists only inside the braces.

## Slice

A borrowed view into part or all of a collection.

```rust
&text[0..3]
&numbers[..]
```

## Statement

Code that performs an action and does not produce a useful value.

```rust
let name = "Rust";
```

## Struct

A type that groups named fields.

```rust
struct Point {
    x: i32,
    y: i32,
}
```

## Trait

A definition of shared behavior.

```rust
trait Speak {
    fn speak(&self) -> String;
}
```

## Type

A category of value, such as `i32`, `bool`, `String`, or your own `User`.

## Vector

A growable list.

```rust
let values = vec![1, 2, 3];
```

