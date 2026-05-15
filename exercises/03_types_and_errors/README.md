# Exercise 03 - Types And Errors

This exercise practices modeling data with structs and enums, then handling absence and failure explicitly.

Run only this exercise:

```sh
cargo test -p exercise_03_types_and_errors
```

Run one test at a time:

```sh
cargo test -p exercise_03_types_and_errors validates_user_email
```

## Concepts Used

- `struct` for grouped data.
- `enum` for a fixed set of variants.
- `impl` for methods and associated functions.
- `Result<T, E>` for validation that can fail.
- `Option<T>` for searches that might find nothing.
- Iterator methods such as `.find()`, `.filter()`, `.map()`, and `.collect()`.
- `matches!` for concise enum checks.

## Function Hints

`User::new`

Return `Err(UserError::EmptyEmail)` if the email is empty.

Return `Err(UserError::MissingAtSign)` if the email does not contain `@`.

Otherwise return:

```rust
Ok(Self { id, email, role })
```

`can_delete_users`

Only `Role::Admin` should return `true`.

This can be solved with `match` or `matches!`.

`parse_role`

Use `.to_lowercase()` or `.eq_ignore_ascii_case(...)`.

Return:

- `Some(Role::Admin)`
- `Some(Role::Member)`
- `Some(Role::Guest)`
- `None`

`find_user`

Use `.iter().find(...)`.

This returns `Option<&User>`, which matches the function signature.

`admin_emails`

Filter users by role, then map each admin to a cloned email.

You need owned `String` values in the returned `Vec<String>`, so cloning each selected email is appropriate here.

