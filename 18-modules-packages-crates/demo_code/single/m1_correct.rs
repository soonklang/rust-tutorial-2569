// Mistake 1 (ถูก): เติม pub  -> พิมพ์ 150
mod bank {
    pub fn deposit(balance: u32, amount: u32) -> u32 {
        balance + amount
    }
}

fn main() {
    println!("{}", bank::deposit(100, 50));
}
