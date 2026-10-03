fn main() {
    // ต้องประกาศตัวแปรเดิมด้วย mut ก่อน จึงจะยืมแบบ mutable reference ได้
    let mut message = String::from("ข้อความก่อนแก้ไข");

    {
        // &mut ให้สิทธิ์แก้ไขข้อมูลผ่าน reference นี้ได้
        let message_reference = &mut message;
        message_reference.push_str(" และข้อความใหม่");
        println!("ข้อความผ่าน mutable reference: {message_reference}");
    }

    println!("ข้อความหลังแก้ไข: {message}");
}