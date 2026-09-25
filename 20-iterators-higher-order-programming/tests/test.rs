
fn apply_operation<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    // Call the function passed as an argument
    operation(value)
}

#[test]
fn test() {
    // Create a closure that adds 1
    let add_one = |x| x + 1;

    // Pass the closure to a higher-order function
    let result = apply_operation(5, add_one);

    println!("Result: {}", result);
}