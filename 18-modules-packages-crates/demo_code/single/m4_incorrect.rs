// Mistake 4 (ผิด): module ลูกเรียก helper() ตรง ๆ  -> คาดว่า error E0425
fn helper() -> u32 {
    42
}

mod inner {
    pub fn run() -> u32 {
        helper()
    }
}

fn main() {
    println!("{}", inner::run());
}
