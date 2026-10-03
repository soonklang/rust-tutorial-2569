
**Problem**

`คุณได้รับ struct `Book` ที่มี field เป็น `String` สองตัว และมี loop ที่ต้องการสร้าง label สำหรับแต่ละเล่ม แล้ว print ข้อมูลเต็มของแต่ละเล่มทีหลัง`
 
```rust
struct Book {
    title: String,
    author: String,
}
 
fn make_label(title: String) -> String {
    format!("[{}]", title)
}
 
fn main() {
    let books = vec![
        Book { title: String::from("Rust programming"), author: String::from("Graydon Hoare") },
        Book { title: String::from("C programming"), author: String::from("Dennis Ritchie") },
    ];
 
    for book in books {
        let label = make_label(book.title);
        println!("{}", label);
        println!("by {} - full title: {}", book.author, book.title);
    }
}
```
โค้ดนี้ compile ไม่ผ่าน หน้าที่ของคุณคือ:
1. หาให้ได้ว่าค่าตัวไหนถูก move และ move ที่จุดไหนกันแน่
2. แก้โค้ดให้ compile ผ่าน **และ** print label, author, กับ full title ของแต่ละเล่มได้ถูกต้อง โดยไม่เปลี่ยนพฤติกรรมของ `make_label` (ยังต้องรับความเป็นเจ้าของ `String` เหมือนเดิม)
3. โบนัส: ลองเขียนวิธีแก้แบบที่สองที่ต่างออกไป (มีวิธีแก้ที่ถูกต้องมากกว่าหนึ่งวิธี)

**Hint**

`book.title` ถูกย้ายเข้าไปใน `make_label(book.title)` หลังจากบรรทัดนั้น `book.title` ยังใช้งานได้อยู่ไหม? แล้ว field อื่นของ `book` (เช่น `book.author`) ยังใช้ได้ปกติหรือเปล่า? นี่เป็นปัญหาแบบเดียวกับการเข้าถึง field ของ struct หลังจากบางส่วนถูกย้ายไปแล้ว ลองคิดดูว่า field ไหนที่ต้องรอดจนถึงหลังจากเรียกฟังก์ชันนั้น แล้วจะทำยังไงให้มันรอด
