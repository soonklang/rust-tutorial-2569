struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User { name: String::from("Alice"), age: 30 };

    let name = user.name;         // ย้ายเฉพาะ field name ออกมา
    println!("{}", user.name);    // ERROR: user.name ถูกย้ายไปแล้ว
    println!("{}", user.age);     // ใช้ได้ปกติ — age เป็น Copy ไม่ได้ถูกย้าย
}