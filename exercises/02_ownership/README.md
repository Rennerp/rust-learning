# Exercise 02 - Ownership

This exercise practices borrowing data instead of moving it unnecessarily.

Run only this exercise:

```sh
cargo test -p exercise_02_ownership
```

Run one test at a time:

```sh
cargo test -p exercise_02_ownership finds_first_word
```

## Concepts Used

- `&str`: borrowed text.
- `&mut String`: borrowed text that can be changed.
- `&[String]`: borrowed slice of strings.
- `&mut Vec<String>`: borrowed vector that can be changed.
- Lifetimes on `longest`.
- Returning references that point into input data.

## Function Hints

`first_word`

`split_whitespace()` breaks text into words and skips extra spaces.

```rust
text.split_whitespace().next()
```

That returns an `Option<&str>`, so decide what to return when there is no first word.

`normalize_name`

The function receives `&mut String`, so it must change the existing string.

Useful methods:

- `.trim()` returns borrowed text without surrounding whitespace.
- `.to_lowercase()` returns a new `String`.

You can assign the final value back into `*name`.

`join_words`

The input is borrowed, so do not consume the vector. A slice of `String` has a `.join(" ")` method.

`longest`

Compare `.len()` values. If the lengths are equal, return `left`.

The lifetime annotation already exists in the signature. Your implementation can focus on the `if`.

`remove_empty`

Use `.retain(...)` to keep only values that should remain.

```rust
values.retain(|value| /* true means keep */);
```

`.trim().is_empty()` detects strings that are empty or only whitespace.

