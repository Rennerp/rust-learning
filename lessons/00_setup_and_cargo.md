# 00 - Setup And Cargo

## Rust Toolchain

Rust development usually uses these tools:

- `rustc`: the Rust compiler.
- `cargo`: the build tool, package manager, test runner, and project manager.
- `rustfmt`: automatic formatter.
- `clippy`: linter that catches common mistakes and non-idiomatic code.
- `rustup`: toolchain manager.

Install Rust with `rustup`, then restart your shell:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Check installation:

```sh
rustc --version
cargo --version
```

## Cargo Basics

Cargo works with packages and crates.

- A package is a project with a `Cargo.toml`.
- A crate is a compilation unit.
- A binary crate produces an executable.
- A library crate exposes reusable code.

Common commands:

```sh
cargo new hello_rust
cargo build
cargo run
cargo test
cargo fmt
cargo clippy
cargo doc --open
```

## Project Shape

A binary crate usually has:

```text
Cargo.toml
src/main.rs
```

A library crate usually has:

```text
Cargo.toml
src/lib.rs
```

This repository is a Cargo workspace. The top-level `Cargo.toml` lists multiple learning crates, so one `cargo test` can run every exercise.

## Tests

Rust tests usually live near the code:

```rust
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Run all tests:

```sh
cargo test
```

Run one package:

```sh
cargo test -p exercise_01_basics
```

## First Mental Model

Rust asks you to be explicit about:

- Whether data can change: immutable by default, `mut` when needed.
- Who owns data: one owner at a time.
- Who can temporarily access data: references and borrowing.
- What can fail: `Result`.
- What can be absent: `Option`.

The compiler is strict because it is proving safety properties before your program runs.

