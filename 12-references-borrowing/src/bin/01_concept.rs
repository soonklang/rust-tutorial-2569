// 01_concept.rs
// จุดประสงค์: สาธิต References, Borrowing และ Ownership / Purpose: Demonstrate References, Borrowing, and Ownership.

fn main() {
    // my_string เป็นเจ้าของข้อมูล / my_string owns the data.
    let my_string = String::from("Silpakorn");

    // ส่ง immutable reference ด้วย & เรียกว่า Borrowing: ฟังก์ชันอ่านข้อมูลได้แต่ไม่รับ Ownership ไป
    // Pass an immutable reference with &: the function can read the data without taking Ownership.
    let length = calculate_length(&my_string);

    // my_string ยังใช้งานได้ เพราะเรา Borrowing แทนการย้าย Ownership
    // my_string is still valid because we borrowed it instead of moving its Ownership.
    println!("The length of '{}' is {}.", my_string, length);

    // ตัวแปรต้องเป็น mut เพื่ออนุญาตให้สร้าง mutable reference
    // The variable must be mut to allow a mutable reference.
    let mut greeting = String::from("Hello");

    // ส่ง mutable reference ด้วย &mut เพื่อให้ฟังก์ชันแก้ไขข้อมูลเดิมได้
    // Pass a mutable reference with &mut so the function can modify the original data.
    append_text(&mut greeting);

    // greeting ยังเป็นเจ้าของข้อมูลและใช้งานได้หลังการ Borrowing
    // greeting still owns and can use the data after Borrowing.
    println!("After borrowing mutably: {}", greeting);
}

// รับ immutable reference จึงไม่รับ Ownership ของ String
// Takes an immutable reference, so it does not take String Ownership.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// รับ mutable reference เพื่อแก้ไข String โดยไม่รับ Ownership
// Takes a mutable reference to modify String without taking Ownership.
fn append_text(s: &mut String) {
    s.push_str(", Rust!");
}

