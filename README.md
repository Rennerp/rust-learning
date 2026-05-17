# Rust Learning Workspace

This repository is a hands-on Rust curriculum for a complete beginner. It teaches Rust through short lessons, test-driven exercises, and small command-line projects.

Start with the syntax lesson, then work through one lesson and one exercise at a time. The exercises intentionally contain `todo!()` so the tests fail until you implement the missing code.

## 1. Install Rust

Rust is installed with `rustup`. It gives you the compiler, Cargo, formatter, linter, and local documentation.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal, then check the install:

```sh
rustc --version
cargo --version
```

This repo also includes [rust-toolchain.toml](./rust-toolchain.toml), so Cargo will use the stable Rust toolchain with `rustfmt` and `clippy`.

## 2. Learn The Layout

The repo is a Cargo workspace. The top-level [Cargo.toml](./Cargo.toml) lists all exercises and projects as workspace members.

Important folders:

- [lessons/](./lessons/) contains beginner-friendly explanations.
- [exercises/](./exercises/) contains test-driven practice crates.
- [projects/](./projects/) contains small runnable programs.

## 3. Follow The Learning Path

Work in this order:

1. [lessons/00_start_here_reading_rust.md](./lessons/00_start_here_reading_rust.md)
   Learn how to read Rust syntax: `fn`, `let`, `mut`, types, blocks, loops, `match`, references, generics, macros, and common symbols.

2. [lessons/00_setup_and_cargo.md](./lessons/00_setup_and_cargo.md)
   Learn the Rust toolchain, Cargo, crates, packages, formatting, and tests.

3. [lessons/01_language_foundations.md](./lessons/01_language_foundations.md)
   Learn variables, mutability, types, functions, expressions, control flow, collections, and basic pattern matching.

4. [exercises/01_basics](./exercises/01_basics/README.md)
   Practice numbers, strings, loops, vectors, conditionals, and simple functions.

5. [lessons/02_ownership_borrowing.md](./lessons/02_ownership_borrowing.md)
   Learn ownership, moves, copies, borrowing, mutable references, slices, and practical lifetimes.

6. [exercises/02_ownership](./exercises/02_ownership/README.md)
   Practice borrowing data instead of moving or cloning unnecessarily.

7. [lessons/03_types_errors_traits.md](./lessons/03_types_errors_traits.md)
   Learn structs, enums, methods, `Option`, `Result`, traits, generics, derives, and visibility.

8. [exercises/03_types_and_errors](./exercises/03_types_and_errors/README.md)
   Practice modeling data and handling invalid or missing values.

9. [lessons/04_rust_language_map.md](./lessons/04_rust_language_map.md)
   Use this as a checklist for the rest of the language.

10. [lessons/05_glossary.md](./lessons/05_glossary.md)
   Use this whenever a Rust word is unfamiliar.

11. [projects/01_guessing_game](./projects/01_guessing_game/)
   Read, run, and extend a small command-line guessing game.

12. [projects/02_todo_cli](./projects/02_todo_cli/)
   Read, run, and extend a small command-line todo manager.

## 4. Work On Exercises

Each exercise is a small library crate with tests. The tests describe the behavior you need to implement.

Run all tests:

```sh
cargo test
```

Run one exercise package:

```sh
cargo test -p exercise_01_basics
```

Run one test by name:

```sh
cargo test -p exercise_01_basics converts_fahrenheit_to_celsius
```

Recommended exercise workflow:

1. Read the matching lesson.
2. Read the exercise README.
3. Open `src/lib.rs` for that exercise.
4. Pick one function with `todo!()`.
5. Read its signature and tests.
6. Implement only that function.
7. Run the matching test.
8. Repeat until the exercise package passes.

Do not delete tests. Do not solve every function at once. Small loops of edit, test, and read the compiler error are the point of the repo.

## 5. Run Projects

Run the guessing game:

```sh
cargo run -p project_01_guessing_game
```

Run the todo CLI:

```sh
cargo run -p project_02_todo_cli -- add "learn ownership"
cargo run -p project_02_todo_cli -- list
cargo run -p project_02_todo_cli -- done 1
```

Use [projects/README.md](./projects/README.md) to understand the important lines and choose extension ideas.

## 6. Use Cargo Tools

Format code:

```sh
cargo fmt
```

Run Clippy:

```sh
cargo clippy --all-targets --all-features
```

Open local Rust docs:

```sh
rustup doc
```

## 7. Read Compiler Errors

Rust compiler errors are part of the teaching material. When a command fails:

1. Read the first error first.
2. Look at the file and line number.
3. Read the short message after `error[...]`.
4. Read any `help:` text.
5. Fix one thing.
6. Run the same command again.

Later errors are often caused by the first error, so do not chase all of them at once.

## Beginner Rules

- Write small functions first.
- Run tests often.
- Prefer readable code over clever code.
- Do not use `.clone()` just to make an error disappear until you understand what moved.
- Do not use `.unwrap()` in library code unless the exercise specifically allows it.
- When confused, identify the type of each value first.
- Use the glossary when a term is unfamiliar.

## Suggested Study Routine

Use short sessions:

- 20 minutes reading a lesson.
- 40 minutes implementing exercises.
- 10 minutes reading compiler errors.
- 10 minutes writing notes about what confused you.

The goal is not to memorize Rust in one pass. The goal is to build a habit of reading types, making small changes, and letting the compiler guide the next step.
