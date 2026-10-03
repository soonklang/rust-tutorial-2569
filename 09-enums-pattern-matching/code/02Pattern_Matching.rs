enum Coin {
    Penny,
    Quarter(String), // เก็บชื่อรัฐของเหรียญ Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        // Pattern Matching ดึงค่า String ออกมาใส่ตัวแปร state
        Coin::Quarter(state) => {
            println!("State quarter from {}!", state);
            25
        }
    }
}

fn main() {
    // สร้างตัวอย่างเหรียญเพื่อทดสอบ
    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(String::from("Alaska"));

    value_in_cents(coin1);
    value_in_cents(coin2);
}
