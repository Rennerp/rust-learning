# 04 - Rust Language Map

This is the checklist of topics you will learn over time. You do not need to master all of it before building useful programs.

## Core Syntax

- `let`, `mut`, constants, shadowing.
- Scalar types: integers, floats, booleans, chars.
- Compound types: tuples, arrays, slices, vectors, strings.
- Functions, expressions, blocks, and return values.
- `if`, `match`, `loop`, `while`, `for`.
- Pattern matching and destructuring.

## Ownership Model

- Ownership, moves, and drops.
- `Copy` types versus moved owned types.
- Shared references: `&T`.
- Mutable references: `&mut T`.
- Slices: `&str`, `&[T]`.
- Lifetimes as the compiler's way of checking reference validity.

## Data Modeling

- Structs for grouped data.
- Tuple structs and unit structs.
- Enums for closed sets of possibilities.
- Methods with `impl`.
- Associated functions such as `new`.
- Visibility with `pub`.
- Modules with `mod`, `use`, and file layout.

## Error Handling

- `Option<T>` for absence.
- `Result<T, E>` for failure.
- `match`, `if let`, and `let else`.
- The `?` operator.
- Custom error enums.
- Panics for unrecoverable programmer errors.

## Abstraction

- Traits for shared behavior.
- Trait bounds.
- Generics over types and lifetimes.
- Associated types.
- Trait objects with `dyn Trait`.
- Derive macros for common traits.

## Standard Library Skills

- `Vec`, `String`, `HashMap`, `HashSet`.
- Iterators: `map`, `filter`, `find`, `fold`, `collect`.
- File I/O with `std::fs`.
- Command-line args with `std::env`.
- Time, paths, formatting, parsing, and conversions.

## Concurrency

- Threads with `std::thread`.
- Message passing with channels.
- Shared state with `Arc` and `Mutex`.
- `Send` and `Sync`.
- Async Rust with `async`, `.await`, futures, and runtimes such as Tokio.

## Advanced Topics

- Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`, `Mutex`.
- Interior mutability.
- Macros.
- Unsafe Rust.
- Foreign function interfaces.
- Workspaces and publishing crates.
- Benchmarking and profiling.

## Practical Order

Learn in this order:

1. Syntax and Cargo.
2. Ownership and borrowing.
3. Structs, enums, and pattern matching.
4. `Option`, `Result`, and error design.
5. Iterators and collections.
6. Traits and generics.
7. Modules and larger project structure.
8. File I/O and command-line tools.
9. Concurrency and async.
10. Advanced topics only when a project needs them.

