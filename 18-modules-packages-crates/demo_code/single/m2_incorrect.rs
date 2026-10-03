// Mistake 2 (ผิด): field balance ยัง private  -> คาดว่า error E0451
mod bank {
    pub struct Account {
        pub owner: String,
        balance: u32,
    }
}

fn main() {
    let acc = bank::Account {
        owner: String::from("Somchai"),
        balance: 100,
    };
    println!("{}", acc.owner);
}
