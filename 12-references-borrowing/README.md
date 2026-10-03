# Rust Tutorial Project — Principles of Programming Languages

> **Topic No.:** `12`
> **Topic Name:** `References & Borrowing`
> **Group No.:** `12`

---

## 1. Members

| # | Name                                      | Student ID | GitHub Username | Main Responsibility                     |
| - | ----------------------------------------- | ---------- | --------------- | --------------------------------------- |
| 1 | Mr.UDTARAKVISETH LAY                      | 670710259  | `@Viseth101`  | Concept + Short Code                    |
| 2 | ถอนรายวิชา                      |            |                 |                                         |
| 3 | นางสาวณัฐณิชา ภู่วงษ์ | 670710291  | `@nncp-fs`    | Rust vs Other Languages + PPL Analysis  |
| 4 | นายเทพพิทักษ์ นิลดำ     | 670710292  | `@670710292`  | Exercises + Common Mistakes + Challenge |

---

## 2. Learning Objectives

หลังจากศึกษาเรื่องนี้แล้ว ผู้เรียนสามารถ:

- อธิบายความสัมพันธ์ระหว่าง Ownership, References และ Borrowing เพื่อจัดการหน่วยความจำอย่างปลอดภัยได้
- แยกความแตกต่างระหว่าง Immutable Reference (`&T`) และ Mutable Reference (`&mut T`) พร้อมเลือกใช้ได้อย่างเหมาะสม
- อธิบายบทบาทของ Borrow Checker และกฎการยืมที่ช่วยป้องกัน Data Race และ Dangling Reference ได้
- วิเคราะห์ข้อดีของการตรวจสอบความปลอดภัยของหน่วยความจำในระยะคอมไพล์ โดยไม่ต้องใช้ Garbage Collector ได้

---

## 3. Introduction

การจัดการหน่วยความจำเป็นความท้าทายสำคัญในการเขียนโปรแกรม ภาษาอย่าง C เปิดให้ผู้พัฒนาจัดการหน่วยความจำด้วยตนเอง จึงอาจเกิดปัญหา เช่น Pointer ที่ชี้ไปยังข้อมูลซึ่งหมดอายุหรือการรั่วไหลของหน่วยความจำ ขณะที่บางภาษาใช้ Garbage Collector เพื่อจัดการหน่วยความจำโดยอัตโนมัติ

Rust ใช้แนวคิด **Ownership** ซึ่งกำหนดให้ข้อมูลแต่ละชิ้นมีเจ้าของได้เพียงหนึ่งราย เมื่อเจ้าของออกจากขอบเขต (Scope) Rust จะคืนหน่วยความจำให้อัตโนมัติ อย่างไรก็ตาม หากต้องย้าย Ownership ทุกครั้งที่ส่งข้อมูลระหว่างฟังก์ชัน โค้ดจะไม่สะดวกและอาจไม่เหมาะกับการใช้งานบางรูปแบบ Rust จึงมี **References และ Borrowing** สำหรับยืมข้อมูลไปใช้งานโดยไม่รับ Ownership มา

การยืมถูกตรวจสอบโดย **Borrow Checker** ในระยะคอมไพล์ ทำให้ Rust ตรวจพบการใช้หน่วยความจำที่ไม่ปลอดภัยก่อนโปรแกรมทำงาน เช่น การใช้ข้อมูลหลังถูกทำลายหรือการแก้ไขข้อมูลพร้อมกับการอ่านจากหลายจุด การตรวจสอบนี้ช่วยรับประกัน Memory Safety โดยไม่ต้องพึ่งพา Garbage Collector ในระยะรันไทม์

---

## 4. แนวคิดสำคัญ (Key Concepts)

### 4.1 Immutable References (การอ้างอิงแบบอ่านอย่างเดียว)

**คำอธิบาย**

References (`&`) ช่วยให้ฟังก์ชันเข้าถึงข้อมูลโดยไม่รับช่วงความเป็นเจ้าของ (Ownership) การใช้งานนี้เรียกว่า **Borrowing (การยืม)** โดย Reference แบบ `&T` เป็นการยืมเพื่ออ่าน จึงไม่สามารถแก้ไขข้อมูลต้นฉบับผ่าน Reference นั้นได้

**ตัวอย่างจากไฟล์ `src/bin/01_concept.rs`**

```rust
let my_string = String::from("Silpakorn");
let length = calculate_length(&my_string);
println!("The length of '{}' is {}.", my_string, length);

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**คำอธิบายการทำงานทีละบรรทัด**

- `let my_string = String::from("Silpakorn");` สร้างค่า `String` ที่มีโครงสร้างอยู่บนสแตกและบัฟเฟอร์ข้อมูลอยู่บนฮีป โดยให้ `my_string` เป็นเจ้าของข้อมูล
- `let length = calculate_length(&my_string);` ใช้ `&` สร้าง Immutable Reference แล้วส่งไปยังฟังก์ชัน จึงเป็นการยืมโดยไม่ย้าย Ownership
- `fn calculate_length(s: &String) -> usize` กำหนดให้ `s` รับ Reference ของ `String` และคืนค่าชนิด `usize`
- `s.len()` อ่านความยาวของ String ผ่าน Reference แล้วส่งค่าความยาวกลับไป โดยไม่ได้แก้ไขข้อมูล
- `println!(...)` ใช้ `my_string` ได้อีกครั้งหลังเรียกฟังก์ชัน เพราะ Ownership ยังคงอยู่ที่ `my_string` และ `s` ไม่ได้เป็นเจ้าของข้อมูล

---

### 4.2 Mutable References (การอ้างอิงแบบแก้ไขค่าได้)

**คำอธิบาย**

หากต้องการแก้ไขข้อมูลผ่าน Reference ต้องใช้ **Mutable Reference (`&mut T`)** ตัวแปรเจ้าของข้อมูลต้องประกาศด้วย `mut` และ Reference ที่ส่งเข้าไปต้องใช้ `&mut` เช่นกัน การยืมชนิดนี้ให้สิทธิ์แก้ไขข้อมูลต้นฉบับได้ภายใต้กฎของ Borrow Checker

**ตัวอย่างจากไฟล์ `src/bin/01_concept.rs`**

```rust
let mut greeting = String::from("Hello");
append_text(&mut greeting);
println!("After borrowing mutably: {}", greeting);

fn append_text(s: &mut String) {
    s.push_str(", Rust!");
}
```

**คำอธิบายการทำงานทีละบรรทัด**

- `let mut greeting = String::from("Hello");` สร้าง String และประกาศตัวแปรเจ้าของให้แก้ไขค่าได้ด้วย `mut`
- `append_text(&mut greeting);` สร้าง Mutable Reference ด้วย `&mut` แล้วส่งให้ฟังก์ชัน โดยยังคงให้ `greeting` เป็นเจ้าของข้อมูล
- `fn append_text(s: &mut String)` กำหนดให้ `s` รับ Mutable Reference ของ String จึงแก้ไขข้อมูลที่ `s` อ้างถึงได้
- `s.push_str(", Rust!");` เติมข้อความลงใน String เดิมผ่าน Reference ทำให้ค่าของ `greeting` เปลี่ยนเป็น `Hello, Rust!`
- `println!(...)` แสดงค่าหลังการยืมสิ้นสุด โดย `greeting` ยังคงเป็นเจ้าของ String เดิม

---

## 5. ไวยากรณ์และกฎสำคัญ (Important Syntax / Rules)

| ไวยากรณ์ | ความหมาย                                                                                                                                         | ตัวอย่าง                  |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| `&T`           | Immutable Reference: ยืมค่าเพื่ออ่าน โดยไม่รับ Ownership และไม่สามารถแก้ไขค่าผ่าน Reference นี้ได้ | `let ref_val = &my_string;`     |
| `&mut T`       | Mutable Reference: ยืมค่าเพื่ออ่านและแก้ไข โดยต้องได้รับอนุญาตจากตัวแปรเจ้าของ                 | `let mut_ref = &mut my_string;` |

### กฎการยืมที่ต้องปฏิบัติตาม

1. ในขอบเขตเวลาที่การยืมยังใช้งานอยู่ สามารถมี Mutable Reference (`&mut T`) ได้เพียงหนึ่งตัว หรือมี Immutable Reference (`&T`) ได้หลายตัว
2. ห้ามมี Mutable Reference และ Immutable Reference ของข้อมูลเดียวกันพร้อมกัน เพราะอาจทำให้เกิด Data Race
3. Reference ทุกตัวต้องชี้ไปยังข้อมูลที่ยังมีอยู่จริงและมีอายุการใช้งาน (Lifetime) ครอบคลุม Reference นั้นเสมอ Rust จึงป้องกัน Dangling Reference ตั้งแต่ระยะคอมไพล์
4. การสร้าง Mutable Reference ต้องใช้ตัวแปรเจ้าของที่ประกาศด้วย `mut`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** ตัวอย่างที่ระบุว่า Runnable ต้อง Compile และ Run ได้จริง ส่วน Code ที่แสดง Compile Error จะใส่คำสั่งที่ทำให้เกิด Error เป็น Comment เพื่อให้โปรเจกต์ Compile ได้ และสามารถ Uncomment เพื่อสาธิต Error ได้

### Example 1 — `Immutable reference`

**Purpose:** `ตัวอย่างนี้แสดงให้เห็นถึงการใช้งาน immutable reference`

```rust
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
```

**Expected Output**

```text
reference_one: สวัสดีจาก Rust
reference_two: สวัสดีจาก Rust
ข้อความต้นฉบับ: สวัสดีจาก Rust
```

**Explanation**

`การสร้าง reference ใน rust ทำให้ตัวแปรสามารถเข้าถึงข้อมูลของอีกตัวแปรหนึ่งได้โดยที่ไม่ต้องส่ง ownership ให้กับตัวแปรที่อ้างอิงข้อมูล นอกจากนี้ การสร้าง immutable reference ใน rust สามารถทำพร้อมกันหลายตัวเพื่ออ่านข้อมูลเดียวกันได้ ตราบใดที่ไม่มีการ borrow ที่ขัดแย้งกัน (n immutable reference xor 1 mutable reference)`

---

### Example 2 — `Mutable Reference showcase`

**Purpose:** `การใช้ mutable reference ในการแก้ไขข้อมูล`

```rust
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
```

**Expected Output**

```text
ข้อความผ่าน mutable reference: ข้อความก่อนแก้ไข และข้อความใหม่
ข้อความหลังแก้ไข: ข้อความก่อนแก้ไข และข้อความใหม่
```

**Explanation**

`หลังจากสร้าง Mutable Reference เราสามารถแก้ไขข้อมูลผ่านตัวแปรที่ถือ mutable reference ได้ โดยสิ่งที่แก้ไขจะส่งผลกระทบต่อตัวแปรต้นทางด้วย`

### Example 3 — `Reference restrictions showcase`

**Purpose:** `แสดงให้เห็น error เมื่อพยายามการแก้ไขข้อมูลผ่าน immutable reference และ borrow conflict`

```rust
fn main() {
    demonstrate_immutable_reference_error();
    demonstrate_mutable_and_immutable_conflict();
}

fn demonstrate_immutable_reference_error() {
    let message = String::from("แก้ไขผ่าน immutable reference ไม่ได้");
    let message_reference = &message;

    println!("ข้อความผ่าน immutable reference: {message_reference}");

    // Uncomment บรรทัดต่อไปนี้เพื่อสาธิต Error:
    // cannot borrow `*message_reference` as mutable, as it is behind a `&` reference
    message_reference.push_str("ข้อความใหม่");
    // เหตุผลคือ & เป็น immutable reference จึงอ่านข้อมูลได้อย่างเดียว
    // หากต้องการแก้ไข ต้องใช้ &mut และตัวแปรต้นฉบับต้องประกาศด้วย mut
}

fn demonstrate_mutable_and_immutable_conflict() {
    let mut number = 10;

    let mutable_reference = &mut number;
    // Uncomment บรรทัดต่อไปนี้เพื่อสาธิต Error:
    // cannot borrow `number` as immutable because it is also borrowed as mutable
    let immutable_reference = &number;
    // Rust ไม่อนุญาตให้มี &mut และ & ที่ชี้ไปยังข้อมูลเดียวกันในช่วงเวลาเดียวกัน
    // เพื่อป้องกันการอ่านค่าที่ไม่สอดคล้องกันขณะมีการแก้ไขข้อมูล

    *mutable_reference += 5;
    println!("ค่าหลังแก้ไขผ่าน mutable reference: {mutable_reference}");
}
```

**Expected Output**

```text
ข้อความผ่าน immutable reference: แก้ไขผ่าน immutable reference ไม่ได้
ค่าหลังแก้ไขผ่าน mutable reference: 15
```

**Explanation**

`ใน demonstrate_immutable_reference_error() แสดงให้เห็นว่า การแก้ไขค่าผ่าน reference จะทำได้ก็ต่อเมื่อ reference ดังกล่าว(และตัวแปรต้นทาง) เป็น mutable เท่านั้น หากพยายามแก้ไขจะก่อให้เกิด error ใน demonstrate_mutable_and_immutable_conflict() เป็นการ showcase borrow conflict หากไม่ยึดตามกฎ n immutable reference xor 1 mutable reference`

---

## 7. Common Mistakes

### Mistake 1 — `[Mutable and Immutable Reference Mixup]`

**Problem**

`การใช้งาน Mutable reference และ Immutable reference กับตัวแปรเดียวกันขณะที่ reference ก่อนหน้ายังถูกใช้งานอยู่ ซึ่ง Rust ไม่อนุญาตให้เกิดการ borrow ที่ขัดแย้งกัน`

**Incorrect Code**

```rust
// Incorrect example
fn main() {
    let mut num = vec![1, 2, 3];
    let first_number = &num[0]; // immutable borrow occurs here
    num.push(4); // mutable borrow occurs here
    println!("{}", first_number); // immutable borrow later used here
    //error[E0502]: cannot borrow `num` as mutable because it is also borrowed as immutable  
}
```

**Correct Code**

```rust
// Correct example
fn main() {
    let mut num = vec![1, 2, 3];
    num.push(4); 
    let first_number = &num[0]; // make an immutable reference after the mutable borrow has ended
    println!("{}", first_number); 
}
```

**Why?**

`Rust ไม่อนุญาตให้ immutable borrow และ mutable borrow ของข้อมูลเดียวกันเกิดขึ้นพร้อมกัน เพราะ mutable borrow สามารถแก้ไขข้อมูลได้ ในขณะที่ immutable borrow ต้องสามารถอ่านข้อมูลได้โดยไม่ต้องกังวลว่าข้อมูลจะถูกแก้ไขระหว่างที่ borrow ยังใช้งานอยู่

ในตัวอย่างแรก first_number เป็น immutable reference ที่ยังถูกใช้งานที่ println! หลังจากเรียก num.push(4) (mutable borrow) ดังนั้น borrow นี้ยังไม่สิ้นสุด จึงเกิด conflicting borrow
เพราะ push อาจทำให้ Vec ทำการ reallocate หน่วยความจำ ซึ่งอาจทำให้ตำแหน่งของข้อมูลที่ first_number อ้างถึงเปลี่ยนไป และทำให้ reference นี้ไม่สามารถใช้งานได้อย่างปลอดภัย

ในตัวอย่างที่ถูกต้องได้ทำการเพิ่มค่าเข้า num ให้เสร็จก่อน หลังจาก mutable borrow สิ้นสุดลงจึงสามารถสร้าง immutable borrow first_number เพื่ออ่านข้อมูล `

---

### Mistake 2 — `[Multiple Mutable References]`

**Problem**

`การสร้าง Mutable reference มากกว่าหนึ่งตัวไปยังตัวแปรเดียวกันขณะที่ mutable reference ก่อนหน้ายังถูกใช้งานอยู่ ซึ่ง Rust ไม่อนุญาตให้เกิดการ borrow ที่ขัดแย้งกัน`

**Incorrect Code**

```rust
// Incorrect example
fn main() {
    let mut num = 10;

    let ref1 = &mut num; // first mutable borrow occurs here
    let ref2 = &mut num; // second mutable borrow occurs here
    // error[E0499]: cannot borrow `num` as mutable more than once at a time
    *ref1 += 5; // first mutable borrow used here
    *ref2 += 10;

    println!("{}", num);
}
```

**Correct Code**

```rust
// Correct example
fn main() {
    let mut num = 10;

    let ref1 = &mut num; 
    *ref1 += 5;

    let ref2 = &mut num; 
    *ref2 += 10;

    println!("{}", num);
}
```

**Why?**

`Rust มีกฎเกี่ยวกับการ borrow ว่า ในช่วงเวลาเดียวกัน ตราบที่ borrow ยังถูกใช้งานอยู่ เราสามารถมี mutable borrow ได้เพียงหนึ่งตัวเท่านั้น
เหตุผลที่ Rust ห้ามมี Mutable reference หลายตัวที่อ้างอิงข้อมูลเดียวกันพร้อมกัน เพราะ Mutable reference สามารถแก้ไขข้อมูลที่อ้างถึงได้ Rust จึงต้องให้ Mutable borrow สามารถเข้าถึงข้อมูลแบบ exclusive
เพื่อป้องกันการเข้าถึงหรือแก้ไขข้อมูลเดียวกันจากหลายจุดในเวลาเดียวกัน

ในตัวอย่างแรก ref1 เป็น mutable borrow ที่ยังถูกใช้งานหลังจากมีการสร้าง ref2 (ที่ *ref1 += 5) ดังนั้น borrow นี้ยังไม่สิ้นสุด แต่มีการสร้าง ref2 ซึ่งเป็น mutable borrow อีกตัวไปยัง num เดียวกัน จึงเกิด conflicting borrow (E0499)

ในตัวอย่างที่ถูกต้อง เราทำการใช้ ref1 ให้เสร็จก่อน เมื่อ Mutable borrow ของ ref1 สิ้นสุดลง จึงสามารถสร้าง ref2 เพื่อแก้ไข num ต่อได้`

---

## 8. Exercises

### Exercise 1 — `[Exercise Name]`

**Problem**

`กำหนดให้ตัวแปร message เก็บข้อความ "Hello, Rust!" จงเติมโค้ดในส่วนที่กำหนดให้โปรแกรมสามารถพิมพ์ข้อความต้นฉบับ และแก้ไขโดยการเพิ่มประโยค " Have a nice day!" แล้วพิมพ์ออกมาอีกครั้ง`

```rust
fn main() {
    let mut message = String::from("Hello, Rust!");
    let message_reference = __________________;
    println!("ข้อความเดิม: {}", message_reference);

    let message_reference = __________________;
    __________________;
    println!("ข้อความใหม่: {}", message_reference);
}
```

**Hint**

`Immutable Reference ใช้ & ส่วน Mutable Reference ใช้ &mut และตัวแปรที่ถูกอ้างอิงต้องประกาศด้วย mut หากต้องการแก้ไขข้อมูล`

**Solution**

```rust
// Solution code
fn main() {
    let mut message = String::from("Hello, Rust!");
    let message_reference = &message;
    println!("ข้อความเดิม: {}", message_reference);

    let message_reference = &mut message;
    message_reference.push_str(" Have a nice day!");
    println!("ข้อความใหม่: {}", message_reference);
}
```

**Explanation**

`ในช่วงแรก message_reference เป็น Immutable Reference ที่สร้างด้วย &message จึงสามารถใช้เข้าถึงข้อมูลของ message ได้ แต่ไม่สามารถแก้ไขข้อมูลผ่าน Reference นี้ได้
หลังจากใช้งาน Immutable Reference เสร็จแล้ว จึงสร้าง Mutable Reference ด้วย &mut message เพื่อให้สามารถแก้ไขข้อความด้วย push_str() ได้
ตัวอย่างนี้แสดงให้เห็นว่า Rust แยกการเข้าถึงข้อมูลแบบ Immutable (&T) และ Mutable (&mut T) อย่างชัดเจน โดย Mutable Reference สามารถใช้แก้ไขข้อมูลที่อ้างอิงได้ แต่ตัวแปรต้นทางต้องประกาศเป็น mut`

---

### Exercise 2 — `Longest Entry in Report`

**Problem**

```rust
fn longest_line(text: &String) -> &str {
    let mut best = "";
    for line in text.lines() {
        if line.len() > best.len() {
            best = line;
        }
    }
    best
}

fn get_report() -> &'static str {
    let text = String::from("short\na much longer line here\nmid");
    longest_line(&text)
}

fn main() {
    let report = get_report();
    println!("{}", report);
}
```

`โค้ดนี้สามารถ compile ได้ไหม ถ้าไม่ได้ เป็นเพราะอะไร แก้ไขอย่างไร`

**Hint**

`ลองพิจารณาว่าการ return ค่าแบบใดที่จะทำให้ข้อมูลยังสามารถถูก reference ได้`

**Solution**

```rust
// Solution code
fn longest_line(text: &String) -> String {
    let mut best = "";
    for line in text.lines() {
        if line.len() > best.len() {
            best = line;
        }
    }
    best.to_string()
}

fn get_report() -> String {
    let text = String::from("short\na much longer line here\nmid");
    longest_line(&text)
}

fn main() {
    let report = get_report();
    println!("{}", report);
}
```

**Explanation**

`โค้ดนี้ไม่สามารถ compile ได้เพราะ get_report() ประกาศว่าจะคืนค่า &'static str แต่ longest_line() คืน reference ที่ยืมมาจาก text ซึ่งเป็น local variable และมีอายุไม่ถึง 'static จึงไม่สามารถคืน reference นี้ออกจากฟังก์ชันได้ วิธีแก้หนึ่งคือเปลี่ยน return type ของทั้งสองฟังก์ชันเป็น String เพื่อคืน ownership ของข้อความ`

---

## 9.Challenge Question
`To go or not to go, that is the question`

**Problem**

`กลุ่มเพื่อนกลุ่มหนึ่งกำลังจะไปเที่ยวด้วยกัน โดย Noah ชวนเพื่อนมาได้อีก 2 คนคือ Oliver กับ James ในภายหลัง Quinn ตัดสินใจที่จะไปด้วย เหลือเพื่อนอีกคนชื่อ Charlotte ที่กำลังตัดสินใจว่าจะไปมั้ย โดยที่ตนคิดไว้ว่าจะไปถ้า Oliver ไปด้วย James จึงอยากเขียนโค้ดออกมาสั้นๆให้เห็นว่าใครเป็นคนชวน ไปเที่ยว รวมถึงคนที่จะไปด้วยตามลำดับเหตุการณ์ แต่โค้ดของเขากลับติด error`

```rust
// James' code
fn main() {
    let mut names = vec![String::from("Noah"), String::from("Oliver"), String::from("James")];

    let first = &names[0];
    names.push(String::from("Quinn"));
    println!("Hosted by: {}", first);

    for name in &names {
        if name == "Oliver" {
            names.push(String::from("Charlotte"));
        }
    }
    println!("{:?}", names);
}
```

`แต่โค้ดของเขากลับ compile ไม่ผ่าน จงหาว่า ทำไม compile ไม่ผ่านเพราะส่วนไหนบ้าง แล้วแก้ไขอย่างไรโดยการ restructure โค้ดดังกล่าว`

**Hint**

`ลองสังเกตการ borrow แต่ละอันว่าเริ่ม/จบตรงไหน`

**Solution**

```rust
// Solution code
fn main() {
    let mut names = vec![String::from("Noah"), String::from("Oliver"), String::from("James")];

    let first = &names[0];
    println!("Hosted by: {}", first);
    names.push(String::from("Quinn"));
  
    let mut add = false;
    for name in &names {
        if name == "Oliver" {
            add = true;
        }
    }
    if add {
        names.push(String::from("Charlotte"));
    }
    println!("{:?}", names);
}
```

**Explanation**

`มีส่วนผิดอยู่ 2 จุด จุดที่ 1: first ถูกยืมใช้งานข้าม push() first เป็น immutable reference ที่ยังถูกใช้งานอยู่ ในขณะที่ names.push(...) เรียกใช้งาน mutable borrow ของ names ซึ่ง Rust ไม่อนุญาต เนื่องจาก push อาจทำให้เกิด vector reallocate memory ทำให้ first กลายเป็น dangling reference ได้ วิธีแก้: ใช้ first ก่อน แล้วนำไปใช้งานเพื่อให้ borrow จบลงก่อนที่จะใช้ push `

```rust
println!("Hosted by: {}", &names[0]); // ยืม ใช้ แล้วจบเลยในบรรทัดเดียว
names.push(String::from("Quinn"));
```

`จุดที่ 2: push ข้างในลูปที่กำลัง borrow names แบบ immutable for name in &names ทำให้ names ถูกยืมแบบ immutable ไปตลอดการวนลูป การจะ push เข้าไปใน names ระหว่างนั้นต้องใช้ mutable borrow ซึ่งขัดกัน วิธีแก้: แบ่งการทำงานเดิมออกเป็น 2 ส่วนคือ ส่วนการตัดสินใจ และ ส่วนการเพิ่มข้อมูลเข้า`

```rust
let mut should_add = false;
for name in &names {
    if name == "Oliver" {
        should_add = true;
    }
}
if should_add {
    names.push(String::from("Charlotte"));
}
```

---

## 10. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

References & Borrowing ในภาษา Rust เกี่ยวข้องกับ Syntax ผ่านรูปแบบการสร้างและประกาศ Reference โดยมี Syntax ที่สำคัญ ได้แก่

- `&T` สำหรับ Immutable Reference
- `&mut T` สำหรับ Mutable Reference
  ตาม Rust Reference รูปแบบทั่วไปของ Reference คือ `&` ตามด้วย `mut` ที่อาจมีหรือไม่มีก็ได้ และตามด้วย Type นอกจากนี้ Syntax ของ Reference ยังปรากฏใน Function Parameters และ Return Types เช่น `&String` ทำให้ลักษณะการเข้าถึงข้อมูลและข้อจำกัดของ Reference สามารถแสดงออกใน Source Code ได้อย่างชัดเจน

**ตัวอย่าง Immutable Reference**

```rust
let x = 10;

let r = &x;
```

ในตัวอย่างนี้เป็นการสร้าง Immutable Reference ไปยัง `x` ทำให้ `r` สามารถใช้เข้าถึงค่าของ `x` ได้โดยไม่เป็นเจ้าของข้อมูล

**ตัวอย่างการ Mutable Reference**

```rust
let mut x = 10;

let r = &mut x;

*r = 20;
```

`&mut` แสดงให้เห็นว่า Reference สามารถใช้แก้ไขค่าของข้อมูลที่ถูก Borrow ได้

### 9.2 Semantics

Semantics คือการศึกษาความหมายและพฤติกรรมของคำสั่งหรือโครงสร้าง (Construct) โดยอธิบายว่าเมื่อโปรแกรมถูกประมวลผลแล้ว คำสั่งนั้นจะทำงานอย่างไรและให้ผลลัพธ์แบบใด

ใน References & Borrowing สิ่งสำคัญคือ `&` และ `&mut` ไม่ได้เป็นเพียงสัญลักษณ์ทาง Syntax แต่มีความหมายเกี่ยวกับวิธีการเข้าถึงข้อมูล

**Immutable Borrow**

```rust
let x = 10;

let r1 = &x;

println!("{r1}");
```

สามารถมี Immutable References หลายตัวพร้อมกันได้ เพราะ References เหล่านี้ใช้สำหรับอ่านข้อมูลและไม่ได้อนุญาตให้แก้ไขข้อมูลต้นทาง

**Mutable Borrow**

```rust
let mut x = 10;

let r = &mut x;

*r = 20;

println!("{x}");
```

`&mut x` หมายถึงการ Borrow `x` แบบ Mutable และ Reference นี้สามารถใช้แก้ไขค่าของ `x` ได้

### 9.3 Type System

References & Borrowing เกี่ยวข้องกับ Type System เพราะ Reference ใน Rust มี Type ของตัวเอง เช่น `&T` และ `&mut T` โดย Type เหล่านี้กำหนดลักษณะการเข้าถึงข้อมูลว่าเป็นการอ่านอย่างเดียวหรือสามารถแก้ไขข้อมูลได้

```rust
let x = 10;
let r: &i32 = &x;
```

- `&x` คือการสร้าง Reference ที่ชี้ไปยัง `x`
- `r: &i32` ระบุว่า `r` มี Type เป็น Reference ไปยังข้อมูลชนิด `i32`
- เนื่องจากเป็น `&i32` จึงไม่สามารถแก้ไขค่า `x` ผ่าน `r` ได้

```rust
let mut x = 10;
let r: &mut i32 = &mut x;
*r = 20;
```

- `mut x` ทำให้ `x` สามารถถูกแก้ไขได้
- `&mut x` สร้าง Mutable Reference ไปยัง `x`
- `r: &mut i32` ระบุ Type ว่าเป็น Mutable Reference
- `*r = 20` คือการ Dereference เพื่อเข้าถึงและเปลี่ยนค่าที่ `r` อ้างถึง

### 9.4 Memory / Resource Management

References & Borrowing มีความสัมพันธ์โดยตรงกับ Ownership ซึ่งเป็นกลไกสำคัญในการจัดการ Memory ของ Rust โดย Borrowing ทำให้สามารถนำข้อมูลไปใช้งานผ่าน Reference ได้โดยไม่ต้องโอน Ownership ให้กับผู้ที่นำข้อมูลไปใช้

```rust
fn print_length(s: &String) {
    println!("{}", s.len());
}

let text = String::from("Rust");
print_length(&text);
println!("{}", text);
```

- `&String` หมายถึง Function รับ Reference ไปยัง `String` แทนการรับ Ownership
- `s` จึงสามารถใช้ข้อมูลของ `text` ได้ แต่ไม่ได้เป็นเจ้าของข้อมูล
- `&text` คือการ Borrow ตัวแปร `text` เพื่อส่ง Reference เข้า Function
- หลังจากเรียก Function แล้ว `text` ยังสามารถใช้งานได้ เพราะ Ownership ยังคงอยู่ที่ `text`

### 9.5 Abstraction / Other PPL Concepts

References & Borrowing เชื่อมโยงกับแนวคิด PPL อื่น ๆ เช่น Abstraction, Scope และ Ownership โดยเฉพาะ Abstraction ที่ช่วยให้ Function สามารถกำหนดเพียงว่าต้องการ “เข้าถึงข้อมูล” โดยไม่จำเป็นต้องรับผิดชอบ Ownership ของข้อมูลนั้น ส่วน Scope จะกำหนดขอบเขตการใช้งานของ Reference

```rust
let mut x = 10;

{
    let r = &mut x;
    *r = 20;
}

println!("{}", x);
```

- `let mut x = 10` สร้างตัวแปรที่สามารถแก้ไขค่าได้
- `let r = &mut x` สร้าง Mutable Reference ภายใน Scope
- `*r = 20` แก้ไขค่าของ `x` ผ่าน Reference
- เมื่อออกจาก `{ }` การ Borrow ของ `r` สิ้นสุดลง
- จึงสามารถกลับมาใช้ `x` ได้ใน `println!`

ตัวอย่างนี้แสดงความสัมพันธ์ระหว่าง Borrowing และ Scope ได้อย่างชัดเจน

### 9.6 Why Rust?

Rust ใช้ References & Borrowing เพื่อสร้างสมดุลระหว่าง Safety, Reliability และ Performance โดย Compiler สามารถตรวจสอบกฎของ Ownership และ Borrowing ตั้งแต่ Compile Time

- Safety: ช่วยป้องกันการใช้ Reference ที่ไม่ถูกต้อง เช่น Reference ที่ชี้ไปยังข้อมูลที่หมดอายุแล้ว
- Reliability: กำหนดกฎการเข้าถึงข้อมูลอย่างชัดเจน เช่น การควบคุมการใช้ Mutable และ Immutable References
- Performance: สามารถส่งข้อมูลผ่าน Reference ได้โดยไม่จำเป็นต้องคัดลอกข้อมูลหรือโอน Ownership

ดังนั้น References & Borrowing จึงเป็นแนวคิดสำคัญที่ช่วยให้ Rust จัดการ Memory ได้อย่างปลอดภัยและมีประสิทธิภาพ โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลักของภาษา

---

## 11. Rust vs. Other Languages

**Comparison Language:** `Python`

| Aspect               | Rust                                                                                                                                                                 | Python                                                                                                                             |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Syntax               | ใช้`&` สำหรับ Reference, `&mut` สำหรับ Mutable Reference และ `*` สำหรับ Dereference                                                    | มี Reference โดยอัตโนมัติ แต่ไม่มี Syntax สำหรับ Borrowing โดยตรง                                |
| Semantics / Behavior | Compiler ตรวจสอบ Ownership และ Borrowing ตั้งแต่ Compile Time                                                                                       | Reference สามารถชี้ไปยัง Object เดียวกันได้ และ Memory ถูกจัดการอัตโนมัติ            |
| Type System          | Statically Typed มี Reference Type เช่น`&T`, `&mut T` ซึ่งกำหนดลักษณะการเข้าถึงและสิทธิ์ในการแก้ไขข้อมูล | Dynamically Typed ตัวแปรไม่จำเป็นต้องประกาศ Type และ Reference ถูกจัดการในระดับ Object |
| Memory Management    | ใช้ Ownership และ Borrowing โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลัก                                                              | ใช้ Automatic Memory Management และ Garbage Collection                                                                       |
| Safety               | Compiler ช่วยตรวจสอบ Memory Safety และกฎการ Borrowing                                                                                             | จัดการ Memory อัตโนมัติ แต่ Type และข้อผิดพลาดหลายอย่างตรวจพบขณะ Runtime          |

### Rust Example

```rust
fn calculate_length(text: &String) -> usize {
    text.len()
}

fn main() {
    let text = String::from("Hello Rust");

    let length = calculate_length(&text);

    println!("Text: {}", text);
    println!("Length: {}", length);
}
```

จุดสำคัญ

- `text` เป็นเจ้าของข้อมูล `String`
- `&text` เป็นการ Borrow ข้อมูลเพื่อส่งเข้า Function
- `&String` หมายถึง Function รับ Reference แทนการรับ Ownership
- หลังจากเรียก `calculate_length()` แล้ว `text` ยังสามารถใช้งานได้ เพราะ Ownership ไม่ได้ถูกโอน

### Python Example

```python
def calculate_length(text):
    return len(text)

text = "Hello Python"

length = calculate_length(text)

print("Text:", text)
print("Length:", length)
```

จุดสำคัญ

- `text` ถูกส่งเข้า Function โดยใช้ระบบ Reference ของ Object ใน Python
- ไม่มีการเขียน `&text` เพื่อระบุการ Borrowing
- Python ไม่ได้บังคับกฎ Ownership และ Borrowing แบบ Rust
- การจัดการ Memory ทำโดยระบบของ Python

### Analysis

*ความแตกต่างสำคัญ:* Rust แยกแนวคิด Ownership กับ Borrowing ออกจากกันอย่างชัดเจน โดย `&text` บอกว่า Function รับ Reference ไปยังข้อมูลแทนการรับ Ownership ขณะที่ Python ใช้ระบบ Object Reference โดยไม่มี Syntax ที่ผู้เขียนโปรแกรมต้องใช้เพื่อระบุ Borrowing แบบ Rust

*เหตุผลด้านการออกแบบ:* Rust ถูกออกแบบให้สามารถตรวจสอบ Memory Safety ตั้งแต่ Compile Time โดยไม่ต้องพึ่ง Garbage Collector เป็นกลไกหลัก จึงต้องทำให้ความสัมพันธ์ระหว่างผู้เป็นเจ้าของข้อมูลกับผู้ที่ยืมข้อมูลมีความชัดเจน ส่วน Python เน้นความเรียบง่ายและความสะดวกในการเขียนโปรแกรม จึงจัดการ Memory ให้โดยอัตโนมัติ ทำให้ผู้เขียนโปรแกรมไม่จำเป็นต้องระบุการ Borrowing ใน Syntax

---

**Comparison Language:** `Java`

| Aspect               | Rust                                                                                                                                                                       | Java                                                                                                                      |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Syntax               | ใช้`&` สำหรับ Reference, `&mut` สำหรับ Mutable Reference และ `*` สำหรับ Dereference                                                          | ใช้ Reference ของ Object โดยตรง แต่ไม่มี Borrowing Syntax แบบ Rust                                 |
| Semantics / Behavior | Compiler ตรวจสอบ Ownership และ Borrowing ตั้งแต่ Compile Time                                                                                             | Reference ใช้เข้าถึง Object และ Object ถูกจัดการโดย Garbage Collector                            |
| Type System          | Statically Typed และมี Reference Type เช่น`&T`, `&mut T` ซึ่งกำหนดลักษณะการเข้าถึงและสิทธิ์ในการแก้ไขข้อมูล | Statically Typed ต้องระบุ Type และ Reference จะระบุชนิดของ Object เช่น String หรือ Object |
| Memory Management    | ใช้ Ownership และ Borrowing โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลัก                                                                    | ใช้ Garbage Collector จัดการ Memory ของ Object โดยอัตโนมัติ                                       |
| Safety               | Compiler ช่วยตรวจสอบ Memory Safety และกฎการ Borrowing                                                                                                   | มี Memory Safety จากการใช้ Reference และ Garbage Collector แต่ไม่มี Borrow Checker แบบ Rust      |

### Rust Example

```rust
fn add_suffix(text: &mut String) {
    text.push_str(" Language");
}

fn main() {
    let mut text = String::from("Rust");

    add_suffix(&mut text);

    println!("{}", text);
}
```

จุดสำคัญ

- `String` เป็นข้อมูลที่ `text` เป็นเจ้าของ
- `&mut String` คือ Mutable Reference ที่อนุญาตให้ Function แก้ไขข้อมูล
- `&mut text` เป็นการ Borrow `text` แบบ Mutable
- Function ไม่ได้รับ Ownership ของ `text` จึงไม่ต้องคืน Ownership กลับมา

### Java Example

```Java
public class Main {
    static void addSuffix(StringBuilder text) {
        text.append(" Language");
    }

    public static void main(String[] args) {
        StringBuilder text = new StringBuilder("Java");

        addSuffix(text);

        System.out.println(text);
    }
}
```

จุดสำคัญ

- `text` เป็น Reference ที่ชี้ไปยัง Object `StringBuilder`
- เมื่อส่ง `text` เข้า Method จะมีการส่งค่าของ Reference ไปไม่ได้ส่งตัวแปร Reference เดิมเข้าไปโดยตรง
- Method สามารถแก้ไข Object ที่ Reference ชี้อยู่ได้
- Memory ของ Object ถูกจัดการโดย Garbage Collector

### Analysis

*ความแตกต่างสำคัญ:* Rust กำหนดสิทธิ์ในการเข้าถึงข้อมูลอย่างชัดเจนผ่าน `&` และ `&mut` และ Compiler ตรวจสอบว่าการ Borrow ไม่ขัดกับกฎของ Ownership ขณะที่ Java ใช้ Reference ที่สามารถอ้างถึง Object และใช้ Garbage Collector จัดการ Memory ของ Object ที่ไม่สามารถเข้าถึงได้แล้ว

*เหตุผลด้านการออกแบบ:* Rust เลือกใช้ Ownership และ Borrowing เพื่อให้สามารถควบคุม Memory และตรวจสอบความปลอดภัยได้ตั้งแต่ Compile Time โดยไม่ต้องมี Garbage Collector ส่วน Java ถูกออกแบบให้เน้น ความง่ายในการพัฒนาและการจัดการ Memory อัตโนมัติ จึงใช้ Garbage Collector เพื่อจัดการ Memory ของ Object ที่ไม่สามารถเข้าถึงได้แล้ว ทำให้ Programmer ไม่ต้องจัดการ Memory ด้วยตนเอง

---

**Comparison Language:** `C`

| Aspect               | Rust                                                                                                                                                                       | C                                                                                                                                                                                                |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Syntax               | ใช้`&` สำหรับ Reference, `&mut` สำหรับ Mutable Reference และ `*` สำหรับ Dereference                                                          | ใช้ Pointer`*` และ Address-of `&` ในการเข้าถึง Memory                                                                                                                      |
| Semantics / Behavior | Compiler ตรวจสอบ Ownership และ Borrowing ตั้งแต่ Compile Time                                                                                             | Pointer สามารถเข้าถึงข้อมูลผ่าน Address และแก้ไขข้อมูลในส่วนที่ Pointer ชี้ได้โดยตรงตามชนิดข้อมูลและการใช้งาน |
| Type System          | Statically Typed และมี Reference Type เช่น`&T`, `&mut T` ซึ่งกำหนดลักษณะการเข้าถึงและสิทธิ์ในการแก้ไขข้อมูล | Statically Typed ต้องระบุ Type ของตัวแปรและ Pointer เช่น`int *p`                                                                                                       |
| Memory Management    | ใช้ Ownership และ Borrowing โดยไม่ต้องใช้ Garbage Collector เป็นกลไกหลัก                                                                    | Programmer สามารถจัดการ Dynamic Memory โดยใช้`malloc()` สำหรับจัดสรร Dynamic Memory และ `free()` สำหรับคืน Memory ที่จัดสรรไว้         |
| Safety               | Compiler ช่วยตรวจสอบกฎของ Ownership และ Borrowing เพื่อเพิ่ม Memory Safety                                                                    | มีความเสี่ยงจาก Pointer เช่น Dangling Pointer และ Memory Leak                                                                                                              |

### Rust Example

```rust
fn double_value(value: &mut i32) {
    *value *= 2;
}

fn main() {
    let mut number = 10;

    double_value(&mut number);

    println!("Number: {}", number);
}
```

จุดสำคัญ

- `&mut i32` คือ Mutable Reference ที่สามารถแก้ไขข้อมูลได้
- `&mut number` เป็นการ Borrow `number` แบบ Mutable
- `*value` ใช้ Dereference เพื่อเข้าถึงค่าที่ Reference ชี้อยู่
- หลังจาก Function ทำงานเสร็จ `number` ยังคงเป็นเจ้าของข้อมูลและสามารถใช้งานต่อได้

### C Example

```C
#include <stdio.h>

void double_value(int *value) {
    *value *= 2;
}

int main() {
    int number = 10;

    double_value(&number);

    printf("Number: %d\n", number);

    return 0;
}
```

จุดสำคัญ

- `int *value` คือ Pointer ที่เก็บ Address ของตัวแปร
- `&number` ใช้หา Address ของ `number`
- `*value` ใช้ Dereference เพื่อเข้าถึงและแก้ไขข้อมูล
- C ให้ Programmer ควบคุม Pointer และ Memory โดยตรง

### Analysis

*ความแตกต่างสำคัญ:* แม้ Rust และ C จะมี `&` และ `*` ที่ดูคล้ายกัน แต่แนวคิดเบื้องหลังแตกต่างกัน โดย C ใช้ Pointer และ Address เพื่อให้ Programmer ควบคุม Memory ได้โดยตรง ขณะที่ Rust ใช้ Reference ภายใต้กฎ Ownership และ Borrowing

*เหตุผลด้านการออกแบบ:* C ถูกออกแบบโดยให้ความสำคัญกับ การควบคุม Hardware และประสิทธิภาพระดับต่ำ จึงเปิดให้เข้าถึง Memory ได้อย่างอิสระ ส่วน Rust ต้องการรักษาความสามารถในการควบคุม Memory และ Performance แบบภาษา System Programming แต่เพิ่ม Compile-time Safety เข้ามา จึงออกแบบ Ownership และ Borrowing เพื่อให้ Compiler ตรวจสอบการใช้ Reference และลดปัญหา เช่น Dangling Pointer และการเข้าถึงข้อมูลที่ไม่ถูกต้อง

---

## 12. Teach Your Topic

การนำเสนอมีสมาชิก **3 คน คนละประมาณ 5 นาที**

Slide Presentation Link : [https://canva.link/group-12-rust-tutorial-presentation](https://canva.link/group-12-rust-tutorial-presentation)

| Member   | Responsibility                          |  Time |
| -------- | --------------------------------------- | ----: |
| Member 1 | Concept + Short Code Illustration       | 5 min |
| Member 2 | Detailed Code + Live Demo               |     - |
| Member 3 | Rust vs Other Languages + PPL Analysis  | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

จัดทำเนื้อหา Concept และ Short Code รวมถึงจัดทำและตรวจสอบ Code ของส่วน Demo

**Member 2**

`ถอนรายวิชา`

**Member 3**

จัดทำเนื้อหา Rust vs Other Languages และ PPL Analysis

**Member 4**

จัดทำ Exercises, Common Mistakes และ Challenge ตามความรับผิดชอบเดิม และจัดทำส่วน Demo ใน README เป็นเอกสารประกอบ

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 13. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. [The Rust Programming Language — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
2. [The Rust Programming Language — Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
3. [Rust Reference — Borrow Expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-expressions)
4. [Rust by Example — Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)

---

## 14. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| Member   | AI Tool                 | Purpose                                                                                                                                 | How the Result Was Verified                                                                                                                                    |
| -------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Member 1 | Gemini, GitHub Copilot | วางแผน, ช่วยเขียน Code และช่วยตรวจสอบ Code, Review                                                        | ตรวจสอบกับ Rust Book,`Compiler`, `cargo check` และตรวจสอบผลลัพธ์ด้วยเอกสารอ้างอิงและการ Compile จริง |
| Member 3 | ChatGPT                 | เรียบเรียงและปรับปรุงคำอธิบายเนื้อหาในหัวข้อ PPL Perspective และ Rust vs. Other Language | ตรวจสอบโดยเทียบกับแหล่งอ้างอิงที่เชื่อถือได้ และทดสอบ Code โดยสมาชิก                              |
| Member 4 | ChatGPT, Claude         | ช่วยตรวจคำผิด การสะกดคำ ไวยากรณ์                                                                          | ให้เพื่อนร่วมทีมตรวจสอบความกระชับและความเข้าใจง่าย                                                           |

### Declaration

- [X] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [X] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [X] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [X] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

สมาชิกใช้ AI ในขั้นตอนการเรียบเรียงและปรับปรุงเนื้อหา รวมถึงการจัดทำและปรับปรุง Code ตัวอย่าง โดยสมาชิกเป็นผู้กำหนดขอบเขตงานและเนื้อหา ตรวจสอบ แก้ไข และเลือกผลลัพธ์จาก AI ก่อนนำมาใช้งาน เนื้อหาถูกตรวจสอบกับแหล่งอ้างอิงที่น่าเชื่อถือ และ Code ถูกทดสอบโดยสมาชิกก่อนนำมาใช้งาน

---

## 15. GitHub Contribution

| Member   | Issues | Commits | Pull Requests | Code Reviews | Contribution                                         |
| -------- | -----: | ------: | ------------: | -----------: | ---------------------------------------------------- |
| Member 1 |      0 |      24 |             7 |           15 | Concept, Short Code และ Code ของ Demo |
| Member 2 |      0 |       0 |             1 |            0 | ถอนรายวิชา                                 |
| Member 3 |      0 |       8 |             3 |            8 | Rust vs Other Language และ PPL Analysis |
| Member 4 |      0 |      15 |             4 |           17 | Common Mistakes, Exercises และ Demo documentation |

### Teamwork Reflection

**How did your team collaborate?**

ทีมแบ่งงานตามหัวข้อของบทเรียน สมาชิกแต่ละคนจัดทำส่วนที่ได้รับมอบหมาย แล้วรวมเนื้อหาและ Code ไว้ใน README เดียวกัน 

**Problems encountered**

ปัญหาที่พบคือสมาชิก 2 ถอนรายวิชา ทำให้ส่วน Detailed Code และ Demo ไม่มีผู้รับผิดชอบ

**How did you solve them?**

ทีมแก้ปัญหาโดยแบ่งงานด้าน Code ให้ Member 1 และงานเขียน Demo ใน README ให้ Member 4 ส่วน Demo จะเก็บไว้เป็นเอกสารประกอบโดยไม่จัดเป็นช่วงนำเสนอแยกต่างหาก

---

## 16. Final Checklist

- [X] Learning Objectives ครบ 3–4 ข้อ
- [X] Key Concepts ครบถ้วน
- [X] Syntax / Rules
- [X] Runnable Code Examples
- [X] Code Compile และ Run ได้จริง
- [X] Common Mistakes
- [X] Exercises 2 ข้อ พร้อม Solutions
- [X] PPL Perspective
- [X] Rust vs Other Language
- [X] References อย่างน้อย 4 แหล่ง
- [X] AI Usage Declaration
- [X] GitHub Contribution
- [X] สมาชิกทั้ง 3 คนมีส่วนร่วม
- [X] สมาชิกทั้ง 3 คนพร้อมนำเสนอคนละ 5 นาที
- [X] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository (Upstream/Main) :** [github.com/soonklang/rust-tutorial-2569](https://github.com/soonklang/rust-tutorial-2569)

**Repository (Forked) :** [github.com/Viseth101/rust-tutorial-2569](https://github.com/Viseth101/rust-tutorial-2569)

**Chapter Path:** `12-references-borrowing/`

**Final PR:** `pull/23`

**Submitted by:** `Group 12`

**Date:** `2569-09-30`
