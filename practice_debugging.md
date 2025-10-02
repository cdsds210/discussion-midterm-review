# Quick practice: very short buggy snippets

Copy a snippet into `src/main.rs`, run `cargo run`, and fix it together.

---

## A

```rust
enum Light { Red, Yellow, Green }

fn action(l: Light) {
    match l {
        Light::Red => println!("stop"),
        Light::Green => println!("go"),
    }
}

fn main() { action(Light::Yellow); }
```

---

## B

```rust
fn main() {
    let x = 1;
    if x = 2 {
        println!("yes");
    }
}
```

---

## C

```rust
fn fibonacci(n: u8) -> u8 {
    if n > 20 {
        return Err("Too Large");
    }
    if n == 0 { return 1; }
    // Yes, this is slow, but that's not the bug
    fibonacci(n - 1) + fibonacci(n-2);
}

fn main() {
    println!("fib(5) = {}", fibonacci(5));
}
```

## Answers

- A: Unmatched Yellow Branch. Add a branch for `Light::Yellow => println!("Wait!")`
- B: Use `x == 2` for comparison
- C: To keep the return `u8`, Instead of returning `Err` the function should `panic!()` instead. Remove the semicolon/add a return statement to the last line
  - Alternatively, modify more of the function to return `Result<u8, String>` (`Err(String::from("Too Large!)))`, `if n == 0 {return Ok(1)}`, and `return fibonacci(n-1)? + fibonacci(n-2)?`)
  - Bonus stylistic bug: `u8` is WAY too small to contain the 20th fibonacci number. What type would be the smallest that allows this?

---
