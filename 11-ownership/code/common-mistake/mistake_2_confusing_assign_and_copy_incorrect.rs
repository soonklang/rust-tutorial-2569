fn main() {
    let s1 = String::from("move not copy");
    let s2 = s1;              // ความเป็นเจ้าของถูกย้ายจาก s1 ไป s2
    println!("{}", s1);       // ERROR: s1 ใช้งานไม่ได้แล้ว
}