fn greet(name: String) {
    println!("Hello, {}", name);
}
fn main() {
    let name = String::from("Alice");

    for i in 0..3 {
        greet(name.clone());
    }
}