# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 05
> **Topic No.:** 05
> **Topic Name:** Expressions & Statements
> **ประเด็นหลักที่ควรครอบคลุม:** expression vs statement, block expression, return value, if/match ในฐานะ expression

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายคมสัน กลิ่นหอม | 670710124 | `@670710124` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายฐิติพงศ์ ราชธานี | 670710125 | `@[กรอก GitHub username]` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายณัฏฐ์ธเนศ กุนทรฐิติวัสส์ | 670710126 | `@[กรอก GitHub username]` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายณัฐพงศ์ อวชัย | 670710127 | `@[กรอก GitHub username]` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายความแตกต่างระหว่าง **Expression** และ **Statement** ใน Rust ได้
2. อธิบายการทำงานของ **Block Expression**, **Tail Expression** และการคืนค่า (return value) จาก block/function ได้
3. เขียนและวิเคราะห์การใช้ `if` และ `match` ในฐานะ Expression ได้
4. เปรียบเทียบแนวคิด Expressions & Statements ของ Rust กับภาษา Python ได้

---

## 3. Introduction

Rust เป็นภาษาที่เน้นการทำงานผ่าน **expressions** เป็นหลัก โดย Expression คือส่วนของโค้ดที่เมื่อถูกประเมิน (evaluate) แล้วจะให้ค่า (value) และในระหว่างการประเมินอาจทำให้เกิดผลจากการทำงาน (effect) ได้ ส่วน **Statement** ใช้สำหรับประกาศสิ่งต่าง ๆ หรือจัดลำดับการประเมิน Expression ภายใน block

การเข้าใจ Expressions & Statements มีความสำคัญ เพราะช่วยให้เข้าใจว่าโค้ดส่วนใดสร้างค่า โค้ดส่วนใดนำค่านั้นไปใช้ต่อ และ semicolon (`;`) มีผลต่อค่าของ block อย่างไร แนวคิดนี้เชื่อมโดยตรงกับ **Block Expression**, การคืนค่าจาก function และการใช้ `if` / `match` เป็น Expression

---

## 4. Key Concepts

### 4.1 Expression vs Statement

**Expression** คือส่วนของโค้ดที่ถูก evaluate แล้วให้ value

```rust
5 + 3
```

`5 + 3` เป็น Expression และให้ value เป็น `8`

**Statement** เป็นองค์ประกอบภายใน block ที่ใช้ประกาศสิ่งต่าง ๆ หรือจัดลำดับการทำงาน

```rust
let x = 5 + 3;
```

ในบรรทัดนี้:

```text
let x = 5 + 3;  → Statement
        5 + 3   → Expression
          8     → Value
```

Rust มี Statement หลัก ๆ 2 กลุ่ม:

- **Declaration Statement** เช่น `let x = 10;`
- **Expression Statement** เช่น `v.pop();` ซึ่ง evaluate Expression แต่ไม่ใช้ค่าผลลัพธ์ต่อ

ตัวอย่าง Expression Statement:

```rust
let mut v = vec![1, 2, 3];
v.pop();
```

`v.pop()` นำสมาชิกตัวท้ายออกจาก vector และคืนค่ากลับมา แต่ในตัวอย่างนี้ค่าที่คืนมาจะไม่ถูกนำไปใช้ต่อ

---

### 4.2 Block Expression

ใน Rust block `{ ... }` สามารถเป็น Expression และมี value ของตัวเองได้

```rust
fn main() {
    let result = {
        let a = 5;
        let b = 3;
        a + b
    };

    println!("{}", result);
}
```

`a + b` เป็น Expression สุดท้ายของ block และไม่มี `;` จึงทำให้ block มี value เป็น `8`

---

### 4.3 Tail Expression and Return Value

Expression สุดท้ายของ block ที่ไม่มี semicolon (`;`) เรียกว่า **Tail Expression** และค่าของมันจะกลายเป็นค่าของ block

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`a + b` เป็น Tail Expression ดังนั้นค่าที่ได้จะถูกใช้เป็น return value ของ function โดยไม่จำเป็นต้องเขียน `return`

ถ้าเขียน:

```rust
let x = {
    5 + 3;
};
```

`5 + 3;` ถูกใช้เป็น Expression Statement ค่าที่ได้จะไม่ถูกใช้เป็นค่าของ block และเมื่อ block จบการทำงานตามปกติโดยไม่มี Tail Expression block จะมีค่าเป็น Unit `()`

---

### 4.4 `if` as an Expression

ใน Rust `if` สามารถเป็น Expression และให้ value ได้

```rust
fn main() {
    let score = 75;

    let grade = if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else {
        "C"
    };

    println!("{}", grade);
}
```

เมื่อ `score = 75` ค่า Expression ของ `if` คือ `"B"` และค่านี้ถูกนำไปกำหนดให้ `grade`

เมื่อใช้ `if` เพื่อสร้างค่า แต่ละ branch ต้องให้ค่าที่มีชนิดข้อมูลเข้ากันได้

---

### 4.5 `match` as an Expression

`match` สามารถเป็น Expression และให้ value จาก arm ที่ match ได้

```rust
fn main() {
    let number = 2;

    let text = match number {
        1 => "One",
        2 => "Two",
        _ => "Other",
    };

    println!("{}", text);
}
```

เมื่อ `number = 2` arm ที่ตรงคือ `2 => "Two"` ดังนั้น `match` ทั้งก้อนมี value เป็น `"Two"`

---

### 4.6 `loop` as an Expression

ใน Rust `loop` ก็สามารถเป็น **Expression** และให้ value ได้เช่นกัน

```rust
fn main() {
    let result = loop {
        break 10;
    };

    println!("{}", result);
}
```

ในตัวอย่างนี้ `loop` จะทำงานจนเจอ

```rust
break 10;
```

`break` จะหยุด `loop` และส่งค่า `10` ออกมาเป็น value ของ `loop` ทั้งก้อน ดังนั้น `result` จะมีค่าเป็น `10`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `let pattern = expression;` | ประกาศตัวแปร และใช้ค่าจาก Expression เป็นค่าเริ่มต้น | `let x = 5 + 3;` |
| `expression;` | ใช้ Expression เป็น Expression Statement และไม่ใช้ค่าผลลัพธ์ต่อ | `v.pop();` |
| `{ ... final_expression }` | Block Expression ที่ใช้ค่าจาก Expression สุดท้ายเป็นค่าของ block | `{ let x = 5; x + 1 }` |
| `if ... { expr } else { expr }` | `if` สามารถสร้าง value ได้ | `let x = if c { 1 } else { 0 };` |
| `match value { ... }` | `match` สามารถสร้าง value จาก arm ที่ตรงได้ | `let x = match n { 1 => "A", _ => "B" };` |

### Important Rules

1. Expression Statement จะ evaluate Expression แต่ไม่ใช้ค่าผลลัพธ์ต่อ
2. Expression สุดท้ายของ block ที่ไม่มี `;` จะเป็น Tail Expression และค่าของมันจะกลายเป็นค่าของ block
3. ถ้า block ไม่มี Tail Expression และจบการทำงานตามปกติ block จะมีค่าเป็น `()`
4. `if` ที่ใช้เป็น Expression ต้องให้ค่าจากแต่ละ branch ที่มี type เข้ากันได้
5. `match` ที่ใช้เป็น Expression จะให้ value จาก arm ที่ถูกเลือก

---
