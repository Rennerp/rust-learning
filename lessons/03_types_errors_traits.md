# 03 - Types, Errors, And Traits

Rust encourages you to model your program's rules directly in types. Good types make invalid states harder to represent.

## Structs

Structs group related data:

```rust
struct User {
    id: u64,
    email: String,
    active: bool,
}
```

Create a struct value like this:

```rust
let user = User {
    id: 1,
    email: String::from("user@example.com"),
    active: true,
};
```

Access fields with `.`:

```rust
println!("{}", user.email);
```

Methods live in `impl` blocks:

```rust
impl User {
    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

Method receivers:

- `&self`: read the value.
- `&mut self`: modify the value.
- `self`: take ownership of the value.

Associated functions do not take `self`. Constructors commonly use `new`:

```rust
impl User {
    fn new(id: u64, email: String) -> Self {
        Self {
            id,
            email,
            active: true,
        }
    }
}
```

## Enums

Enums represent a value that can be one of several variants:

```rust
enum PaymentStatus {
    Pending,
    Paid { receipt_id: String },
    Failed(String),
}
```

Enums can carry data, which makes them stronger than many traditional enum systems.

Use `match` to handle enum variants:

```rust
fn describe(status: PaymentStatus) -> String {
    match status {
        PaymentStatus::Pending => String::from("pending"),
        PaymentStatus::Paid { receipt_id } => format!("paid: {receipt_id}"),
        PaymentStatus::Failed(reason) => format!("failed: {reason}"),
    }
}
```

## Option

Use `Option<T>` for values that might be missing:

```rust
fn first(values: &[i32]) -> Option<i32> {
    values.first().copied()
}
```

`Option<T>` is either `Some(value)` or `None`.

Use `Option` instead of null. Rust has no ordinary null value.

```rust
let first = values.first();

match first {
    Some(value) => println!("first value is {value}"),
    None => println!("there is no first value"),
}
```

## Result

Use `Result<T, E>` for operations that can fail:

```rust
fn parse_age(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse::<u8>()
}
```

`Result<T, E>` is either `Ok(value)` or `Err(error)`.

Use `Result` when failure is expected and the caller should decide what to do.

The `?` operator returns early on error:

```rust
fn parse_and_double(input: &str) -> Result<u8, std::num::ParseIntError> {
    let value = input.parse::<u8>()?;
    Ok(value * 2)
}
```

Read this as:

1. Try to parse the input.
2. If parsing succeeds, put the parsed number in `value`.
3. If parsing fails, return the error immediately.
4. If parsing succeeds, return `Ok(value * 2)`.

Do not use `unwrap()` in real error-handling code while learning. `unwrap()` means "crash if this is an error or missing value." It is acceptable in quick examples and some tests, but it hides the error-handling lesson.

## Traits

Traits define shared behavior:

```rust
trait Summary {
    fn summary(&self) -> String;
}
```

Implement a trait for a type:

```rust
struct Article {
    title: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        self.title.clone()
    }
}
```

Traits are Rust's main abstraction mechanism. They power formatting, parsing, comparison, iteration, conversion, and async ecosystems.

Examples from the standard library:

- `Debug`: lets a value print with `{:?}`.
- `Display`: lets a value print with `{}`.
- `Clone`: lets you call `.clone()`.
- `Iterator`: powers loops and iterator methods.
- `FromStr`: parses a value from text.

## Generics

Generics let functions and types work with multiple concrete types:

```rust
fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}
```

Read `T` as "some type chosen by the caller." This function works for `&[i32]`, `&[String]`, `&[User]`, and many other slices.

Trait bounds constrain generic types:

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summary());
}
```

This says `print_summary` accepts any `T` as long as `T` implements `Summary`.

## Derives

Many common traits can be generated:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
```

Common derives:

- `Debug`: print with `{:?}`.
- `Clone`: explicit deep copy.
- `Copy`: implicit cheap copy.
- `PartialEq`, `Eq`: equality.
- `PartialOrd`, `Ord`: ordering.
- `Default`: default value.

## Visibility

Items are private by default. Use `pub` to make them visible outside the module:

```rust
pub struct User {
    pub id: u64,
    email: String,
}
```

Here `User` and `id` are public, but `email` is private.

## Type-Driven Design

Prefer this:

```rust
enum Role {
    Admin,
    Member,
    Guest,
}
```

Over this:

```rust
let role = "admin";
```

The enum lets the compiler check every role case. A string can contain typos, unsupported roles, or inconsistent casing.
