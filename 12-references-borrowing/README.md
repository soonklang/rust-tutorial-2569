
# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม
> **Topic No.:** `12`
> **Topic Name:** `References & Borrowing`
> **Group No.:** `12`

---

## 1. Members

| # | Name                                      | Student ID | GitHub Username | Main Responsibility                     |
| - | ----------------------------------------- | ---------- | --------------- | --------------------------------------- |
| 1 | Mr.UDTARAKVISETH LAY                      | 670710259  | `@Viseth101`  | Concept + Short Code                    |
| 2 | นายกรันต์ชัย คำทรัพย์ | 670710290  | `@[username]` | Detailed Code + Live Demo               |
| 3 | นางสาวณัฐณิชา ภู่วงษ์ | 670710291  | `@[username]` | Rust vs Other Language + PPL Analysis   |
| 4 | นายเทพพิทักษ์ นิลดำ     | 670710292  | `@670710292` | Exercises + Common Mistakes + Challenge |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายแนวคิดสำคัญของการใช้ References และ Borrowing ในภาษา Rust ได้
2. เขียนโปรแกรม Rust ที่มีการยืมข้อมูล (Borrowing) ทั้งแบบอ่านอย่างเดียว (Immutable) และแบบแก้ไขค่าได้ (Mutable)
3. วิเคราะห์พฤติกรรมของ Borrow Checker และกฎการจัดการหน่วยความจำที่ป้องกัน Data Races ของภาษา Rust ได้
4. เปรียบเทียบความปลอดภัยในการจัดการหน่วยความจำของ Rust (Compile-time) กับภาษาอื่นที่ใช้ Garbage Collector หรือ Manual Memory Management ได้

---

## 3. Introduction

การจัดการหน่วยความจำ (Memory Management) เป็นความท้าทายหลักในการเขียนโปรแกรม ภาษาอย่าง C ใช้การจัดการแบบ Manual (เช่น `malloc`/`free`) ซึ่งเสี่ยงต่อการเกิดข้อผิดพลาดอย่าง Dangling Pointers หรือ Memory Leaks ส่วนภาษาอย่าง Java ใช้ Garbage Collector (GC) ซึ่งมีความปลอดภัยแต่แลกมาด้วยประสิทธิภาพที่ลดลงระหว่างรันไทม์

Rust นำเสนอวิธีการใหม่ที่เรียกว่า **Ownership** โดยข้อมูลจะมีเจ้าของได้เพียงคนเดียว อย่างไรก็ตาม การส่งข้อมูลไปมาระหว่างฟังก์ชันโดยย้ายความเป็นเจ้าของ (Move) ตลอดเวลาเป็นเรื่องที่ไม่สะดวก Rust จึงมีระบบ **References & Borrowing** ซึ่งอนุญาตให้เรา "ยืม" ข้อมูลไปใช้งานโดยไม่ต้องยึดความเป็นเจ้าของ ระบบนี้ถูกควบคุมอย่างเข้มงวดโดย **Borrow Checker** ในช่วง Compile-time ทำให้ Rust รับประกันความปลอดภัยของหน่วยความจำ (Memory Safety) และป้องกัน Data Races ได้ 100% โดยไม่มี Overhead รันไทม์เหมือน GC

---

## 4. Key Concepts

### 4.1 Immutable References (การยืมแบบอ่านอย่างเดียว)

**คำอธิบาย**

References (`&`) อนุญาตให้เราอ้างอิงถึงค่าบางอย่างโดยไม่ต้องรับช่วงความเป็นเจ้าของ (Ownership) มา การกระทำนี้เรียกว่า **Borrowing (การยืม)** โดยค่าเริ่มต้น Reference จะเป็นแบบอ่านอย่างเดียว (Immutable) เราไม่สามารถแก้ไขข้อมูลที่ยืมมาได้

**ตัวอย่าง**

```rust
fn main() {
    let my_string = String::from("Silpakorn");
    let length = calculate_length(&my_string);
    println!("ความยาวของ '{}' คือ {}", my_string, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**Explanation**

* บรรทัด `let length = calculate_length(&my_string);`: เครื่องหมาย `&` สร้าง Reference ที่ชี้ไปยังค่าของ `my_string` แต่ไม่ได้เป็นเจ้าของมัน
* ในฟังก์ชัน `calculate_length`: พารามิเตอร์ `s` มีชนิดข้อมูลเป็น `&String` (Reference ของ String)
* เมื่อฟังก์ชันทำงานจบ `s` จะหายไปจาก Scope แต่เนื่องจาก `s` ไม่ได้เป็นเจ้าของข้อมูล ข้อมูลต้นฉบับใน `my_string` จึงไม่ถูกทำลาย (Drop) ทำให้เรายังสามารถใช้ `my_string` ในคำสั่ง `println!` ต่อได้

---

### 4.2 Mutable References (การยืมแบบแก้ไขค่าได้)

**คำอธิบาย**

หากเราต้องการแก้ไขข้อมูลที่เรายืมมา เราต้องใช้ **Mutable Reference (`&mut`)** ทั้งตัวแปรต้นฉบับและ Reference ที่สร้างขึ้นจะต้องถูกประกาศเป็น `mut` ทั้งคู่

**ตัวอย่าง**

```rust
fn main() {
    let mut greeting = String::from("Hello");
    add_world(&mut greeting);
    println!("{}", greeting);
}

fn add_world(s: &mut String) {
    s.push_str(", World!");
}
```

**Explanation**

* `let mut greeting = String::from("Hello");`: ตัวแปรต้นฉบับต้องเป็น `mut` ถึงจะยอมให้คนอื่นยืมไปแก้ไขได้
* `&mut greeting`: เราส่ง Mutable Reference เข้าไปในฟังก์ชัน
* ฟังก์ชัน `add_world` รับพารามิเตอร์ชนิด `&mut String` ทำให้สามารถเรียกใช้เมธอด `.push_str()` เพื่อแก้ไขข้อมูลต้นฉบับได้โดยตรง
* ผลลัพธ์ที่พิมพ์ออกมาจะเป็น `Hello, World!`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning                                                                                                                              | Example                           |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------- |
| `&T`        | **Immutable Reference:** ยืมข้อมูลไปเพื่ออ่านอย่างเดียว ไม่สามารถแก้ไขค่าได้ | `let ref_val = &my_string;`     |
| `&mut T`    | **Mutable Reference:** ยืมข้อมูลไปและสามารถแก้ไขค่านั้นได้                                  | `let mut_ref = &mut my_string;` |

### Important Rules

1. ในช่วงเวลาใดเวลาหนึ่ง คุณสามารถมี Mutable Reference (`&mut T`) **ได้เพียง 1 ตัวเท่านั้น** หรือมี Immutable Reference (`&T`) **กี่ตัวก็ได้** แต่ไม่สามารถมีทั้งสองอย่างพร้อมกันได้ (ป้องกัน Data Races)
2. Reference ทุกตัวต้องชี้ไปยังข้อมูลที่มีอยู่จริงเสมอ (Rust จะป้องกันการเกิด "Dangling Pointers" ในขั้นตอนการ Compile อย่างเข้มงวด)

---

---

> **[ส่วนของ Member คนอื่นๆ เริ่มต้นที่นี่]**

---

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`

---

### Example 2 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code]`

---

## 7. Common Mistakes

### Mistake 1 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

### Exercise 2 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`

### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`

### 9.3 Type System

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect               | Rust               | Other Language     |
| -------------------- | ------------------ | ------------------ |
| Syntax               | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System          | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management    | `[อธิบาย]` | `[อธิบาย]` |
| Safety               | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member   | Responsibility                          |  Time |
| -------- | --------------------------------------- | ----: |
| Member 1 | Concept + Short Code Illustration       | 5 min |
| Member 2 | Detailed Code + Live Demo               | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis   | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool                | Purpose                        | How the Result Was Verified        |
| ---------------------- | ------------------------------ | ---------------------------------- |
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]`          | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member   |           Issues |          Commits |    Pull Requests |     Code Reviews | Contribution               |
| -------- | ---------------: | ---------------: | ---------------: | ---------------: | -------------------------- |
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`

**Problems encountered**

`[ปัญหาที่พบ]`

**How did you solve them?**

`[วิธีแก้ปัญหา]`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** [github.com/Viseth101/12.References-Borrowing_PL-Group12.git](https://github.com/Viseth101/12.References-Borrowing_PL-Group12.git)

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group 12]`

**Date:** `[YYYY-MM-DD]`
