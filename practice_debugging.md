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

Fix: the function tries to return `Err(...)` but the signature says `-> u64`. Change the signature to `-> Result<u64, &'static str>` and wrap returns with `Ok(...)`, or remove the `Err` branch and handle limits differently.

---
