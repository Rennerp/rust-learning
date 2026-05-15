# 02 - Ownership And Borrowing

Ownership is the central Rust concept. It is the part of Rust that feels most different when you are new.

Most programming languages hide memory management from you or ask you to manage it manually. Rust uses ownership rules checked by the compiler. That lets Rust free memory automatically without a garbage collector.

## The Rules

1. Each value has one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

```rust
{
    let message = String::from("hello");
}
// message was dropped here
```

## Move

Most owned values move by default:

```rust
let first = String::from("hello");
let second = first;
// first can no longer be used
```

The bytes were not copied. Ownership moved from `first` to `second`.

This fails:

```rust
let first = String::from("hello");
let second = first;
println!("{first}");
```

After the move, `first` no longer owns the string. Rust prevents using `first` because that would risk using data after it was freed.

## Copy

Simple stack-only values often implement `Copy`:

```rust
let a = 10;
let b = a;
println!("{a} {b}");
```

Both values remain usable because `i32` is cheap to copy.

Common `Copy` types include integers, booleans, floats, chars, and tuples containing only `Copy` types. `String` is not `Copy`.

## Borrowing

Borrowing lets code use a value without taking ownership:

```rust
fn length(text: &String) -> usize {
    text.len()
}
```

Prefer `&str` over `&String` for text inputs:

```rust
fn length(text: &str) -> usize {
    text.len()
}
```

`&str` accepts string literals, `String`, and string slices.

Borrowing is how you avoid moving data into every function.

```rust
fn print_length(text: &str) {
    println!("{}", text.len());
}

let message = String::from("hello");
print_length(&message);
println!("{message}");
```

This works because `print_length` only borrowed the text. `message` remains usable afterward.

## Mutable Borrowing

Use `&mut` to mutate borrowed data:

```rust
fn push_period(text: &mut String) {
    text.push('.');
}
```

The key rule:

- You can have many immutable references.
- Or exactly one mutable reference.
- But not both at the same time.

That rule prevents data races and iterator invalidation.

This is allowed:

```rust
let mut message = String::from("hello");
let borrowed = &mut message;
borrowed.push('!');
```

This is not allowed:

```rust
let mut message = String::from("hello");
let read = &message;
let write = &mut message;
println!("{read}");
```

Rust rejects this because one part of the code is reading while another part could change the same value.

## Slices

A slice is a borrowed view into part of a collection:

```rust
let text = String::from("hello world");
let hello = &text[0..5];
```

String slicing must happen on valid UTF-8 boundaries. For text processing, prefer iterator methods such as `.chars()` and `.split_whitespace()`.

The most common slice types are:

- `&str`: a borrowed slice of text.
- `&[T]`: a borrowed slice of a list of `T` values.

Example:

```rust
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
```

This function can accept a vector slice or an array slice.

## Lifetimes In Plain Language

A lifetime is the amount of code where a reference is valid.

Most of the time Rust infers lifetimes, so you do not write them. You write lifetimes when a function returns a reference and Rust needs to know which input that reference came from.

```rust
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
```

Read `<'a>` as "there is some lifetime called `a`." This signature says the returned reference will be valid as long as both input references are valid.

## Practical Advice

When writing a function:

- Use owned parameters when the function must keep the value.
- Use shared references when the function only reads.
- Use mutable references when the function modifies existing data.
- Return owned values when creating new data.

Avoid cloning to silence the compiler until you understand what ownership the program actually needs.

Cloning is fine when you intentionally need a new owned copy. The bad habit is cloning only because a borrow or move error appeared.

## Choosing A Parameter Type

Use this as a beginner guide:

- `text: String`: the function takes ownership of the string.
- `text: &str`: the function reads borrowed text.
- `text: &mut String`: the function modifies an existing string.
- `numbers: Vec<i32>`: the function takes ownership of the vector.
- `numbers: &[i32]`: the function reads borrowed numbers.
- `numbers: &mut Vec<i32>`: the function modifies an existing vector.
