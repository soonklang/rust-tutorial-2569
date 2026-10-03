# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 19
> **Topic No.:** 19
> **Topic Name:** Closures & Functional Programming
> **ประเด็นหลักที่ควรครอบคลุม:** closures, parameters, capture, higher-order functions และ functional programming

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายก้องภพ คุณดำรงชัย | 670710639 | `@670710639` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายคีรเทพ ก้องสุวรรณ | 670710641 | `@670710641` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นางสาวจิณณพัต แหล่งหล้า | 670710642 | `@670710642` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นางสาวจุฑารัตน์ รู้วงษ์ | 670710643 | `@670710643` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

Functional Programming เป็นแนวคิดการเขียนโปรแกรมที่เน้นการใช้ฟังก์ชันในการประมวลผลข้อมูลและแบ่งการทำงานออกเป็นส่วนย่อย ๆ เพื่อให้โค้ดสามารถนำกลับมาใช้ซ้ำได้

Closure เป็นฟังก์ชันรูปแบบหนึ่งที่สามารถกำหนดขึ้นภายในโปรแกรม และสามารถเข้าถึงตัวแปรจากบริบทภายนอกได้ ใน Rust Closure สามารถนำมาใช้ร่วมกับแนวคิด Functional Programming เพื่อเขียนโค้ดให้กระชับและจัดการข้อมูลได้สะดวกขึ้น เช่น การใช้ map และ filter

---

## 4. Key Concepts

### 4.1 `Functional Programming`

**คำอธิบาย**

`Functional Programming เป็นแนวคิดการเขียนโปรแกรมที่เน้นการใช้ฟังก์ชันในการประมวลผลข้อมูล และแบ่งการทำงานออกเป็นส่วนย่อย ๆ เพื่อให้โค้ดสามารถนำกลับมาใช้ซ้ำได้`

**ตัวอย่าง**

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(5, 3);
    println!("{}", result);
}
```

**Explanation**

`ฟังก์ชัน add รับค่าจำนวนเต็ม 2 ค่า คือ a และ b จากนั้นนำค่าทั้งสองมาบวกกันและคืนค่าผลลัพธ์กลับมา เมื่อเรียก add(5, 3) จะได้ผลลัพธ์เป็น 8 และนำไปแสดงด้วย println!`

---

### 4.2 `Closure`

**คำอธิบาย**

`Closure คือฟังก์ชันแบบไม่ระบุชื่อที่สามารถกำหนดขึ้นภายในโปรแกรม โดยมีรูปแบบการเขียนที่กระชับ และสามารถนำไปเก็บไว้ในตัวแปรเพื่อเรียกใช้งานได้`

**ตัวอย่าง**

```rust
fn main() {
    let add = |a, b| a + b;

    let result = add(5, 3);
    println!("{}", result);
}
```

**Explanation**

`|a, b| a + b คือ Closure ที่รับค่า a และ b แล้วนำมาบวกกัน จากนั้นเก็บ Closure นี้ไว้ในตัวแปร add เมื่อเรียก add(5, 3) จะได้ผลลัพธ์เป็น 8 แล้วเก็บไว้ในตัวแปร result ก่อนนำไปแสดงผล`

---

### 4.3 `Closure Parameters`

**คำอธิบาย**

`Closure Parameters คือพารามิเตอร์ที่ใช้รับค่าที่ส่งเข้ามาเมื่อเรียกใช้งาน Closure โดยสามารถกำหนดชนิดข้อมูลของพารามิเตอร์ และนำค่าที่รับมาไปประมวลผลภายใน Closure ได้`

**ตัวอย่าง**

```rust
fn main() {
    let add = |x: i32, y: i32| x + y;

    let result = add(5, 3);

    println!("{}", result);
}
```

**Explanation**

`ใน main สร้าง Closure add ที่รับพารามิเตอร์ 2 ตัว คือ x และ y โดยกำหนดให้ทั้งสองตัวเป็นชนิด i32 จากนั้นนำค่าทั้งสองมาบวกกันและคืนผลลัพธ์ออกมา เมื่อเรียก add(5, 3) จะนำค่า 5 และ 3 เข้าไปเป็นพารามิเตอร์ของ Closure จึงคำนวณ 5 + 3 และได้ผลลัพธ์เป็น 8`

---

### 4.4 `Closure Capture`

**คำอธิบาย**

`Closure ใน Rust สามารถเข้าถึงตัวแปรที่อยู่นอกขอบเขตของ Closure ได้ ความสามารถนี้เรียกว่า Closure Capture`

**ตัวอย่าง**

```rust
fn main() {
    let x = 10;

    let add_x = |n| n + x;

    println!("{}", add_x(5));
}
```

**Explanation**

`ตัวแปร x ถูกสร้างขึ้นภายนอก Closure และมีค่าเป็น 10 จากนั้น Closure add_x รับค่า n และนำไปบวกกับ x เมื่อเรียก add_x(5) ค่า n จะเป็น 5 และ Closure สามารถเข้าถึง x ที่อยู่ภายนอกได้ จึงคำนวณ 5 + 10 และแสดงผลเป็น 15`

---

### 4.5 `Higher-Order Functions`

**คำอธิบาย**

`Higher-Order Function คือฟังก์ชันที่สามารถรับฟังก์ชันหรือ Closure เป็นพารามิเตอร์ หรือคืนฟังก์ชัน/Closure กลับมาเป็นผลลัพธ์ได้ แนวคิดนี้ช่วยให้สามารถส่งพฤติกรรมการทำงานเข้าไปให้ฟังก์ชันจัดการได้`

**ตัวอย่าง**

```rust
fn apply(value: i32, operation: fn(i32) -> i32) -> i32 {
    operation(value)
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let result = apply(5, double);
    println!("{}", result); // 10
}
```

**Explanation**

`main() จะเรียกใช้ฟังก์ชัน apply() โดยส่งค่า 5 และฟังก์ชัน double เข้าไปเป็นพารามิเตอร์ จากนั้น apply() จะนำฟังก์ชัน double ที่ได้รับมาเรียกใช้กับค่า 5 ผ่านคำสั่ง operation(value) ซึ่งจะทำให้เกิดการทำงานเหมือน double(5) ฟังก์ชัน double() จะนำค่า 5 ไปคูณด้วย 2 และส่งผลลัพธ์ 10 กลับมายัง apply() หลังจากนั้นค่าที่ได้จะถูกเก็บไว้ในตัวแปร result และนำไปแสดงผลด้วย println!() ทำให้ผลลัพธ์ที่แสดงบนหน้าจอคือ 10`

---
## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| \|x\| expr | นิยาม Closure รับพารามิเตอร์เดียว และประเมินค่านิพจน์สั้นๆ ทันที | \|x\| x + 1 |
| \|a: i32, b: i32\| -> i32 { ... } | นิยาม Closure แบบระบุชนิดข้อมูล (Type Annotations) และมีบล็อกคำสั่งหลายบรรทัด | \|a: i32, b: i32\| -> i32 { a + b } |
| move \|...\| { ... } | บังคับย้าย Ownership ของตัวแปรภายนอกที่ถูก Capture เข้ามาใน Closure อย่างเด็ดขาด | move \|\| println!("{:?}", data) |
| Fn(&self) | Trait สำหรับ Closure ที่ยืมตัวแปรภายนอกแบบอ่านอย่างเดียว (Immutable Reference) เรียกซ้ำได้หลายครั้ง | fn call_fn<F: Fn()>(f: F) |
| FnMut(&mut self) | Trait สำหรับ Closure ที่ยืมตัวแปรภายนอกมาแก้ไขค่า (Mutable Reference) เรียกซ้ำได้ | fn call_fn_mut<F: FnMut()>(mut f: F) |
| FnOnce(self) | Trait สำหรับ Closure ที่ย้าย Ownership ของตัวแปรภายนอกเข้าสู่บริบทของตน เรียกใช้งานได้เพียงครั้งเดียว | fn call_fn_once<F: FnOnce()>(f: F) |

### Important Rules

1. **Type Inference Latching:** คอมไพเลอร์ของ Rust จะอนุมานชนิดข้อมูลของพารามิเตอร์และค่าที่ส่งกลับของ Closure จากการเรียกใช้งานครั้งแรกโดยอัตโนมัติ และจะยึด Type นั้นไว้อย่างถาวร ไม่สามารถส่งอาร์กิวเมนต์ต่างชนิดกันในการเรียกครั้งถัดไปได้
2. **Least Privilege Capture Mechanism:** ตัว Borrow Checker จะเลือกวิธีการ Capture ตัวแปรจากสภาพแวดล้อมโดยใช้วิธีที่จำกัดสิทธิ์น้อยที่สุดก่อนเสมอ (&T -> &mut T -> T by-value) เว้นแต่จะระบุคีย์เวิร์ด move เพื่อบังคับย้าย Ownership
3. **Trait Hierarchy & Dispatching:** โครงสร้างลำดับขั้นของ Closure Trait เป็นไปตามกฎ Fn: FnMut: FnOnce (Closure ที่ implement Fn จะ implement FnMut และ FnOnce ด้วยเสมอ) และเนื่องจาก Closure แต่ละตัวมีชนิดข้อมูลเฉพาะตัวที่ไม่ระบุชื่อ (Unique Anonymous Type) การคืนค่า Closure ออกจากฟังก์ชันจึงต้องระบุผ่าน Static Dispatch (impl Fn(...) -> ...) หรือ Dynamic Dispatch ผ่าน Heap (Box<dyn Fn(...) -> ...>)

---

## 6. Runnable Code Examples


### Example 1 — Closure Traits: Fn, FnMut, FnOnce

**Purpose:** สาธิต 3 รูปแบบการ capture ตัวแปรของ Closure ตาม Trait ที่ Compiler เลือกให้อัตโนมัติ

```rust
fn demo_closure_traits() {
    println!("\n--- [Demo 1] Closure Traits (Fn, FnMut, FnOnce) ---");

    // 1.1 Fn: ยืมอ่านอย่างเดียว (Immutable Borrow)
    let greeting = String::from("Hello");
    let print_greeting = || println!("Fn trait: {}", greeting);
    print_greeting();
    println!("ค่า greeting ยังใช้ต่อได้: {}", greeting); // compile ผ่านเพราะแค่ยืมอ่าน

    // 1.2 FnMut: ยืมแบบแก้ไขค่าได้ (Mutable Borrow)
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("FnMut trait count: {}", count);
    };
    increment();
    increment();
    println!("Final count: {}", count);

    // 1.3 FnOnce: ย้าย Ownership (Move/Consume) รันได้ครั้งเดียว
    let data = vec![1, 2, 3];
    let consume_data = || {
        println!("FnOnce trait: vector length = {}", data.len());
        drop(data); // data ถูกทำลายทิ้งตรงนี้
    };
    consume_data();
    // consume_data(); // <-- หากเอาคอมเมนต์ออกจะ Compile Error ทันที!
}
```

**Expected Output**

```text
--- [Demo 1] Closure Traits (Fn, FnMut, FnOnce) ---
Fn trait: Hello
ค่า greeting ยังใช้ต่อได้: Hello
FnMut trait count: 1
FnMut trait count: 2
Final count: 2
FnOnce trait: vector length = 3
```

**Explanation**

- **Fn (Immutable Borrow):** print_greeting ยืมค่า greeting แบบอ่านอย่างเดียว (&String) — เรียกซ้ำกี่ครั้งก็ได้ และตัวแปรเดิมยังใช้ต่อได้หลัง closure
- **FnMut (Mutable Borrow):** increment ยืม count แบบ &mut เพื่อแก้ไขค่า — ต้องประกาศ let mut increment จึงจะเรียกได้ และเรียกซ้ำได้หลายครั้ง
- **FnOnce (Move/Consume):** consume_data ย้าย Ownership ของ data เข้ามาแล้วทำ drop() — เรียกได้ครั้งเดียวเท่านั้น หากเรียกซ้ำจะ Compile Error

---

### Example 2 — Concurrency with move Closure

**Purpose:** สาธิตการใช้ move keyword บังคับ Closure ย้าย Ownership เพื่อส่ง data ข้าม Thread อย่างปลอดภัย

```rust
use std::thread;

fn demo_move_concurrency() {
    println!("\n--- [Demo 2] Concurrency with `move` ---");

    let thread_data = vec![10, 20, 30];

    // หากไม่ใส่ keyword move ตัว Compiler จะเตือนว่า data อาจมีอายุสั้นกว่า Thread (Lifetime issue)
    let handle = thread::spawn(move || {
        println!("Thread worker ได้รับ data: {:?}", thread_data);
    });

    handle.join().unwrap();
    // println!("{:?}", thread_data); // Compile Error: ownership ย้ายไปที่ Thread แล้ว
}
```

**Expected Output**

```text
--- [Demo 2] Concurrency with `move` ---
Thread worker ได้รับ data: [10, 20, 30]
```

**Explanation**

- move บังคับให้ thread_data ย้าย Ownership เข้าไปใน Closure ที่ส่งให้ Thread — ทำให้ Thread เป็นเจ้าของ data ได้อย่างสมบูรณ์
- หากไม่ใส่ move Compiler จะ Error เพราะ thread_data อาจถูก drop ก่อนที่ Thread จะทำงานเสร็จ (Lifetime ไม่ตรง)
- หลัง move แล้ว ตัวแปร thread_data ใน scope เดิมจะใช้ไม่ได้อีกต่อไป

---

### Example 3 — Returning Closures (impl Fn vs Box<dyn Fn>)

**Purpose:** สาธิตการคืนค่า Closure จากฟังก์ชัน ทั้งแบบ Static Dispatch (impl Fn) และ Dynamic Dispatch (Box<dyn Fn>)

```rust
// Static Dispatch (Zero-cost): Compiler รู้ type ตอน compile
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// Dynamic Dispatch (Heap Allocated): รองรับ Closure หลาย type ในค่าคืน
fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    if op == "add" {
        Box::new(|a, b| a + b)
    } else {
        Box::new(|a, b| a * b)
    }
}

fn demo_returning_closures() {
    println!("\n--- [Demo 3] Returning Closures ---");

    let add_five = create_adder(5);
    println!("create_adder(5)(10) = {}", add_five(10));

    let calc = make_operation("multiply");
    println!("make_operation('multiply')(4, 5) = {}", calc(4, 5));
}
```

**Expected Output**

```text
--- [Demo 3] Returning Closures ---
create_adder(5)(10) = 15
make_operation('multiply')(4, 5) = 20
```

**Explanation**

- **impl Fn(i32) -> i32:** Compiler รู้ type ของ Closure ตอน compile → ไม่มี overhead (Static Dispatch / Zero-cost) แต่คืนได้แค่ Closure ชนิดเดียว
- **Box<dyn Fn(i32, i32) -> i32>:** ใช้ Trait Object บน Heap → รองรับการคืน Closure ต่างชนิดกันผ่าน if/else (Dynamic Dispatch) แต่มี overhead จาก heap allocation และ vtable lookup
- ทั้งสองแบบต้องใช้ move เพื่อย้าย captured variable (เช่น x) เข้า Closure ไม่ให้เกิด dangling reference

---

### Example 4 — Functional Programming Pipeline (Zero-Cost Abstractions)

**Purpose:** สาธิต Iterator Chain แบบ Functional (filter → map → fold) ที่ Rust compile เป็น loop เดียวโดยไม่มี Heap Overhead

```rust
fn demo_functional_pipeline() {
    println!("\n--- [Demo 4] Functional Programming (Zero-Cost Abstractions) ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Pipeline: Filter -> Map -> Fold (Declarative Style)
    // Rust จะ compile เป็น Loop ภาษาเครื่องตัวเดียว ไม่มี Heap Overhead
    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // คัดเฉพาะเลขคู่
        .map(|&x| x * x)          // ยกกำลังสอง
        .fold(0, |acc, x| acc + x); // รวมผลลัพธ์

    println!("ผลรวมของเลขคู่ยกกำลังสอง (2^2 + 4^2 + 6^2 + 8^2 + 10^2) = {}", sum_of_even_squares);
}
```

**Expected Output**

```text
--- [Demo 4] Functional Programming (Zero-Cost Abstractions) ---
ผลรวมของเลขคู่ยกกำลังสอง (2^2 + 4^2 + 6^2 + 8^2 + 10^2) = 220
```

**Explanation**

- **.iter():** สร้าง Iterator จาก Vector (ไม่ copy ข้อมูล แค่สร้าง pointer)
- **.filter(|&&x| x % 2 == 0):** คัดเฉพาะเลขคู่ — &&x เป็นการ destructure double reference (&&i32 → i32)
- **.map(|&x| x * x):** แปลงค่าแต่ละตัวเป็นยกกำลังสอง
- **.fold(0, |acc, x| acc + x):** สะสมผลรวม เริ่มจาก 0 — เป็น consuming adaptor ที่ทำให้ทั้ง pipeline ทำงานจริง
- Rust ใช้ **Zero-Cost Abstraction** คือ code ที่เขียนแบบ Declarative/Functional จะถูก compile เป็น loop ภาษาเครื่องตัวเดียว ประสิทธิภาพเท่ากับเขียน for-loop ด้วยมือ

---

## 7. Common Mistakes

### Mistake 1 — พยายามคืน `Fn(...)` ตรง ๆ จากฟังก์ชัน

**Problem**

ต้องการคืน Closure จากฟังก์ชันโดยระบุชนิดคืนค่าเป็น `Fn(...)` ตรง ๆ แต่เกิด error เพราะ `Fn(...)` เป็น trait ไม่ใช่ concrete type และ trait ตรง ๆ ไม่มีขนาดแน่นอนในช่วง compile time ขณะที่ Rust ต้องรู้ขนาดของค่าที่ฟังก์ชันจะคืนเสมอ

**Incorrect Code**

```rust
fn factory() -> Fn(i32) -> i32 {
    let num = 5;

    |x| x + num
}

fn main() {
    let f = factory();
    println!("{}", f(1));
}
```
**Output**
```rust
error[E0277]: the trait bound Fn(i32) -> i32: Sized is not satisfied

```

**Correct Code**

```rust
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}

fn main() {
    let f = factory();
    println!("{}", f(1));
}
```
**Output**
```rust
6
```

**Why?**

ในโค้ดนี้มีการพยายามคืนค่าเป็น:

```rust
Fn(i32) -> i32
```

แต่ `Fn(i32) -> i32` เป็น trait ไม่ใช่ชนิดข้อมูลแบบ concrete ที่มีขนาดแน่นอน
Rust จึงไม่สามารถรู้ได้ว่าค่าที่จะคืนจากฟังก์ชันมีขนาดเท่าไรในช่วง compile time

ฟังก์ชันใน Rust ต้องมีชนิดคืนค่าที่ทราบขนาดแน่นอน เว้นแต่จะใช้การห่อด้วยชนิดที่มีขนาดแน่นอน เช่น pointer หรือ smart pointer

ดังนั้นแนวทางที่ใช้ได้คือห่อ Closure ด้วย `Box<dyn Fn(i32) -> i32>`:

```rust
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}
```
`Box` มีขนาดแน่นอน จึงสามารถใช้เป็นชนิดคืนค่าของฟังก์ชันได้ ส่วน `dyn Fn(...)` คือ trait object ที่ใช้แทน Closure ที่แท้จริงซึ่งมีชนิดเป็น anonymous type


ใน Rust สมัยใหม่ อีกทางเลือกหนึ่งคือใช้ `impl Fn(i32) -> i32` หากฟังก์ชันคืน Closure เพียงชนิดเดียว
```rust
fn factory() -> impl Fn(i32) -> i32 {
    let num = 5;
    move |x| x + num
}
```

---

### Mistake 2 — ลืมใช้ `move` ตอนคืน Closure ที่ capture ตัวแปรภายในฟังก์ชัน

**Problem**

ต้องการคืน Closure จากฟังก์ชัน และ Closure นั้นมีการใช้งานตัวแปร `num` ที่อยู่ภายในฟังก์ชันเดียวกัน แต่เกิด error เพราะแม้จะใช้ `Box` เพื่อเก็บ Closure แล้ว ก็ยังไม่เพียงพอ หาก Closure ยัง borrow ตัวแปรจาก stack frame เดิมของฟังก์ชันอยู่ จึงต้องใช้ `move` เพื่อย้ายค่าที่ capture เข้าไปใน Closure โดยตรง

**Incorrect Code**

```rust
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(|x| x + num)
}

fn main() {
    let f = factory();
    println!("{}", f(1));
}

```

**Output**
```rust
error[E0373]: closure may outlive the current function, but it borrows `num`
```

**Correct Code**

```rust
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}

fn main() {
    let f = factory();
    println!("{}", f(1));
}
```
**Output**
```rust
6
```

**Why?**

ในโค้ดนี้ Closure มีการอ้างถึงตัวแปร `num`

```rust
|x| x + num
```
ถ้าไม่ใส่ `move` Rust จะพยายามให้ Closure borrow `num` จาก scope เดิมของฟังก์ชัน `factory()`

ปัญหาคือเมื่อ `factory()` ทำงานจบลง ตัวแปร `num` จะถูกทำลายไปพร้อมกับ stack frame ของฟังก์ชันนั้น ทำให้ Closure ที่ถูกคืนออกไปไม่สามารถอ้างถึง `num` ที่ยืมมาได้อย่างปลอดภัย

ดังนั้นจึงต้องใช้ `move`:

```rust
Box::new(move |x| x + num)
```
เพื่อย้ายค่า `num` เข้าไปเก็บใน Closure environment โดยตรง ทำให้ Closure มีข้อมูลของตัวเองและสามารถถูกคืนออกจากฟังก์ชันได้อย่างปลอดภัย

---

## 8. Exercises

### Exercise 1 — Once Function

**Problem**

จงเขียน Higher-Order Function ชื่อ `once` ที่รับ Closure `fn` เข้ามา แล้วคืนค่าเป็น Closure ใหม่ที่สามารถเรียกใช้งานฟังก์ชัน `fn` ได้เพียงครั้งเดียว เมื่อเรียกใช้งานครั้งแรก ให้ Closure ทำการประมวลผลและเก็บผลลัพธ์ที่ได้ไว้ หากมีการเรียกใช้งานในครั้งถัดไป ไม่ว่าจะส่ง Parameter อะไรเข้ามา ให้คืนค่าผลลัพธ์เดิมที่คำนวณได้จากครั้งแรก โดยไม่เรียกใช้ `fn` ซ้ำอีก

**Hint**

ใช้ Closure ในการเก็บสถานะ โดยเก็บผลลัพธ์ที่คำนวณได้จากการเรียกครั้งแรกไว้ภายใน Closure
เนื่องจาก Closure ที่คืนออกมาต้องสามารถเปลี่ยนแปลงสถานะภายในได้ จึงสามารถใช้ `FnMut` ได้

**Solution**

```rust
fn once<F, A, R>(mut f: F) -> impl FnMut(A) -> R
where
    F: FnMut(A) -> R,
    R: Clone,
{
    let mut result: Option<R> = None;

    move |arg| {
        if let Some(value) = &result {
            return value.clone();
        }

        let value = f(arg);
        result = Some(value.clone());

        value
    }
}

fn main() {
    let mut initialize_app = once(|app_name: &str| {
        println!("Initializing {}...", app_name);
        format!("{} is ready", app_name)
    });

    println!("{}", initialize_app("MySystem"));
    println!("{}", initialize_app("OtherSystem"));
}
```

**Output**

```text
Initializing MySystem...
MySystem is ready
MySystem is ready
```

**Explanation**

1. `once` เป็น Higher-Order Function เพราะรับ Closure `f` เข้ามาเป็น Argument และคืน Closure กลับออกมา

2. ตัวแปร `result` ใช้สำหรับเก็บผลลัพธ์จากการเรียก `f` ครั้งแรก

   ```rust
   let mut result: Option<R> = None;
   ```

   ในตอนเริ่มต้น `result` ยังไม่มีค่า จึงเป็น `None`

3. `move` ทำให้ Closure ที่ถูกคืนออกมาสามารถเป็นเจ้าของ `result` และ `f` ได้เอง

   ```rust
   move |arg| {
       ...
   }
   ```

4. ในการเรียกครั้งแรก `result` เป็น `None` ดังนั้น `f(arg)` จะถูกเรียก:

   ```rust
   let value = f(arg);
   ```

   จากนั้นผลลัพธ์จะถูกเก็บไว้ใน `result`

5. ในการเรียกครั้งถัดไป `result` มีค่าแล้ว:

   ```rust
   if let Some(value) = &result {
       return value.clone();
   }
   ```

   Closure จึงไม่เรียก `f` อีก แต่คืนผลลัพธ์เดิมออกมา

6. `R: Clone` จำเป็นในตัวอย่างนี้ เพราะผลลัพธ์ `R` ต้องสามารถถูกนำกลับมาคืนซ้ำในการเรียกครั้งถัดไป Closure ที่คืนออกมาจึงมี State ของตัวเอง คือค่า `result` ที่ถูกเก็บไว้ภายใน Closure

แนวคิดสำคัญของ Exercise นี้คือ **Closure สามารถเก็บ State และรักษา State นั้นไว้ระหว่างการเรียกใช้งานแต่ละครั้ง** ซึ่งเป็นหนึ่งในคุณสมบัติสำคัญของ Closure ใน Rust

---

### Exercise 2 — Higher-Order Iterator Filter Generator

**Problem**

จงเขียน Higher-Order Function ชื่อ `create_filter(property, condition)` ที่รับ Closure สำหรับดึงค่าจาก `item` และฟังก์ชันเงื่อนไข `condition` จากนั้นคืนค่าเป็น Predicate Function ที่สามารถนำไปใช้กับ `.filter()` ของ Iterator เพื่อคัดกรองข้อมูลตามเงื่อนไขที่กำหนดได้

**Hint**

ใช้หลักการ Higher-Order Function และ Closure โดยฟังก์ชัน `create_filter` จะรับ Closure `property` สำหรับดึงค่าที่ต้องการจาก `item` และรับ Closure `condition` สำหรับตรวจสอบค่านั้น จากนั้นคืน Closure ที่รับ `item` และส่งค่าที่ดึงออกมาให้ `condition`

**Solution**

```rust
fn create_filter<T, V, P>(
    property: impl Fn(&T) -> V,
    condition: P,
) -> impl Fn(&T) -> bool
where
    P: Fn(V) -> bool,
{
    move |item| {
        let value = property(item);
        condition(value)
    }
}

fn main() {
    let products = vec![
        ("Laptop", 1200),
        ("Mouse", 25),
        ("Keyboard", 75),
    ];

    let is_price_over_50 =
        create_filter(|product: &(&str, i32)| product.1, |price| price > 50);

    let result: Vec<_> = products
        .iter()
        .filter(|product| is_price_over_50(product))
        .collect();

    println!("{:?}", result);
}
```

**Output**

```text
[("Laptop", 1200), ("Keyboard", 75)]
```

**Explanation**

1. `create_filter` เป็น Higher-Order Function เพราะรับ Closure เข้ามาเป็น Argument และคืน Closure กลับออกมา

2. `property` ทำหน้าที่กำหนดว่าเราต้องการดึงข้อมูลส่วนไหนจาก `item`

3. `condition` ทำหน้าที่ตรวจสอบค่าที่ `property` ดึงออกมา และต้องคืนค่า `bool`

4. `create_filter` คืน Closure นี้ออกมา:

```rust
move |item| {
    let value = property(item);
    condition(value)
}
```

Closure ที่คืนออกมาจึงมีหน้าที่รับ `item` แล้วนำไปผ่านขั้นตอน `property` → `condition`

5. เมื่อใช้กับ `.filter()`:

```rust
products
    .iter()
    .filter(|product| is_price_over_50(product))
```

`.filter()` จะส่งแต่ละ `item` เข้ามาให้ `is_price_over_50` และ Closure จะคืน `true` หรือ `false` เพื่อกำหนดว่าจะเก็บ `item` นั้นไว้หรือไม่

จุดสำคัญคือ `property` และ `condition` ถูกเก็บไว้ใน Closure ที่ `create_filter` คืนกลับมา ทำให้เราสามารถสร้าง Predicate Function ที่นำกลับมาใช้ซ้ำได้

---
## 9. PPL Perspective

Closures เป็นฟีเจอร์ของ Rust ที่เกี่ยวข้องกับแนวคิดของ Programming Language หลายด้าน เช่น Syntax, Semantics, Type System, Memory Management และ Functional Programming

### 9.1 Syntax

Closure ใน Rust ใช้เครื่องหมาย | | สำหรับกำหนด parameter และตามด้วย expression หรือ body ของ Closure
let add = |x| x + 10;
ในตัวอย่าง |x| x + 10 คือ Closure โดย x เป็น parameter และ x + 10 เป็นการทำงานของ Closure
Rust สามารถอนุมานชนิดข้อมูลของ parameter และ return value ของ Closure ได้จากบริบท ทำให้สามารถเขียน Closure ได้สั้นกว่า function ในบางกรณี

### 9.2 Semantics / Behavior

Closure สามารถ capture ตัวแปรจาก environment ที่มันถูกสร้างขึ้นมาได้ ซึ่งเป็นพฤติกรรมสำคัญที่แตกต่างจาก regular function ใน Rust
let n = 10;
let add_n = |x| x + n;
ในตัวอย่าง Closure สามารถนำ n ซึ่งเป็นตัวแปรภายนอกมาใช้งานได้ โดย Rust จะกำหนดวิธีการ capture ให้เหมาะสมกับการใช้งาน เช่น การยืมแบบ immutable reference, mutable reference หรือการย้าย ownership
แนวคิดนี้เกี่ยวข้องกับ Scope, Environment และ Binding เพราะ Closure สามารถเก็บความสัมพันธ์กับตัวแปรที่อยู่ใน environment ของมันไว้ได้

### 9.3 Type System

Closure แต่ละตัวใน Rust จะมี anonymous type ที่เป็นเอกลักษณ์ของตัวเอง และไม่สามารถระบุชนิดของ Closure ด้วยชื่อ type ทั่วไปได้โดยตรง
Rust ใช้ Closure Traits เพื่อกำหนดลักษณะการเรียกใช้งาน Closure ได้แก่
Fn เรียกใช้งานซ้ำได้โดยไม่แก้ไขค่าที่ capture
FnMut เรียกใช้งานซ้ำได้ และสามารถแก้ไขค่าที่ capture ได้
FnOnce Closure ที่อาจนำค่าที่ capture ออกไปใช้จนไม่สามารถเรียกซ้ำได้
Closure จะ implement trait เหล่านี้ตามลักษณะการใช้งานค่าที่ capture ภายใน body ของ Closure

### 9.4 Memory / Resource Management

Closures ใน Rust ทำงานร่วมกับระบบ Ownership และ Borrowing ของภาษา
เมื่อ Closure ใช้ตัวแปรจากภายนอก Rust จะตรวจสอบว่าควร capture ตัวแปรนั้นในรูปแบบใด โดยทั่วไปจะเลือกวิธีที่จำเป็นน้อยที่สุดก่อน เช่น การยืมค่า และสามารถใช้ move เพื่อบังคับให้ Closure capture ค่าโดยการย้ายหรือคัดลอกค่าเข้าไป
ดังนั้น Closure จึงเกี่ยวข้องโดยตรงกับการจัดการ ownership และ lifetime ของข้อมูลที่ถูก capture

### 9.5 Abstraction / Other PPL Concepts

Closure เป็นตัวอย่างของ Higher-Order Function เนื่องจากสามารถส่ง Closure เป็น argument ให้กับ function หรือ method อื่นได้
ตัวอย่างเช่น map() สามารถรับ Closure เพื่อกำหนดว่าต้องการเปลี่ยนข้อมูลแต่ละตัวอย่างไร
let numbers = vec![1, 2, 3];
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
แนวคิดนี้เกี่ยวข้องกับ Functional Programming เพราะสามารถนำ function หรือ Closure มาใช้เป็นข้อมูลและนำไปประกอบกับการประมวลผลข้อมูล เช่น map() และ filter()

### 9.6 Why Rust

Rust ออกแบบ Closures ให้ทำงานร่วมกับระบบ Type System, Ownership และ Borrowing ของภาษา
ข้อดีคือ compiler สามารถตรวจสอบการใช้งานตัวแปรที่ Closure capture รวมถึงชนิดของข้อมูลและการเข้าถึง memory ได้ตั้งแต่ compile time
นอกจากนี้ Closures และ Iterators ยังเป็น abstraction ระดับสูงที่ช่วยให้เขียนโค้ดแบบ Functional Programming ได้กระชับ โดย Rust มุ่งเน้นให้ abstraction เหล่านี้มีประสิทธิภาพโดยไม่เพิ่ม runtime overhead ที่ไม่จำเป็น

---

## 10. Rust vs Other Language
| Aspect                   | Rust                                                                                        | Python                                                                         | Java                                                                                                                | C++                                                                               |
| ------------------------ | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| **Syntax**               | ใช้ `\|x\| x + 10` สำหรับ Closure                                                           | ใช้ `lambda x: x + 10`                                                         | ใช้ Lambda เช่น `x -> x + 10`                                                                                       | ใช้ Lambda เช่น `[n](int x) { return x + n; }`                                    |
| **Semantics / Behavior** | Closure สามารถ **capture ตัวแปรจาก environment** และ Rust จะกำหนดวิธี capture ตามการใช้งาน  | Lambda สามารถเข้าถึงตัวแปรจาก scope ภายนอกได้                                  | Lambda สามารถ capture ตัวแปรจาก scope ภายนอกได้ แต่ตัวแปร local ที่ capture ต้องเป็น `final` หรือ effectively final | Lambda สามารถ capture ตัวแปรภายนอกได้ โดยกำหนดวิธี capture ได้                    |
| **Type System**          | Static type system และ Closure แต่ละตัวมี type เฉพาะของตัวเอง พร้อม `Fn`, `FnMut`, `FnOnce` | Dynamic typing และ Lambda เป็น object                                          | Static type system และ Lambda ใช้งานผ่าน Functional Interface                                                       | Static type system และ Lambda มี unnamed closure type                             |
| **Memory Management**    | ใช้ **Ownership, Borrowing และ `move`** ในการจัดการค่าที่ capture                           | ใช้ Garbage Collection / การจัดการ memory อัตโนมัติ                            | ใช้ **Garbage Collection**                                                                                          | ใช้ RAII และสามารถจัดการ memory แบบ explicit ได้                                  |
| **Safety**               | Compiler ตรวจสอบ Type, Ownership และ Borrowing ช่วยป้องกัน memory-related errors            | มีการตรวจสอบแบบ runtime และ Garbage Collection แต่ไม่มีระบบ Ownership แบบ Rust | มี Static Type Checking และ Garbage Collection                                                                      | มี Static Type Checking และ RAII แต่ต้องจัดการ lifetime / pointer อย่างระมัดระวัง |

### Rust Example
```rust
fn main() {
    let n = 10;
    let add_n = |x| x + n;

    println!("{}", add_n(5)); // 15
}
```
### Python Example
```py
n = 10
add_n = lambda x: x + n

print(add_n(5))  # 15
```
### Java Example
```java
import java.util.function.Function;

public class Main {
    public static void main(String[] args) {
        int n = 10;
        Function<Integer, Integer> addN = x -> x + n;

        System.out.println(addN.apply(5)); // 15
    }
}
```

### C++ Example
```cpp
#include <iostream>
using namespace std;

int main() {
    int n = 10;

    auto addN = [n](int x) {
        return x + n;
    };

    cout << addN(5) << endl; // 15
}
```
### Analysis
แม้ Rust, Python, Java และ C++ จะสามารถสร้าง Closure หรือ Lambda ที่ทำงานคล้ายกันได้ แต่แต่ละภาษามีแนวทางการออกแบบที่แตกต่างกัน
* Rust เน้นความปลอดภัยของ Memory โดย Closure ทำงานร่วมกับระบบ Ownership, Borrowing และ Closure Traits (Fn, FnMut, FnOnce)

* Python มี Syntax ที่สั้นและยืดหยุ่น และจัดการ Memory ให้อัตโนมัติ แต่ไม่มีระบบ Ownership และ Borrowing แบบ Rust

* Java ใช้ Lambda Expression ร่วมกับ Functional Interface และมีกฎเรื่องตัวแปรที่ถูก capture ต้องเป็น final หรือ effectively final

* C++ ให้ Programmer กำหนดวิธีการ Capture ได้อย่างชัดเจน เช่น Capture แบบ Value หรือ Reference

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

`Concept + Short Code Illustration + silde`

**Member 2**

`Detailed Code + Live Demo + silde`

**Member 3**

`Rust vs Other Language + PPL Analysis + silde`

**Member 4**

`Exercises + Common Mistakes + Challenge + silde`

---

## 12. References

1. `https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/closures.html#closures`
2. `https://rust-book.cs.brown.edu/ch13-00-functional-features.html`
3. `https://rust-book.cs.brown.edu/ch13-01-closures.html`
4. `https://doc.rust-lang.org/rust-by-example/fn.html`
5. `https://doc.rust-lang.org/rust-by-example/fn/closures.html`

---

## 13. AI Usage Declaration

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `ใช้เพื่อช่วยร่าง code` | `นำโค้ดไปทดลองรันและทดสอบผลลัพธ์ รวมถึงตรวจสอบว่าโค้ดตรงตามโจทย์และไม่มีข้อผิดพลาด` |
| `Gemini` | `หาข้อมูล Syntax` | `ไปเปิดดู References ว่าตรงกันไหม` |

### Declaration

- [✓] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [✓] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [✓] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [✓] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`ใช้ AI ในการช่วยหาข้อมูลร่วมกับ เอกสารอ้างอิงที่น่าเชื่อถือ ตรวจสอบโดยเปรียบเทียบข้อมูลที่ได้จาก AI กับเอกสารอ้างอิง`
`ใช้ AI ช่วยคิดและออกแบบโจทย์ ตรวจสอบโดยนำโค้ดไปทดลองรันจริง ถ้าผลลัพธ์ถูกต้อง จะนำโจทย์นี้มาใช้`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |

| Member 1 | `0` | `51` | `1` | `0` | `2` |

| Member 2 | `0` | `1` | `1` | `0` | `98` |

| Member 3 | `0` | `1` | `1` | `0` | `3` |

| Member 4 | `0` | `1` | `1` | `0` | `37` |

### Teamwork Reflection

**How did your team collaborate?**

`นัดประชุมหาวัน deadline ส่งงาน แบ่งงานของแต่ละคน สมาชิกแต่ละคนแบ่งงานตามส่วนที่รับผิดชอบ โดยแต่ละคนทำงานบน Branch ของตนเอง เมื่อทำงานเสร็จจะ Commit และ Push ขึ้น GitHub แล้วสร้าง Pull Request เพื่อให้สมาชิกคนอื่นตรวจสอบโค้ด ก่อนที่จะ Merge เข้าสู่ Branch หลัก หากพบข้อผิดพลาดหรือมีข้อเสนอแนะ จะปรับแก้และส่ง Pull Request อีกครั้งจนกว่าจะเรียบร้อย แชร์ลิงก์ไฟล์ Canva ให้สมาชิกคนอื่นเข้าถึงและแก้ไขงานร่วมกัน แต่ละคนสามารถแก้ไขเนื้อหาในส่วนที่ได้รับมอบหมาย เมื่อแก้ไขเสร็จช่วยกันตรวจสอบความเรียบร้อยก่อนส่งงาน`

**Problems encountered**

`เกิด conflinct request`

**How did you solve them?**

`ค่อยๆ ให้สมาชิกแต่ละคนแตก Branch และ PR ส่งต่อกันทีละคนจนเสร็จ`

---

## 15. Final Checklist

- [✓] Learning Objectives ครบ 3–4 ข้อ
- [✓] Key Concepts ครบถ้วน
- [✓] Syntax / Rules
- [✓] Runnable Code Examples
- [✓] Code Compile และ Run ได้จริง
- [✓] Common Mistakes
- [✓] Exercises 2 ข้อ พร้อม Solutions
- [✓] PPL Perspective
- [✓] Rust vs Other Language
- [✓] References อย่างน้อย 4 แหล่ง
- [✓] AI Usage Declaration
- [✓] GitHub Contribution
- [✓] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [✓] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [✓] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/670710639/rust-tutorial-2569/tree/group19-closures`

**Chapter Path:** `chapter 19-closures-functional-programming `

**Final PR:** `#27`

**Submitted by:** `Group 19`

**Date:** `2026-10-02`
