// Rust code
fn main() {
    let config_max = Some(3u8);

    // แบบที่ 1: ใช้ match (ต้องเขียน _ => () เพื่อดักกรณีที่เหลือ)
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // แบบที่ 2: ใช้ if let (กระชับกว่า อ่านง่ายกว่า)
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}