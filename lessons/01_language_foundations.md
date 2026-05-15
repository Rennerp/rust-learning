# 01 - Language Foundations

This lesson explains the everyday building blocks of Rust programs. If syntax feels unfamiliar, read `00_start_here_reading_rust.md` first.

## Variables And Mutability

Variables are immutable by default:

```rust
let name = "Ferris";
```

This means `name` cannot be assigned a different value later. Rust does this because immutable data is easier to reason about and safer to share.

Use `mut` when a binding needs to change:

```rust
let mut count = 0;
count += 1;
```

Use `mut` deliberately. A variable that changes over time is harder to reason about than one that does not.

Rust also supports shadowing, which creates a new binding with the same name:

```rust
let input = "42";
let input = input.parse::<i32>().unwrap();
```

Shadowing is useful when transforming data from one type to another.

This is different from `mut`. With shadowing, you create a new variable. With `mut`, you change the existing variable.

## Types

Rust is statically typed. Every value has a type, and the compiler checks types before the program runs.

Common scalar types:

- Integers: `i32`, `u32`, `usize`, etc.
- Floating point: `f32`, `f64`.
- Boolean: `bool`.
- Character: `char`.

Common compound types:

- Tuple: `(i32, bool, char)`.
- Array: `[i32; 3]`.
- Vector: `Vec<i32>`.
- String slice: `&str`.
- Owned string: `String`.

`String` owns growable UTF-8 text. `&str` is a borrowed view into UTF-8 text.

Start with these defaults:

- Use `i32` for ordinary whole numbers.
- Use `usize` for indexes and lengths.
- Use `f64` for ordinary decimal numbers.
- Use `String` when storing or changing text.
- Use `&str` when reading text passed into a function.

## Functions And Expressions

Rust functions declare parameter and return types:

```rust
fn double(value: i32) -> i32 {
    value * 2
}
```

The final expression has no semicolon, so it becomes the return value. You can also write `return value * 2;`, but Rust style usually prefers the final expression.

Blocks are expressions. The final expression without a semicolon becomes the return value:

```rust
fn absolute(value: i32) -> i32 {
    if value < 0 {
        -value
    } else {
        value
    }
}
```

This is why `if` can be used to assign a value:

```rust
let status = if score >= 50 {
    "pass"
} else {
    "fail"
};
```

Both branches must produce the same type. Here both branches produce `&str`.

## Control Flow

`if` is an expression:

```rust
let label = if score >= 50 { "pass" } else { "fail" };
```

Loops:

```rust
for number in 1..=5 {
    println!("{number}");
}

while condition() {
    do_work();
}

loop {
    break;
}
```

Use `for` first when possible. It is usually clearer and avoids index mistakes:

```rust
let names = ["Ada", "Grace", "Linus"];

for name in names {
    println!("{name}");
}
```

You can use ranges for repeated work:

```rust
for number in 0..3 {
    println!("{number}");
}
```

This prints `0`, `1`, and `2`. The end of `0..3` is excluded.

## Collections

Vectors are growable arrays:

```rust
let mut numbers = Vec::new();
numbers.push(10);
numbers.push(20);
```

You can also create a vector with the `vec!` macro:

```rust
let numbers = vec![10, 20, 30];
```

Accessing an element can be done two ways:

```rust
let first = numbers[0];
let maybe_first = numbers.get(0);
```

`numbers[0]` panics if the index does not exist. `numbers.get(0)` returns `Option<&i32>`, which forces you to handle missing values.

Iterators are central in Rust:

```rust
let total: i32 = numbers.iter().sum();
```

Use `.iter()` when borrowing elements, `.into_iter()` when consuming a collection, and `.iter_mut()` when mutating elements.

Beginner examples:

```rust
let numbers = vec![1, 2, 3];

for number in numbers.iter() {
    println!("{number}");
}

let doubled: Vec<i32> = numbers.iter().map(|number| number * 2).collect();
```

The `|number| number * 2` part is a closure, which is an inline function.

## Pattern Matching

`match` forces you to handle every possible case:

```rust
fn describe(number: i32) -> &'static str {
    match number {
        0 => "zero",
        1..=9 => "small",
        _ => "large",
    }
}
```

This exhaustiveness is one of Rust's strongest safety features.

You will use `match` heavily with `Option` and `Result`:

```rust
let maybe_number = "42".parse::<i32>();

match maybe_number {
    Ok(number) => println!("parsed {number}"),
    Err(error) => println!("could not parse: {error}"),
}
```

## Comments

Use `//` for ordinary comments:

```rust
// Convert Fahrenheit to Celsius.
let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
```

Use comments to explain why code exists, not to repeat what the next line already says.

## Modules In One Minute

Rust code is organized into modules and crates.

- A crate is a compiled Rust unit.
- A package is a Cargo project.
- A module groups code inside a crate.
- `pub` makes an item visible outside its module.
- `use` brings a path into scope.

You will not need complex module structure for the first exercises, but you will see `pub fn` because tests need to call your functions.

```rust
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

`pub` means code outside this module can call `add`.
