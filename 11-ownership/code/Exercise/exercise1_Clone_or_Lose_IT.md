**Problem**

ให้ฟังก์ชัน `describe(item: String) -> String` ที่รับความเป็นเจ้าของ `String` เข้ามา แล้ว return `String` ใหม่ในรูปแบบ `"Item: {item}"`
 
ใน `main` ให้ทำตามนี้:
1. สร้างตัวแปร `String` ชื่อ `item` ที่มีค่าเป็น `"Book"`
2. เรียก `describe(item)` แล้ว print ผลลัพธ์
3. print `item` อีกครั้งหลังจากนั้น

เขียน main โดยไม่เปลี่ยนพฤติกรรมของ `describe` (ยังต้องรับความเป็นเจ้าของ `String` เหมือนเดิม)
```rust
fn describe(item: String) -> String {
    format!("Item: {}", item)
}
```

**Hint**

``describe` รับ `item: String` แบบ by value ดังนั้นการเรียก `describe(item)` จะย้ายความเป็นเจ้าของออกไปจาก `item` ใน `main` ต้องหาวิธีที่ทำให้ `item` ยังใช้งานได้หลังจากนั้น โดยไม่เปลี่ยน signature ของ `describe` มีคำสั่งอะไรที่ช่วยให้คุณส่ง *สำเนา* ไปแทนตัวจริงได้บ้าง?`