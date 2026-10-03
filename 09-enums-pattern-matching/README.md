# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 09
> **Topic No.:** 09
> **Topic Name:** Enums & Pattern Matching
> **ประเด็นหลักที่ควรครอบคลุม:** enum, variants, pattern matching, match, if let

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายวุฒิชัย หลักเพชร | 670710142 | @670710142 | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวศุภิสรา สอนดี | 670710143 | @670710143 | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายจารุเดช พินิตศักดา | 670710144 | @670710144 | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นางสาวชัชชา เภาเสน | 670710145 | @670710145 | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |


---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---


## 3. Introduction

### Enums & Pattern Matching  

เป็นหนึ่งในหัวข้อสำคัญของ Rust ที่ครอบคลุมแนวคิดเรื่องการกำหนดชนิดข้อมูลที่มีค่าได้หลายแบบ (`enum`) ควบคู่กับกลไกการแยกกรณีและดึงข้อมูลออกมาใช้งาน (pattern matching) 
ผ่าน `match` และ `if let`

### Enum คืออะไร

    `enum` คือ ชนิดข้อมูลที่กำหนดว่าตัวแปรหนึ่งตัวสามารถมีค่าที่เป็นไปได้กี่แบบ `(variant)` โดยตัวแปรนั้นจะเป็นได้เพียง 1 variant เท่านั้น 
    จุดเด่นของ Rust คือแต่ละ variant สามารถแนบข้อมูลหลายชนิดไว้ด้วยกันได้ ทำให้ enum ใช้แทนได้ทั้ง tag ง่าย ๆ และ container ของข้อมูลที่ซับซ้อน 

### ความสำคัญของหัวข้อ

    `Enum & Pattern Matching` เป็นหัวใจของการออกแบบภาษา Rust เพราะเป็นกลไกหลักที่ช่วยให้ภาษามีความปลอดภัยด้าน type และ memory ตั้งแต่ระดับ compile time 
    โดยเมื่อใช้ `match` แล้ว คอมไพเลอร์จะบังคับให้ครอบคลุมทุก variant ที่เป็นไปได้ (exhaustiveness checking) ทำให้ข้อผิดพลาดที่ปกติจะเกิดตอน runtime 
    ถูกจับได้ตั้งแต่ก่อนโปรแกรมจะถูกรัน ต่อยอดไปสู่ enum มาตรฐานที่ใช้บ่อยที่สุดอย่าง `Option<T>` (แทนค่าไม่มีอยู่) และ `Result<T, E>` (แทนความสำเร็จ/ล้มเหลว) 
    ซึ่งเป็นพื้นฐานของการเขียนโปรแกรมภาษา Rust แทบทุกส่วน

### ปัญหาที่สามารถแก้ได้

    1. **Null pointer / unhandled null** — จัดการกรณีต่าง ๆ ให้ชัดเจนตั้งแต่ตอนเขียนโค้ด โดยการใช้ `Option<T>` เพื่อจัดการกับสถานการณ์ที่ค่าอาจจะไม่อยู่จริงหรือไม่มีค่า ช่วยป้องกัน runtime error
    2. **Unhandled case ** — ภาษาที่ใช้ `switch`/`if-else` ส่วนใหญ่ไม่เขียนบังคับให้ครอบคลุมทุกกรณีส่งผลให้ทำงานผิดพลาด แต่ `match` ของ Rust ตรวจสอบว่าครอบคลุมทุกกรณีแล้วหรือยัง ( exhaustiveness checking) ที่ compile time 
    3. **ข้อมูลสื่อความหมายไม่ชัดเจน** — การใช้ตัวเลขแทนความหมาย (เช่น `-1` แทน "ไม่พบข้อมูล") ทำให้โค้ดอ่านยากและเสี่ยงต่อการตีความผิด enum ที่แนบข้อมูลได้ช่วยให้แต่ละสถานะสื่อความหมายชัดเจนในตัวเอง (self-documenting) 

      ทั้งหมดในหัวข้อนี้จึงเป็นทั้งเรื่องของการออกแบบภาษา Rust แก้ไขข้อผิดพลาดจาก runtime มาเป็น compile time ให้ได้มากที่สุดส่งผลให้โปรแกรมมีความปลอดภัยทำงานตรงตามเป้าหมายมากขึ้น


---

## 4. Key Concepts

### 4.1 การประกาศ Enum พื้นฐาน (Defining Enums)

**คำอธิบาย**

`Enum` ใน Rust ใช้สร้าง Custom Type ที่ค่าของมันสามารถเป็นไปได้เพียงรูปแบบใดรูปแบบหนึ่งจากที่กำหนดไว้ (Variants) จุดเด่นคือ แต่ละ Variant ไม่จำเป็นต้องหน้าตาเหมือนกัน
โครงสร้างทั้ง 3 แบบประกอบด้วย
>1. Unit Variant: แบบไม่มีข้อมูลภายใน 
>2. Tuple Variant: แบบเก็บข้อมูลเรียงกันตามลำดับ (เหมือน Tuple)
>3. Struct Variant: แบบเก็บข้อมูลเป็นฟิลด์กำหนดชัดเจน (เหมือน Struct)

**ตัวอย่าง**

```rust
// การสร้าง Enum ที่เก็บข้อมูลได้หลายรูปแบบ
enum Message {
    Quit,                       // Unit-like: ไม่มีข้อมูลข้างใน
    Move { x: i32, y: i32 },    // Struct-like: เก็บพิกัด x, y
    Write(String),              // Tuple-like: เก็บข้อความ
}

fn main() {
    let msg1 = Message::Write(String::from("Hello PPL"));
    let msg2 = Message::Move { x: 10, y: 20 };
}
```

**Explanation**

`จากโค้ด Message ที่สามารถบรรจุข้อมูลที่แตกต่างกันโดยสิ้นเชิงได้ ทำให้เราสามารถจัดกลุ่มข้อมูลที่เกี่ยวข้องกันไว้ที่เดียวกันได้อย่างเป็นระเบียบ`

---

### 4.2 Pattern Matching (match)

`match คือคำสั่งควบคุมทิศทางโปรแกรม (Control Flow) คล้ายกับ switch/case แต่หน้าที่เฉพาะตัวในทำการดึงข้อมูลที่ซ่อนอยู่ใน Enum ออกมาใช้งานได้ และมีกฎเหล็กคือ Exhaustiveness (เขียนครอบคลุมทุกกรณีที่ Enum เป็นไปได้ หากเขียนไม่ครบ Compiler จะแจ้ง Error )`

```rust
// Rust code
enum Coin {
    Penny,
    Quarter(String), // เก็บชื่อรัฐของเหรียญ Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        // Pattern Matching ดึงค่า String ออกมาใส่ตัวแปร state
        Coin::Quarter(state) => {
            println!("State quarter from {}!", state);
            25
        }
    }
}
```

**Explanation**

`match จะเปรียบเทียบค่า coin กับแต่ละ Pattern หากตรงกับ Quarter จะดึงค่า String ที่อยู่ข้างในออกมาเก็บไว้ในตัวแปร state เพื่อนำไปแสดงผลหรือประมวลผลต่อได้ทันที`


---

### 4.3 การแทนที่ Null ด้วย Option<T>

`Rust เป็นภาษาที่ไม่มีค่า Null (Null-safety) เพื่อป้องกันปัญหา Runtime Error  Rust ใช้ Enum พิเศษที่มีอยู่ใน Standard Library ชื่อว่า Option<T> มาใช้แทน เพื่อสื่อถึงแนวคิดที่ว่า "อาจจะมีค่า (Some)" หรือ "ไม่มีค่า (None)"`

```rust
// Rust code
// Option Enum ถูก Build-in มาในภาษา หน้าตาเป็นแบบนี้:
// enum Option<T> { Some(T), None, }

fn main() {
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    match some_number {
        Some(val) => println!("มีตัวเลขคือ: {}", val),
        None => println!("ไม่มีข้อมูล (คล้าย Null แต่ปลอดภัย)"),
    }
}
```
**Explanation**

`เนื่องจาก Option<T> เป็น Enum การจะเอาค่า 5 ออกมาใช้บวกเลขตรง ๆ ไม่ได้ จะต้องใช้ `match` ดึงค่าออกามาแทนและจัดการกรณีที่ไม่มีค่า ทำให้ไม่มีโอกาสเกิด `Null Pointer Exception`


---

### 4.4 การจัดการ Error ด้วย Result<T, E>

`Rust ไม่มีระบบ try/catch สำหรับ Exception Handling แบบภาษา OOP ทั่วไป แต่ใช้ Enum ที่ชื่อว่า Result<T, E> สำหรับฟังก์ชันที่อาจเกิดข้อผิดพลาดได้ โดยจะคืนค่า Ok(ข้อมูล) หากสำเร็จ และคืนค่า Err(ข้อผิดพลาด) หากล้มเหลว`

```rust
// Result Enum ถูก Build-in มาในภาษา หน้าตาเป็นแบบนี้:
// enum Result<T, E> { Ok(T), Err(E), }

fn main() {
    // การแปลง String เป็นตัวเลข อาจเกิด Error ได้
    let parse_result: Result<i32, _> = "100a".parse(); 

    match parse_result {
        Ok(number) => println!("แปลงสำเร็จ ได้เลข: {}", number),
        Err(e) => println!("แปลงไม่สำเร็จ เกิดข้อผิดพลาด: {}", e),
    }
}
```
**Explanation**

`การทำงานนี้ช่วยจัดการข้อผิดพลาดที่เกิดจากการแปลงข้อความเป็นจำนวนเต็ม โดยออกแบบให้มีการตรวจสอบผลลัพธ์ก่อนว่าแปลงสำเร็จหรือเกิด error ช่วยให้โปรแกรมยังคงทำงานต่อไปได้อย่างปลอดภัย แทนที่จะหยุดทำงานกระทันหัน`

---

### 4.5 Concise Control Flow ด้วย if let

`ในบางครั้ง Enum มีหลายกรณี แต่เราสนใจแค่ "กรณีเดียว" การเขียน match สำหรับทุกทางอาจทำให้โค้ดยาวเกินไป Rust จึงให้ Syntax Sugar ที่ชื่อว่า if let มาเพื่อลดรูปการทำ Pattern Matching `

```rust
// Rust code
fn main() {
    let config_max = Some(3u8);

    // แบบที่ 1: ใช้ match (ต้องเขียน _ => () เพื่อดักกรณีที่เหลือ)
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // แบบที่ 2: ใช้ if let (กระชับกว่า อ่านง่ายกว่า)
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```
**Explanation**

`if let เป็นเพียงตัวย่อของ match ที่มีแค่ variant เดียว ช่วยลดความซ้ำซ้อนของโค้ด (Boilerplate) แต่ยังคงคุณสมบัติความปลอดภัยในการแกะดึงค่า Some เช่นเดิม`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `enum Name { ... }` | `การประกาศสร้าง Custom Type ที่มีได้หลายรูปแบบ (Variants) โดยแต่ละแบบสามารถเก็บข้อมูลต่างชนิดกันได้` | `enum Status { Ok, Err(String) }` |
| `match value { ... }` | `คำสั่งสำหรับตรวจสอบและแยกแยะ Enum คล้าย switch/case แต่สามารถดึงข้อมูลที่อยู่ข้างในออกมาได้` | `match status { Status::Ok => ... }` |
| `_ => ...` | `Catch-all Pattern (Wildcard): ใช้ใน match เพื่อจัดการ "กรณีที่เหลือทั้งหมด" (เหมือน default ใน switch)` | `_ => println!("Ignore others"),` |
| `if let Pattern = value` | `การทำ Pattern Matching แบบสั้น (Syntax Sugar) ใช้เมื่อเราต้องการดึงข้อมูลและจัดการแค่ เงื่อนไขเดียว` | `if let Some(x) = option_val { ... }` |
| `Option<T>`  | `Enum มาตรฐานของ Rust Option ใช้แทนค่า Null`  | `let data: Option<i32> = Some(5);` |
| `Result<T, E>` | ` Result ใช้สำหรับจัดการ Error` | `Ok(num) => println!("Success! ", num), `|

### Important Rules

1. `Exhaustive Matching (ต้องเช็คให้ครบทุกกรณี): กฎเหล็กของภาษา Rust คือคำสั่ง match จะต้องครอบคลุม ทุกความเป็นไปได้ ของ Enum นั้นเสมอ หากเขียนไม่ครบและไม่ใช้ ( '_' ) ในการกำหนดเงื่อนไข Default Compiler จะแจ้ง Error ทำให้ลดข้อผิดพลาดจากการลืมเช็คเงื่อนไขใดเงื่อนไขหนึ่ง`
2. `No Direct Data Access (ห้ามเข้าถึงข้อมูลข้างในตรงๆ): การเข้าถึงข้อมูลที่บรรจุอยู่ใน Enum ต้อง ใช้ Pattern Matching (ผ่าน match หรือ if let) เพื่อทำหน้าที่ Extract และดึงข้อมูลออกมาสู่ Scope ปัจจุบัน`
3. `Top-to-Bottom Evaluation (ประเมินจากบนลงล่าง): Pattern ใน match จะถูกตรวจสอบจากบนลงล่างทีละบรรทัด เมื่อเจอ Pattern แรกที่ตรงกัน โปรแกรมจะทำงานใน Block นั้นแล้วออกจาก match ทันที ดังนั้น การใช้  ( '_' ) หรือ  Catch-all จะอยู่ด้านล่างสุดเสมอ`


---

## 6. Runnable Code Examples

### Example 1 — `ระบบแจ้งเตือนสถานะพัสดุ (Package Status Tracker)`

**Purpose:** ต้องการสาธิตการสร้าง Enum ที่มี variant ครบทั้ง 3 ประเภทด้วยกันคือ (unit-like, triple-like, struct-like) และพร้อมการใช้ `match` ในการดึงข้อมูลแต่ละแบบออกมาใช้งาน (destructure) โดยให้แต่ละกรณีสร้างข้อความคืนกลับมาเป็น `String`

```rust
fn main() {
    println!("======== Package Status Tracker ========");
    let status = vec![
        PackageStatus::Ordered,
        PackageStatus::Shipped("Kerry Express".to_string()), //ต้องแปลงเพราะที่รับมาคือ &str แต่ส่งไปหา String ใข้ String::from("Kerry Express")ได้
        PackageStatus::OutForDelivery{
            courier_name: String::from("Peewara"),
            estimated_hours: 1
        },
        PackageStatus::Delivered,
    ];
    for s in &status {
        println!("{}",describe_status(s));
    }
}
enum PackageStatus{
    Ordered, // unit-like
    Shipped(String),//triple-like
    OutForDelivery{//strcut-like
        courier_name:String,
        estimated_hours :u32
    },
    Delivered,// unit-like
    
}
fn describe_status(status: &PackageStatus) -> String {
    match status{
        PackageStatus::Ordered => format!("Your order has been placed."),
        PackageStatus::Shipped(courier) => format!("Your order has been shipped via {}.",courier),
        PackageStatus::OutForDelivery{courier_name , estimated_hours } => format!("{} is delivering your package, arriving in about {} hour(s).",courier_name,estimated_hours ),
        PackageStatus::Delivered=> format!("Your package has been delivered!"),
    }

}
```

**Expected Output**
```text
====================== Package Status Tracker ======================
Your order has been placed.
Your order has been shipped via Kerry Express.
Peewara is delivering your package, arriving in about 1 hour(s).
Your package has been delivered!
============================ Thank you ==============================
```
**Explanation**

   โปรแกรมจำลองสถานะพัสดุทั้ง 4 แบบและใช้ pattern matching ผ่าน `match` แปลแต่ละสถานะเป็นข้อความ
ความพิเศษของตัวอย่างนี้ คือ ฟังก์ชัน `describe_status`  คืนค่าเป็น `String`  ซึ่งใช้ macro format!() เข้ามาช่วยสร้างข้อความเก็บไว้ในตัวแปร แทนการพิมพ์ออกมาตรง ๆ ในฟังก์ชันตัวเอง 
อีกจุดหนึ่งคือค่าที่ใส่เข้าไปใน field ประเภท `String`  ต้องแปลงจาก `&str` เป็น `String`  เป็นเพียงการยืมข้อมูลที่ต้องพึ่งพา lifetime ในขณะที่ enum ต้อง**เป็นเจ้าของ**ข้อมูลของตัวเองเพื่อให้เก็บใน `Vec` 


---

### Example 2 — `ระบบตรวจเกรด (Grade Checker)`
**Purpose:** ต้องการสาธิตการสร้าง Enum มาตรฐานโดยใช้ Option ในการช่วยตัดเกรด ร่วมกับการใช้ range pattern ช่วยแบ่งแกณฑ์คะแนน 
เพื่อทำการเช็คและป้องกันข้อมูลที่ผิดพลาดที่เกิดได้ทั้งความตั้งใจหรือไม่ก็ตาม เช่น ไม่มีนักศึกษาในระบบ รายชื่อผิด หรือ คะแนนไม่อยู่ในช่วงที่กำหนดไว้ เป็นต้น


```rust
use std::collections::HashMap;
// [1] แปลงคะแนนเป็นเกรด 
fn get_grade(score: i32) -> Option<char> {
    match score {
        80..=100 => Some('A'),
        70..=79 => Some('B'),
        60..=69 => Some('C'),
        0..=59 => Some('D'),
        _ => None, //คะแนนติดลบหรือเกินช่วง
    }
}
//[2] รายงานผลคะแนน
fn report(db: &HashMap<&str, i32>, name: &str) {
    // ชั้นที่ 1: ค้นหาคะแนนจากรายชื่อนักศึกษา
    match db.get(name) { 
        None => println!("{:<11} : Error! Not found", name), //ไม่พบชื่อ = ไม่มีเกรด (none)
        //ขั้นที่ 2 : พบรายชื่อตรวจสอบเกรดที่่ได้
        Some(&score) => { match get_grade(score) {
                Some(g) => println!("{:<11} : {} points [grade {}]", name, score, g),
                None => println!("{:<11} : Warning! {} points --> Incorrect scores", name, score),
            }
        }
    }
}
//[3] การเรียกใช้งานจริง
fn main() {
    println!("=========== Grade Checker ===========");
//รายชื่อที่เก็บไว้พร้อมคะแนนรายบุคคล
    let db = HashMap::from([
        ("Sanruethai", 85),
        ("Sakaodeuan", 50),
        ("Pancheewa", 67),
        ("Thaenrak", 150),
    ]);
 //รายชื่อที่จะทำการค้นหา
    let names = ["Sanruethai", "Sakaodeuan", "Pancheewa", "Thaenrak", "Mether"];
    for name in names {
        report(&db, name); //รายงานผล
    }
}


**Expected Output**
```text
=========== Grade Checker ===========
Sanruethai  : 85 points [grade A]
Sakaodeuan  : 50 points [grade D]
Pancheewa   : 67 points [grade C]
Thaenrak    : Warning! 150 points --> Incorrect scores
Mether      : Error! Not found
```
**Explanation**

     ระบบคำนวนคะแนนเกรดเริ่มต้นจากค้นหาว่ามีนักศึกษาคนนี้จริงไหมในระบบรายวิชา หากมีจริงให้ทำการส่ง score ของคนนั้นเข้าไป `get_grade()` เพื่อคำนวณว่าคะแนนนั้นถูกต้องและแปลงเป็นเกรดต่อไป โดยจะคืนค่าเป็น Option<char> 
     แทนการใช้ "sentinel value" (เช่น คืนตัวอักษรพิเศษแทนค่าไม่ถูกต้อง)   เพื่อบังคับให้ต้องตรวจสอบก่อนใช้งานค่าเสมอ จากนั้นผลลัพธ์จะถูกส่งต่อเข้า `report()` ซึ่ง `match` ทั้ง 2 กรณี (`Some`/`None`) และแสดงผลตามเงื่อนไขที่ได้กำหนดไว้


## 7. Common Mistakes

### Mistake 1 — `Non-exhaustive match`

**Problem**

`match ใน Rust ต้องครอบคลุมทุก variant ของ enum ถ้าเขียนไม่ครบ โปรแกรมจะ compile ไม่ผ่าน`

**Incorrect Code**

```rust
enum Payment {
    Cash,
    CreditCard,
    PromptPay,
}
fn main() {
    let payment = Payment::Cash;

    match payment {
        Payment::Cash => println!("ชำระด้วยเงินสด"),
    }
}
```

**Correct Code**

```rust
enum Payment {
    Cash,
    CreditCard,
    PromptPay,
}
fn main() {
    let payment = Payment::Cash;

    match payment {
        Payment::Cash => println!("ชำระด้วยเงินสด"),
        Payment::CreditCard => println!("ชำระด้วยบัตรเครดิต"),
        Payment::PromptPay => println!("ชำระด้วย PromptPay"),
    }
}
```

**Why?**

`Payment มี variants 3 อัน คือ Cash , CreditCard , PromptPay แต่ match จัดการเพียง Cash จึงทำให้เกิด error non-exhaustive เพราะ match ใน Rust ต้องครอบคลุมทุก variant ของ enum หรือใช้ _ (wildcard) เพื่อครอบคลุมกรณีที่ไม่ได้กำหนดไว้ให้แสดงผลออกมาเป็น "ชำระด้วยวิธีอื่น"`

---

### Mistake 2 — `สับสนเรื่อง Ownership เมื่อใช้ match`

**Problem**

`ใช้ match value แล้วข้อมูลอย่าง String อาจถูกย้ายความเป็นเจ้าของไปใน match ทำให้ไม่สามารถใช้ตัวแปรนั้นซ้ำได้`

**Incorrect Code**

```rust
enum Message {
    Write(String),
}
fn main() {
    let msg = Message::Write(String::from("have a nice day")) ;
    match msg {
        Message::Write(text) => println!("{}",text),
    }
    match msg {
        Message::Write(text) => println!("{}",text),
    }
}
```

**Correct Code**

```rust
enum Message {
    Write(String),
}
fn main() {
    let msg = Message::Write(String::from("have a nice day")) ;
    match &msg {
        Message::Write(text) => println!("{}",text),
    }
    match &msg {
        Message::Write(text) => println!("{}",text),
    }
}
```

**Why?**

`match msg จะเอาค่าที่อยู่ใน msg มาใช้ใน match ถ้าข้อมูลนั้นเป็น String อาจทำให้ค่าถูกย้ายออกไปแล้วทำให้ msg ใช้ต่อไม่ได้ แต่การใช้ match &msg จะเป็นการยืมข้อมูลมาดู ทำให้ยังสามารถใช้ msg ต่อได้หลังจาก match 
เนื้อหาที่เกี่ยวข้อง : Ownership & Borrowing`

---

### Mistake 3 — `ลำดับ Pattern ใน match ผิด`

**Problem**

`การเขียน pattern ที่ครอบคลุมทุกกรณีไว้ก่อน pattern ที่เฉพาะเจาะจงกว่า ทำให้ pattern ที่อยู่ด้านหลังไม่สามารถทำงานได้`

**Incorrect Code**

```rust
enum Status {
    Success,
    Error,
    Pending,
}
fn show_status(status: Status) {
    match status {
        Status::Success => println!("สถานะสำเร็จ"),
        _ => println!("สถานะอื่น"),
        Status::Error => println!("เกิดข้อผิดพลาด"),
    }
}
fn main() {
    show_status(Status::Error);
}
```

**Correct Code**

```rust
enum Status {
    Success,
    Error,
    Pending,
}
fn show_status(status: Status) {
    match status {
        Status::Success => println!("สถานะสำเร็จ"),
        Status::Error => println!("เกิดข้อผิดพลาด"),
        _ => println!("สถานะอื่น"),
    }
}
fn main() {
    show_status(Status::Error);
}
```

**Why?**

`match จะตรวจสอบ pattern จากบนลงล่าง ใน Incorrect Code _ หมายถึงทุกกรณีที่เหลือ จึงทำให้ Status::Error ที่อยู่ด้านหลังไม่สามารถทำงานได้ ดังนั้นควรวาง pattern ที่เฉพาะเจาะจงไว้ก่อน แล้วค่อยใช้ _ สำหรับกรณีที่เหลือ`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `Calculate Shape Area`

**Problem**

`เขียน enum ชื่อ Shape มี variant 3 ตัว ได้แก่ Circle, Rectangle, Triangle จากนั้นเขียนฟังก์ชัน area ที่รับ Shape และคำนวณพื้นที่แต่ละรูปโดยใช้ match`

**Hint**

`พื้นที่วงกลม = π × r²`  
`พื้นที่สี่เหลี่ยม = กว้าง × ยาว`  
`พื้นที่สามเหลี่ยม = 0.5 × ฐาน × สูง`

**Solution**

```rust
enum Shape {
    Circle(f64),   //tuple-like
    Rectangle(f64, f64),   //tuple-like
    Triangle { base: f64, height: f64 },   //struct-like
}
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,   //std::f64::consts::PI หมายถึง ค่าคงที่ของ π (ประมาณ 3.14)
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}
fn main() {
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle { base: 5.0, height: 6.0 };
    println!("Circle = {:.2}", area(&circle));   //แสดงผลลัพธ์เป็นทศนิยม 2 ตำแหน่ง
    println!("Rectangle = {:.2}", area(&rectangle));
    println!("Triangle = {:.2}", area(&triangle));
}
```

**Explanation**

`Shape ใช้เก็บข้อมูลของรูปแต่ละแบบ ฟังก์ชัน area ใช้ match เพื่อเช็คว่าเป็นรูปอะไรแล้วคำนวณหาพื้นที่ตามข้อมูลที่เก็บไว้ในแต่ละ variant โดยฟังก์ชันรับ &Shape เพื่อยืมค่า ทำให้สามารถนำ Shape ไปใช้ต่อได้ variant ที่ใช้วงเล็บ() จะเก็บข้อมูลตามลำดับ ส่วน variant ที่ใช้ปีกกา{} จะเก็บข้อมูลโดยระบุชื่อ field`

---

### Exercise 2 — `Stock Checker`

**Problem**

`เขียน enum ชื่อ Product มี 2 variants ได้แก่ Snack(ใช้ Struct-variant) และ Drink(ใช้ Tuple-variant) ทั้งคู่เก็บชื่อ,ราคา,จำนวนคงเหลือ จากนั้นเขียนฟังก์ชัน check_availbility ที่คืนราคาถ้าสินค้ายังมีสต็อก และคืน None ถ้าสินค้าหมด แล้วแสดงผลลัพธ์ทั้งหมดโดยใช้ if let`

**Hint**

`ให้ price เป็นทศนิยม(f64) และ stock เป็นจำนวนเต็ม(u32)`  
`Tuple Variant: แบบเก็บข้อมูลเรียงตามลำดับ → ( )`  
`Struct Variant: แบบเก็บข้อมูลเป็นฟิลด์ระบุชื่อชัดเจน → { }`

**Solution**

```rust
enum Product {
    Snack{name:String ,price: f64 ,stock: u32},   //struct-like
    Drink(String ,f64 ,u32),   //tuple-like
}
fn check_availability(product: &Product) -> Option<f64> {
    match product {
        Product::Snack{price , stock , ..} => {   //ดึง price และ stock จาก field โดยใช้ชื่อ และไม่สนใจค่าที่เหลือ
            if *stock > 0 {
                Some(*price)
            }
            else {
                None
            }
        }
        Product::Drink(_ , price , stock) => {   //ดึงค่าตามตำแหน่ง และใช้_เพื่อไม่สนใจค่าตัวแรก
            if *stock > 0 {
                Some(*price)
            }
            else {
                None
            }
        }
    }
}
fn main() {
    let items = vec![
        Product::Snack{name:String::from("Chips"), price: 20.0, stock: 5},
        Product::Snack{name:String::from("Cookie"), price: 22.5, stock: 0},
        Product::Drink(String::from("Cocoa"), 39.0, 1),
        Product::Drink(String::from("Water"), 7.0, 4),
    ];
    for item in &items {
        let name = match item {
            Product::Snack{name, ..} => name,   //ใน struct-like ใช้ .. หมายถึงไม่สนใจค่าที่เหลือ
            Product::Drink(name ,_ ,_) => name,   //ใน tuple-like ใช้ _ หมายถึงไม่สนใจค่าที่ตำแหน่งนั้น
        };
        if let Some(price) = check_availability(item) {
            println!("{}: There are items price at {} baht", name, price);
        } else {
            println!("{}: Out of stock", name);
        }
    }
}
```

**Explanation**

`Product ใช้เก็บข้อมูลของสินค้า โดย Snack ใช้ struct-variant และ Drink ใช้ tuple-variant ทั้งสองเก็บชื่อ ราคา และจำนวนสินค้าคงเหลือ ฟังก์ชัน check_availability ใช้ match เพื่อตรวจสอบว่า Product เป็น variant อะไร แล้วทำการตรวจสอบจำนวนสินค้าใน stock และคืน Some(price) หากยังมีสินค้า หรือ None หากสินค้าหมด`

---

### Challenge — คำถามท้าทายผู้ฟัง

**Problem**

โค้ดด้านล่างนี้ **compile ผ่านหรือไม่?** ถ้าไม่ผ่าน ติดตรงไหน?

```rust
enum BookStatus {
    Available,
    Borrowed(String, u32),
    Reserved { name: String, days: u32 },
}
fn check_book(status: &BookStatus) {
    match status {
        BookStatus::Borrowed(name, days) if * days > 30 => {
            println!("{} borrowed for more than 30 days", name);
        }
        BookStatus::Borrowed(name, days) => {
            println!("{} borrowed for {} days", name, days);
        }
        BookStatus::Available => {
            println!("The book is available");
        }
    }
    if let BookStatus::Reserved { name, days } = status {
        if *days > 3 {
            println!("{} has reserved the book for more than 3 days", name);
        }
    }
}
fn main() {
    let book = BookStatus::Reserved {
        name: String::from("Alice"),
        days: 5,
    };
    check_book(&book);
}
```

**Solution**

`❌ compileไม่ผ่าน -> error[E0004]: non-exhaustive patterns: BookStatus::Reserved(_) not covered`

**Explanation**

`จาก Code จะเห็นว่า match มี 3 แขน แต่ 2 แขนแรกเป็น Borrowed ทั้งคู่ ส่วน Reserved ยังไม่ถูกจัดการ ทำให้ match ยังไม่ครอบคลุมทุก variant ของ enum`

**Correct Code**

```rust
enum BookStatus {
    Available,
    Borrowed(String, u32),
    Reserved { name: String, days: u32 },
}
fn check_book(status: &BookStatus) {
    match status {
        BookStatus::Borrowed(name, days) if *days > 30 => {
            println!("{} borrowed for more than 30 days", name);
        }
        BookStatus::Borrowed(name, days) => {
            println!("{} borrowed for {} days", name, days);
        }
        BookStatus::Available => {
            println!("The book is available");
        }
        BookStatus::Reserved { .. } => {
            println!("The book is reserved");
        }
    }
    if let BookStatus::Reserved { name, days } = status {
        if *days > 3 {
            println!("{} has reserved the book for more than 3 days", name);
        }
    }
}
fn main() {
    let book = BookStatus::Reserved {
        name: String::from("Alice"),
        days: 5,
    };
    check_book(&book);
}
```

___

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`ประกาศ enum เพื่อรวมกลุ่มของข้อมูลหลายแบบไว้ด้วยกันและใช้ไวยากรณ์ match หรือ if let เพื่อทำการเช็คเงื่อนไขและดึงข้อมูลข้างในออกมาใช้งานในบรรทัดเดียวกันได้เลย`

### 9.2 Semantics

`Enum ของ Rust แต่ละตัว สามารถผูกข้อมูลต่างชนิดกันไว้ข้างในได้ และตัว match ใน Rust มีพฤติกรรมเป็น Expression คือสามารถประมวลผลแล้วคืนค่า ออกมาใส่ตัวแปรต่อได้ทันที`

### 9.3 Type System

`enum ถูกจัดเป็น sum type คือชนิดข้อมูลที่เลือกได้แค่อย่างเดียวจากตัวเลือกที่มี ต่างจาก struct ที่เป็น product type คือ รวมทุกอย่างพร้อมกัน เมื่อนำ enum มาผสมกับ generics (การใส่ชนิดข้อมูลแบบยืดหยุ่นได้ เช่นใน Option<T> และ Result<T, E>) ตัวคอมไพเลอร์จะบังคับให้เราต้องจัดการทั้งกรณี "สำเร็จ" และ "ผิดพลาด" ตั้งแต่ตอนเขียนโค้ด นอกจากนี้ Rust นำ Option<T> มาแทนที่การใช้ null ทำให้ไม่มีปัญหา error ที่มาจากการเรียกใช้ค่าที่ไม่มีอยู่จริง `

### 9.4 Memory / Resource Management

`เบื้องหลังการทำงาน Enum จะถูกจัดเก็บแบบ Tagged Union ใน Memory โดยจองพื้นที่เท่ากับขนาดของตัวที่ใหญ่ที่สุด บวกกับ Tag เล็กๆ ไว้ระบุประเภท`

### 9.5 Abstraction / Other PPL Concepts

`Enum และ pattern matching คือแนวคิด Algebraic Data Type (ADT) ซึ่งดึงเอาข้อดีของภาษาฝั่ง Functional เข้ามาผสมผสานกับภาษาเชิงระบบ `

### 9.6 Why Rust?

`เขียนง่ายเหมือนภาษา High-level แต่คอมไพล์ออกมาเป็น Machine code ที่ทำงานเร็วพอๆกับ C/C++ โดยไม่มีระบบ Garbage Collection มาดึงเครื่องให้ช้าลง อีกทั้งยังตรวจจับข้อผิดพลาดได้ตั้งแต่ compile time เป็นส่วนใหญ่ ทำให้โค้ดปลอดภัยและมีประสิทธิภาพสูง โดยไม่มี runtime overhead เพิ่มขึ้น`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `เขียนสั้น ชัดเจน รวมการสร้างตัวแปรและแกะค่าในตัวเดียว ด้วยmatch หรือ if let` | `เน้นใช้ Class Hierarchy (Interface/Inheritance) หรือการต่อยอด enum ดั้งเดิมร่วมกับคำสั่ง switch / match-case (หรือใช้ std::variant ใน C++)   ` |
| Semantics / Behavior | `enum แต่ละตัวเก็บข้อมูลต่างชนิดกันได้เลย และmatchสามารถคืนค่าออกมาใช้ได้ทันที` | `Enum ดั้งเดิมเน้นเก็บเฉพาะค่าคงที่ (Constants) หากต้องการเก็บข้อมูลต่างชนิดกัน ต้องสร้าง Class แยกเป็นคลาสย่อย หรือใช้ไลบรารีเสริม` |
| Type System | `ออกแบบเป็น Algebraic Data Types ตัดปัญหาเรื่อง null ทิ้ง แล้วแทนด้วย Option`|`ตรวจสอบความครอบคลุมได้ไม่สมบูรณ์ตั้งแต่ตอน Compile โดย C++ / Python เสี่ยงตกหล่นในรันไทม์ ขณะที่ Java เพิ่งรองรับการตรวจแบบครอบคลุมใน Sealed Types เวอร์ชันใหม่ ๆ ` |
| Memory Management | `จองหน่วยความจำแบบ Tagged Union เท่าที่จำเป็น ทำงานไว ไม่มีระบบ Garbage Collection มาดึงเครื่อง` | `ภาษาอย่าง Java และ Python ต้องจองพื้นที่สร้าง Object บน Heap และพึ่งพา Garbage Collector ส่วน C++ บริหารหน่วยความจำเองแต่เสี่ยงเรื่อง Type Access Error   ` |
| Safety | `Compiler ดี แต่ต้องเขียนดักให้ครบทุกกรณี ไม่งั้นจะ Compile ไม่ผ่าน` | `มีโอกาสเกิด Runtime Error จากกรณีตกหล่น (Unhandled Exception), ข้อผิดพลาดจาก null หรือการเข้าถึงประเภทข้อมูลผิดประเภทในรันไทม์` |

### Rust Example

```rust
// rust
// ประกาศ enum 
enum PaymentStatus {
    Pending,                 // กำหนดสถานะ
    Success(u32),            
    Failed(String),          
}

// ประกาศฟังก์ชัน รับค่า status 
fn check_payment(status: PaymentStatus) {
    
    // ตรวจสอบค่าของ status ว่าตรงกับสถานะไหน
    match status {
        PaymentStatus::Pending => println!("กำลังตรวจสอบชำระเงิน..."),
        PaymentStatus::Success(id) => println!("ชำระเงินสำเร็จ! รหัสสลิป: {id}"),
        PaymentStatus::Failed(reason) => println!("ชำระเงินไม่สำเร็จ: {reason}"),
    }
}


fn main() {
    
    let status = PaymentStatus::Success(98765);
    
    // เรียกใช้ฟังก์ชัน 
    check_payment(status);
}
```

### `[Other Language]` Example

```java
// java code

// ประกาศ interface 
interface PaymentStatus {
    void check();
}

// สร้างคลาส Pending 
class Pending implements PaymentStatus {
    
    public void check() {
        System.out.println("กำลังตรวจสอบชำระเงิน...");
    }
}


class Success implements PaymentStatus {
    private int transactionId; 
    public Success(int id) { this.transactionId = id; } // คอนสตรัคเตอร์รับค่า id มาบันทึกไว้

    
    public void check() {
        System.out.println("ชำระเงินสำเร็จ! รหัสสลิป: " + transactionId);
    }
}

// สร้างคลาส
class Failed implements PaymentStatus {
    private String reason; 
    public Failed(String reason) { this.reason = reason; } // คอนสตรัคเตอร์รับข้อความสาเหตุมาบันทึกไว้

    
    public void check() {
        System.out.println("ชำระเงินไม่สำเร็จ: " + reason);
    }
}


public class Main {
    
    public static void main(String[] args) {
        
        PaymentStatus status = new Success(98765);
        
        // เรียกใช้เมธอด 
        status.check();
    }
}
```
```C++
// C++
#include <iostream>
#include <string>

// ประกาศ Interface (Abstract Base Class)
class PaymentStatus {
public:
    virtual void check() const = 0; // Pure virtual function
    virtual ~PaymentStatus() = default; // Virtual destructor สำหรับ Polymorphism
};

// สร้างคลาส Pending
class Pending : public PaymentStatus {
public:
    void check() const override {
        std::cout << "กำลังตรวจสอบชำระเงิน..." << std::endl;
    }
};

// สร้างคลาส Success
class Success : public PaymentStatus {
private:
    int transactionId;

public:
    // คอนสตรัคเตอร์รับค่า id มาบันทึกไว้
    Success(int id) : transactionId(id) {}

    void check() const override {
        std::cout << "ชำระเงินสำเร็จ! รหัสสลิป: " << transactionId << std::endl;
    }
};

// สร้างคลาส Failed
class Failed : public PaymentStatus {
private:
    std::string reason;

public:
    Failed(const std::string& reason) : reason(reason) {}

    void check() const override {
        std::cout << "ชำระเงินไม่สำเร็จ: " << reason << std::endl;
    }
};

int main() {
    // ใช้ Pointer ของ Class แม่ (Interface) เพื่อเรียกใช้ Polymorphism
    PaymentStatus* status = new Success(98765);

    // เรียกใช้เมธอด
    status->check();

    // คืนหน่วยกิต
    delete status;

    return 0;
}
```
```Python
#python
from abc import ABC, abstractmethod


# ประกาศ Interface (Abstract Base Class)
class PaymentStatus(ABC):

  @abstractmethod
  def check(self):
    pass


# สร้างคลาส Pending
class Pending(PaymentStatus):

  def check(self):
    print("กำลังตรวจสอบชำระเงิน...")


# สร้างคลาส Success
class Success(PaymentStatus):

  def __init__(self, transaction_id: int):
    self.transaction_id = transaction_id  # บันทึกค่า id ไว้

  def check(self):
    print(f"ชำระเงินสำเร็จ! รหัสสลิป: {self.transaction_id}")


# สร้างคลาส Failed
class Failed(PaymentStatus):

  def __init__(self, reason: str):
    self.reason = reason  # บันทึกข้อความสาเหตุไว้

  def check(self):
    print(f"ชำระเงินไม่สำเร็จ: {self.reason}")


# การใช้งานในฟังก์ชั่นหลัก
if __name__ == "__main__":
  status: PaymentStatus = Success(98765)

  # เรียกใช้เมธอด
  status.check()
```

### Analysis

`ความแตกต่างสำคัญระหว่าง Rust กับภาษาอื่น (C++, Java, Python) คือการเปลี่ยนวิธีจัดการสถานะและข้อมูลหลากรูปแบบ จากเดิมที่ภาษาอื่นใช้แนวคิด OOP / Dynamic Typing (การใช้ Class Hierarchy, Dynamic Dispatch หรือ std::variant) มาเป็นการใช้ Algebraic Data Types (Sum Types) ผ่าน enum ที่แนบข้อมูลไว้ใน Variant ได้โดยตรง ร่วมกับระบบ Exhaustive Pattern Matching (match) และการจัดเก็บแบบ Tagged Union บน Stack ทำให้ Rust สามารถจัดการข้อมูลได้กะทัดรัด ตรวจสอบกรณีต่างๆ ได้อย่างรวดเร็ว และไม่มี Overhead จาก Garbage Collector (GC) หรือการจอง Heap โดยไม่จำเป็น`

`เหตุผลด้านการออกแบบภาษา (Design Philosophy)
Rust ถูกออกแบบมาบนปรัชญา "Zero-cost Abstractions, Compile-time Safety และ Data-Oriented Design" โดยมุ่งเน้นย้ายข้อผิดพลาดทั้งหมดที่อาจเกิดขึ้นขณะทำงาน (Runtime Errors เช่น การลืมเช็คบางกรณี, Null Pointer หรือ Type Error) มาให้ Compiler บังคับตรวจจับให้ครบถ้วนตั้งแต่ขั้นตอน Compile Time ภาษาจึงเลือกใช้ Sum Types และ Pattern Matching เพื่อให้ได้ความปลอดภัยสูงสุดโดยไม่ต้องแลกมาด้วย Garbage Collector เหมือน Java/Python และยังคงประสิทธิภาพระดับ System-level Programming เท่า C/C++ `

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

`เขียนเนื้อหาหัวข้อที่ 1 - 5 โดยทำหน้าที่สรุปแนวคิดหลัก อธิบายโค้ด syntax พร้อมตัวอย่างโค้ดอย่างสั้น `

**Member 2**

`จัดทำหัวข้อที่ 6 Runnable Code Examples ทดสอบโค้ด ปรับแก้คำ และตรวจเช็คเอกสารทั้งหมด `

**Member 3**

`จัดทำหัวข้อที่ 9 PPL Perspective และ หัวข้อที่ 10 Rust vs Other Language  `

**Member 4**

`จัดทำหัวข้อที่ 7 Common Mistakes , หัวข้อที่ 8 Exercises , Challenge และทดสอบโค้ดทั้งหมด`


---

## 12. References

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[ Rust Documentation ]`
4. `[ W3Schools ]`
5. `[ Workshop EDM - Rust 2024 ]`

---

## 13. AI Usage Declaration
| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[Gemini,ChatGPT]` | `[แปลเอกสาร Rust Book พร้อมสรุปเนื้อหา]` | `[อ่านทำความเข้าใจและเรียบเรียงข้อมูลใหม่]` |
| `[Claude]` | `[ใช้เพื่อตรวจสอบภาพรวมของเนื้อหาเป็นหัวข้อ พร้อมบอกข้อควรระวัง ช่วยหาปัญหาสำหรับจัดทำโค้ดตัวอย่าง และช่วยประเมินความยากของโจทย์แล้วปรับปรุงให้เหมาะสมกับเนื้อหา]` | `[นำข้อมูลที่ได้เทียบกับ Rust Book พร้อมกับทดสอบโค้ดเพื่อความถูกต้องก่อนนำมาใส่ในเอกสาร]` |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**
`ในขั้นตอนเตรียมเนื้อหา ใช้ในการเรียนรู้ภาษา Rust ทำความเข้าใจเนื้อหา  เรียบเรียงคำอธิบาย และทบทวนหัวข้อให้เข้าใจมากขึ้น`
`ในขั้นตอนการออกแบบ Exercise ใช้ AI ในการประเมินความยากของโจทย์เพื่อปรับปรุงให้เหมาะสมกับเนื้อหาที่เรียน`
`ในขั้นตอนเปรียบเทียบความต่างของภาษา Rust และ java ใช้ AI ในการเลือกหัวข้อที่เหมาะสม เพื่อแสดงความแตกต่างให้เห็นได้ชัดเจน`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `5` | `1` | `0` | `เขียนเนื้อหาหัวข้อที่ 1 - 5 โดยทำหน้าที่สรุปแนวคิดหลัก อธิบายโค้ด syntax พร้อมตัวอย่างโค้ดอย่างสั้น` |
| Member 2 | `0` | `28` | `1` | `0` | `จัดทำหัวข้อที่ 6 Runnable Code Examples ทดสอบโค้ด ปรับแก้คำ และตรวจเช็คเอกสารทั้งหมด` |
| Member 3 | `0` | `33` | `0` | `0` | `จัดทำหัวข้อที่ 9 PPL Perspective และ หัวข้อที่ 10 Rust vs Other Language ` |
| Member 4 | `0` | `15` | `2` | `0` | `จัดทำหัวข้อที่ 7 Common Mistakes และ หัวข้อที่ 8 Exercises ทดสอบโค้ด` |

### Teamwork Reflection

**How did your team collaborate?**

`วางแผนกำหนดวันทำงานพร้อมแบ่งงานกันตามหน้าที่ตนเองได้รับผ่านกลุ่มไลน์ จากนั้นรับผิดชอบงานตามหน้าที่และอัพเดตงานทั้งหมดใน github พร้อมตรวจสอบเอกสารทุกส่วน`

**Problems encountered**

`ระหว่างที่แต่ละคนทำงานจะยังไม่เห็นการแก้ไขของเพื่อนคนอื่นในทันที ทำให้บางครั้งเนื้อหาหรือตัวอย่างที่แต่ละคนจัดทำเกิดความซ้ำซ้อนกัน`

**How did you solve them?**

`ช่วยกันตรวจสอบเนื้อหา พูดคุยกันในกลุ่มมากขึ้นเพื่อหาข้อผิดพลาดหรือเนื้อหาที่ซ้ำกันแล้วทำการแก้ไข`

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

**Repository:** `https://github.com/670710143/rust-tutorial-2569`

**Chapter Path:** `chapters/09-enums-pattern-matching`

**Final PR:** `#15`

**Submitted by:** `Group 09`

**Date:** `[2026-10-02]`
