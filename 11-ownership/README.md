# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม  
> **Topic No.:** `11`  
> **Topic Name:** `Ownership`  
> **Group No.:** `11`
> 
> **ประเด็นหลักที่ควรครอบคลุม:** ownership rules, move, copy, scope, memory management

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | `สิริญญาธร ปุณกะบุตร` | `670710151` | `@670710151` | Concept + Code |
| 2 | `อังกฤษ ถ้ำสุวรรณ` | `670710152` | `@670710152` | Code + Demo |
| 3 | `ภูริณัฐ สุวรรณสังโส` | `670710153` | `@670710153` | Rust vs Other Language + PPL |
| 4 | `Sothea Sokea` | `670710258` | `@sotheasokea` | Exercises + Common Mistakes |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `ทำความเข้าใจกฎพื้นฐานของระบบ Ownership ในภาษา Rust`
2. `นำแนวคิด Ownership มาประยุกต์ใช้ในการเขียนโปรแกรมจริง`
3. `วิเคราะห์ระบบ Ownership ในมุมมองของวิชาหลักการภาษาคอมพิวเตอร์ (PPL)`
4. `พัฒนาทักษะการแก้ปัญหาผ่านการระบุ อธิบาย และแก้ไขข้อผิดพลาดจากคอมไพเลอร์ที่เกี่ยวข้องกับ Ownership`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

1.`Topic นี้คืออะไร`<br>
--> `ระบบ (set of rules) ที่ Rust ใช้จัดการหน่วยความจำ (memory management) โดยไม่ต้องมี Garbage Collector`<br>
2.`ทำไมถึงสำคัญ`<br>
--> `Ownership เป็น แนวคิดที่เป็นเอกลักษณ์ที่สุด ของ Rust และเป็นรากฐานของฟีเจอร์อื่นเกือบทั้งหมดในภาษา (borrowing, lifetimes, smart pointers ล้วนต่อยอดจากแนวคิดนี้)`<br>
-`ปลอดภัยเท่าภาษาที่มี Garbage Collector แต่เร็วเท่าภาษาระดับต่ำ`<br>
-`ตรวจจับ bug ตั้งแต่ compile time`<br>
-`ไม่มี runtime overhead`<br>
3.`ใช้แก้ปัญหาอะไรในการเขียนโปรแกรม`<br>
--> `ช่วยแก้ปัญหาความปลอดภัยของหน่วยความจำที่พบบ่อยในการเขียนโปรแกรม ได้แก่ `<br>
-`dangling pointer (การเข้าถึงหน่วยความจำที่ถูกคืนไปแล้ว) `<br>
-`double free (การคืนหน่วยความจำซ้ำ) `<br>
-`memory leak (การลืมคืนหน่วยความจำ)`<br>
`โดยไม่ต้องแลกกับ performance ของโปรแกรม ทำให้ Rust สามารถให้ทั้งความปลอดภัยและความเร็วไปพร้อมกันได้`<br>

---

## 4. Key Concepts

### 4.1 `Ownership คืออะไร (กฎพื้นฐาน 3 ข้อ)`

**คำอธิบาย**

`Ownership คือระบบจัดการหน่วยความจำของ Rust โดยไม่ใช้ Garbage Collector `<br>
`มีกฎ 3 ข้อ: `<br>
`(1) ทุกๆค่า ใน Rust จะมี "เจ้าของ" (Owner) เสมอ `<br>
`(2) เมื่อ owner หลุด scope ค่านั้นถูก drop ทันที โดยอัตโนมัติ`<br>
`(3) มี owner ได้เพียง "คนเดียว" เท่านั้นในเวลาเดียวกัน`<br>

**ตัวอย่าง**

```rust
fn main() {
    let s = String::from("hello");
    println!("{}", s);
} // s หลุด scope ที่นี่ -> ถูก drop อัตโนมัติ
```

**Explanation**

`ตัวแปร s เป็นเจ้าของค่า "hello" บน heap เมื่อโค้ดมาถึงปิดวงเล็บ } ซึ่งเป็นจุดที่ s หลุดออกจาก scope`<br>
`Rust จะเรียก drop() ให้อัตโนมัติเพื่อคืนหน่วยความจำ โดยไม่ต้องเขียน free() เอง`<br>

---

### 4.2 `Move Semantics`

`Move คือการ "ย้าย" ความเป็นเจ้าของ (ownership) จากตัวแปรเดิมไปยังตัวแปรใหม่`<br>
`Rust ออกแบบให้ตัวแปรเดิมใช้งานต่อไม่ได้ทันทีหลัง move เพื่อป้องกันปัญหา double free`<br>
`ช่วยแก้ปัญหาการที่สองตัวแปรชี้ไปยังข้อมูล heap เดียวกัน แล้วพยายาม drop ข้อมูลซ้ำตอน scope จบ`<br>
`ตัวแปรที่ถูก move แล้ว compiler จะบล็อกไม่ให้ใช้ต่อ (compile error ทันที)`<br>

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // ownership ย้ายจาก s1 ไป s2
    println!("{}", s2);
}
```

**Explanation**

`หลังบรรทัด let s2 = s1; ความเป็นเจ้าของถูกย้ายจาก s1 ไปยัง s2 ถ้าพยายามใช้ s1 ต่อ `<br>
`เช่น println!("{}", s1) จะเกิด compile error ทันที เพราะ Rust ไม่ยอมให้มีสอง owner ชี้ไปยังข้อมูลก้อนเดียวกัน` <br>
`ป้องกันปัญหาที่ทั้งสองตัวแปรจะพยายาม drop ข้อมูลเดียวกันซ้ำ`<br>


---

### 4.3 `Copy & Clone`

`Copy Trait: type พื้นฐานบน stack (i32, bool, char, f64) จะถูก copy อัตโนมัติแทนการ move เพราะคัดลอกแบบcost น้อย `<br>
`Clone: สำหรับข้อมูลบน heap (เช่น String) ต้องเรียก .clone() เพื่อคัดลอกข้อมูลจริงแบบ deep copy อย่างชัดเจน`<br>
`ช่วยแก้ปัญหากรณีต้องการใช้ตัวแปรสองตัวพร้อมกัน โดยไม่ทำให้ตัวแปรเดิมถูก move ทิ้ง`<br>

```rust
fn main() {
    let x = 5;
    let y = x;              // copy อัตโนมัติ (stack)
    let s1 = String::from("hello");
    let s2 = s1.clone();    // ต้องเรียก clone เอง (heap)
    println!("{} {} {} {}", x, y, s1, s2);
}
```

**Explanation**

`เนื่องจาก i32 มีขนาดคงที่และอยู่บน stack การคัดลอกค่ามีต้นทุนต่ำมาก Rust จึงอนุญาตให้ x และ y ใช้งานได้พร้อมกันโดยไม่ error` <br>
`s1.clone() คัดลอกข้อมูลบน heap ทั้งหมดไปสร้างเป็นก้อนใหม่ให้ s2 ทำให้ s1 และ s2 ต่างมีข้อมูลของตัวเองแยกกันคนละก้อน จึงใช้งานได้พร้อมกันทั้งคู่โดยไม่เกิด error`<br>


---

### 4.4 `Scope`

`Scope คือขอบเขตของตัวแปรในโปรแกรม ตั้งแต่จุดที่ถูกประกาศจนถึงจุดที่ปิดวงเล็บ {}`<br>
`Rust ผูก ownership เข้ากับ scope โดยตรง ตัวแปรมีผลใช้งานได้เฉพาะภายใน scope ของมันเท่านั้น`<br>
`ช่วยแก้ปัญหาการจัดการหน่วยความจำแบบ Manual เพราะไม่ต้องกำหนดจุดคืนหน่วยความจำเอง(free)`<br>
`เมื่อออกจาก scope ตัวแปรที่เป็น owner จะถูก drop ทันทีโดยอัตโนมัติ`<br>

```rust
fn main() {
    {
        let s = String::from("hello"); // s เริ่มมีผล
        println!("{}", s);
    } // s หลุด scope ที่นี่ -> ถูก drop
    // println!("{}", s); //  error: s ไม่มีผลแล้วนอก scope
}
```
**Explanation**

`ตัวแปร s มีผลใช้งานได้เฉพาะภายใน block {} ที่ประกาศเท่านั้น เมื่อโปรแกรมมาถึง } ซึ่งเป็นจุดสิ้นสุด scope ของ s`<br>
`Rust จะ drop ค่านั้นให้อัตโนมัติทันที ถ้าเรียกใช้ s นอก scope จะเกิด compile error เพราะ ownership ผูกติดกับ scope โดยตรง`<br>

---

### 4.5 `Memory Management`

`Memory Management คือการจัดการหน่วยความจำที่โปรแกรมขอใช้ (allocate) และคืน (deallocate) ให้ระบบ`<br>
`Rust ไม่ใช้ Garbage Collector และไม่ให้เขียน free() เอง แต่ผูกการคืนหน่วยความจำเข้ากับ Ownership + Scope โดยตรง`<br>
`ช่วยแก้ปัญหา runtime overhead ของภาษาที่มี GC และปัญหาความผิดพลาดจากการจัดการเองแบบ C/C++`<br>
`compiler แทรกการเรียก drop() ให้อัตโนมัติตอน compile time ทำให้ปลอดภัยโดยไม่มี cost ตอน runtime`<br>

```rust
fn main() {
    let s = String::from("hello"); // ขอหน่วยความจำบน heap
    println!("{}", s);
} // ออกจาก scope -> Rust คืนหน่วยความจำอัตโนมัติ ไม่ต้องเขียน free()
```
**Explanation**

`เมื่อ main() จบการทำงาน Rust ไม่ต้องรอ Garbage Collector และไม่ต้องเขียน free() เอง`<br>
`compiler จะแทรกการเรียก drop() ให้อัตโนมัติตอน compile time ทันทีที่ owner หลุด scope ทำให้คืนหน่วยความจำได้แน่นอน ไม่มี runtime overhead และไม่เสี่ยง memory leak`<br>

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `let x = y;` | `Assign ค่า — ถ้า y เป็น type บน heap (เช่น String) จะเกิด move; ถ้าเป็น type ที่มี Copy trait จะ copy อัตโนมัติ` | `let s2 = s1;` |
| `.clone()` | `คัดลอกข้อมูลบน heap แบบ deep copy ทำให้ทั้งสองตัวแปรใช้งานได้พร้อมกัน` | `let s2 = s1.clone();` |
| `{ }` | `กำหนด scope ของตัวแปร — เมื่อปิด block ตัวแปรที่เป็น owner ภายในจะถูก drop อัตโนมัติ` | `{ let s = String::from("hi"); }` |
| `drop()` | `กฟังก์ชันที่ Rust เรียกอัตโนมัติเมื่อ owner หลุด scope เพื่อคืนหน่วยความจำ (ไม่ต้องเรียกเอง)` | `เรียกอัตโนมัติตอนปิด }` |

### Important Rules

1. `ทุกๆค่า ใน Rust จะมี "เจ้าของ" (Owner) เสมอ`
2. `เมื่อ owner หลุด scope ค่านั้นถูก drop ทันที โดยอัตโนมัติ`
3. `มี owner ได้เพียง "คนเดียว" เท่านั้นในเวลาเดียวกัน`



---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `Move`

**Purpose:** `การย้าย ownership`

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
}
```

**Expected Output**

```text
hello
```

**Explanation**

`ตอนแรก s1 ownership อยู่กับ s1 พอประกาศ s2 = s1 ownership ก็ถูกย้ายไปอยู่ที่ s2 s1ก็จะไม่สามารถใช้ได้`

---

### Example 2 — `Clone`

**Purpose:** `การ clone ค่าเพื่อที่จะนำไปใช้ใน function โดยที่ownershipไม่ถูกย้ายเข้าไป function ด้วย `

```rust
fn main(){
  let s: String = String::from("Hello, world!");
  print_string(s.clone());
  println!("{}", s); 
}


fn print_string(s: String) {
  println!("{}", s);
}
```

**Expected Output**

```
Hello, world!
Hello, world!
```

**Explanation**

`ในตอนที่เราใช้ function print_string แล้วรับparameterไป ownership ก็จะถูกย้ายไปที่ print_string ทำให้ค่า s ใน main ใช้ไม่ได้ เราจึง clone s แล้วค่อยส่งเป็น parameterไปในfunction ค่าที่ส่งก็จะเป็นค่าที่ copy มาแล้วก็จะถูก drop ตอนจบ functionไป ค่า s ใน main ก็จะไม่ถูกแตะ ownership ก็ไม่ถูกย้าย

`

---

## 7. Common Mistakes

### Mistake 1 — `การพยายามใช้ค่าที่ถูกย้าย (Move) ไปแล้ว`

**Problem**

`เมื่อ my_name ถูกส่งไปยังฟังก์ชัน print_name() สิทธิ์ความเป็นเจ้าของ (ownership) ของ String จะถูกย้ายไปยังฟังก์ชันนั้น ดังนั้น my_name จึงไม่สามารถนำมาใช้งานต่อใน main() ได้`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_1_moved_value_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_1_moved_value_correct.rs)

**Why?**

`.clone() จะสร้าง สำเนาแบบ deep copy ของ String จัดสรรหน่วยความจำ heap ใหม่ แต่มีเนื้อหาเดียวกัน ตัว clone นี่แหละที่จะถูกย้ายเข้าไปใน print ส่วน message ตัวเดิมใน main ไม่ถูกแตะต้องเลย จึงยังใช้งานต่อได้หลังจากนั้น`


---

### Mistake 2 — `เข้าใจผิดว่าการ assign คือการ copy ทั้งที่จริงๆ คือการ move`

**Problem**

`มาจากภาษาอย่าง Python, Java หรือ JS การเขียน let s2 = s1; อาจดูเหมือนแค่สร้างตัวแปรตัวที่สองที่ชี้ไปยังข้อมูลเดียวกัน แล้วใช้ได้ทั้งสองชื่อ แต่ใน Rust สำหรับ type ที่ไม่ใช่ Copy นี่คือการ move ไม่ใช่การ copy s1 จะใช้งานไม่ได้ทันทีที่ s2 ถูกสร้างขึ้น`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_2_confusing_assign_and_copy_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_2_confusing_assign_and_copy_correct.rs)

**Why?**

`ใช้ .clone() ถ้าต้องการให้มีเจ้าของสองตัวจริงๆ ที่เป็นอิสระจากกัน หรือใช้แค่ s2 ต่อไป แล้วเลิกพยายามใช้ s1`

---
### Mistake 3 — `Move บางส่วนออก struct (Partial move)`

**Problem**

`การย้าย field เดียวออกจาก struct จะทำให้ struct นั้น "ใช้งานไม่ได้บางส่วน" จะใช้ struct ทั้งก้อน (หรือ field ที่ถูกย้ายไปนั้น) อีกไม่ได้ ถึงแม้ field อื่นๆ จะยังใช้งานได้ปกติก็ตาม จุดนี้มักทำให้คนงงตอนแรกที่เจอ เพราะ error message อาจดูสับสน struct ยัง "มีอยู่" แต่บาง field ในนั้นใช้ไม่ได้แล้ว`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_3_partial_move_from_struct_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_3_partial_move_from_struct_correct.rs)

**Why?**

`clone field นั้นถ้าต้องการใช้ทั้งสองที่ หรือ destructure struct ทั้งหมดแล้วสร้างใหม่ตามที่ต้องการ หรือจัดโครงสร้างโค้ดใหม่ให้การ move เกิดขึ้นเป็นลำดับสุดท้าย`

---
### Mistake 4 — `Move ค่าเข้าไปใน loop แล้วพยายามใช้ซ้ำ`

**Problem**

`การเรียก greet(name) ครั้งแรกจะย้าย name เข้าไปในฟังก์ชัน พอถึงรอบถัดไปของ loop name ก็ไม่มีอยู่แล้ว compiler จะฟ้องว่าการเรียกครั้งที่สองใช้ค่าที่ถูกย้ายไปแล้ว นี่เป็นข้อผิดพลาดที่พบบ่อยมากเวลาแปลงโค้ดแบบ "loop ที่ใช้ตัวแปรซ้ำ" มาจากภาษาอื่น`


**Incorrect Code**

[View the incorrect code](./code/common-mistake/mistake_4_moved_value_in_loop_incorrect.rs)

**Correct Code**

[View the correct code](./code/common-mistake/mistake_4_moved_value_in_loop_correct.rs)

**Why?**

`clone ข้างในลูปถ้าต้องการสำเนาใหม่ทุกรอบ หรือจัดโครงสร้างโค้ดใหม่ให้ฟังก์ชันรับค่าไปแล้ว return กลับมา`

---
### Mistake 5 — `Anti-Pattern: "Clone ทุกอย่าง"`

**Problem**

`แม้จะไม่ใช่ข้อผิดพลาดระดับคอมไพเลอร์ แต่นี่คือข้อผิดพลาดทางพฤติกรรม เมื่อ Borrow Checker แจ้งเตือนข้อผิดพลาด ผู้เริ่มต้นมักจะใส่ .clone() ไว้ในทุกตัวแปรเพียงเพื่อบังคับให้โค้ดสามารถคอมไพล์ผ่าน`


**Incorrect Code**

[View the incorrect (not recommended) code](./code/common-mistake/mistake_5_clone_everything_incorrect.rs)

**Correct Code**

[View the correct (recommended) code](./code/common-mistake/mistake_5_clone_everything_correct.rs)

**Why?**

`การถอยกลับมาทบทวนโครงสร้างโปรแกรมใหม่: พิจารณาว่าตัวแปรใดควรเป็นเจ้าของข้อมูลอย่างแท้จริง และให้ส่วนที่เหลือในโค้ดทำการยืม (Borrow) ไปใช้แทน`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `Clone or Lose It`

**Problem**

[View Problems](./code/Exercise/exercise1_Clone_or_Lose_IT.md)

**Hint**

`describe` รับ `item: String` แบบ by value ดังนั้นการเรียก `describe(item)` จะย้ายความเป็นเจ้าของออกไปจาก `item` ใน `main` ต้องหาวิธีที่ทำให้ `item` ยังใช้งานได้หลังจากนั้น โดยไม่เปลี่ยน signature ของ `describe` มีคำสั่งอะไรที่ช่วยให้คุณส่ง *สำเนา* ไปแทนตัวจริงได้บ้าง?


**Solution**

[View Solution](./code/Exercise/exercise1_solution.rs)

**Explanation**

เนื่องจากโจทย์มีข้อบังคับว่าห้ามเปลี่ยนโครงสร้างของฟังก์ชัน describe (ไม่สามารถเปลี่ยนให้ไปรับค่าแบบยืม หรือ Reference &String ได้) ฟังก์ชันนี้จึง บังคับ ว่าต้องรับสิทธิ์ความเป็นเจ้าของไปเท่านั้น

+วิธีแก้คือการใช้คำสั่ง .clone() เมื่อเราเรียกใช้ describe(item.clone()):

>โปรแกรมจะสร้างสำเนาของข้อความ "Book" ขึ้นมาใหม่ในหน่วยความจำ Heap อย่างสมบูรณ์แบบและแยกขาดจากกัน

>ฟังก์ชัน describe จะรับเอาสิทธิ์ความเป็นเจ้าของของ ตัวสำเนา นี้ไปใช้แทน และทำลายตัวสำเนานั้นทิ้งเมื่อฟังก์ชันทำงานจบ

>ตัวแปร item ต้นฉบับที่อยู่ใน main จะไม่เคยถูกย้ายสิทธิ์หรือถูกแตะต้องเลย มันจึงยังคงใช้งานได้ตามปกติและสามารถนำมาสั่งพิมพ์ในบรรทัดสุดท้ายได้

---

### Exercise 2 — `The Half-Moved Book`

**Problem**

[View Problems](./code/Exercise/exercise2_The_Half_Moved_Book.md)

**Hint**

`book.title` ถูกย้ายเข้าไปใน `make_label(book.title)` หลังจากบรรทัดนั้น `book.title` ยังใช้งานได้อยู่ไหม? แล้ว field อื่นของ `book` (เช่น `book.author`) ยังใช้ได้ปกติหรือเปล่า? นี่เป็นปัญหาแบบเดียวกับการเข้าถึง field ของ struct หลังจากบางส่วนถูกย้ายไปแล้ว ลองคิดดูว่า field ไหนที่ต้องรอดจนถึงหลังจากเรียกฟังก์ชันนั้น แล้วจะทำยังไงให้มันรอด


**Solution**

[View Solution](./code/Exercise/exercise2_solution.rs)

**Explanation**

`ปัญหาคือ **partial move**: `book.title` ถูกย้ายเข้าไปใน `make_label` ดังนั้นหลังจากบรรทัดนั้น `book.title` จะใช้งานต่อใน `println!` ที่อ้างอิงถึง `book.title` อีกครั้งไม่ได้ ส่วน `book.author` ไม่ได้รับผลกระทบเพราะไม่ได้ถูกแตะต้อง`


---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

ในภาษา Rust รูปแบบไวยากรณ์ (Syntax) ถูกออกแบบให้รองรับกฎของ Ownership ผ่านระบบ **Variable Binding** โดยการประกาศตัวแปรและการกำหนดค่าใช้ Syntax ปกติ เช่น `let x = String::from("hello");` ซึ่งเป็นการสร้าง Binding ระหว่างชื่อ `x` กับ Value ที่ตัวแปรเป็นเจ้าของ

ในกรณีของ Type ที่มีการจัดการ Resource เช่น `String` ตัวแปรจะมี Ownership เหนือข้อมูลที่เกี่ยวข้องกับ Resource นั้น และ Rust ไม่จำเป็นต้องใช้ Syntax สำหรับการจัดการหน่วยความจำด้วยตนเอง เช่น `free()` ในภาษา C โดยทั่วไปการสิ้นสุดการใช้งาน Resource จะถูกจัดการตามกฎของ Ownership และ Scope

นอกจากนี้ Syntax ของการกำหนดค่า เช่น `let y = x;` สามารถเกี่ยวข้องกับการ **Move** ของ Ownership ได้ โดย `x` จะไม่สามารถถูกใช้งานต่อในลักษณะเดิมหลังจาก Ownership ถูกย้ายไปยัง `y` สำหรับ Type ที่ไม่ได้มีพฤติกรรมแบบ `Copy`

ดังนั้น แม้ Syntax ของ Rust ในระดับพื้นฐานจะมีรูปแบบคล้ายกับการประกาศและกำหนดค่าของภาษาอื่น แต่การดำเนินการเหล่านี้อยู่ภายใต้กฎ Ownership ของภาษา

### 9.2 Semantics

ในเชิงความหมายและพฤติกรรม (Semantics) Rust กำหนดให้การดำเนินการกับ Value บางรูปแบบมีผลต่อสถานะความเป็นเจ้าของของตัวแปร โดยเฉพาะกรณีของ Type ที่ไม่ได้เป็น `Copy` การกำหนดค่า เช่น `let y = x;` จะทำให้ Ownership ถูก **Move** จาก `x` ไปยัง `y`

แนวคิดนี้แตกต่างจากการตีความการกำหนดค่าในบางภาษา เช่น C++ ซึ่งสามารถมีการ Copy Object หรือ Copy ค่าได้ตามชนิดและรูปแบบการเขียนโปรแกรม โดย Rust ใช้ Move Semantics เป็นกลไกสำคัญในการควบคุมการเป็นเจ้าของ Resource

ในกรณีที่มีการส่ง Value เข้า Function การส่ง Value ที่เป็น Ownership จะสามารถทำให้ Ownership ย้ายไปยัง Parameter ของ Function ได้เช่นกัน หากโปรแกรมพยายามใช้ตัวแปรเดิมหลังจาก Ownership ถูก Move ไปแล้ว Compiler จะปฏิเสธโปรแกรมในช่วง Compile Time

Semantics ดังกล่าวทำให้การเปลี่ยนแปลง Ownership เป็นส่วนหนึ่งของพฤติกรรมของโปรแกรม ไม่ใช่เพียงการเปลี่ยนค่าของตัวแปร และช่วยป้องกันการใช้งาน Resource ที่ไม่ถูกต้อง เช่น การใช้ข้อมูลหลังจาก Ownership ถูกย้ายไปแล้ว

### 9.3 Type System

Rust ใช้กลไกของ Type System ร่วมกับ Ownership และ Borrow Checking เพื่อควบคุมการใช้งาน Value อย่างปลอดภัย โดยแนวคิดของ Rust สามารถอธิบายได้ในลักษณะของ **Affine Type System** ซึ่ง Value ที่เป็น Resource ไม่สามารถถูกนำไปใช้ในลักษณะที่ละเมิดกฎการเป็นเจ้าของได้อย่างอิสระ

ตัวอย่างเช่น Type อย่าง `String` มี Ownership และเมื่อ Ownership ถูก Move ไปยังตัวแปรอื่น ตัวแปรเดิมจะไม่สามารถนำมาใช้งานต่อในลักษณะที่ขัดกับกฎของภาษาได้ ในขณะที่ Type บางชนิด เช่น `i32` สามารถใช้พฤติกรรมแบบ `Copy` ทำให้สามารถสร้างค่าซ้ำได้โดยไม่เป็นการย้าย Ownership

Compiler ของ Rust ใช้การวิเคราะห์แบบ Static Analysis ในช่วง Compile Time เพื่อตรวจสอบกฎที่เกี่ยวข้องกับ Ownership และ Borrowing หากโปรแกรมละเมิดกฎดังกล่าว Compiler จะปฏิเสธโปรแกรมก่อนการ Execute

กลไกนี้มีส่วนช่วยป้องกันปัญหาด้าน Memory Safety หลายประเภท และในกรณีของ **Safe Rust** ยังช่วยป้องกัน Data Race ที่เกิดจากการเข้าถึงข้อมูลร่วมกันอย่างไม่ปลอดภัย

### 9.4 Memory / Resource Management

การจัดการหน่วยความจำ (Memory Management) คือกระบวนการจัดสรร (Allocation) และจัดการการสิ้นสุดการใช้งาน (Deallocation) ของหน่วยความจำหรือ Resource เมื่อโปรแกรมไม่ต้องการใช้งานอีกต่อไป แนวคิด Ownership ของ Rust เชื่อมโยงอายุการใช้งานของ Resource เข้ากับ Owner และ Scope ของตัวแปร

เมื่อ Owner ออกจาก Scope Rust จะดำเนินการทำลาย Value ตามกฎของภาษา โดยสำหรับ Type ที่มี `Drop` จะมีการเรียกกระบวนการ `drop` เพื่อจัดการ Resource ที่เกี่ยวข้องโดยอัตโนมัติ

แนวทางนี้มีลักษณะใกล้เคียงกับแนวคิด **RAII (Resource Acquisition Is Initialization)** ที่พบในภาษา C++ โดย Resource จะถูกผูกเข้ากับอายุการใช้งานของ Object/Value แทนที่จะต้องให้ Programmer เรียกคำสั่งคืนหน่วยความจำด้วยตนเอง

ผลที่ได้คือการจัดการ Resource มีลักษณะที่คาดการณ์ได้ (Deterministic) และไม่จำเป็นต้องใช้ Garbage Collector เพื่อค้นหา Object ที่ไม่มีการใช้งานแล้ว

### 9.5 Abstraction / Other PPL Concepts

Ownership สามารถมองได้ว่าเป็น **Abstraction สำหรับ Resource Management** ที่ทำให้ Programmer สามารถควบคุมอายุการใช้งานและความรับผิดชอบต่อ Resource ผ่านกฎของภาษา แทนที่จะต้องจัดการ Pointer และการคืน Resource ด้วยตนเองในระดับต่ำ

แนวคิดดังกล่าวเชื่อมโยงกับแนวคิดพื้นฐานของ PPL หลายประการ ได้แก่

**Scope & Binding:**
Scope กำหนดขอบเขตการใช้งานของ Binding และมีความสัมพันธ์กับช่วงเวลาที่ตัวแปรสามารถเป็น Owner ของ Value ได้ ดังนั้น Binding ใน Rust จึงมีความหมายมากกว่าการจับคู่ชื่อกับค่า แต่ยังสัมพันธ์กับสถานะและความรับผิดชอบต่อ Resource

**Aliasing & Mutability:**
การมีหลายส่วนของโปรแกรมเข้าถึงข้อมูลเดียวกัน (Aliasing) พร้อมกับการแก้ไขข้อมูล (Mutability) สามารถทำให้เกิดปัญหาด้าน Memory Safety ได้ Rust จึงใช้ Ownership และ Borrowing Rules เป็นส่วนหนึ่งของกลไกในการควบคุมการเข้าถึงข้อมูลดังกล่าว โดยรายละเอียดของ References และ Borrowing จะกล่าวถึงในหัวข้อที่เกี่ยวข้องโดยเฉพาะ


### 9.6 Why Rust?

Rust เลือกใช้ Ownership เป็นส่วนสำคัญของการออกแบบภาษาเพื่อสร้าง Memory Safety ผ่านการตรวจสอบในช่วง Compile Time โดยไม่จำเป็นต้องใช้ Garbage Collector เป็นกลไกหลักในการจัดการ Memory

แนวทางนี้ช่วยให้ Rust สามารถจัดการ Resource ได้อย่างเป็นระบบและมีค่าใช้จ่ายขณะ Runtime ที่คาดการณ์ได้ ขณะเดียวกันยังสามารถป้องกัน Memory-Safety Bugs หลายประเภท เช่น การใช้ข้อมูลหลังจาก Ownership ถูกย้ายไปแล้ว และปัญหาบางประเภทที่เกี่ยวข้องกับการเข้าถึง Memory อย่างไม่ปลอดภัย

ดังนั้น Ownership จึงเป็นตัวอย่างสำคัญของการออกแบบ Programming Language ที่นำกฎด้าน Resource Management และ Safety เข้ามาเป็นส่วนหนึ่งของภาษาและ Compiler แทนที่จะพึ่งพาการตรวจสอบโดย Programmer หรือ Garbage Collector เพียงอย่างเดียว

---

## 10. Rust vs. Other Language

เปรียบเทียบตามแนวคิด Memory Management แบ่งเป็น 3 กลุ่ม คือ <br>
1. Manual Memory Management คือ Programmer มีบทบาทในการจัดการ lifetime/resource เอง เช่น malloc/free, new/delete หรือใช้ RAII/smart pointers ใน C++ เช่น C, C++ <br>
2. Automatic Memory Management คือ Runtime / Garbage Collector ช่วยจัดการ memory lifetime เช่น Java, Python, C#, Go <br>
3. Ownership-based Memory Management  คือ Ownership + Borrow Checking ตรวจสอบกฎสำคัญตอน Compile Time โดยไม่ต้องใช้ GC สำหรับ memory management ปกติ เช่น Rust 


| Aspect                     | C                            | C++                                | Java       | Python                 | C#         | Go                           | **Rust**                  |
| -------------------------- | ---------------------------- | ---------------------------------- | ---------- | ---------------------- | ---------- | ---------------------------- | ------------------------- |
| Memory Model               | Manual                       | Manual + RAII                      | GC         | GC / ref counting + GC | GC         | GC                           | **Ownership**             |
| Memory Deallocation        | Programmer                   | RAII / programmer / smart pointers | GC         | Automatic              | GC         | GC                           | **Scope / Drop**          |
| Garbage Collector          | No                           | No                                 | Yes        | Yes*                   | Yes        | Yes                          | **No**                    |
| Ownership Checking         | No built-in ownership system | No Rust-like ownership system      | No         | No                     | No         | No                           | **Yes**                   |
| Compile-time Memory Safety | Limited                      | Depends on usage/features          | Partial    | Partial                | Partial    | Stronger runtime/type safety | **Strong**                |
| Main Trade-off             | Control vs safety            | Control + abstractions             | Runtime GC | Runtime management     | Runtime GC | Runtime GC                   | **Compile-time checking** |

### 10.1 การจัดการหน่วยความจำด้วยตนเอง Manual Memory Management

#### C — Normal

ในภาษา C โปรแกรมเมอร์จะต้องเป็นผู้จอง (Allocate) และคืน (Release) พื้นที่ในหน่วยความจำด้วยตัวเองอย่างชัดเจน

```c
#include <stdio.h>
#include <stdlib.h>

void print_message(char *message) {
    printf("%s\n", message);
    free(message);
}

int main() {
    char *message = malloc(20);

    if (message != NULL) {
        snprintf(message, 20, "Hello C");
        print_message(message);
    }

    return 0;
}
```

โปรแกรมเมอร์มีหน้าที่รับผิดชอบในการดูแลช่วงอายุ (Lifetime) ของหน่วยความจำที่ถูกจองไว้

#### C — ตัวอย่างความเสี่ยง Risk Example

If the programmer accesses memory after it has been released, a use-after-free bug can occur.

```c
#include <stdio.h>
#include <stdlib.h>

void print_message(char *message) {
    printf("%s\n", message);
    free(message);
}

int main() {
    char *message = malloc(20);

    if (message != NULL) {
        snprintf(message, 20, "Hello C");

        print_message(message);

        printf("%s\n", message); // Use-after-free
    }

    return 0;
}
```

หากโปรแกรมเมอร์เข้าถึงหน่วยความจำหลังจากที่มันถูกคืนไปแล้ว อาจทำให้เกิดบั๊กประเภท Use-after-free ได้

---

#### Rust — Ownership

Rust ใช้กฎความเป็นเจ้าของ (Ownership Rules) ในการควบคุมวิธีใช้งานค่าต่าง ๆ และกำหนดว่าทรัพยากรเหล่านั้นจะถูกคืนเมื่อใด

```rust
fn print_message(message: String) {
    println!("{}", message);
}

fn main() {
    let message = String::from("Hello Rust");

    print_message(message);

    // println!("{}", message);
    // Compile-time error: use of moved value
}
```

ในตัวอย่างนี้ ความเป็นเจ้าของของ message ได้ถูกย้าย (Move) เข้าไปในฟังก์ชัน print_message() แล้วคอมไพเลอร์ (Compiler) จะป้องกันไม่ให้ตัวแปรเดิมถูกนำกลับมาใช้งานอีกหลังจากที่ถูกย้ายไปแล้ว

---

### 10.2 การจัดการหน่วยความจำอัตโนมัติ Automatic Memory Management

ภาษาอย่าง Java จะใช้ระบบการจัดการหน่วยความจำแบบอัตโนมัติ

#### Java — Normal

```java
class Data {
    String message;

    Data(String message) {
        this.message = message;
    }
}

public class Main {
    public static void main(String[] args) {
        Data d = new Data("Hello Java");

        System.out.println(d.message);

        d = null; // Object may become unreachable
    }
}
```

โปรแกรมเมอร์ไม่จำเป็นต้องเรียกใช้ฟังก์ชันอย่าง free() เพื่อคืนออบเจกต์ด้วยตนเอง

เมื่อออบเจกต์นั้นไม่สามารถเข้าถึงได้อีกต่อไป (Unreachable) ในภายหลังมันจะถูกเก็บกวาดและคืนพื้นที่โดยตัวรวบรวมขยะ (Garbage Collector หรือ GC)

#### Java —  ข้อดีข้อเสียในเชิงแนวคิด Conceptual Trade-off Java

```text
ออบเจกต์เข้าถึงไม่ได้แล้ว (Unreachable)
        ↓
GC ตรวจพบออบเจกต์นั้น
        ↓
Runtime คืนพื้นที่หน่วยความจำ

```

แนวทางนี้ช่วยลดภาระหน้าที่ของโปรแกรมเมอร์ในการคืนหน่วยความจำด้วยตัวเอง แต่ส่งผลให้การจัดการหน่วยความจำกลายไปเป็นภาระงานส่วนหนึ่งของระบบรันไทม์ (Runtime System) แทน

---

### 10.3 การจัดการหน่วยความจำด้วยระบบความเป็นเจ้าของ Ownership-Based Memory Management

Rust ใช้กฎความเป็นเจ้าของและการตรวจสอบตั้งแต่ตอนคอมไพล์ (Compile-time Checking) แทนที่จะพึ่งพา Garbage Collector ในการจัดการหน่วยความจำทั่วไป

#### Rust — Ownership

```rust
fn main() {
    let s = String::from("hello");

    let t = s;

    // println!("{}", s);
    // Compile-time error: use of moved value
}
```

เมื่อ s ถูกกำหนดค่าให้กับ t ความเป็นเจ้าของของ String จะถูกย้ายไป

คอมไพเลอร์จะคอยติดตามและตรวจสอบกฎข้อนี้ตั้งแต่ก่อนที่โปรแกรมจะทำงาน

---

#### Rust — ขอบเขตและการทำลายค่า Scope and Drop

```rust
fn main() {
    {
        let data = String::from("hello");

        println!("{}", data);
    }

    // data is dropped at the end of its scope
}
```

เมื่อ data หลุดออกนอกขอบเขต (Scope) Rust จะเรียกใช้งานพฤติกรรมการทำลายค่า (Drop) ที่เหมาะสมให้กับค่านั้นโดยอัตโนมัติ

สิ่งนี้ทำให้ Rust มีพฤติกรรมการจัดการทรัพยากรที่คาดเดาได้แน่นอน (Deterministic) โดยไม่จำเป็นต้องใช้ Garbage Collector สำหรับค่าทั่วไปที่ถูกจัดการด้วยระบบความเป็นเจ้าของ

---

### 10.4ข้อดีข้อเสียด้านประสิทธิภาพและรันไทม์ Performance and Runtime Trade-off

รูปแบบการจัดการหน่วยความจำส่งผลกระทบต่อจำนวนภาระงานที่ระบบรันไทม์ (Runtime) ต้องแบกรับด้วยเช่นกัน

#### Java

Java ใช้ Garbage Collector ในการตามเก็บกวาดออบเจกต์ที่ไม่สามารถเข้าถึงได้แล้วโดยอัตโนมัติ

```text
โปรแกรม (Program)
   ↓
จองพื้นที่ออบเจกต์
   ↓
ออบเจกต์เข้าถึงไม่ได้แล้ว
   ↓
ตัวรวบรวมขยะ (Garbage Collector)
   ↓
คืนพื้นที่หน่วยความจำ

```

การเก็บกวาดขยะช่วยให้การจัดการหน่วยความจำสะดวกและเป็นอัตโนมัติ แต่ก็ต้องแลกมาด้วยภาระงานของรันไทม์ (Runtime Work) ที่เกิดขึ้นจากการคอยติดตามและเก็บกวาดออบเจกต์เหล่านั้น

#### Rust

Rust ทำการตรวจสอบกฎความเป็นเจ้าของและการยืมข้อมูล (Borrowing Rules) ตั้งแต่ขั้นตอนการคอมไพล์เป็นหลัก

```text
ซอร์สโค้ด Rust (Rust Source Code)
       ↓
คอมไพเลอร์ตรวจสอบกฎความเป็นเจ้าของ
       ↓
โปรแกรมที่คอมไพล์เสร็จแล้ว (Compiled Program)
       ↓
รันไทม์ (Runtime)
       ↓
ไม่จำเป็นต้องมี GC สำหรับการจัดการความเป็นเจ้าของทั่วไป

```

เนื่องจาก Rust ไม่จำเป็นต้องใช้ Garbage Collector สำหรับการจัดการหน่วยความจำที่อยู่ภายใต้ระบบความเป็นเจ้าของทั่วไป การจัดการทรัพยากรจึงมีความเสถียรและคาดเดาประสิทธิภาพได้ง่ายกว่าในระหว่างที่โปรแกรมทำงาน

```rust
fn main() {
    let data = String::from("Hello Rust");

    println!("{}", data);

} // data is dropped here
```

อย่างไรก็ตาม สิ่งนี้ไม่ได้หมายความว่า Rust จะเร็วกว่า Java เสมอไป เพราะประสิทธิภาพที่แท้จริงจะขึ้นอยู่กับตัวโปรแกรม, ลักษณะของภาระงาน (Workload), การออปติไมซ์ของคอมไพเลอร์, รูปแบบการจัดสรรหน่วยความจำ (Allocation Patterns) 
และพฤติกรรมของรันไทม์ทว่า โมเดลความเป็นเจ้าของของ Rust ช่วยให้การตรวจสอบความปลอดภัยของหน่วยความจำเสร็จสิ้นลงตั้งแต่ตอนคอมไพล์ โดยสามารถหลีกเลี่ยงความจำเป็นในการใช้ Garbage Collector แบบทั่วไปได้.

---

### 10.5 โจทย์เดียวกัน แต่การออกแบบภาษาต่างกัน Same Problem, Different Language Design

เราสามารถเขียนโปรแกรมเพื่อทำงานชิ้นเดียวกันได้ โดยใช้โมเดลการจัดการหน่วยความจำที่แตกต่างกันตามการออกแบบของแต่ละภาษา

#### C

```c
char *message = malloc(20);

if (message != NULL) {
    snprintf(message, 20, "Hello C");
    printf("%s\n", message);

    free(message);
}
```

โปรแกรมเมอร์ต้องเข้ามาควบคุมและจัดการทรัพยากรด้วยตนเองอย่างเด่นชัด

#### Java

```java
Data d = new Data("Hello Java");

System.out.println(d.message);

d = null;
```

โปรแกรมเมอร์ไม่ต้องสั่งคืนพื้นที่ออบเจกต์ด้วยตนเอง

ระบบรันไทม์จะเป็นผู้กำหนดเองว่าออบเจกต์ที่เข้าถึงไม่ได้แล้วเหล่านั้นควรจะถูกเก็บกวาดเมื่อใด

#### Rust

```rust
fn main() {
    let message = String::from("Hello Rust");

    println!("{}", message);

} // message is automatically dropped here
```

Rust ผูกช่วงอายุของทรัพยากร (Resource Lifetime) ไว้กับความเป็นเจ้าของและขอบเขตของตัวแปร (Scope)

---

### Analysis

ผลการเปรียบเทียบแสดงให้เห็นว่า ภาษาโปรแกรมมิ่งต่างๆ มีกลยุทธ์การออกแบบที่แตกต่างกันในการจัดการหน่วยความจำและทรัพยากร

ภาษา **C และ C++** มอบอำนาจการควบคุมการจัดการหน่วยความจำให้แก่โปรแกรมเมอร์ในระดับสูง วิธีการนี้ช่วยให้สามารถควบคุมทรัพยากรได้อย่างแม่นยำและคาดเดาผลลัพธ์ได้ แต่หากจัดการวงจรชีวิต (Lifetime) ของข้อมูลไม่ถูกต้อง ก็อาจนำไปสู่ปัญหาต่างๆ เช่น หน่วยความจำรั่วไหล (Memory leaks) และข้อผิดพลาดจากการเรียกใช้หน่วยความจำที่ถูกคืนไปแล้ว (Use-after-free) อย่างไรก็ตาม C++ ได้มีการจัดเตรียมส่วนนามธรรม (Abstractions) เช่น RAII และสมาร์ทพอยเตอร์ (Smart pointers) เพื่อช่วยลดความเสี่ยงเหล่านี้บางส่วน

ในขณะที่ **Java, Python, C# และ Go**เลือกใช้กลไกการจัดการหน่วยความจำแบบอัตโนมัติ ซึ่งขับเคลื่อนด้วยระบบจัดการหน่วยความจำทิ้ง (Garbage Collection) เป็นหลัก วิธีนี้ช่วยลดความจำเป็นที่โปรแกรมเมอร์จะต้องคืนหน่วยความจำด้วยตัวเอง และสามารถป้องกันข้อผิดพลาดส่วนใหญ่ที่เกิดจากการจัดการด้วยมือ (Manual) ได้ ทว่า การเรียกคืนทรัพยากรจะถูกจัดการโดยตัวรันไทม์ (Runtime) แทนที่จะเป็นการควบคุมโดยตรงจากโปรแกรมเมอร์

ด้านภาษา**Rust** ได้เลือกใช้แนวทางที่แตกต่างออกไปผ่านระบบความเป็นเจ้าของข้อมูล (Ownership model) แทนที่จะพึ่งพาการคืนหน่วยความจำด้วยมือหรือระบบ Garbage Collector เป็นหลัก Rust จะใช้กฎความเป็นเจ้าของและการตรวจสอบในขั้นตอนการคอมไพล์ (Compile-time checking) เพื่อกำหนดวิธีการใช้งานและระยะเวลาที่จะปล่อย (Drop) ค่าและทรัพยากรเหล่านั้น

เมื่อมองจากมุมมองของหลักการออกแบบภาษาโปรแกรม (PPL) สิ่งนี้แสดงให้เห็นถึงข้อแลกเปลี่ยน (Trade-off) ที่สำคัญในการออกแบบภาษา นั่นคือ จุดที่กำหนดความรับผิดชอบในการจัดการทรัพยากร โดยแนวทางแบบดั้งเดิม (Manual) จะผลักความรับผิดชอบไปที่โปรแกรมเมอร์ ส่วนภาษาที่มีระบบ Garbage-collected จะย้ายความรับผิดชอบส่วนใหญ่ไปที่ตัวรันไทม์ ในขณะที่ Rust จะย้ายการตรวจสอบความเป็นเจ้าของที่สำคัญไปไว้ในขั้นตอนการคอมไพล์

**ดังนั้น** ระบบความเป็นเจ้าของข้อมูลของ Rust จึงไม่ใช่เพียงแค่ไวยากรณ์ที่แตกต่างออกไปในการจัดการหน่วยความจำ แต่เป็นการตัดสินใจออกแบบภาษาที่ผสานรวม ไวยากรณ์ (Syntax), อรรถศาสตร์ (Semantics), การตรวจสอบชนิดข้อมูล (Type checking) และการจัดการทรัพยากร เข้าด้วยกัน เพื่อบังคับใช้กฎความปลอดภัยของหน่วยความจำ (Memory safety) ตั้งแต่ขั้นตอนการคอมไพล์ โดยไม่จำเป็นต้องพึ่งพาระบบ Garbage Collector สำหรับการจัดการหน่วยความจำทั่วไป

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`Introduction + Key Concept + Important Syntax/Rule`

**Member 2**

`Runable Code Example`

**Member 3**

`PPL Perspective + Compare Rust with other languages`

**Member 4**

`Common Mistakes + Exercise`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `The Rust Programming Language: https://doc.rust-lang.org/book/`
2. `Rust by Example : https://doc.rust-lang.org/rust-by-example/`
3. `W3School: https://www.w3schools.com/rust/`
4. `The Rust Programming Language Book: https://www.scs.stanford.edu/~zyedidia/docs/rust/rust_book.pdf page 82-96`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `translate from English to Thai` | `Human Verification` |
| `Gemini` | `find reference and document` | `Use the official website` |
| `Claude` | `Code verification and mistake correction` | `run and compile the code` |

### Declaration

- [✔] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [✔] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [✔] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [✔] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`ใช้ Generative AI เป็นผู้ช่วยสนับสนุนในขั้นตอนการค้นคว้าและพัฒนาโปรเจกต์นี้ โดยมีการใช้ ChatGPT สำหรับการแปลเนื้อหาจากภาษาอังกฤษเป็นภาษาไทย (ผ่านการตรวจสอบความถูกต้องด้วยตนเอง) ใช้ Gemini ในการค้นหาแหล่งอ้างอิงและเอกสารประกอบ (ตรวจสอบความถูกต้องเทียบกับเอกสารอ้างอิงบนเว็บไซต์ทางการ) และใช้ Claude ในการวิเคราะห์โค้ดรวมถึงแก้ไขข้อผิดพลาด (ตรวจสอบความถูกต้องผ่านการรันและคอมไพล์โค้ดด้วย Rust compiler) ทั้งนี้ เนื้อหา แหล่งอ้างอิง และตรรกะของโค้ดในผลงานชิ้นสุดท้ายทั้งหมด ได้รับการตรวจสอบและยืนยันความถูกต้องโดยผู้จัดทำเองทั้งสิ้น`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `3` | `1` | `1` | `Introduction + Key Concept + Important Syntax/Rule` |
| Member 2 | `0` | `1` | `1` | `0` | `Runable Code Example` |
| Member 3 | `0` | `1` | `1` | `0` | `PPL Perspective + Compare Rust with other languages` |
| Member 4 | `0` | `6` | `2` | `0` | `Common Mistakes + Exercise + others` |

### Teamwork Reflection

**How did your team collaborate?**

`ทีมของเราทำงานร่วมกันโดยใช้ระบบควบคุมเวอร์ชัน (Version Control) และแอปพลิเคชันสื่อสารควบคู่กันเพื่อให้การทำงานมีประสิทธิภาพสูงสุด เราใช้ Git และ GitHub สำหรับการจัดการโค้ดจากศูนย์กลาง ซึ่งช่วยให้เราสามารถควบคุมเวอร์ชันของงาน แบ่งกันพัฒนาใน Branch ต่างๆ ได้พร้อมกัน และรีวิวโค้ดของทีมได้ สำหรับการสื่อสารในแต่ละวันและการประสานงานที่ต้องการความรวดเร็ว เราใช้กลุ่มใน LINE เพื่อพูดคุยเกี่ยวกับความคืบหน้าของงาน อัปเดตข้อมูล และนัดหมายเวลาทำงาน นอกจากนี้ สิ่งสำคัญคือพวกเรายังมีการแลกเปลี่ยนความรู้ โดยผลัดกันอธิบายและสอนเนื้อหางานในส่วนที่แต่ละคนรับผิดชอบให้เพื่อนร่วมทีมฟัง เพื่อให้ทุกคนมีความเข้าใจในภาพรวมของโปรเจกต์ไปพร้อมๆ กันอย่างแท้จริง`

**Problems encountered**

`ไม่คุ้นเคยกับการใช้ Git และ GitHub ทำให้เกิดปัญหาทางเทคนิค เช่น โค้ดทับซ้อนกัน (Merge Conflict), pull request และความสับสนในการจัดการ Branch`

**How did you solve them?**

`ศึกษาคำสั่งพื้นฐานที่จำเป็นของ Git เราได้เรียนรู้วิธีการ Pull ดึงข้อมูล อัปเดตงาน (Commit) และการแก้ไขปัญหา Conflict ในเครื่องของตัวเองให้เรียบร้อยก่อนที่จะ Push งานขึ้นไปยัง Repository หลัก`

---

## 15. Final Checklist

- [✔] Learning Objectives ครบ 3–4 ข้อ
- [✔] Key Concepts ครบถ้วน
- [✔] Syntax / Rules
- [✔] Runnable Code Examples
- [✔] Code Compile และ Run ได้จริง
- [✔] Common Mistakes
- [✔] Exercises 2 ข้อ พร้อม Solutions
- [✔] PPL Perspective
- [✔] Rust vs Other Language
- [✔] References อย่างน้อย 4 แหล่ง
- [✔] AI Usage Declaration
- [✔] GitHub Contribution
- [✔] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [✔] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [✔] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/sotheasokea/rust-tutorial-2569`

**Chapter Path:** [View Repository Path](/11-ownership)

**Final PR:** `5`

**Submitted by:** `Group 11`

**Date:** `2026-10-02`
