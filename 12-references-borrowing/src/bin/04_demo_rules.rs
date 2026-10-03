fn main() {
    demonstrate_immutable_reference_error();
    demonstrate_mutable_and_immutable_conflict();
}

fn demonstrate_immutable_reference_error() {
    let message = String::from("แก้ไขผ่าน immutable reference ไม่ได้");
    let message_reference = &message;

    println!("ข้อความผ่าน immutable reference: {message_reference}");

    // บรรทัดนี้จะเกิด error: cannot borrow `*message_reference` as mutable,
    // as it is behind a `&` reference
    // message_reference.push_str("ข้อความใหม่");
    // เหตุผลคือ & เป็น immutable reference จึงอ่านข้อมูลได้อย่างเดียว
    // หากต้องการแก้ไข ต้องใช้ &mut และตัวแปรต้นฉบับต้องประกาศด้วย mut
}

fn demonstrate_mutable_and_immutable_conflict() {
    let mut number = 10;

    let mutable_reference = &mut number;
    // บรรทัดนี้จะเกิด error: cannot borrow `number` as immutable because it is
    // also borrowed as mutable
    // let immutable_reference = &number;
    // Rust ไม่อนุญาตให้มี &mut และ & ที่ชี้ไปยังข้อมูลเดียวกันในช่วงเวลาเดียวกัน
    // เพื่อป้องกันการอ่านค่าที่ไม่สอดคล้องกันขณะมีการแก้ไขข้อมูล

    *mutable_reference += 5;
    println!("ค่าหลังแก้ไขผ่าน mutable reference: {mutable_reference}");
}