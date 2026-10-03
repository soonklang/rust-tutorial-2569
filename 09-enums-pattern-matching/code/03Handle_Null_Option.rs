// Rust code
// Option Enum ถูก Build-in มาในภาษา หน้าตาเป็นแบบนี้:
// enum Option<T> { Some(T), None, }

fn main() {
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    match some_number {
        Some(val) => println!("มีตัวเลขคือ: {}", val),
        None => println!("ไม่มีข้อมูล (คล้าย Null แต่ปลอดภัย)"),
    }
}
