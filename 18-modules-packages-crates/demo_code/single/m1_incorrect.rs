// Mistake 1 (ผิด): ลืม pub  -> คาดว่า error E0603
mod bank {
    fn deposit(balance: u32, amount: u32) -> u32 {
        balance + amount
    }
}

fn main() {
    println!("{}", bank::deposit(100, 50));
}
