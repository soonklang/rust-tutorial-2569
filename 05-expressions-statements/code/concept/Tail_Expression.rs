fn add(a: i32, b: i32) -> i32 {
    a + b //tail expression
}
fn main() {
    let result = add(5, 3);
    println!("ผลรวมคือ: {}", result); // Output: ผลรวมคือ: 8
}
