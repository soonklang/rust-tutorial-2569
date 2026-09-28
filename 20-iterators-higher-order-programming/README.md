# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 20
> **Topic No.:** 20
> **Topic Name:** Iterators & Higher-Order Programming
> **ประเด็นหลักที่ควรครอบคลุม:** iter(), map, filter, fold, collect, lazy evaluation

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายณัฐชนน รักวงศ์ | 670710646 | `@670710646` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวณัฐฐาพร เสตะวีระ | 670710647 | `@670710647` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายณัฐดนัย ศรีไวย | 670710648 | `@[กรอก GitHub username]` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายพรรสวกร สมใจ | 670710651 | `@[กรอก GitHub username]` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`Iterators คือ interface ที่ใช้ดูข้อมูลใน collection/array ทีละตัว มีคุณสมบัติ lazy evaluation โดยจะไม่ทำงานจนกว่าจะต้องส่งผลลัพธ์ ช่วยประหยัดหน่วยความจำ ส่วน High-order programming ฟังก์ชันที่สามารถรับฟังก์ชันอื่นเป็น Argument หรือส่งคืนฟังก์ชันอื่นเป็นผลลัพธ์ได้ นิยมใช้กับเมธอดอย่าง map, filter, และ fold เพื่อประมวลผลข้อมูลใน Vector หรือ Iterator โดยไม่ต้องใช้ลูป for`

---

## 4. Key Concepts

### 4.1 `iter()`

**คำอธิบาย**

`สร้างตัววนซ้ำ (Iterator) โดยขอยืมอ่านข้อมูล โดยส่งคืนค่าเป็น Reference`

**ตัวอย่าง**

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for number in numbers.iter() {
        println!("{}", number);
    }
}
```

**Explanation**

`numbers.iter() สร้าง Iterator ที่ขอยืมข้อมูลจาก numbers `

---

### 4.2 `map()`

`เปลี่ยนแปลง (Transform) ข้อมูลทุกตัวใน Iterator โดยส่งคืนค่าเป็น Iterator ตัวใหม่ ในแต่ละรอบ number จะเป็น Reference (&i32) ที่ชี้ไปยังข้อมูลเดิม ดังนั้นข้อมูลใน numbers ยังคงสามารถใช้งานต่อได้หลังจากวนลูปเสร็จ`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = numbers.iter().map(|number| number * 2);

    for number in doubled {
        println!("{}", number);
    }
}
```

**Explanation**

`map() นำข้อมูลแต่ละตัวใน Iterator ไปผ่าน Closure ที่กำหนด (|number| number * 2) หมายถึงนำแต่ละค่าไปคูณ 2 โดย map() จะคืนค่าเป็น Iterator ใหม่ และเป็น Lazy Iterator คือจะยังไม่ประมวลผลทันที จนกว่าจะมีการเรียกใช้งาน Iterator เช่น for loop`

---

### 4.3 `filter()`

`คัดกรองข้อมูลตามเงื่อนไข โดย Closure ต้องคืนค่าเป็น true หรือ false`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers = numbers
        .iter()
        .filter(|number| **number % 2 == 0);

    for number in even_numbers {
        println!("{}", number);
    }
}
```

**Explanation**

`filter() ตรวจสอบข้อมูลทีละตัวด้วยเงื่อนไข (**number % 2 == 0) ถ้าได้ true จะเก็บค่านั้นไว้ถ้าได้ false จะไม่ส่งค่านั้นต่อไป`

---

### 4.4 `fold()`

`ยุบรวมข้อมูลทั้งหมดใน Iterator ให้เหลือค่าเดี่ยว โดยคืนค่าผลลัพธ์สุดท้ายชนิดเดียว`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let sum = numbers
        .iter()
        .fold(0, |total, number| total + number);

    println!("{}", sum);
}
```

**Explanation**

`โดย fold() เริ่มต้นด้วยค่า 0 จากนั้นนำข้อมูลแต่ละตัวมาสะสมใน total (0 + 1 = 1, 1 + 2 = 3, 3 + 3 = 6, 6 + 4 = 10) ดังนั้นผลลัพธ์ = 10`

---

### 4.5 `collect()`

`รวบรวมข้อมูลจาก Iterator กลับไปเป็น Collection คืนค่าเป็น collection ใหม่ เช่น Vec, HashMap`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled: Vec<i32> = numbers
        .iter()
        .map(|number| number * 2)
        .collect();

    println!("{:?}", doubled);
}
```

**Explanation**

`map() สร้าง Iterator ใหม่จากข้อมูลเดิม เป็น (2, 4, 6) จากนั้น collect() จะดึงข้อมูลทั้งหมดจาก Iterator และรวบรวมกลับมาเป็น Vec<i32> ได้เป็น [2, 4, 6]`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `.iter()` | สร้าง Iterator ที่ยืมข้อมูล และส่งคืนค่าเป็น Reference | `numbers.iter()` |
| `.map(\|x\| ...)` | Transform ข้อมูลแต่ละตัว และคืนค่าเป็น Iterator ใหม่ | `.map(\|x\| x * 2)` |
| `.filter(\|x\| ...)` | คัดกรองข้อมูล โดยเก็บเฉพาะค่าที่เงื่อนไขเป็น `true` | `.filter(\|x\| x % 2 == 0)` |
| `.fold(init, \|acc, x\| ...)` | รวมข้อมูลทั้งหมดให้เหลือค่าเดียว โดยมีค่าเริ่มต้น | `.fold(0, \|sum, x\| sum + x)` |
| `.collect()` | รวบรวมข้อมูลจาก Iterator กลับเป็น Collection | `.collect::<Vec<_>>()` |
| `Iterator` เป็น Lazy | จะยังไม่ประมวลผลข้อมูลทันที จนกว่าเราจะเรียกใช้ Iterator เช่น `collect()`, `for`, หรือ `next()` | `.map(...).collect()` |
| `collect()` ต้องรู้ชนิดปลายทาง | Rust ต้องรู้ว่าต้องการสร้าง Collection ชนิดใด | `let v: Vec<_> = iter.collect()` |
| `Closure` | ฟังก์ชันแบบไม่ต้องตั้งชื่อที่สามารถรับค่าและใช้ตัวแปรจาก scope ภายนอกได้ | `let add = \|a, b\| a + b;` |
| `i32` | ชนิดข้อมูลจำนวนเต็ม (integer) แบบมีเครื่องหมาย (signed) ขนาด 32 บิต | `let x: i32 = 100;`| 

### Important Rules

1. `map()` และ `filter()` คืนค่าเป็น `Iterator` ใหม่ และยังไม่ประมวลผลทันที เพราะ Iterator ใน Rust เป็น Lazy
2. `iter()` จะยืมข้อมูล (`borrow`) ดังนั้นข้อมูลต้นฉบับยังสามารถใช้งานต่อได้
3. `collect()` ใช้ Consume Iterator และรวบรวมค่าที่ได้กลับเป็น Collection เช่น `Vec` หรือ `HashMap`
4. `fold()` จะ Consume Iterator และคืนค่าเป็นค่าเดี่ยว ไม่ใช่ Iterator
5. `map()` ใช้สำหรับ Transform ข้อมูล ส่วน `filter()` ใช้สำหรับคัดกรองข้อมูล และสามารถ Chain ต่อกันได้
6. `collect()` ต้องสามารถอนุมานได้ว่าต้องการ Collection ชนิดใด หาก Rust อนุมานไม่ได้ ต้องระบุ Type เช่น `Vec<_>`
7. Iterator สามารถ Chain หลาย operation ต่อกันได้ เช่น:
```rust
let result: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|x| *x > 5)
    .collect();
```

---

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
