// Mistake 2 (ถูก): ใช้ constructor + getter  -> พิมพ์ Somchai has 100
mod bank {
    pub struct Account {
        pub owner: String,
        balance: u32,
    }

    impl Account {
        pub fn new(owner: &str, balance: u32) -> Account {
            Account {
                owner: String::from(owner),
                balance,
            }
        }

        pub fn balance(&self) -> u32 {
            self.balance
        }
    }
}

fn main() {
    let acc = bank::Account::new("Somchai", 100);
    println!("{} has {}", acc.owner, acc.balance());
}
