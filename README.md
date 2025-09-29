# DS210 Intro to Rust – Midterm Concept Review

Use this as a loose study guide to walk through the main themes of the course so far.

## 1. Linux / Git / Cargo Basics

- **`cd`** → moves to a  directory.
- **`ls -la`** → lists all files (including hidden ones) in long format.
- **`cat`** → prints the contents of a file
- **`touch`** → initializes a new file

- **`git init`** creates a new git repository
- **`git clone <url>`** clones a github repository into a directory of the same name
- **git add vs git commit**
  - `git add` stages changes (puts them in the "index").
  - `git commit` records staged changes permanently in the repo.
- **git pull vs git push**
  - `git pull` = fetch + merge changes from remote to local.
  - `git push` = send your local commits to remote.
- **Cargo basics**
  - `cargo build` → compiles only
  - `cargo run` → compiles + runs
  - `cargo build --release` → optimize compile
  - `cargo run --release` → optimized compile + run
  - `cargo check` → quickly checks for errors (no binary output)

## 2. Rust Syntax and Types

- Rust functions require explicit type annotations on parameters.
- Tuples: access with `.0`, `.1`, etc.
- Arrays vs Vectors:
  - `[1,2,3]` is an array of fixed size.
  - `vec![1,2,3]` is a vector, growable at runtime.
- Range syntax: `1..5` excludes 5, `1..=5` includes 5.

## 3. Derive

- **`#[derive(Debug)]`** → enables `{:?}` printing.
- **`#[derive(PartialEq)]`** → enables `==` and `!=` comparisons.
- Other common derives: `Clone`, `Copy`, `Eq`
  - **Clone**: Allows a `struct` or `enum` to use the `.clone()` method.
  - **Copy**: Allows for a variable to be `copied` or assigned using `=` (Ex: `let copied = old_var`)
  - **Eq**: Requires `PartialEq`, types cannot contain floats

## 4. Error Handling

- **`panic!`** → unrecoverable errors, program crashes.
- **`Option<T>`**: use when something may or may not exist (Some/None).
- **`Result<T, E>`** → recoverable errors, caller can handle.
- **unwrap()**: quick way to assume `Ok`, crashes if `Err`.
- **`match`**: safe, explicit way to handle all Result/Option cases.
- **? operator**: propagates errors upward.

## 5. Pattern Matching

- `match` can destructure enums, tuples, and use guards.
- Example:

  ```rust
  match x {
      Some(v) if v > 10 => println!("big {}", v),
      Some(v) => println!("small {}", v),
      None => println!("none"),
  }
  ```

- Always make sure match arms cover all possibilities.
- Underscore `_` is a wildcard pattern, like `else`.

## 6. Enums

- Enums define finite choices (like Coin, Status, Result).
- Deriving traits on enums/structs allows equality, debugging, etc.
- Enums often paired with `match` for branching logic.

## 7. Control Flow

- `if/else` expressions return values.
- `loop`, `while`, `for` for iteration.
- `for x in 0..5 {}` iterates over ranges.
- `for item in array` or `for (index, value) in array.iter().enumerate()` to cleanly iterate over arrays.
- `break` exits a loop, `continue` skips to next iteration.

## 8. Functions and Return Values

- Functions must declare input and output types.
- Default return = `()` if nothing specified.
- `return x;` vs implicit last expression (no semicolon).
- Example:

  ```rust
  fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```

## 9. Common Bugs and Fixes

- Use `==` not `=` for equality tests.
- Variables are immutable unless declared `mut`.
- Don’t end the last expression in a function with `;` if you want it as return.
- Always cover all enum cases in `match`.
