// การสร้าง Enum ที่เก็บข้อมูลได้หลายรูปแบบ
enum Message {
    Quit,                       // Unit-like: ไม่มีข้อมูลข้างใน
    Move { x: i32, y: i32 },    // Struct-like: เก็บพิกัด x, y
    Write(String),              // Tuple-like: เก็บข้อความ
}

fn main() {
    let msg1 = Message::Write(String::from("Hello PPL"));
    let msg2 = Message::Move { x: 10, y: 20 };
    
    println!("{:?}", msg1);
    println!("{:?}", msg2);
}
