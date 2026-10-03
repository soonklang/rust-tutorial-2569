mod calculator;
mod utils;

use calculator::{add, subtract, multiply, divide};
use utils::print_title;

fn main() {
    print_title("Rust Modules Demo");

    let a = 20;
    let b = 10;

    let addition = add(a, b);
    let subtraction = subtract(a, b);
    let multiplication = multiply(a, b);

    println!("Addition: {}", addition);
    println!("Subtraction: {}", subtraction);
    println!("Multiplication: {}", multiplication);
    println!("Division: {}", divide(a, b));
}
