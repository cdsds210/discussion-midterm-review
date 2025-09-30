fn main() {
    println!("Hello, world!");
}

fn uses_match(option1: Option<i32>, option2: Option<i32>) -> i32 {
    match (option1, option2) {
        (Some(v1), Some(v2)) => return v1 + v2,
        _ => panic!("Expected a value in both options, got None.")
    }
}

fn uses_unwrap(option1: Option<i32>, option2: Option<i32>) -> i32 {
    // Will cause a panic! with a default error msg if None
    return option1.unwrap() + option2.unwrap();
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