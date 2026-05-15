# Exercises

Each exercise package is a small library crate with tests.

Run all exercises:

```sh
cargo test
```

Run one exercise:

```sh
cargo test -p exercise_01_basics
```

The intended workflow is:

1. Read the matching lesson.
2. Open `src/lib.rs`.
3. Replace one `todo!()` at a time.
4. Run the package tests.
5. Read the compiler error before changing code.

Avoid deleting tests while learning. The tests define the behavior you are trying to implement.

The first time you run tests, they will fail because each unfinished function contains `todo!()`. That is expected. Replace one `todo!()` at a time and rerun the package tests.

Useful test commands:

```sh
cargo test -p exercise_01_basics fahrenheit
cargo test -p exercise_02_ownership first_word
cargo test -p exercise_03_types_and_errors validates_user_email
```

The final word is a test-name filter. It lets you focus on one problem at a time.

## Exercise Order

- `01_basics`: numbers, strings, loops, conditionals, vectors.
- `02_ownership`: references, mutable references, slices, lifetimes.
- `03_types_and_errors`: structs, enums, `Option`, `Result`, custom errors.

## How To Approach A TODO

For each unfinished function:

1. Read the function signature.
2. Identify each input type.
3. Identify the return type.
4. Read the matching test.
5. Write the simplest implementation that satisfies that test.
6. Add or improve the implementation only when another test requires it.

Example:

```rust
pub fn double(number: i32) -> i32 {
    number * 2
}
```

The signature says the function receives one `i32` and must return one `i32`.
