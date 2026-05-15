# Projects

These projects are intentionally small. Their purpose is to connect language features to real programs.

Read the source slowly. For each unfamiliar line, identify:

- Is this defining a value, calling a function, calling a method, or matching a case?
- What type is being passed in?
- What type is coming back?
- Can this operation fail?

## 01 Guessing Game

Run:

```sh
cargo run -p project_01_guessing_game
```

Learning goals:

- Read from standard input.
- Parse strings into numbers.
- Use `match`.
- Handle invalid input.
- Use loops and `break`.

Important source lines:

```rust
use std::cmp::Ordering;
use std::io;
```

`use` brings names into scope so the code can write `Ordering` and `io` instead of full paths.

```rust
let mut guess = String::new();
```

Creates an empty, growable string. It is `mut` because `read_line` writes into it.

```rust
io::stdin().read_line(&mut guess)
```

Borrows `guess` mutably so standard input can fill it with user text.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(number) => number,
    Err(_) => continue,
};
```

Trims whitespace, tries to parse a number, and handles both success and failure. `parse()` returns a `Result`.

```rust
match guess.cmp(&secret) {
    Ordering::Less => println!("Too small."),
    Ordering::Greater => println!("Too large."),
    Ordering::Equal => break,
}
```

Compares the guess with the secret number. `cmp` returns an enum, so `match` handles each variant.

Possible improvements:

- Generate a random secret number with the `rand` crate.
- Let the user choose the range.
- Limit the number of attempts.
- Track previous guesses.

Suggested order:

1. Add a maximum attempt count.
2. Print previous guesses.
3. Ask the user whether to play again.
4. Add random number generation after learning dependencies in Cargo.

## 02 Todo CLI

Run:

```sh
cargo run -p project_02_todo_cli -- add "learn ownership"
cargo run -p project_02_todo_cli -- list
cargo run -p project_02_todo_cli -- done 1
```

Learning goals:

- Read command-line arguments.
- Use `Result` for fallible operations.
- Read and write files.
- Model command behavior with `match`.

Important source lines:

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
```

`main` delegates to `run`, then handles any error in one place. `if let` is a shorter way to match one pattern.

```rust
let args: Vec<String> = env::args().collect();
```

Reads command-line arguments into an owned vector of strings.

```rust
match args.get(1).map(String::as_str) {
```

Looks at the first command after the program name. `get(1)` returns `Option<&String>`. `map(String::as_str)` turns it into `Option<&str>`.

```rust
let todos = load_todos().map_err(|error| error.to_string())?;
```

Loads todos from disk. The `?` returns early if file reading fails.

```rust
fs::write(TODO_FILE, todos.join("\n"))
```

Joins all todos with newline characters and writes the result to the file.

Possible improvements:

- Store todo completion state instead of deleting completed items.
- Add due dates.
- Use JSON with `serde`.
- Split the program into modules.
- Add tests for parsing commands.

Suggested order:

1. Add a `clear` command.
2. Add a `count` command.
3. Store each todo as `pending:text` or `done:text`.
4. Move todo loading and saving into a library crate.
5. Add tests for the library functions.

## When A Project Feels Too Hard

Drop back to the matching exercise. Projects combine several concepts at once, while exercises isolate one concept. That back-and-forth is normal.
