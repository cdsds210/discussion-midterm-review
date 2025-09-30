# Practice Coding Questions

For the following questions, write your solutions on pencil and paper. When complete, copy your results into a rust project and see if they work!

## Q1: Factorial With Error Handling

Write a function `factorial()` that for an input `n`:

- Takes input `n` (think about what type this should be)
- RETURNS an Error "Too Large" if `n > 20`
- Otherwise calculates the factorial of `n` recursively

> Thought question: If `fn main() -> {}` uses factorial should it use `.expect()`, `.unwrap()`, or `?`

## Q2: Student Grading

Define an enum `Grade` with two possible fields, `Failing` and `Passing`. Both variants should include the student's grade in a `u32`

Implement a function `is_passing` that takes a grade and returns a boolean

## Q3: Iteration practice

Write a function `sum_positive_indices` that given an array of `i32` inputs of unspecified size, returns the sum of the **indices** of the positive values in the array

Ex: `[-1, 0, 2, -2, 3] -> 1 + 2 + 4 = 7`

Bonus: Can you do this without using a range? (`0..5`)

## Q4: Library Tracker

For an enum

```rust
enum BookStatus{
    Available,
    CheckedOut(String)
}
```

Implement a function `status_message`:

- `status_message(BookStatus::Available)` -> "This book is available!"
- `status_message(BookStatus::CheckedOut(String::from("Alice")))` -> "This book is currently checked out by Alice."
