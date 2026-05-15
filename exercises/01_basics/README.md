# Exercise 01 - Basics

This exercise practices the building blocks from `lessons/01_language_foundations.md`.

Run only this exercise:

```sh
cargo test -p exercise_01_basics
```

Run one test at a time:

```sh
cargo test -p exercise_01_basics converts_fahrenheit_to_celsius
```

## Concepts Used

- `f64` for decimal numbers.
- `u64` and `u32` for positive whole numbers.
- `String` for owned text returned from a function.
- `&str` for borrowed text input.
- `&[i32]` for a borrowed slice of numbers.
- `if`, `else if`, and `else`.
- `for` loops.
- Iterator methods such as `.iter()`, `.filter()`, and `.sum()`.

## Function Hints

`fahrenheit_to_celsius`

Use the formula:

```text
(fahrenheit - 32) * 5 / 9
```

Use decimal numbers such as `32.0`, `5.0`, and `9.0` because the input is `f64`.

`factorial`

Factorial means multiplying all numbers from `1` to `number`.

```text
5! = 1 * 2 * 3 * 4 * 5
```

By definition, `0!` is `1`.

`fizzbuzz`

Rules:

- Divisible by 15 returns `"FizzBuzz"`.
- Divisible by 3 returns `"Fizz"`.
- Divisible by 5 returns `"Buzz"`.
- Otherwise return the number as text.

Use `%` for remainder:

```rust
number % 3 == 0
```

`count_vowels`

Loop over `text.chars()`. Use `matches!` or `match` to count `a`, `e`, `i`, `o`, and `u` in lowercase and uppercase.

`sum_even_numbers`

An even number has no remainder when divided by 2:

```rust
number % 2 == 0
```

You can use a loop or iterators. Start with a loop if iterators feel unfamiliar.

