// Exercise 2: ผลลัพธ์ *** สวัสดี Rust ***
use thai_greeter::greeting::hello;

fn main() {
    println!("{}", hello("Rust"));

    // ลองเอา // ข้างล่างออก เพื่อดู error E0603 (decorate เป็น private)
    // println!("{}", thai_greeter::greeting::decorate("test"));
}
