# Option vs Result in Rust

## `Option<T>`

- Represents a value that may or may not be present
- Variants
  - `Some(value)` -> Has Something
  - `None` -> Has Nothing

> Use when absence of a value is **not necessarily an error**

## `Result<T, E>`

- Represents either a success or failure
- Variants
  - `Ok(value)` -> Successful result
  - `Err(msg)` -> The error that occured

> Use when there is a chance of **failure with meaningful information**

## `.unwrap()` and `.expect()`

- `.unwrap()` extracts the value from an `Option` but **panics** on `None`
- `.expect()` same for unwrap but with a custom error message for `Result`

Risk: Panics crash the program, making them unsafe in production unless you’re absolutely sure failure is impossible/really should be caught.

## To `panic` or not to `panic`

panic!

- Immediately stops the program with an error message.
- Use when the error is unrecoverable or indicates a serious bug (e.g., corrupted state, impossible logic branch, programmer mistake).

Example: indexing past the end of an array.

Result<T, E>

- A type that represents either success (Ok) or failure (Err).
- Use when the error is expected and recoverable (e.g., file not found, invalid user input, network timeout).

Lets the caller decide how to handle the failure.

## Error Propagation with ?

The ? operator:

- On `Option`: returns None early if the value is None.
- On `Result`: returns the Err early if it’s an error.

It’s shorthand for early-return handling, so you don’t need to write out a full match.

Helps keep functions concise while still handling errors

```rust
fn unwraps_enum(e: Option<i32>)

```

## How does this actually work?

To get further clarification on what expect, unwrap and ? actually DO, take a look at [main.rs](./src/main.rs) for more details 

## Practice Classification (`Option`, `Result`, or `panic`)

1. Looking up a key in a hashmap
2. Opening a file from disk
3. Parsing a user's input from a string
4. Getting the second element of an empty array
5. Checking if a temperature is above 100
6. Dividing two integers
7. Asserting that a list is of length 3
