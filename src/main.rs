fn main() {
    let opt1 = Some(1);
    let opt2 = None;
    
    // Propagation moves the error handling up! In libraries, this 
    // allows users to decide how they handle error
    let sum = uses_propagation(opt1, opt2);
    println!("{:?}", sum);
    // For example, users may want to raise an error with expect
    sum.expect("Oops! I unwrapped in `main`");
    // They may want to default to some value (probably something like 0)
    sum.unwrap_or(0); 
    
    // Without using propagation none of this would be possible, since the function PANICS inside the function call
    let without_prop = uses_unwrap(opt1, opt2);
    println!("{}", without_prop);

    
}
/// A basic function that takes two Options, and sums them
fn sum_options(option1: Option<i32>, option2: Option<i32>) -> i32 {
    // Do this manually using match
    let value1 = match option1{
        Some(v) => v,
        None => panic!("Expected Some, got None")
    };

    let value2 = match option2{
        Some(v) => v,
        None => panic!("Expected Some, got None")
    };

    return value1 + value2;
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

/// Implementing "error propagation" manually
fn manual_propagation(option1: Option<i32>, option2: Option<i32>) -> Option<i32> {
    // Match both option 1 and 2
    let sum = match (option1, option2) {
        // If Both are Some, return Some(v1 + v2)
        (Some(v1), Some(v2)) => Some(v1 + v2),
        // Otherwise return None
        _ => None
    };
    return sum;
}

fn uses_propagation(option1: Option<i32>, option2: Option<i32>) -> Option<i32> {
    // Cleaner implementation of propagation with the propagation operator ?
    let v1 = option1?;
    let v2 = option2?;
    return Some(v1 + v2)
    // Alterantively in a one liner
    // return Some(option1? + option2?)
}

