fn apply(value: i32, operation: fn(i32) -> i32) -> i32 {
    operation(value)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let result = apply(5, double);
    println!("{}", result); // 10
}
