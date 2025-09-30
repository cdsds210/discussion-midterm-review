fn main() {
    println!("Hello, world!");
}

/// A basic function that takes two Options, and sums them
fn sum_options(option1: Option<i32>, option2: Option<i32>) -> i32 {
    // Do this manually using match
    unimplemented!();
}

fn uses_unwrap(option1: Option<i32>, option2: Option<i32>) -> i32 {
    // Will cause a panic! with a default error msg if None
    return option1.unwrap() + option2.unwrap();
}

// What if we want specific error messages for which option doesn't have a value? 
fn manual_expect(option1: Option<i32>, option2: Option<i32>) -> i32 {
    let v1 = match option1{
        Some(v) => v,
        None => panic!("Option1 was None"),
    };
    
    let v2 = match option2{
        Some(v) => v,
        None => panic!("Option 2 was None"),
    };

    return v1 + v2;
}

fn uses_expect(option1: Option<i32>, option2: Option<i32>) -> i32 {
    return option1.expect("Option1 was None") + option2.expect("Option2 was None");
}


fn uses_propagation(option1: Option<i32>, option2: Option<i32>) -> Option<i32> {
    // Maybe we don't want this function fail immediately, instead this will just pass the option up
    let v1 = option1?;
    let v2 = option2?;
    return Some(v1 + v2)
    // Alterantively in a one liner
    // return Some(option1? + option?)
}

fn manual_propagation(option1: Option<i32>, option2: Option<i32>) -> Option<i32> {
    let sum = match (option1, option2) {
        (Some(v1), Some(v2)) => Some(v1 + v2),
        _ => None
    };
    return sum;
}