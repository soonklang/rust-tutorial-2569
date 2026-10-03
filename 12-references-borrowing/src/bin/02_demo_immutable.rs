fn main() {
    let message = String::from("สวัสดีจาก Rust");

    // เราสามารถสร้าง immutable reference หลายตัวไปยังข้อมูลเดียวกันได้
    // เพราะ reference เหล่านี้มีไว้สำหรับอ่านข้อมูลเท่านั้น ไม่ได้แก้ไขข้อมูล
    let reference_one = &message;
    let reference_two = &message;

    println!("reference_one: {reference_one}");
    println!("reference_two: {reference_two}");
    println!("ข้อความต้นฉบับ: {message}");
}