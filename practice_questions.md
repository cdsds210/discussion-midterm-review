# Practice Coding Questions

For the following questions, write your solutions on pencil and paper. When complete, copy your results into a rust project and see if they work!

## Q1: Factorial With Error Handling

Write a function `factorial()` that for an input `n`:

- Takes input `n` (think about what type this should be)
- PANICS with an Error "Too Large" if `n > 20`
- Otherwise calculates the factorial of `n` recursively or iteratively

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

## Example Answers

### Q1

```rust
// NOTE: u64 or larger is required for 20!
// i64 technically works, but should not be used since we only take factorials
// of positive numbers

///Example recursive solution
fn factorial(n: u64) -> u64{
    if n > 20 {
        panic!("Too Large!");
    }

    if n == 0 {
        return 1;
    }

    return n * factorial(n -1);
}


/// Example iterative sol
fn factorial_iter(n: u64) -> u64{
    if n > 20 {
        panic!("Too Large")
    }

    let mut fact = 1;
    //INCLUSIVE range necessary
    for i in 1..=n{
        fact = fact * i;
    }

    return fact;
}
```

### Q2

```rust
enum Grade {
    Passing(u32),
    Failing(u32)
}

fn is_passing(grade: Grade) -> bool{
    return match grade{
        // NOTE: Using underscore here allows pattern matching despite value
        Grade::Passing(_) => true,
        // Grade::Passing(v) => true, IS ALSO CORRECT
        // Grade::Passing => true, IS NOT CORRECT
        Grade::Failing(_) => false
    }
}
```

### Q3

```rust
//usize is used because the index of iteration is of type `usize`
fn sum_positive_indices(arr: &[i32]) -> usize {
    let len = arr.len();
    let mut total = 0;
    for index in 0..len{
        let value = arr[index];

        if value > 0 {
            // NOTE: Sums by INDEX not value
            total += index;
        }
    }

    return total;
}

/// Cleaner implementation using .iter().enumerate()
fn sum_positive_indices_without_range(arr: &[i32]) -> usize {
    let mut total = 0;
    // NOTE TAKING THE REFERENCE OF VALUE for proper behavior
    for (index, &value) in arr.iter().enumerate(){
        // If used (index, value during iteration) this line has to be 
        // if value > &0
        if value > 0 {
            total += index;
        }
    }
    total
}
```

### Q4

```rust
enum BookStatus{
    Available,
    CheckedOut(String)
}

fn status_message(status: BookStatus){
    match status{
        BookStatus::Available => println!("This book is available"),
        BookStatus::CheckedOut(name) => println!("This book is currently checked out by {}", name),
    }
}
```
