# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 02
> **Topic No.:** 02
> **Topic Name:** Rust Environment, Cargo & First Program
> **ประเด็นหลักที่ควรครอบคลุม:** การติดตั้ง, rustc, Cargo, project structure, cargo new, cargo run, cargo build

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายระพีพัทธ์ ชอบสูงเนิน | 660710097 | `@660710097` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายสุธิสิทธิ์ กลิ่นพยอม | 660710102 | `@660710102` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายกฤตภณธ์ เดชาถิรปัญญา | 660710107 | `@660710107` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายจักรกฤช คิดดี | 660710108 | `@660710108` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

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

`หัวข้อนี้เป็นการแนะนำพื้นฐานของภาษา Rust ตั้งแต่การเตรียมโปรแกรมและสภาพแวดล้อมสำหรับเขียน Rust การใช้ Cargo เพื่อสร้างและจัดการโปรเจกต์ รวมถึงการเขียนโปรแกรม Rust โปรแกรมแรก เพื่อให้เข้าใจขั้นตอนการเขียน ตรวจสอบ และรันโปรแกรมได้อย่างถูกต้อง ซึ่งเป็นพื้นฐานสำคัญก่อนที่จะไปเรียนเรื่องอื่น ๆ ในภาษา Rust`

---

## 4. Key Concepts

### 4.1 `[Rust Environment]`

`Rust Environment คือเครื่องมือและสภาพแวดล้อมที่จำเป็นสำหรับการเขียนและรันโปรแกรม Rust โดยเครื่องมือหลัก ได้แก่ Rust Compiler (rustc) และ Cargo`

**ตัวอย่าง**

```rust
rustc --version
cargo --version
```

**Explanation**

`rustc --version` ใช้ตรวจสอบเวอร์ชันของ Rust Compiler และ `cargo --version` ใช้ตรวจสอบว่า Cargo ติดตั้งอยู่ในเครื่องหรือไม่ 

---

### 4.2 `[Cargo]`

`Cargo คือ **Package Manager และ Build System ของ Rust** ใช้สำหรับสร้างโปรเจกต์ จัดการ Dependencies, Compile และ Run โปรแกรม ทำให้การพัฒนา Rust เป็นระบบมากขึ้น`

```rust
cargo new hello_rust
cd hello_rust
cargo run
```
**Explanation**

`cargo new` สร้างโปรเจกต์ Rust ใหม่พร้อมโครงสร้างพื้นฐาน จากนั้น `cargo run` จะ Compile และรันโปรแกรมในโปรเจกต์นั้น`

---

### 4.3 `[First Rust Program]`

`โปรแกรม Rust แบบง่ายจะเริ่มต้นการทำงานจากฟังก์ชัน main() และสามารถใช้ println!() เพื่อแสดงข้อความออกทางหน้าจอ`

```rust
fn main() {
    println!("Hello, Rust!");
}
```
**Explanation**

 `fn main()` คือจุดเริ่มต้นของโปรแกรม ส่วน `println!()` เป็น Macro ที่ใช้แสดงข้อความ `Hello, Rust!` ออกทางหน้าจอ
 
---

## 5. Important Syntax / Rules

| Syntax / Rule                          | Meaning                                                                                      | Example                              |
| -------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------ |
| `fn main() { ... }`                    | เป็นฟังก์ชันหลักและเป็นจุดเริ่มต้นการทำงานของโปรแกรม Rust                                    | `fn main() { println!("Hello"); }`   |
| `use std::io;`                         | เรียกใช้ `std::io` จาก Rust Standard Library เพื่อใช้งานการรับข้อมูลจากผู้ใช้                | `use std::io;`                       |
| `let mut variable = String::new();`    | สร้างตัวแปรชนิด `String` ที่สามารถเปลี่ยนแปลงค่าได้                                          | `let mut name = String::new();`      |
| `println!("...");`                     | ใช้แสดงข้อความหรือผลลัพธ์บนหน้าจอ                                                            | `println!("Enter your name:");`      |
| `io::stdin().read_line(&mut variable)` | ใช้รับข้อมูลจากผู้ใช้ทางแป้นพิมพ์และเก็บข้อมูลไว้ในตัวแปร                                    | `io::stdin().read_line(&mut name)`   |
| `&mut variable`                        | ส่งตัวแปรแบบ mutable reference เพื่อให้ `read_line()` สามารถนำข้อมูลที่รับมาใส่ลงในตัวแปรได้ | `&mut name`                          |
| `.expect("...")`                       | ใช้จัดการกรณีที่การรับข้อมูลเกิดข้อผิดพลาด โดยจะแสดงข้อความที่กำหนด                          | `.expect("Failed to read line")`     |
| `.trim()`                              | ใช้ตัดช่องว่างและตัวขึ้นบรรทัดใหม่ที่ติดมากับข้อมูลที่ผู้ใช้ป้อน                             | `name.trim()`                        |
| `{}` ใน `println!`                     | ใช้ใส่ค่าของตัวแปรลงในข้อความที่ต้องการแสดง                                                  | `println!("Name: {}", name.trim());` |
| `;`                                    | ใช้ปิดคำสั่งแต่ละคำสั่งใน Rust                                                               | `let mut name = String::new();`      |
| `{ ... }`                              | ใช้กำหนดขอบเขตของฟังก์ชันหรือกลุ่มคำสั่ง                                                     | `fn main() { ... }`                  |

### Important Rules

1. **โปรแกรม Rust เริ่มทำงานจาก `fn main()`**
   โปรแกรมจะเริ่มทำงานจาก `fn main()` และทำงานตามลำดับคำสั่งภายในฟังก์ชัน

2. **ตัวแปรที่รับข้อมูลจากผู้ใช้ใช้ `mut`**
   ใช้ `mut` เพื่อให้ตัวแปรสามารถเปลี่ยนค่าได้เมื่อรับข้อมูลจากผู้ใช้

```rust
let mut name = String::new();
```

3. **การรับข้อมูลจากผู้ใช้ใช้ `io::stdin().read_line()`**
   คำสั่งนี้ใช้รับข้อมูลจากแป้นพิมพ์และเก็บไว้ในตัวแปร

```rust
io::stdin().read_line(&mut name).expect("Failed to read line");
```

4. **ใช้ `.trim()` ก่อนแสดงข้อมูลที่รับจากผู้ใช้**
   `trim()` ใช้ตัดช่องว่างและตัวขึ้นบรรทัดใหม่ที่ติดมากับข้อมูลที่ผู้ใช้ป้อน

```rust
println!("Name : {}", name.trim());
```

5. **ใช้ `println!()` สำหรับแสดงข้อความและข้อมูล**
   โปรแกรมใช้ `println!()` เพื่อแสดงคำสั่งให้ผู้ใช้กรอกข้อมูล และแสดงชื่อกับคะแนนที่รับมา
   
---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[รับชื่อจากผู้ใช้]`

**Purpose:** `ตัวอย่างนี้เป็นการรับข้อมูลจากผู้ใช้ โดยให้ผู้ใช้พิมพ์ชื่อ แล้วโปรแกรมนำชื่อที่พิมพ์มาแสดงบนหน้าจอ`

```rust
use std::io;

fn main() {
    let mut name = String::new();

    println!("Enter your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}!", name.trim());
}
```

**Expected Output**

```text
Enter your name:
First
Hello, First!
```

**Explanation**

เริ่มจาก

```rust
use std::io;
```

เพื่อเรียกใช้เครื่องมือจาก Rust Standard Library สำหรับการรับข้อมูลจากผู้ใช้

จากนั้นสร้างตัวแปร `name` สำหรับเก็บชื่อ

```rust
let mut name = String::new();
```

คำว่า `mut` ทำให้ตัวแปรสามารถเปลี่ยนค่าได้ เพราะข้อมูลจะถูกเพิ่มเข้าไปหลังจากผู้ใช้กรอกชื่อ

ส่วน

```rust
io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");
```

ใช้สำหรับรับข้อความจากแป้นพิมพ์และเก็บข้อมูลไว้ใน `name`

สุดท้าย

```rust
println!("Hello, {}!", name.trim());
```

ใช้แสดงชื่อของผู้ใช้ โดย `trim()` ใช้ตัดตัวขึ้นบรรทัดใหม่ที่ติดมากับการกด Enter

---

### Example 2 — `[รับคะแนนและแสดงผล]`

**Purpose:** ตัวอย่างนี้แสดงการรับคะแนนจากผู้ใช้และนำคะแนนที่รับมาแสดงผล ซึ่งเป็นแนวคิดเดียวกับโปรแกรม `Student Score Recorder PPL`

```rust
use std::io;

fn main() {
    let mut midterm = String::new();

    println!("Enter Midterm score (35):");
    io::stdin()
        .read_line(&mut midterm)
        .expect("Failed to read line");

    println!("Midterm: {}/35", midterm.trim());
}
```

**Expected Output**

```text
Enter Midterm score (35):
30
Midterm: 30/35
```

**Explanation**

โปรแกรมสร้างตัวแปร `midterm` เพื่อใช้เก็บคะแนนที่ผู้ใช้กรอก

```rust
let mut midterm = String::new();
```

จากนั้นใช้

```rust
io::stdin()
    .read_line(&mut midterm)
    .expect("Failed to read line");
```

เพื่อรับคะแนนจากผู้ใช้และเก็บไว้ในตัวแปร `midterm`

สุดท้ายใช้

```rust
println!("Midterm: {}/35", midterm.trim());
```

เพื่อแสดงคะแนนที่ผู้ใช้ป้อน โดย `{}` ใช้สำหรับแทรกค่าของตัวแปรลงในข้อความ และ `trim()` ใช้ตัดตัวขึ้นบรรทัดใหม่ออก

ตัวอย่างนี้สามารถนำหลักการเดียวกันไปใช้กับคะแนน `Final`, `Kahoot`, `Project`, `Attendance` และ `Typing` ในโปรแกรมหลักได้

---

## 7. Common Mistakes

### Mistake 1 — `รัน cargo run นอกโฟลเดอร์โปรเจกต์`

**Problem**

`cargo new สร้างโฟลเดอร์ใหม่ให้ แต่ไม่ได้ย้าย directory ให้ ถ้ารัน cargo run ทันทีจะเกิด error`

**Incorrect Code**

```rust
cargo new hello_cargo
cargo run
```

**Correct Code**

```rust
cargo new hello_cargo
cd hello_cargo
cargo run
```

**Why?**

`Cargo อ่านค่ากำหนดโปรเจกต์จากไฟล์ Cargo.toml จึงต้องรันคำสั่งในโฟลเดอร์ของโปรเจกต์ (หรือโฟลเดอร์ย่อยของมัน)`

---

### Mistake 2 — `ลืมส่ง argument ผ่าน -- ตอนใช้ cargo run`

**Problem**

`ต้องการส่ง argument ให้โปรแกรมของเรา แต่พิมพ์ต่อท้าย cargo run ตรงๆ โดยเฉพาะ argument ที่ขึ้นต้นด้วย - Cargo จะตีความว่าเป็น option ของ Cargo เอง และแจ้ง error`

**Incorrect Code**

```rust
cargo run --name Silpakorn
```

**Correct Code**

```rust
cargo run -- --name Silpakorn
```

**Why?**

`เครื่องหมาย -- คั่นระหว่าง option ของ Cargo กับ argument ที่จะส่งต่อให้โปรแกรมของเรา สิ่งที่อยู่หลัง -- Cargo จะไม่ตีความเอง`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `คอมไพล์ผ่านหรือไม่? (Immutable Variable)`

**Problem**

`โค้ดต่อไปนี้คอมไพล์ผ่านหรือไม่ ถ้าผ่านผลลัพธ์คืออะไร ถ้าไม่ผ่านให้บอกว่าบรรทัดไหนผิดเพราะอะไร`

```rust
fn main() {
    let x = 10;
    let mut y = 20;
    y = y + x;
    x = y - 5;
    println!("{} {}", x, y);
}
```

**Hint**

`ตัวแปรใน Rust เป็น immutable โดยค่าเริ่มต้นสังเกตว่าตัวแปรใดถูกกำหนดค่าใหม่ และตัวแปรนั้นประกาศด้วย mut หรือไม่`

**Solution**

```rust
fn main() {
    let mut x = 10;
    let mut y = 20;
    y = y + x;
    x = y - 5;
    println!("{} {}", x, y);
}
```

**Explanation**

`คอมไพล์ไม่ผ่าน ไม่มีผลลัพธ์ เกิด error ที่บรรทัด x = y - 5;
x ประกาศด้วย let x = 10; จึงเป็น immutable ไม่สามารถกำหนดค่าใหม่ได้
y ประกาศด้วย let mut y จึงแก้ค่าได้ บรรทัด y = y + x; ไม่มีปัญหา
Rust ตรวจจับข้อผิดพลาดนี้ตั้งแต่ขั้นคอมไพล์ ก่อนที่โปรแกรมจะรัน
ค่าหลังแก้: y = 20 + 10 = 30 แล้ว x = 30 - 5 = 25 จึงพิมพ์ 25 30`

---

### Exercise 2 — `ผลลัพธ์คืออะไร? (Cargo และ First Program)`

**Problem**

`โปรแกรมจะแสดงผลลัพธ์ใดออกมา`

```rust
fn main() {
    let language = "Rust";
    let tool = "Cargo";

    println!("Language: {}", language);

    if tool == "Cargo" {
        println!("Tool: {}", tool);
    } else {
        println!("Unknown tool");
    }
}
```

`รันคำสั่ง cargo run`

**Hint**

`cargo run จะ Compile และ Run โปรแกรมใน Cargo project
ดูค่าของตัวแปร language และ tool
ตรวจสอบเงื่อนไข if ว่าเป็น true หรือ false
println! แต่ละคำสั่งจะทำงานตามลำดับจากบนลงล่าง`

**Solution**

```rust
    fn main() {
    let language = "Rust";
    let tool = "Cargo";

    println!("Language: {}", language);

    if tool == "Cargo" {
        println!("Tool: {}", tool);
    } else {
        println!("Unknown tool");
    }
}


```

**Explanation**

`โปรแกรม คอมไพล์ผ่านและทำงานได้ตามปกติ
ผลลัพธ์คือ
Language: Rust
Tool: Cargo
cargo run จะทำการ Compile และ Run โปรแกรม
โดยเริ่มจาก println! แรกจึงแสดง Language: Rust จากนั้นตรวจสอบว่า tool มีค่าเท่ากับ "Cargo" หรือไม่ 
ซึ่งเป็นจริง จึงทำงานในส่วน if และแสดง Tool: Cargo ส่วน else จะไม่ถูกทำงาน เพราะเงื่อนไขเป็นจริง`

**Output**
```
Language: Rust
Tool: Cargo
```
---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`เกี่ยวข้องกับ Syntax ของคำสั่ง Rust เช่น fn main(), println!(), cargo new, cargo build และ cargo run ซึ่งแต่ละคำสั่งมีรูปแบบการเขียนที่กำหนดไว้`

### 9.2 Semantics

`cargo new ใช้สร้าง Project, cargo build ใช้ Compile โปรแกรม และ cargo run ใช้ Compile และ Run โปรแกรม ส่วน rustc ทำหน้าที่ Compile โค้ด Rust
`

### 9.3 Type System

`Rust เป็น Static Typing โดย rustc ตรวจสอบ Type และข้อผิดพลาดบางส่วนตั้งแต่ Compile Time ก่อนรันโปรแกรม`

### 9.4 Memory / Resource Management

`Rust ใช้ Ownership, Borrowing และ Lifetime ในการจัดการ Memory โดย Compiler ตรวจสอบกฎเหล่านี้ก่อนรัน และไม่ต้องใช้ Garbage Collector`

### 9.5 Abstraction / Other PPL Concepts

`Cargo ช่วยจัดการ Project และ Dependencies โดยใช้ Cargo.toml และแบ่งโครงสร้างเป็น src/main.rs ทำให้เกิด Modularity และจัดการ Scope ของโปรแกรมได้เป็นระบบ`

### 9.6 Why Rust?

`Rust ใช้ Static Typing, Ownership และ Compiler Checking เพื่อเพิ่ม Memory Safety และ Reliability พร้อมรักษา Performance สูง`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / Ruby]`

| Aspect | Rust | Python | C | C++ | Java | Kotlin | Ruby |
|---|---|---|---|---|---|---|---|
| Syntax | `ใช้ fn, let และโครงสร้างของ Cargo` | `Syntax กระชับ ใช้ Indentation` | `Syntax เป็นโครงสร้างพื้นฐาน` | `มี Syntax และคุณสมบัติค่อนข้างหลากหลาย` | `เน้น Class และ Object` | `Syntax กระชับกว่า Java` | `Syntax กระชับและอ่านง่าย` |
| Semantics / Behavior | `ใช้ rustc Compile และ Cargo จัดการ Build/Run` | `ทำงานผ่าน Python Runtime` | `Compile เป็น Native Code` | `Compile เป็น Native Code` | `Compile เป็น Bytecode และทำงานบน JVM` | `Compile และทำงานบน JVM` | `ทำงานผ่าน Ruby Runtime` |
| Type System | `Static Typing และ Type Inference` | `Dynamic Typing` | `Static Typing` | `Static Typing` | `Static Typing` | `Static Typing` | `Dynamic Typing` |
| Memory Management | `Ownership, Borrowing และ Lifetime` | `Garbage Collection` | `จัดการ Memory ได้โดยตรง` | `ใช้ RAII และ Smart Pointer` | `Garbage Collection` | `Garbage Collection` | `Garbage Collection` |
| Safety | `เน้น Memory Safety และตรวจสอบก่อน Runเน้น Memory Safety และตรวจสอบก่อน Run` | `จัดการ Memory อัตโนมัติ` | `มีความเสี่ยงจากการจัดการ Memory` | `มีความยืดหยุ่นแต่ยังมีความเสี่ยงด้าน Memory` | `มีการจัดการ Memory อัตโนมัติ` | `มี Memory Management และ Null Safety` | `จัดการ Memory อัตโนมัติ` |

### Rust Example

```rust
fn main() {
    let name = "Rust";
    println!("Hello, {}!", name);
}
```

### Python Example

```python
name = "Python"
print(f"Hello, {name}!")
```

### C Example

```C
#include <stdio.h>

int main() {
    printf("Hello, C!\n");
    return 0;
}
```

### C++ Example

```C++
#include <iostream>

int main() {
    std::cout << "Hello, C++!" << std::endl;
    return 0;
}
```

### Java Example

```Java
public class Main {
    public static void main(String[] args) {
        String name = "Java";
        System.out.println("Hello, " + name + "!");
    }
}
```

### Kotlin Example

```Kotlin
fun main() {
    val name = "Kotlin"
    println("Hello, $name!")
}
```

### Ruby Example

```Ruby
name = "Ruby"
puts "Hello, #{name}!"
```
### Analysis

`Rust เน้น Safety, Reliability และ Performance โดยใช้ Static Typing, Ownership และ Compiler Checking ขณะที่ Python และ Ruby เน้นความง่ายในการเขียน, C และ C++ เน้นการควบคุมระบบและประสิทธิภาพ และ Java กับ Kotlin เน้นการทำงานบน JVM และการจัดการ Memory อัตโนมัติ`

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

`รับผิดชอบสรุปแนวคิดหลักของ Rust Environment, Cargo และ First Program พร้อมจัดทำตัวอย่างโค้ดสั้น ๆ เพื่อให้เข้าใจการเขียนและรันโปรแกรม Rust เบื้องต้น`

**Member 2**

`รับผิดชอบเขียนโค้ด Rust แบบละเอียด อธิบายโครงสร้างและการทำงานของโปรแกรม พร้อมสาธิตการสร้างโปรเจกต์และรันโปรแกรมจริงด้วย Cargo`

**Member 3**

`รับผิดชอบเปรียบเทียบภาษา Rust กับภาษาอื่น เช่น C/C++ และวิเคราะห์แนวคิดของภาษาโปรแกรม (PPL) ในด้าน Syntax, Type System, Memory Management, Compilation และ Performance`

**Member 4**

`รับผิดชอบจัดทำแบบฝึกหัดเกี่ยวกับ Rust และ Cargo พร้อมยกตัวอย่างข้อผิดพลาดที่พบบ่อย วิธีแก้ไข และคำถามท้าทายเพื่อทดสอบความเข้าใจของผู้เรียน`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `W3Schools`
2. `https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html`
3. `https://doc.rust-lang.org/book/ch01-02-hello-world.html`
4. `https://doc.rust-lang.org/book/ch01-03-hello-cargo.html`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `ใช้เพื่อตรวจสอบความถูกต้องของข้อมูล เพื่อคอนเฟิร์มข้อมูลให้ถูกต้อง` | `ตรวจสอบโดยการเปรียบเทียบกับแหล่งข้อมูลของที่ Ai หามาด้วยอีกที` |
| `Claude.ai` | `ใช้เป็นตัวช่วยตรวจสอบอีกชั้น เพื่อเช็กความถูกต้องของข้อมูลและโค้ดอย่างละเอียด` | `เปรียบเทียบคำตอบกับ ChatGPT และทดสอบโค้ดก่อนนำเสนอ` |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `0` | `0` | `0` | `เขียน Concept + Short Code Illustration และจัดการ Repository รวบรวมงานกลุ่ม` |
| Member 2 | `0` | `0` | `0` | `0` | `เขียน Important Syntax / Rules และ Runnable Code Examples พร้อม Detailed Code + Live Demo` |
| Member 3 | `0` | `6` | `0` | `0` | ` Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL)` |
| Member 4 | `0` | `12` | `1` | `0` | `ทำ Exercises 2ข้อ + Common Mistakes 2 ช้อ และ Challengeข้อทายผลลัพธ์ของโค้ดพร้อมเฉลยและคำอธิบาย` |

### Teamwork Reflection

**How did your team collaborate?**

`กลุ่มของเราแบ่งหน้าที่กันรับผิดชอบตามหัวข้อย่อย และทำงานร่วมกันผ่าน GitHub โดยเริ่มต้นจากให้ตัวแทนกลุ่ม ทำการ Fork Repository หลักของรายวิชามาไว้ที่บัญชีของตนเอง จากนั้นได้ทำการเชิญ (Invite) สมาชิกคนอื่นๆ ในกลุ่มเข้ามาเป็น Collaborator ใน Repository นั้น เพื่อให้ทุกคนสามารถเข้ามาแก้ไขไฟล์และกด Commit โค้ดลงใน Branch "main" ร่วมกันได้โดยตรง ทำให้สามารถรวบรวมงานทั้งหมดไว้ในที่เดียวกันได้อย่างเป็นระบบ`

**Problems encountered**

`ในช่วงแรก กลุ่มของเราพบปัญหาเรื่องความเข้าใจในการทำ Pull Request (PR) และการจัดการ Branch โดยสมาชิกมีการแก้ไขไฟล์และ Commit แยกกันไปคนละ Branch (เช่น patch-1, patch-4) และต่างคนต่างเปิด PR ซ้อนกัน ทำให้โค้ดของสมาชิกแต่ละคนไม่มารวมอยู่ใน PR เดียวกัน นอกจากนี้ยังพบปัญหาความสับสนในการเลือกเป้าหมาย (Base/Compare repository) ในการรวมไฟล์ ทำให้หน้าต่างเปรียบเทียบไม่แสดงผลอัปเดตล่าสุด`

**How did you solve them?**

`เราแก้ปัญหาโดยการปรับโครงสร้างการทำงานใหม่ทั้งหมด โดยเข้าไปตั้งค่าในเมนู Settings > Collaborators เพื่อมอบสิทธิ์ให้เพื่อนทุกคนสามารถเข้าถึง Repository ของตัวแทนกลุ่มได้โดยตรง เมื่อทุกคนกดยอมรับคำเชิญแล้ว เราได้ตกลงกันให้สมาชิกทุกคนทำการแก้ไขและ Commit งานลงใน Branch "main" เพียงที่เดียว เมื่อไฟล์งานของทุกคนรวมกันเสร็จสมบูรณ์แล้ว ตัวแทนกลุ่มจึงทำการเปิด Pull Request ใหม่ที่ดึงข้อมูลจาก Branch "main" ของกลุ่ม ส่งไปยัง Branch "main" ของอาจารย์ ทำให้สามารถรวบรวมงานเป็นชิ้นเดียวได้สำเร็จ`

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
- [ ] GitHub Contribution
- [x] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [x] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [x] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `https://github.com/660710097/rust-tutorial-2569`

**Chapter Path:** `02-rust-environment-cargo-first-program`

**Final PR:** `#21`

**Submitted by:** `[Group 02]`

**Date:** `[2036-09-30]`

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
