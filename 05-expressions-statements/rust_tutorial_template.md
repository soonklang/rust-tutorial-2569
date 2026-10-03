# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม  
> **Topic No.:** `5`  
> **Topic Name:** `Expressions & Statements`  
> **Group No.:** `5`

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายคมสัน กลิ่นหอม | 670710124 | `@670710124` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายฐิติพงศ์ ราชธานี | 670710125 | `@670710125` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายณัฏฐ์ธเนศ กุนทรฐิติวัสส์ | 670710126 | `@670710126` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายณัฐพงศ์ อวชัย | 670710127 | `@670710127` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายความแตกต่างระหว่าง **Expression** และ **Statement** ใน Rust ได้
2. อธิบายการทำงานของ **Block Expression**, **Tail Expression** และการคืนค่า (return value) จาก block/function ได้
3. เขียนและวิเคราะห์การใช้ `if` และ `match` ในฐานะ Expression ได้
4. เปรียบเทียบแนวคิด Expressions & Statements ของ Rust กับภาษาอื่นๆได้

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

**Statement** คือคำสั่งที่ใช้ประกาศหรือสั่งให้โปรแกรมทำงาน โดยไม่ได้ใช้ผลลัพธต่อ

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
fn main() {
    let mut v = vec![1, 2, 3];
    v.pop();
    println!("{:?}", v);
}
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

### 4.7 Why Expression-Oriented?

การที่ `if` และ `match` สามารถให้ value ได้ ทำให้สามารถนำผลลัพธ์มากำหนดให้ตัวแปรได้โดยตรง โดยไม่ต้องสร้างตัวแปรก่อนแล้วเปลี่ยนค่าภายหลัง ซึ่งช่วยลด mutable state ที่ไม่จำเป็น และทำให้ติดตามที่มาของค่าได้ง่ายขึ้น

```rust
// เปลี่ยนค่าภายหลัง
let mut price = 0;

if is_member {
    price = 80;
} else {
    price = 100;
}
```
สามารถเขียนเป็น:

```rust
// สร้างค่าจาก Expression โดยตรง
let price = if is_member {
    80
} else {
    100
};
```

นอกจากนี้ ผลลัพธ์ของ Expression ยังสามารถนำไปเป็นส่วนหนึ่งของ Expression อื่นได้โดยตรง

```rust
fn main() {
    let is_member = true;
    let quantity = 3;

    let total = quantity * if is_member {
        80
    } else {
        100
    };

    println!("{}", total);
}
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `let pattern = expression;` | ประกาศตัวแปร และใช้ค่าจาก Expression เป็นค่าเริ่มต้น | `let x = 5 + 3;` |
| `expression;` | ใช้ Expression เป็น Expression Statement และไม่ใช้ค่าผลลัพธ์ต่อ | `v.pop();` |
| `{ ... final_expression }` | Block Expression ที่ใช้ค่าจาก Expression สุดท้ายเป็นค่าของ block | `{ let x = 5; x + 1 }` |
| `if ... { expr } else { expr }` | `if` สามารถสร้าง value ได้ | `let x = if c { 1 } else { 0 };` |
| `match value { ... }` | `match` สามารถสร้าง value จาก arm ที่ตรงได้ | `let x = match n { 1 => "A", _ => "B" };` |
| `loop { ... break value; }`     | `break` สามารถส่ง value ออกจาก `loop` ได้ | `let x = loop { break 10; };` |

### Important Rules

1. Expression Statement จะ evaluate Expression แต่ไม่ใช้ค่าผลลัพธ์ต่อ
2. Expression สุดท้ายของ block ที่ไม่มี `;` จะเป็น Tail Expression และค่าของมันจะกลายเป็นค่าของ block
3. ถ้า block ไม่มี Tail Expression และจบการทำงานตามปกติ block จะมีค่าเป็น `()`
4. `if` และ `match` ที่ใช้เป็น Expression จะให้ value ที่มี type เข้ากันได้
5. `break value;` สามารถใช้ส่ง value ออกจาก `loop` ได้
6. Value จาก Expression สามารถนำไปประกอบเป็นส่วนหนึ่งของ Expression อื่นได้

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `การคำนวณเกรดด้วย Block Expression`

**Purpose:** `สาธิตการใช้ Block Expression ในการคืนค่า (Return Value) เข้าสู่ตัวแปรโดยตรงโดยไม่ต้องใช้คำสั่ง return และแสดงความแตกต่างระหว่างการลงท้ายด้วย Expression (ไม่มี ;) กับ Statement (มี ;)`

```rust
fn main() {
    let score = 85;

    // Block Expression: คืนค่า String slice เข้าตัวแปร grade โดยตรง
    let grade = {
        let bonus = 5;
        let total_score = score + bonus;

        // ไม่ใส่ Semicolon (;) เพื่อให้เป็น Expression คืนค่าออกไป
        if total_score >= 80 {
            "A"
        } else if total_score >= 70 {
            "B"
        } else {
            "F"
        }
    };

    println!("Total calculated grade: {}", grade);
}
```

**Expected Output**

```text
Total calculated grade: A
```

**Explanation**

```
1. let score = 85; เป็น Statement (Declaration Statement) สำหรับประกาศตัวแปร
2. let grade = { ... }; เป็นการนำ Block Expression มากำหนดค่าให้ตัวแปร grade
3. ตัวแปร bonus และ total_score เป็น Local Variables ที่อยู่ภายใน Block Scope เท่านั้น ไม่สามารถเรียกใช้นอก {} ได้
4. บรรทัดสุดท้ายภายใน Block (if total_score >= 80 { ... }) ไม่มี Semicolon ; ทำให้ทำหน้าที่เป็น Expression ที่ถูกประเมินค่าและคืนค่าเป็น &str ออกมาให้กับตัวแปร grade
```

---

### Example 2 — `การคืนค่าด้วย match Expression และคำสั่ง return`

**Purpose:** `สาธิตการใช้ match ในฐานะ Expression เพื่อประเมินค่าผลลัพธ์ (Value) รวมถึงการใช้คำสั่ง return สำหรับการออกจากฟังก์ชันล่วงหน้า (Early Return) เมื่อเจอเงื่อนไขขอบเขต`

```rust
fn check_user_role(level: u32) -> &'static str {
    // Early Return: ใช้คำสั่ง return เพื่อคืนค่าและออกจากฟังก์ชันทันที
    if level == 0 {
        return "Guest";
    }

    // match ในฐานะ Expression: คืนค่า String slice ออกจากฟังก์ชันโดยไม่ต้องใช้คำสั่ง return
    match level {
        1 => "Member",
        2 => "Moderator",
        3 => "Admin",
        _ => "Unknown Role",
    }
}

fn main() {
    let user_level = 2;
    let role = check_user_role(user_level);

    println!("User role is: {}", role);
}
```

**Expected Output**

```text
User role is: Moderator
```

**Explanation**

```
1. คำสั่ง return (Explicit Return): บรรทัด return "Guest"; ใช้สำหรับหยุดการทำงานและส่งค่าออกจากฟังก์ชันทันทีก่อนจะไปถึงโค้ดส่วนอื่น (Early Return)  
2. match ในฐานะ Expression: โครงสร้าง match ทำหน้าที่ประเมินค่าและส่งผลลัพธ์จาก Arm ที่จับคู่สำเร็จออกมาเป็น Value เพื่อคืนค่าออกจากฟังก์ชันโดยตรง 
3. การละเว้น Semicolon ;: ท้ายโครงสร้าง match ไม่มีการใส่ ; เพื่อให้ผลลัพธ์ประเมินค่าเป็น Expression สำหรับคืนค่าให้ฟังก์ชัน check_user_role   
```

---

### Example 3 — `การคืนค่าจาก Loop ด้วยคำสั่ง break`

**Purpose:** `สาธิตการใช้ loop ในฐานะ Expression ที่สามารถประมวลผลการทำงานซ้ำ และคืนค่าผลลัพธ์กลับมาเข้าตัวแปรได้ทันทีผ่านคำสั่ง break value;`

```rust
fn main() {
    let mut counter = 0;

    // loop เป็น Expression ที่ส่งค่ากลับมาเข้าตัวแปร result ได้โดยตรง
    let result = loop {
        counter += 1;

        if counter == 3 {
            // คืนค่า counter * 10 ออกไปให้ตัวแปร result แล้วหยุด loop ทันที
            break counter * 10;
        }
    };

    println!("The result from loop execution is: {}", result);
}
```

**Expected Output**

```text
The result from loop execution is: 30
```

**Explanation**

```
1. ในภาษา Rust โครงสร้างควบคุมอย่าง loop ถือเป็น Expression ไม่ใช่แค่ Statement เหมือนภาษา C หรือ Java     
2. การใส่ค่าไว้หลังคำสั่ง break (เช่น break counter * 10;) เป็นการส่งผลลัพธ์ออกจาก Loop มายังตัวแปรที่รับค่าทันที 
3. ช่วยให้เขียนโค้ดกระชับขึ้น เพราะตัวแปร result จะได้รับค่าประมวลผลทันที โดยไม่ต้องสร้างตัวแปร mut เปล่าๆ ไว้นอก Loop ก่อน   
```

---

## 7. Common Mistakes

### Mistake 1 — `ใส่ Semicolon หลัง Expression สุดท้าย`

**Problem**

`การใส่ `;` หลัง expression สุดท้ายของ block โดยไม่ตั้งใจ จะทำให้ block นั้นไม่คืนค่าที่ต้องการ แต่จะมีค่าเป็น `()` แทน`

**Incorrect Code**

```rust
let x = {
    5 + 3;
};
```

**Correct Code**

```rust
let x = {
    5 + 3
};
```

**Why?**

`Rust ใช้ expression สุดท้ายของ block เป็นค่าที่ส่งออกจาก block ได้ แต่ expression นั้นต้องไม่มี `;` ต่อท้าย เพราะถ้ามี `;` Rust จะมองเป็น statement และ block จะมีค่าเป็น `()` แทน`
`() เป็นชนิดข้อมูลที่ใช้แทนกรณีที่ “ไม่มีค่าข้อมูลที่มีความหมายให้ส่งกลับ”`

---

### Mistake 2 — `ให้ค่าจาก if แต่ละ branch เป็นคนละชนิด`

**Problem**

`เมื่อใช้ if เป็น expression ค่าที่ได้จากแต่ละ branch ต้องมีชนิดข้อมูลที่เข้ากันได้`

**Incorrect Code**

```rust
let condition = true;

let result = if condition {
    10
} else {
    "ten"
};
```

**Correct Code**

```rust
let condition = true;

let result = if condition {
    10
} else {
    20
};
```

**Why?**

`Rust สามารถใช้ if เป็น expression เพื่อสร้างค่าได้ แต่ค่าที่ได้จาก if ต้องมี type เดียวกัน`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `Statement หรือ Expression`

**Problem**

`จงระบุว่าแต่ละบรรทัดเป็น Statement หรือ Expression`

```rust
let x = 10;
x + 5
x + 5;
if x > 5 { 1 } else { 0 }
```

**Hint**

`Expression สร้างค่า ส่วน statement ใช้ทำงานบางอย่างและมักจบด้วย ;`

**Solution**

```rust
let x = 10;                  -> Statement
x + 5                       -> Expression
x + 5;                      -> Expression ที่ถูกใช้เป็น statement
if x > 5 { 1 } else { 0 }   -> Expression
```

**Explanation**

`ใน Rust expression คือโค้ดที่ให้ค่าออกมา เช่น `x + 5` หรือ `if ... { ... } else { ... }``

`ส่วน statement คือคำสั่งที่ใช้ทำงานบางอย่าง เช่น `let x = 10;``

`เมื่อเติม `;` หลัง expression เช่น `x + 5;` ค่าที่ได้จาก expression จะไม่ถูกนำไปใช้ต่อ และ expression นั้นจะถูกใช้ในรูปของ statement`

---

### Exercise 2 — `[ใช้ if Expression เพื่อสร้างค่า]`

**Problem**

จงเติมโค้ดให้ตัวแปร `grade` มีค่าเป็น

- `"A"` เมื่อ `score >= 80`
- `"B"` เมื่อ `score >= 70`
- `"C"` ในกรณีอื่น

```rust
let score = 75;

let grade = ??????????;



println!("{}", grade);
```

**Hint**

`Rust สามารถใช้ if และ else if เป็น expression เพื่อสร้างค่าได้`

**Solution**

```rust
let score = 75;

let grade = if score >= 80 {
    "A"
} else if score >= 70 {
    "B"
} else {
    "C"
};

println!("{}", grade);
```

**Explanation**

`Rust สามารถใช้ if เป็น expression ได้ โดยค่าจาก branch ที่ตรงกับเงื่อนไขจะกลายเป็นค่าของ expression และถูกนำไปเก็บใน grade เมื่อ score = 75 เงื่อนไข score >= 70 เป็นจริง จึงได้ค่า "B"`

### Challenge — `[รวม if expression + block expression + semicolon]`
**Problem**

`1.result มีค่าเท่าไร`
`2.if ... else ... .ในโค้ตที่แนบให้เป็น Statement หรือ Expression`
`3.ถ้าเติม ; หลัง if ทั้งก้อน จะเกิดอะไรขึ้น`

`จากโค้ต`
```rust
let x = 4;

let result = {
    let y = x + 2;

    if y > 5 {
        y * 2
    } else {
        y - 1
    }
};

println!("{}", result);
```

**Hint**
`ลองไล่ค่าจากด้านในออกมาด้านนอก โดยดูว่า `y` มีค่าเท่าไร จากนั้นพิจารณาว่า branch ไหนของ `if` จะถูกเลือก และ expression สุดท้ายของ block คืออะไร`
`ถ้ามี `;` ต่อท้าย `if` ทั้งก้อน ลองคิดว่าค่าของ block ด้านนอกจะยังเป็นตัวเลขอยู่หรือไม่`

**Solution**
`result = 12 และ if เป็น expression ส่วนถ้าเติม ; หลัง if block ด้านนอกจะได้ () แทน`

**Explanation**

`x = 4` ทำให้ `y = 6` และเงื่อนไข `y > 5` เป็นจริง จึงได้ค่าจาก branch แรกคือ `y * 2 = 12`

เพราะ `if` เป็น expression สุดท้ายของ block และไม่มี `;` ต่อท้าย ค่า `12` จึงถูกใช้เป็นค่าของ block และเก็บใน `result`

ถ้าเติม `;` หลัง `if` ค่าที่ได้จะถูกทิ้ง และ block จะมีค่าเป็น `()` แทน

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

Syntax คือกฎที่กำหนดว่าโครงสร้างของโปรแกรมต้องเขียนอย่างไรจึงจะถูกต้องตามภาษานั้น

ใน Rust, Expression คือโครงสร้างที่สามารถประเมินผลแล้วได้ค่า (Value) ส่วน Statement คือคำสั่งที่ใช้ดำเนินการบางอย่าง โดย Rust ใช้ `{ }` เพื่อกำหนด Block และใช้ `;` เพื่อจบ Statement ในหลายกรณี

ตัวอย่าง:
```rust
let x = 5 + 3;
```
`10 + 5` เป็น `Expression` เพราะให้ค่า 15
`let x = ...;` เป็น `Statement`

Rust มีลักษณะเด่นคือ Expression สามารถอยู่ในตำแหน่งที่ต้องการค่าได้ เช่น if สามารถใช้เป็น Expression ได้

```rust
let result = if x > 0 {
    1
} else {
    -1
};
```
**ดังนั้นในมุมมอง PPL Syntax ของ Rust แสดงให้เห็นว่า ภาษาออกแบบ Grammar ให้ Control Flow บางชนิดสามารถทำหน้าที่เป็น Expression ได้**

### 9.2 Semantics

Semantics คือความหมายหรือพฤติกรรมของโครงสร้างภาษาเมื่อถูกประเมินหรือทำงาน
สำหรับ Rust:  
`Expression` มีหน้าที่ประเมินผลและสร้าง Value  
`Statement` มีหน้าที่ดำเนินการหรือเปลี่ยนแปลงสถานะของโปรแกรม

ตัวอย่าง:
```rust
let x = {
    let a = 10;
    a + 5
};
```

การทำงานคือ

1. สร้าง `a` และกำหนดค่า `10 `   
2. ประเมิน `a + 5`  
3. ได้ค่า `15`  
4. Block คืนค่า `15`  
5. `x` จึงมีค่า `15`

อีกจุดสำคัญคือ `;` มีผลต่อ Semantics
```rust
{
    10 + 5
}
```
Block นี้คืนค่า `15`

แต่
```rust
{
    10 + 5;
}
```
Expression ถูกเปลี่ยนให้เป็น Statement และค่าของ Block จะเป็น `()` หรือ **Unit Type**

ดังนั้น `;` ใน Rust ไม่ได้มีหน้าที่เพียง "จบคำสั่ง" แต่สามารถมีผลต่อ **ความหมายของ Expression และค่าที่ Expression คืนออกมา**

### 9.3 Type System

Expression ใน Rust ทุกตัวจะมี Type และ Compiler จะตรวจสอบ Type ตั้งแต่ Compile Time  
ตัวอย่าง:
```rust
let x = 10 + 20;
```
Expression `10 + 20` มี Type เป็นจำนวนเต็ม เช่น `int` ตามบริบทของการอนุมาน Type  

Rust ยังตรวจสอบว่า Expression มี Type ที่สอดคล้องกันหรือไม่
```rust
let result = if true {
    10
} else {
    20
};
```
ถูกต้อง เพราะทั้งสอง Branch ให้ค่า `int`

```rust
let result = if true {
    10
} else {
    "Hello"
};
```
ไม่ถูกต้อง เพราะ Branch หนึ่งให้ `int` และอีก Branch ให้ `string `

ดังนั้น Expression & Statements **มีความสัมพันธ์กับ Static Type System, Type Checking และ Type Inference ของ Rust โดย Compiler สามารถตรวจพบข้อผิดพลาดก่อนโปรแกรมทำงาน**


### 9.4 Memory / Resource Management

Expression และ Statement ใน Rust ทำงานร่วมกับระบบ **Ownership, Borrowing และ Lifetime** ซึ่งเป็นหัวใจสำคัญในการจัดการ Memory ของภาษา  

ตัวอย่าง:
```rust
let s1 = String::from("Hello");
let s2 = s1;
```
เมื่อ `s1` ถูกกำหนดให้ `s2` จะเกิด Move ทำให้ Ownership ของข้อมูลย้ายไปยัง `s2 ` 

**ดังนั้น:**
```rust
// println!("{}", s1);
```
จะเกิด Compile Error เพราะ `s1` ไม่ได้เป็นเจ้าของข้อมูลแล้ว  

Rust ยังสามารถใช้ Borrowing:
```rust
let s = String::from("Hello");
let len = calculate_length(&s);
```
`&s` คือการยืมข้อมูลโดยไม่ย้าย **Ownership**

ในมุมมอง PPL สิ่งนี้แสดงให้เห็นว่า Semantics ของการกำหนดค่าและการส่งค่าให้ Expression/Function มีความเกี่ยวข้องกับ Ownership และ Resource Management  

ข้อดีคือ Rust สามารถตรวจสอบปัญหา Memory จำนวนมากใน Compile Time โดยไม่ต้องใช้ Garbage Collector

### 9.5 Abstraction / Other PPL Concepts

Expression & Statements เชื่อมโยงกับแนวคิด PPL หลายด้าน  

#### 1. Scope 
ตัวแปรที่ประกาศภายใน Block จะมี Scope อยู่ภายใน Block นั้น
```rust
{
    let x = 10;
    println!("{}", x);
}

// x ไม่สามารถใช้ตรงนี้ได้
```
เมื่อออกจาก Scope ตัวแปรจะหมดขอบเขต และถ้าเป็น Resource ที่มี Ownership ก็สามารถถูกทำลายตามกฎของ Rust ได้  

#### 2. Binding  
การประกาศ:
```rust
let x = 10;
```
เป็นการสร้าง Binding ระหว่างชื่อ `x` กับ Value `10 ` 

Rust มีคุณสมบัติที่ Binding เป็น **Immutable โดย Default**  
```rust
let x = 10;
// x = 20; // Error
```
หากต้องการให้เปลี่ยนค่าได้:  

```rust
let mut x = 10;
x = 20;
```
#### 3. Abstraction  
Function สามารถใช้ซ่อนรายละเอียดการทำงานและรับ Expression เป็น Input/Output
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
`a + b` เป็น Expression ที่สร้างผลลัพธ์ให้ Function  

#### 4. Paradigm  
Rust เป็น **Multi-paradigm Language** และมีลักษณะ **Expression-oriented**

สามารถเขียนได้ทั้งรูปแบบ Imperative และ Functional-style  

ตัวอย่าง:
```rust
let result = if x > 0 {
    x * 2
} else {
    0
};
```
แทนที่จะต้องสร้างตัวแปรแล้วกำหนดค่าภายในแต่ละ Branch

### 9.6 Why Rust?
Rust ออกแบบ Expression & Statements ให้ทำงานร่วมกับ **Static Type System** และ **Ownership System** เพื่อให้ได้ทั้ง **Safety, Reliability และ Performance**  

**Safety**  
Compiler ตรวจสอบ **Type, Ownership และ Borrowing** ทำให้สามารถตรวจพบปัญหาหลายอย่างก่อน Runtime  

**Reliability**  
การบังคับใช้ Type และ Ownership ทำให้พฤติกรรมของโปรแกรมมีความชัดเจนและลดข้อผิดพลาดจากการจัดการข้อมูล  

**Memory Safety**  
Ownership และ Borrowing ช่วยป้องกันปัญหา เช่น  
- Use-after-free  
- Double-free  
- Dangling Reference บางรูปแบบ  
- Data Race บางประเภท

**Performance**
Rust ไม่มี Garbage Collector จึงสามารถควบคุม Resource ได้อย่างมีประสิทธิภาพ และมี Runtime Overhead ต่ำ เหมาะกับงานที่ต้องการ Performance สูง  

**Expression-oriented Design**
การที่ `if`, `match` และ `Block` สามารถคืนค่าได้ ทำให้เขียน Logic ได้กระชับและสามารถนำผลลัพธ์ไปประกอบกับ Expression อื่นได้ทันที

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

## Rust Vs Python
| Aspect | Rust | Python |
|---|---|---|
| Syntax | `ใช้ {} สำหรับ Block และ ; ใช้แยก Statement` | `ใช้ indentation เพื่อกำหนด Block` |
| Semantics / Behavior | `if, loop และ Block สามารถเป็น Expression และคืนค่าได้` | `if และ loop ใช้ในลักษณะ Statement เป็นหลัก` |
| Type System | `Static Type System` | `Dynamic Type System` |
| Memory Management | `ใช้ Ownership และ Borrowing` | `จัดการ Memory อัตโนมัติ` |
| Safety | `Compiler ตรวจสอบ Type และกฎ Ownership/Borrowing` | `ตรวจสอบ Type หลัก ๆ ขณะ Runtime` |

### Rust Example

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
Output:
```rust
B
```
จุดสำคัญคือ `if` สามารถเป็น Expression และคืนค่า `"B"` ให้กับตัวแปร `grade` ได้

### `[Python]` Example

```python
score = 75

if score >= 80:
    grade = "A"
elif score >= 70:
    grade = "B"
else:
    grade = "C"

print(grade)
```
Output:
```python
B
```
ใน Python ต้องกำหนดค่าให้ `grade` ภายในแต่ละ branch ของ `if` ขณะที่ Rust สามารถใช้ `if` เป็น Expression แล้วกำหนดผลลัพธ์ให้ `grade` โดยตรง

## Rust Vs Java
| Aspect | Rust | Java |
|---|---|---|
| Syntax | `ใช้ {} สำหรับ Block และ ; สำหรับสิ้นสุด Statement โดย if, match และ Block สามารถเป็น Expression ได้` | `ใช้ {} สำหรับ Block และ ; สำหรับสิ้นสุด Statement โดย if แบบปกติเป็น Statement` |
| Semantics / Behavior | `Expression สามารถประเมินผลและคืนค่าได้ เช่น if สามารถคืนค่าให้ตัวแปรโดยตรง` | `if แบบปกติใช้ควบคุมการทำงาน และไม่คืนค่าโดยตรง แต่สามารถใช้ Ternary Operator ?: เพื่อสร้างค่าได้` |
| Type System | `Static Type System มี Type Checking และ Type Inference` | `Static Type System มี Type Checking และ Type Inference` |
| Memory Management | `ใช้ Ownership, Borrowing และ Lifetime` | `ใช้ Garbage Collector (GC) จัดการ Memory อัตโนมัติ` |
| Safety | `Compiler ตรวจสอบ Type, Ownership และ Borrowing ช่วยป้องกัน Memory Error หลายประเภทตั้งแต่ Compile Time` | `มี Memory Safety จาก Garbage Collector และไม่มี Pointer ให้จัดการโดยตรง แต่ข้อผิดพลาดบางอย่างเกิดขึ้นได้ตอน Runtime` |

### Rust Example

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
Output:
```rust
B
```
จุดสำคัญคือ `if` สามารถเป็น Expression และคืนค่า `"B"` ให้กับตัวแปร `grade` ได้

### `[Java]` Example

```java
public class Main {
    public static void main(String[] args) {
        int score = 75;
        String grade;

        if (score >= 80) {
            grade = "A";
        } else if (score >= 70) {
            grade = "B";
        } else {
            grade = "C";
        }

        System.out.println(grade);
    }
}
```
Output:
```java
B
```
ใน Java `if` แบบปกติเป็น Statement จึงต้องกำหนดค่าให้ `grade` ภายในแต่ละ Branch

---

## Rust Vs C
| Aspect | Rust | C |
|---|---|---|
| Syntax | `ใช้ {} สำหรับ Block และ ; สำหรับสิ้นสุด Statement โดย if, match และ Block สามารถเป็น Expression ได้` | `ใช้ {} สำหรับ Block และ ; สำหรับสิ้นสุด Statement โดย if และ switch เป็น Statement` |
| Semantics / Behavior | `Expression สามารถประเมินผลและคืนค่าได้ เช่น if สามารถคืนค่าให้ตัวแปรโดยตรง` | `if ใช้ควบคุมลำดับการทำงาน และไม่สามารถคืนค่าโดยตรง ต้องกำหนดค่าภายในแต่ละ Branch` |
| Type System | `Static Type System มี Type Checking และ Type Inference ที่เข้มงวด` | `Static Type System แต่มี Implicit Conversion และการแปลง Type ที่ยืดหยุ่นกว่า` |
| Memory Management | `ใช้ Ownership, Borrowing และ Lifetime เพื่อควบคุมการใช้ Memory` | `Programmer จัดการ Memory เอง เช่น malloc() และ free()` |
| Safety | `Compiler ตรวจสอบ Type, Ownership และ Borrowing ช่วยป้องกัน Memory Error หลายประเภทตั้งแต่ Compile Time` | `Programmer ต้องรับผิดชอบ Memory Safety เอง จึงมีโอกาสเกิด Memory Leak, Dangling Pointer หรือ Use-after-free` |

### Rust Example

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
Output:
```rust
B
```
จุดสำคัญคือ `if` สามารถเป็น Expression และคืนค่า `"B"` ให้กับตัวแปร `grade` ได้

### `[C]` Example

```c
#include <stdio.h>

int main() {
    int score = 75;
    char *grade;

    if (score >= 80) {
        grade = "A";
    } else if (score >= 70) {
        grade = "B";
    } else {
        grade = "C";
    }

    printf("%s\n", grade);

    return 0;
}
```
Output:
```c
B
```
ใน C `if` เป็น Statement จึงไม่ได้คืนค่าโดยตรง แต่ต้องกำหนดค่าให้ `grade` ภายในแต่ละ Branch

### Analysis  

Rust, Python, Java และ C มีแนวคิดเกี่ยวกับ Expression & Statements ที่แตกต่างกัน เนื่องจากถูกออกแบบโดยมีเป้าหมายของภาษาไม่เหมือนกัน
- **Rust** เน้น Safety และ Performance โดยออกแบบให้ `if`, `match` และ Block สามารถเป็น Expression และคืนค่าได้โดยตรง เช่น `let x = if ... { ... } else { ... };` นอกจากนี้ยังใช้ Ownership และ Borrowing เพื่อให้ Compiler ตรวจสอบการจัดการ Memory ตั้งแต่ Compile Time
  
- **Python** เน้น ความอ่านง่ายและความสะดวกในการเขียนโปรแกรม จึงใช้ Indentation กำหนด Block และ `if` แบบปกติเป็น Statement หากต้องการ Expression ที่เลือกค่าจะใช้ Conditional Expression เช่น `x = "A" if score >= 80 else "B"` Python ใช้ Dynamic Typing และจัดการ Memory อัตโนมัติ

- **Java** เน้น Object-Oriented Programming, Portability และความปลอดภัยในการจัดการ Memory โดย `if` แบบปกติเป็น Statement แต่มี Ternary Operator และ `switch expression` ที่สามารถคืนค่าได้ Java ใช้ Static Type System และ Garbage Collector ในการจัดการ Memory
  
- **C** เน้น Performance, การควบคุม Hardware และความใกล้ชิดกับระบบ จึงออกแบบ `if` และ `switch` เป็น Statement และให้ Programmer ควบคุม Memory ได้โดยตรงผ่าน Pointer, `malloc()` และ `free()` ข้อแลกเปลี่ยนคือ Programmer ต้องรับผิดชอบ Memory Safety เอง

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

Rust vs Other Language + PPL Analysis

**Member 4**

Exercises + Common Mistakes + Challenge

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

### Rust
1. [The Rust Programming Language — Statements and Expressions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions)
2. [Rust By Example — Expressions](https://doc.rust-lang.org/rust-by-example/expression.html)
3. [The Rust Reference — Statements and Expressions](https://doc.rust-lang.org/reference/statements-and-expressions.html)
4. [The Rust Reference — Statements](https://doc.rust-lang.org/reference/statements.html)

### Python
5. [Python Language Reference — Expressions](https://docs.python.org/3/reference/expressions.html)
6. [Python Language Reference — Simple Statements](https://docs.python.org/3/reference/simple_stmts.html)
7. [Python Language Reference — Compound Statements](https://docs.python.org/3/reference/compound_stmts.html)

### Java
8. [Java Language Specification — Chapter 14: Blocks, Statements, and Patterns](https://docs.oracle.com/javase/specs/jls/se25/html/jls-14.html)
9. [Java Language Specification — Chapter 15: Expressions](https://docs.oracle.com/javase/specs/jls/se25/html/jls-15.html)
10. [Oracle Java Tutorial — Expressions, Statements, and Blocks](https://docs.oracle.com/javase/tutorial/java/nutsandbolts/expressions.html)

### C
- [ISO/IEC JTC1/SC22/WG14 — Official C Working Group](https://www.open-std.org/jtc1/sc22/wg14/)
- [WG14 — C Standard Project and Drafts](https://www.open-std.org/jtc1/sc22/wg14/www/projects.html)

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `ค้นหาข้อมูล อธิบาย และสรุปหัวข้อ` | `ตรวจสอบกับเอกสารทางการของ Rust` |

### Declaration

- [X] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [X] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [X] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [X] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

ใช้ ChatGPT เพื่อช่วยค้นหา อธิบาย เปรียบเทียบ และสรุปเนื้อหาเกี่ยวกับ Expressions and Statements ใน Rust รวมถึงช่วยออกแบบตัวอย่างและแบบฝึกหัด โดยสมาชิกตรวจสอบข้อมูลกับเอกสารทางการของ Rust เพื่อยืนยันความถูกต้องก่อนนำมาใช้

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `20` | `3` | `0` | `Concept + Short Code Illustration` |
| Member 2 | `0` | `3` | `1` | `0` | `Detailed Code + Live Demo` |
| Member 3 | `0` | `3` | `1` | `0` | `Rust vs Other Language + PPL Analysis` |
| Member 4 | `0` | `32` | `2` | `0` | `Exercises + Common Mistakes + Challenge` |

### Teamwork Reflection

**How did your team collaborate?**

สมาชิกในกลุ่มร่วมกันกำหนดขอบเขตหัวข้อ จากนั้นแบ่งหน้าที่ให้แต่ละคนศึกษาภาษา Rust และเนื้อหาที่รับผิดชอบ ก่อนนำความรู้มารวมกันเป็นงานของกลุ่ม หากมีปัญหาหรือข้อสงสัย จะนำมาปรึกษาเพื่อช่วยกันตรวจสอบและแก้ไข

**Problems encountered**

บางข้อมูลจาก AI จำเป็นต้องตรวจสอบเพิ่มเติมกับเอกสาร และตัวอย่างโค้ดบางส่วนต้องทดลองรันเพื่อยืนยันผลลัพธ์

**How did you solve them?**

ตรวจสอบข้อมูลกับเอกสารทางการและทดลองรันโค้ดเพื่อยืนยันความถูกต้อง

---

## 15. Final Checklist

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
- [X] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [X] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [X] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[[GitHub repository URL]](https://github.com/670710127/rust-tutorial-2569/tree/main)`

**Chapter Path:** `05-expressions-statements/`

**Final PR:** `#32`

**Submitted by:** `Group 05`

**Date:** `2026-10-3`
