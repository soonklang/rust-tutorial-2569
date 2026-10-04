# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 01
> **Topic No.:** 01
> **Topic Name:** Introduction to Rust & Programming Paradigms
> **ประเด็นหลักที่ควรครอบคลุม:** ความเป็นมาของ Rust, จุดเด่น, แนวคิดของภาษา, Rust กับ Imperative / OOP / Functional Programming

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นางสาวขวัญฐิตา การดี | 650710069 | `@650710069` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายคีญภัสน์ แก้วผ่อง | 660710072 | `@660710072` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายจักรภพ ภูมิพัฒน์ | 660710073 | `@660710073` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายณทรรศน์ พรหมประดิษฐ์ | 660710079 | `@660710079` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `รู้จุดเด่นและความเป็นมาของ Rust`
2. `เข้าใจการเขียนโปรแกรม Rust ขั้นพื้นฐาน`
4. `สามารถวิเคราะห์ Rust ในมุมมองของ PL `
5. `สามารถเปรียบเทียบ Rust กับภาษาอื่น`

---

## 3. Introduction

Rust เป็นภาษาโปรแกรมระบบ ที่พัฒนาโดย Mozilla ซึ่งปัจจุบันอยู่ในการดูแลโดย Rust Foundation โดยมีเป้าหมายหลักคือการมอบความเร็วและความปลอดภัยของหน่วยความจำโดยไม่ต้องพึ่งพา Garbage Collector (GC) เหมือนภาษาอย่าง Python, Java 

### **ความเป็นมาของ Rust** ###
  - ภาษา Rust ไม่ได้ถูกสร้างขึ้นมาจากความต้องการขององค์กรขนาดใหญ่ในตอนแรก แต่เริ่มต้นในปี 2006 โดยมี Graydon Hoare วิศวกรซอฟต์แวร์ของ Mozilla เป็นผู้คิดค้น โดย Graydon พบว่าลิฟต์ที่อพาร์ตเมันท์เสียอีกครั้งเพราะซอฟต์แวร์ในระบบเกิด Crash จากปัญหาเรื่อง Memory Bug เขาจึงเริ่มพัฒนาภาษาใหม่ขึ้นมา โดยตั้งชื่อภาษาว่า Rust ตามชื่อของเชื้อรา Rust Fungi ที่ขึ้นชื่อเรื่องความอึด ถึก และทน
  -  ต่อมาในปี 2009 Mozilla เห็นศักยภาพของภาษา Rust ว่าสามารถนำมาแก้ปัญหาด้านความปลอดภัยและประสิทธิภาพของเว็บเบราว์เซอร์ได้ จึงได้สปอนเซอร์โปรเจกต์นี้
  - ปี 2012  Mozilla ประกาศพัฒนา Servo ซึ่งเป็น Browser Engine ยุคใหม่ด้วยภาษา Rust เพื่อพิสูจน์ว่า Rust สามารถประมวลผลแบบ Parallelism ได้อย่างปลอดภัย
  - ปี 2014 ได้มีการคิดค้นระบบ Ownership & Borrowing รูปแบบปัจจุบันขึ้นมา เพื่อตัด Garbage Collector ออกไปโดยสิ้นเชิง
  - ปี 2015 ได้ปล่อย Rust 1.0 ออกมาอย่างเป็นทางการ
  - ปี 2020 - ปัจจุบัน เนื่องจากปัญหาทางการเงิน Mozilla ได้ปลดพนักงานทีม Rust ออกบางส่วน ทำให้บริษัทยักษ์ใหญ่เทคโนโลยี ได้แก่ AWS, Google, Microsoft, Meta และ Huawei ร่วมมือกันก่อตั้ง Rust Foundation เพื่อดูแลและสนับสนุนภาษา Rust ในฐานะโครงการ Open Source 

### **จุดเด่นของภาษา Rust** ###
- **1.Memory Safety** - แก้ปัญหาเรื่อง Memory Leak หรือ Dangling Pointers ที่มักเกิดใน C/C++ ผ่านระบบ Ownership, Borrowing, และ Lifetimes โดยตัวตรวจสอบของคอมไพเลอร์ Borrow Checker จะตรวจสอบความถูกต้องของการใช้หน่วยความจำตั้งแต่ช่วง Compile
- **2.Fearless Concurrency ** - คือกฎการจัดการหน่วยความจำที่เข้มงวด ช่วยป้องกันปัญหา Data Race หรือก็คือการที่หลาย thread เข้าถึงหน่วยความจำตำแหน่งเดียวกันพร้อมกันโดยไม่มีการจัดระเบียบ ทำให้การเขียนโปรแกรมแบบทำงานพร้อมกันมีความปลอดภัยสูงมากขึ้น
- **3.Zero-Cost Abstractions ** - คือการใช้ฟีเจอร์ระดับสูง เช่น Iterators, Generics หรือ Closure ไม่ทำให้ประสิทธิภาพการทำงานลดลงเมื่อเทียบกับการเขียนโค้ดระดับล่าง
- **4.Tooling แบบครบวงจร ** - มี Cargo ซึ่งเป็นทั้ง Package Manager และ Build System ในตัว ช่วยจัดการ Dependencies, การบิวด์โปรเจกต์, การรัน Test และการสร้าง Documentation ได้อย่างสะดวก

### **ตัวอย่างโค้ด Rust แบบสั้นๆ** ###
```Rust
fn main() {
    let name = "Rust"; 
    let mut score = 90; 
    score = score + 10;

    println!("Hello, {}! Your score is {}.", name, score);
}
```
**Expected Output**
```rust
(Output):PlaintextHello, Rust! Your score is 100.
```
**Explanation**
- การทำงานfn main() { ... }จุดเริ่มต้นการทำงานของโปรแกรม (Main function) โค้ดทั้งหมดจะเริ่มรันจากตรงนี้
- let name = "Rust";สร้างตัวแปรชื่อ name โดยค่าเริ่มต้นของ Rust ตัวแปรจะ แก้ไขไม่ได้ (Immutable)
- let mut score = 90;สร้างตัวแปรชื่อ score โดยใส่คีย์เวิร์ด mut (Mutable) เพื่อบอกว่าตัวแปรนี้ สามารถเปลี่ยนค่าได้
- score = score + 10;อัปเดตค่าใน score เพิ่มขึ้นอีก 10 (กลายเป็น 100)
- println!(...);คำสั่งพิมพ์ข้อความออกทางหน้าจอ โดยวงเล็บปีกกา {} จะถูกแทนที่ด้วยค่าของตัวแปรตามลำดับ (name และ score)



---
## 4. Key Concepts ##
### **4.1 Ownership (สิทธิ์เจ้าของ)** ###
ข้อมูล 1 ชิ้น มีเจ้าของได้แค่ตัวเดียวเท่านั้น หากมีการส่งต่อหรือย้ายสิทธิ์ (Move) ให้ตัวแปรอื่น ตัวแปรเดิมจะใช้ต่อไม่ได้ทันที
**ตัวอย่าง**
```Rust
fn main() {
    let s1 = String::from("Hi");
    let s2 = s1; // โอนสิทธิ์ความเป็นเจ้าของจาก s1 ให้ s2

    // println!("{}", s1); // จะ Compile Error เพราะ s1 ถูกโอนสิทธิ์ไปแล้ว
    println!("{}", s2);
}
```
**Explanation**
- เมื่อกำหนด let s2 = s1; Rust จะทำการย้ายสิทธิ์ความเป็นเจ้าของข้อมูล "Hi" บน Heap ไปให้ s2 ทำให้ s1 กลายเป็นตัวแปรเปล่าที่ไม่สามารถใช้งานได้อีก เพื่อป้องกันปัญหาการคืนหน่วยความจำซ้ำซ้อน (Double Free)
### **4.2 Borrowing (การยืม)** ###
การเข้าถึงข้อมูลโดยใช้เครื่องหมาย & เพื่อยืมอ่านค่า (Immutable Reference) โดยไม่ต้องเปลี่ยนหรือย้ายสิทธิ์ความเป็นเจ้าของ ทำให้ตัวแปรต้นทางยังใช้งานต่อได้ตามปกติ
ตัวอย่าง
```Rust
fn main() {
    let s1 = String::from("Hi");
    let len = s1.len(); // ยืมอ่านความยาวข้อมูลโดย s1 ยังเป็นเจ้าของอยู่

    println!("ข้อความ: {}, ความยาว: {}", s1, len);
}
```
**Explanation**
- เมธอด .len() ยืมอ่านข้อมูลผ่าน &s1 เพียงอย่างเดียว ไม่ได้ยึดสิทธิ์ความเป็นเจ้าของไป ดังนั้นเมื่อคำนวณความยาวเสร็จแล้ว ตัวแปร s1 จึงยังสามารถนำมาใช้งานต่อในบรรทัดถัดไปได้
### **4.3 Mutability (การเปลี่ยนค่า)** ###
ตัวแปรใน Rust ทั้งหมดจะแก้ไขค่าไม่ได้ (Immutable) เป็นค่าเริ่มต้น หากต้องการให้ตัวแปรนั้นๆ สามารถเปลี่ยนแปลงหรืออัปเดตค่าได้ จะต้องใส่คีย์เวิร์ด mut กำกับไว้เสมอ
ตัวอย่าง
```Rust
fn main() {
    let mut x = 5;
    println!("ค่าเริ่มต้น: {}", x);

    x = 10; // แก้ไขค่าได้เพราะมี mut
    println!("ค่าใหม่: {}", x);
}
```
**Explanation**
- การประกาศ let mut x = 5; ช่วยบอกคอมไพเลอร์ว่าตัวแปร x อนุญาตให้แก้ไขข้อมูลในภายหลังได้ ทำให้คำสั่ง x = 10; ทำงานได้สำเร็จโดยไม่เกิด Compile Error
___

## 5. Important Syntax / Rules ##
| Syntax / Rule | Meaning | Example |
|---|---|---|
| `let` / `let mut` | ประกาศตัวแปร (ค่าเริ่มต้นเปลี่ยนไม่ได้) | `let mut x = 0;` |
| `fn` | ประกาศฟังก์ชัน | `fn add(a: i32, b: i32) -> i32 { a + b }` |
| closure `\|x\| ...` | ฟังก์ชันนิรนามที่จับตัวแปรได้ | `\|n\| n * 2` |
| `trait` / `impl ... for` | กำหนดและทำพฤติกรรมให้ type | `impl Shape for Rect { ... }` |
| `&` / `&mut` | การยืมแบบอ่านอย่างเดียว / แก้ไขได้ | `len(&s)` |

### Important Rules

1. ทุกค่ามีเจ้าของเพียงหนึ่งเดียว และถูกคืนเมื่อเจ้าของหมดขอบเขต
2. ที่เวลาหนึ่งมีได้อย่างใดอย่างหนึ่ง คือ reference แบบอ่านได้หลายตัว หรือ reference แบบแก้ไขได้หนึ่งตัว
3. reference ต้องไม่อยู่นานกว่าค่าที่มันชี้ไป
___

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — โจทย์เดียว 3 แนวทาง (Imperative, Functional, OOP-style)

**Purpose:** แก้โจทย์ "หาผลรวมกำลังสองของเลขคู่" ด้วย 3 paradigm เพื่อแสดงว่า Rust เป็นภาษา multi-paradigm

```rust
// OOP style: struct + impl
struct NumberList {
    numbers: Vec<i32>,
}

impl NumberList {
    fn new(numbers: Vec<i32>) -> Self {
        NumberList { numbers }
    }

    fn sum_even_squares(&self) -> i32 {
        self.numbers
            .iter()
            .filter(|&&n| n % 2 == 0)
            .map(|&n| n * n)
            .sum()
    }
}

// Imperative style: ตัวแปร mutable + loop
fn imperative(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for &n in numbers {
        if n % 2 == 0 {
            total += n * n;
        }
    }
    total
}

// Functional style: iterator chain ไม่มีตัวแปรที่ถูกแก้ไข
fn functional(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    println!("Imperative: {}", imperative(&data));
    println!("Functional: {}", functional(&data));

    let list = NumberList::new(data);
    println!("OOP-style:  {}", list.sum_even_squares());
}
```

**Expected Output**

```text
Imperative: 56
Functional: 56
OOP-style:  56
```

**Explanation**

- **Imperative:** ประกาศ `let mut total = 0` แล้ววน `for` เพื่อสะสมค่าทีละรอบ เป็นการบอกคอมพิวเตอร์ว่า "ทำอะไรทีละขั้น" โดยมีการเปลี่ยนค่าตัวแปร (mutable state)
- **Functional:** ใช้ iterator chain `filter` (เลือกเลขคู่) → `map` (ยกกำลังสอง) → `sum` (รวมค่า) โดยไม่มีตัวแปรที่ถูกแก้ไขเลย
- **OOP-style:** Rust ไม่มี `class` แต่ใช้ `struct` เก็บข้อมูล และ `impl` กำหนด method (`new`, `sum_even_squares`) เพื่อรวมข้อมูลกับพฤติกรรมไว้ด้วยกัน
- ทั้ง 3 แบบได้ผลลัพธ์เท่ากันคือ 4 + 16 + 36 = 56

---

### Example 2 — Immutable by default และ Ownership

**Purpose:** แสดงจุดเด่นด้านความปลอดภัยของ Rust ได้แก่ ตัวแปรแก้ค่าไม่ได้โดย default และระบบ ownership

```rust
fn main() {
    // ตัวแปรเป็น immutable โดย default
    let x = 5;
    println!("x = {}", x);

    // ต้องประกาศ mut ถึงจะแก้ค่าได้
    let mut y = 5;
    y += 1;
    println!("y = {}", y);

    // ownership: การ "move" ค่า
    let s1 = String::from("Rust");
    let s2 = s1; // s1 ถูก move ไปให้ s2 แล้ว
    println!("s2 = {}", s2);

    // ถ้าต้องการใช้ทั้งสองตัว ต้อง clone
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
}
```

**Expected Output**

```text
x = 5
y = 6
s2 = Rust
s2 = Rust, s3 = Rust
```

**Explanation**

- `let x = 5;` ตัวแปรเป็น immutable ถ้าพยายามกำหนดค่าซ้ำ compiler จะแจ้ง error (E0384)
- `let mut y = 5;` ใส่ `mut` เพื่อบอกว่าตั้งใจให้แก้ค่าได้
- `let s2 = s1;` เนื่องจาก `String` เก็บข้อมูลบน heap ค่าจึงถูก **move** ไปให้ `s2` และ `s1` ใช้งานต่อไม่ได้ ถ้าเรียกใช้ `s1` อีกจะเกิด error (E0382)
- `s2.clone()` สร้างสำเนาข้อมูลใหม่ ทำให้ใช้ได้ทั้ง `s2` และ `s3`

---

## 7. Common Mistakes

### Mistake 1 — ใส่ semicolon ท้ายค่าที่ต้องการ return

**Problem**

ใน Rust บรรทัดสุดท้ายของฟังก์ชันที่ไม่มี `;` คือ expression ที่เป็นค่า return ถ้าใส่ `;` จะกลายเป็น statement และฟังก์ชันคืนค่า `()` แทน

**Incorrect Code**

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}

fn main() {
    println!("{}", add(2, 3));
}
```

**Correct Code**

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
}
```

**Why?**

Rust เป็นภาษา expression-oriented โดย block `{ ... }` ให้ค่าเป็น expression สุดท้าย การใส่ `;` ทำให้ค่าถูกทิ้ง compiler จึงฟ้อง mismatched types (E0308) เพราะประกาศว่าจะคืน `i32` แต่ได้ `()`

---

### Mistake 2 — บวกตัวเลขต่างชนิดกันโดยตรง

**Problem**

Rust ไม่แปลงชนิดตัวเลขให้อัตโนมัติ (no implicit conversion)

**Incorrect Code**

```rust
fn main() {
    let a: i32 = 5;
    let b: f64 = 2.5;
    let sum = a + b;
    println!("{}", sum);
}
```

**Correct Code**

```rust
fn main() {
    let a: i32 = 5;
    let b: f64 = 2.5;
    let sum = a as f64 + b;
    println!("{}", sum);
}
```

ผลลัพธ์: `7.5`

**Why?**

Rust เป็น statically typed และ strongly typed ต้องแปลงชนิดด้วย `as` อย่างชัดเจน เพื่อป้องกันข้อผิดพลาดจากการแปลงที่ผู้เขียนไม่ได้ตั้งใจ (error E0277)

---

## 8. Exercises

### Exercise 1 — ผลรวมกำลังสองแบบ Imperative และ Functional

**Problem**

เขียนโปรแกรมหาผลรวมของ 1² + 2² + 3² + 4² + 5² สองแบบ คือ Imperative (ใช้ loop) และ Functional (ใช้ iterator)

**Hint**

แบบ Imperative ใช้ `let mut` กับ `for n in 1..=5` แบบ Functional ใช้ `(1..=5).map(...).sum()`

**Solution**

```rust
fn main() {
    // Imperative
    let mut total = 0;
    for n in 1..=5 {
        total += n * n;
    }

    // Functional
    let total_f: i32 = (1..=5).map(|n| n * n).sum();

    println!("Imperative: {}", total);
    println!("Functional: {}", total_f);
}
```

ผลลัพธ์:

```text
Imperative: 55
Functional: 55
```

**Explanation**

แบบแรกสะสมค่าในตัวแปร mutable ทีละรอบ แบบที่สองสร้างช่วง 1..=5 แล้วแปลงค่าและรวมด้วย iterator โดยไม่มีตัวแปรที่ถูกแก้ไข ผลลัพธ์เท่ากัน

---

### Exercise 2 — struct และ method

**Problem**

สร้าง `struct Rectangle` ที่มี `width` และ `height` พร้อม method `area()` และ `is_square()` แล้วทดสอบกับสี่เหลี่ยม 2 รูป

**Hint**

ใช้ `impl Rectangle { ... }` และ method รับ `&self`

**Solution**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let r = Rectangle::new(4, 5);
    let s = Rectangle::new(3, 3);
    println!("r: area = {}, square? {}", r.area(), r.is_square());
    println!("s: area = {}, square? {}", s.area(), s.is_square());
}
```

ผลลัพธ์:

```text
r: area = 20, square? false
s: area = 9, square? true
```

**Explanation**

`struct` เก็บข้อมูล ส่วน `impl` กำหนดพฤติกรรม `&self` คือการยืมค่ามาอ่านโดยไม่ย้าย ownership ซึ่งเป็นรูปแบบ OOP-style ของ Rust
---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

Rust ใช้ C-style syntax (มีการใช้วงเล็บปีกกา {} และเครื่องหมายอัฒภาค ;) ทำให้ผู้ที่คุ้นเคยกับ C/C++ หรือ Java สามารถอ่านทำความเข้าใจได้ง่าย แต่จุดเด่นคือ Rust เป็น Expression-oriented language คำสั่งส่วนใหญ่ในภาษา (รวมถึง if, match, หรือ block scope) จะประเมินค่าและส่งคืนค่า (Return value) เสมอ ทำให้สามารถเขียนโค้ดที่กระชับและลดการประกาศตัวแปรชั่วคราวได้

### 9.2 Semantics

Rust มี Semantics หรือพฤติกรรมพื้นฐานที่เน้นความปลอดภัยเป็นหลัก (Safety-first)
- Immutability by default: ตัวแปรทุกตัวใน Rust จะไม่สามารถแก้ไขค่าได้หลังจากการประกาศ เว้นแต่จะระบุ Keyword mut อย่างชัดเจน
- Move Semantics: เมื่อมีการกำหนดค่า (Assign) ตัวแปรที่เป็น Complex type ให้กับตัวแปรใหม่ สิทธิ์การครอบครองข้อมูลนั้นจะถูกย้าย (Move) ไปยังตัวแปรใหม่ทันที ไม่ใช่การคัดลอก (Copy) แบบในภาษาอื่น ซึ่งช่วยป้องกันข้อผิดพลาดทางหน่วยความจำ

### 9.3 Type System

Rust ใช้ระบบชนิดข้อมูลแบบ Statically Typed (ตรวจสอบชนิดข้อมูลตอน Compile) และ Strongly Typed (ไม่มีการแปลงชนิดข้อมูลข้ามประเภทแบบ Implicit ที่ไม่ปลอดภัย) และยังมี Type Inference ที่ตัวคอมไพเลอร์สามารถอนุมานชนิดของตัวแปรได้เองจากค่าที่กำหนดให้ โดยที่โปรแกรมเมอร์ไม่ต้องระบุ Type ให้ยุ่งยากในทุกๆครั้ง นอกจากนี้ Rust ได้ตัดแนวคิดของ Null Pointers ออกไปโดยสิ้นเชิง (ซึ่งเป็นสาเหตุของข้อผิดพลาดมหาศาลในภาษาอย่าง Java หรือ C++) และใช้ Algebraic Data Types ผ่าน Option<T> และ Result<T, E> แทน ทำให้โปรแกรมเมอร์ถูกบังคับให้จัดการกรณีที่ข้อมูลสูญหายหรือเกิด Error ตั้งแต่ตอน Compile

### 9.4 Memory / Resource Management

นี่คือนวัตกรรมที่สำคัญที่สุดของ Rust ภาษาไม่มี Garbage Collector (GC) แบบ Java หรือ Python และไม่ต้องให้โปรแกรมเมอร์จอง/คืนหน่วยความจำเอง (Manual Memory Management แบบ malloc/free ใน C) แต่ Rust ใช้ระบบ Ownership, Borrowing และ Lifetimes ซึ่งคอมไพเลอร์จะทำการตรวจสอบกฎเหล่านี้ตั้งแต่ตอน Compile (Compile-time) ทำให้ได้ประสิทธิภาพความเร็วเทียบเท่า C/C++ แต่การันตีความปลอดภัย (Memory Safety) และไม่มีปัญหาเรื่อง Memory Leak หรือ Dangling Pointers

### 9.5 Abstraction / Other PPL Concepts

Rust เป็นภาษาแบบ Multi-paradigm ที่ดึงข้อดีของหลายกระบวนทัศน์มารวมกัน:
- Imperative: รองรับการเขียนโปรแกรมแบบลำดับขั้น การเปลี่ยนสถานะตัวแปร (mut) และโครงสร้างควบคุมแบบดั้งเดิม (Loop, if-else)
- Functional Programming (FP): ได้รับอิทธิพลจากภาษา ML และ Haskell รองรับ First-class functions, Closures, Iterators, Pattern Matching (match) และ Algebraic Data Types (ผ่าน enum ที่เก็บค่าได้)
- Object-Oriented Programming (OOP): Rust ไม่มี Class และ Inheritance แบบดั้งเดิม แต่จัดการข้อมูลผ่าน struct และพฤติกรรมผ่าน impl การทำ Polymorphism และ Interface จะใช้ระบบ Traits (คล้าย Interface ใน Java) ซึ่งเป็นการออกแบบที่เน้น  Composition over Inheritance ช่วยลดปัญหาซับซ้อนอย่าง Diamond Problem

### 9.6 Why Rust?

Rust ถูกออกแบบมาเพื่อแก้ปัญหา "ความเร็ว vs ความปลอดภัย" (Performance vs Safety) ที่ภาษาในอดีตต้องเลือกอย่างใดอย่างหนึ่ง Rust นำเสนอ Memory Safety และ "Fearless Concurrency" (การเขียนโปรแกรมแบบทำงานพร้อมกันโดยไม่เกิด Data Races) โดยไม่ต้องพึ่งพา Garbage Collector ทำให้ Rust เหมาะอย่างยิ่งสำหรับการเขียนระบบ (Systems Programming), WebAssembly, และโปรแกรมที่ต้องการประสิทธิภาพขั้นสูงสุด แต่แม้จะเขียนโค้ดระดับสูงด้วย Iterators หรือ Closures (แบบ FP) แต่ Rust มีคุณสมบัติ Zero-cost Abstraction คอมไพเลอร์จะแปลงโค้ดเหล่านี้ให้ทำงานได้เร็วเทียบเท่ากับการเขียน Loop แบบดั้งเดิมในภาษา C โดยไม่เพิ่มภาระตอนรันไทม์

---

## 10. Rust vs. Other Language

**Comparison Language:** C++

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | C-style, เน้น Expression-oriented | C-style, เน้น Statement-oriented |
| Semantics / Behavior | ตัวแปรเป็น Immutable โดยค่าเริ่มต้น (ต้องระบุ mut เพื่อให้แก้ค่าได้) | ตัวแปรเป็น Mutable โดยค่าเริ่มต้น (ต้องระบุ const เพื่อไม่ให้แก้ค่าได้) |
| Type System | Static & Strong, มี Type inference และไม่มี Implicit conversion ที่เสี่ยงต่อข้อมูลหาย และไม่มีแนวคิด Null Pointer | Static & Strong, อนุญาตให้ทำ Implicit conversion ได้หลายกรณี (เช่น int เป็น float) |
| Memory Management | จัดการผ่านระบบ Ownership และ Borrow Checker ในตอน Compile-time (ไม่มี GC) | จัดการด้วยตนเอง (new/delete) หรือใช้ Smart Pointers (RAII) เพื่อช่วยจัดการ |
| Safety | การันตี Memory Safety และ Thread Safety ตั้งแต่ตอน Compile จะไม่เกิด Segfaults ถ้าไม่ใช้ unsafe block | ไม่มีระบบป้องกันที่เข้มงวด โปรแกรมเมอร์ต้องรับผิดชอบเรื่อง Memory Leak, Dangling Pointers และ Data Races เอง |

### Rust Example

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side: {}", self.side);
    }
}

impl Drawable for i32 {
    fn draw(&self) {
        println!("Drawing a point at: {}", self);
    }
}

fn render(shape: &impl Drawable) {
    shape.draw();
}

fn main() {
    render(&Circle { radius: 5.0 });
    render(&Square { side: 3.0 });
    render(&42_i32);
}
```

### `[C++]` Example

```C++
#include <iostream>

class Drawable {
public:
    virtual void draw() const = 0;
    virtual ~Drawable() = default;
};

class Circle : public Drawable {
private:
    double radius;
public:
    Circle(double r) : radius(r) {}
    void draw() const override {
        std::cout << "Drawing a circle with radius: " << radius << std::endl;
    }
};

class Square : public Drawable {
private:
    double side;
public:
    Square(double s) : side(s) {}
    void draw() const override {
        std::cout << "Drawing a square with side: " << side << std::endl;
    }
};

void render(const Drawable& shape) {
    shape.draw();
}

int main() {
    render(Circle(5.0));
    render(Square(3.0));
    // render(42);  //ใช้ไม่ได้: int ไม่ได้สืบทอด Drawable และเพิ่มภายหลังไม่ได้
    return 0;
}
```

### Analysis

จากมุมมองของ PPL การออกแบบสถาปัตยกรรมโปรแกรมในสองภาษานี้มีความแตกต่างกันอย่างชัดเจน C++ ใช้กระบวนทัศน์ OOP แบบดั้งเดิม ที่ผูกพันกับ Inheritance (การสืบทอด) ซึ่งทำให้เกิดลำดับชั้น (Class Hierarchy) ที่ซับซ้อน และอาจนำไปสู่ปัญหาความสัมพันธ์ที่แน่นเกินไป (Tight Coupling) ในทางตรงกันข้าม Rust ปฏิเสธแนวคิด Inheritance แต่เลือกใช้กระบวนทัศน์ที่เน้น Composition และ Traits (คล้าย Type Classes ในภาษา Haskell)

Rust แยกส่วนของ "ข้อมูล" (struct) ออกจาก "พฤติกรรม" (trait) อย่างเด็ดขาด การออกแบบนี้ทำให้โค้ดมีความยืดหยุ่นกว่า ปลอดภัยกว่า และเอื้อต่อการทำ Data-oriented design ซึ่งส่งผลดีต่อประสิทธิภาพของ Cache ในระดับฮาร์ดแวร์ นอกจากนี้ แม้ทั้งสองภาษาจะจัดการหน่วยความจำโดยไม่ใช้ GC แต่ระบบ Ownership ของ Rust จะบังคับเช็คความถูกต้องตั้งแต่ตอน Compile (Compile-time) ช่วยลดภาระทางความคิด (Cognitive load) ของนักพัฒนาเมื่อเทียบกับ C++ ที่มักเกิดปัญหา Memory Corruption หรือ Undefined Behavior (UB) จากความผิดพลาดของคนได้ง่ายกว่า

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

Concept + Short Code Illustration

**Member 2**

Detailed Code + Live Demo

**Member 3**

Githup และ PowerPoint ในส่วนของ Rust vs Other Language + PPL

**Member 4**

Exercies, Common Mistakes, Challenge

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. https://doc.rust-lang.org/book/ch01-00-getting-started.html
2. https://doc.rust-lang.org/book/ch17-00-oop.html
3. https://doc.rust-lang.org/rust-by-example/trait.html
4. https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| Gemini | ใช้เพื่อหาแหล่งข้อมูลและวิเคราะห์แนวคิดเชิง PPL | ตรวจสอบความถูกต้องโดยเทียบเคียงกับ Official Documentation และทดสอบรันโค้ดตัวอย่าง |
| Claude | เขียนโค้ดเปรียบเทียบ Rust/C++ | คอมไพล์และรันโค้ด Rust ด้วย `cargo run` และโค้ด C++ ด้วย `g++ -Wall` แล้วเทียบ output กับที่ระบุในเอกสาร, เปิดอ่านบทที่อ้างใน Rust Book (ch.4, 13, 18) และเอกสาร C++ เพื่อยืนยันข้อความ, ตรวจลิงก์อ้างอิงทุกลิงก์ว่าเปิดได้และตรงกับเนื้อหา |
| Gemini | ใช้เพื่อศึกษาหาความรู้ แนะนำภาษา Rust | https://share.gemini.google/roSMoGRnYVoO |
| Claude | ใช้ยกตัวอย่าง Code และเปรียบเทียบกับภาษา C และอธิบายโค้ดที่ของภาษา Rust | https://claude.ai/share/f52cbaa9-7cc6-43b8-9ba4-3264eb1286bb |
| Claude | ใช้อธิบายพื้นฐานของ Rust วิธีใช้แล้วก็ศึกษาโค้ด | https://claude.ai/share/91b55ed2-e563-467a-9288-a46c88848ac8 |
| Claude | ใช้ถามศึกษา Important/Syntax | https://claude.ai/share/c0cf2c49-bafc-4e51-8518-a906b3667962 |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`ใช้ Claude และ Gemini เพื่อศึกษาข้อมูลของภาษา PPL ทั้งเรื่อง Syntax ของแต่ละภาษาพร้อมทั้งให้ช่วยคิด Exercise,Example,วิธีแก้ไขโค้ดพร้อมปัญหาที่พบ`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `33` | `1` | `5` | `Concept + Short Code Illustration` |
| Member 2 | `0` | `3` | `1` | `8` | `Detailed Code + Live Demo` |
| Member 3 | `0` | `10` | `1` | `2` | `Rust vs Other Language + PPL Analysis` |
| Member 4 | `0` | `6` | `1` | `5` | `Exercises + Common Mistakes + Challenge` |

### Teamwork Reflection

**How did your team collaborate?**

`ได้มีการพูดคุยผ่าน Line และมีการปรึกษาหาข้อมูลพร้อมวิธีแก้ไขร่วมกัน`

**Problems encountered**

`มีโค้ดบางอย่างที่ไม่เข้าใจถึงวิธีแก้ไขปัญหา`

**How did you solve them?**

`ก็ได้วิเคราะห์ช่วยเหลือกันจนแก้ไขโค้ดได้`

---

## 15. Final Checklist

- [x] Learning Objectives ครบ 3–4 ข้อ
- [x] Key Concepts ครบถ้วน
- [x] Syntax / Rules
- [x] Runnable Code Examples
- [x] Code Compile และ Run ได้จริง
- [x] Common Mistakes
- [x] Exercises 2 ข้อ พร้อม Solutions
- [x] PPL Perspective
- [x] Rust vs Other Language
- [x] References อย่างน้อย 4 แหล่ง
- [x] AI Usage Declaration
- [x] GitHub Contribution
- [x] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [x] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [x] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/soonklang/rust-tutorial-2569`

**Chapter Path:** `01-introduction-to-rust-programming-paradigms`

**Final PR:** ``

**Submitted by:** `Group 1`

**Date:** `2026-10-04`

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
