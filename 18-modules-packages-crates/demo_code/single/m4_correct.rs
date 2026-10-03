// Mistake 4 (ถูก): ใช้ super::  -> พิมพ์ 42
fn helper() -> u32 {
    42
}

mod inner {
    pub fn run() -> u32 {
        super::helper()
    }
}

fn main() {
    println!("{}", inner::run());
}
