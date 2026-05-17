# 00 - Start Here: Reading Rust Syntax

This lesson teaches you how to look at Rust code without getting lost. You do not need to memorize everything now. The goal is to recognize the shapes.

## A Tiny Rust Program

```rust
fn main() {
    let name = "Rust";
    println!("Hello, {name}!");
}
```

Read it like this:

- `fn` means "define a function".
- `main` is the function Rust runs first in a binary program.
- `()` means this function takes no inputs.
- `{ ... }` contains the function body.
- `let name = "Rust";` creates a variable named `name`.
- `println!` prints text to the terminal.
- The `!` means `println!` is a macro, not a normal function.
- `;` ends a statement.

## Statements And Expressions

Rust uses both statements and expressions.

A statement does something and usually ends with `;`:

```rust
let count = 3;
```

An expression produces a value:

```rust
3 + 4
```

The last expression in a block can become the block's value if it has no semicolon:

```rust
fn double(number: i32) -> i32 {
    number * 2
}
```

This returns `number * 2`. If you write `number * 2;`, the function would not return that value.

## Function Syntax

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

Read the signature:

- `fn add`: define a function named `add`.
- `left: i32`: first parameter is named `left` and has type `i32`.
- `right: i32`: second parameter is named `right` and has type `i32`.
- `-> i32`: this function returns an `i32`.
- `left + right`: final expression, so it is returned.

Rust usually wants function parameter and return types written explicitly. That makes APIs easier to understand and helps the compiler check your program.

## Variables

```rust
let score = 10;
```

This creates an immutable variable. Immutable means you cannot change it later.

```rust
let mut score = 10;
score = 11;
```

`mut` means mutable. Use it only when the value really needs to change.

## Type Annotations

Rust can often infer types:

```rust
let score = 10;
```

You can also write the type:

```rust
let score: i32 = 10;
```

The `:` means "has type".

You will see `:` in several places:

```rust
let age: u8 = 30;
fn birthday(age: u8) -> u8 { age + 1 }
```

Both mean "this name has this type".

## Common Types

```rust
let whole_number: i32 = -10;
let positive_number: u32 = 10;
let size: usize = 5;
let decimal: f64 = 3.14;
let is_ready: bool = true;
let letter: char = 'R';
let borrowed_text: &str = "hello";
let owned_text: String = String::from("hello");
```

Useful rules:

- `i` integer types can be negative: `i32`, `i64`.
- `u` integer types cannot be negative: `u32`, `usize`.
- `usize` is commonly used for indexes and lengths.
- `f64` is the default decimal type.
- `char` uses single quotes.
- `&str` is borrowed text.
- `String` is owned, growable text.

## `String` Versus `&str`

This is one of the first confusing Rust topics.

```rust
let literal = "hello";
let owned = String::from("hello");
```

`"hello"` is a string literal. Its type is `&str`, which means borrowed text.

`String::from("hello")` creates an owned string. It can grow:

```rust
let mut message = String::from("hello");
message.push_str(" world");
```

Beginner rule:

- Use `&str` for function inputs when you only need to read text.
- Use `String` when you need to own, store, or modify text.

## Blocks And Scope

A scope is an area where a variable exists.

```rust
fn main() {
    let outer = 1;

    {
        let inner = 2;
        println!("{outer} {inner}");
    }

    // inner is not available here
}
```

Variables live until the end of their scope. This matters because Rust frees owned values automatically when their owner leaves scope.

## `if` Syntax

```rust
let score = 80;

if score >= 50 {
    println!("pass");
} else {
    println!("fail");
}
```

Conditions must be `bool`. Rust does not treat `0`, empty strings, or empty lists as false.

`if` can produce a value:

```rust
let label = if score >= 50 { "pass" } else { "fail" };
```

Both branches must produce the same type.

## Loop Syntax

Use `for` when iterating over a range or collection:

```rust
for number in 1..=3 {
    println!("{number}");
}
```

`1..=3` means 1 through 3, including 3.

`1..3` means 1 through 2, excluding 3.

Use `while` when repeating while a condition is true:

```rust
let mut count = 3;

while count > 0 {
    println!("{count}");
    count -= 1;
}
```

Use `loop` when you want an infinite loop and will `break` manually:

```rust
loop {
    break;
}
```

## Match Syntax

`match` compares one value against patterns:

```rust
fn describe(number: i32) -> &'static str {
    match number {
        0 => "zero",
        1..=9 => "small",
        _ => "large",
    }
}
```

Read it like this:

- `match number`: inspect `number`.
- `0 => "zero"`: if it is exactly `0`, return `"zero"`.
- `1..=9 => "small"`: if it is from 1 to 9, return `"small"`.
- `_ => "large"`: `_` catches everything else.

Rust checks that `match` covers every possible case.

## Methods, Functions, And `::`

Normal function call:

```rust
add(2, 3)
```

Method call on a value:

```rust
message.len()
```

Associated function call with `::`:

```rust
String::from("hello")
Vec::new()
```

Use this beginner rule:

- `value.method()` means "do this using this value".
- `Type::function()` means "call a function attached to this type".

## References: `&`

`&` means borrow.

```rust
fn length(text: &str) -> usize {
    text.len()
}

let name = String::from("Rust");
let size = length(&name);
```

`length(&name)` lets the function read `name` without taking ownership of it.

`&mut` means mutable borrow:

```rust
fn add_period(text: &mut String) {
    text.push('.');
}
```

The ownership lesson explains this in detail. For now, read `&T` as "borrowed T" and `&mut T` as "borrowed mutable T".

## Generics: `<T>`

Angle brackets often mean a type goes inside another type:

```rust
Vec<i32>
Option<String>
Result<u32, String>
```

Read them as:

- `Vec<i32>`: a vector containing `i32` values.
- `Option<String>`: either a `String` or nothing.
- `Result<u32, String>`: either a successful `u32` or an error `String`.

You may also see:

```rust
input.parse::<u32>()
```

The `::<u32>` tells Rust exactly what type to parse into. This syntax is nicknamed the turbofish.

## Attributes: `#[...]`

Attributes add instructions for the compiler or tools.

```rust
#[derive(Debug, Clone)]
struct User {
    name: String,
}
```

`derive` asks Rust to generate common behavior.

Tests use attributes too:

```rust
#[test]
fn adds_numbers() {
    assert_eq!(2 + 2, 4);
}
```

## Macros: `!`

Macros look like function calls but have `!`:

```rust
println!("hello");
vec![1, 2, 3];
assert_eq!(left, right);
```

You do not need to understand how macros are built yet. Just recognize the `!`.

## Important Symbols

- `:` means "has type" in variables and function parameters.
  Example: `let score: i32 = 10;` and `fn double(value: i32) -> i32 { value * 2 }`
- `->` means "returns".
  Example: `fn name() -> &'static str { "Rust" }`
- `::` accesses something inside a type or module.
  Example: `String::from("hello")`
- `.` calls a method or accesses a field.
  Example: `message.push_str(" world")` or `point.x`
- `&` borrows a value.
  Example: `fn print_name(name: &str) { println!("{name}"); }`
- `&mut` mutably borrows a value.
  Example: `fn add_exclamation(text: &mut String) { text.push('!'); }`
- `!` marks a macro call or the never type in advanced code.
  Example: `println!("hello");`
- `?` returns early if a `Result` is an error.
  Example: `let text = std::fs::read_to_string("notes.txt")?;`
- `_` means "ignore this" or "catch everything".
  Example: `let (name, _) = ("Rust", 2015);`
- `=>` connects a pattern to a result in `match`.
  Example: `match score { 0 => "zero", _ => "non-zero" }`
- `..` and `..=` create ranges.
  Example: `1..4` means 1 through 3, and `1..=4` means 1 through 4.
- `<T>` means a generic type parameter.
  Example: `fn first<T>(items: &[T]) -> &T { &items[0] }`

## How To Read Compiler Errors

When Rust reports an error:

1. Read the first error first.
2. Look at the file and line number.
3. Read the sentence after `error[...]`.
4. Read the hint after `help:` if there is one.
5. Fix one error and run the command again.

Do not try to fix every error at once. Later errors are often caused by the first one.

