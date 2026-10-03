// Exercise1
fn main() {
    let x = 10;
    let mut y = 20;
    y = y + x;
    x = y - 5;
    println!("{} {}", x, y);
}
/* Solution 1
fn main() {
    let mut x = 10;
    let mut y = 20;
    y = y + x;
    x = y - 5;
    println!("{} {}", x, y);
}