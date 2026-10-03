# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** `04`<br>
> **Topic No.:** `04`<br>
> **Topic Name:** `Functions`<br>
> **ประเด็นหลักที่ควรครอบคลุม:** `function declaration, parameters, return values, expressions ใน function, scope`<br>

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายจักรพรรดิ โพธิพิภัทรกุล | 660710976 | `@660710976` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายกฤตเมธ ไทยภักดี | 670710121 | `@670710121` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นางสาวกัญญรัชต์ สมหวังพรเจริญ | 670710122 | `@670710122` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นางสาวกัญญาณัฐ เขมนรากร | 670710123 | `@670710123` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

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

`Function ในภาษา Rust คือ ชุดคำสั่งย่อยที่ประกาศด้วย fn ใช้จัดระเบียบตรรกะและสามารถเรียกใช้งานซ้ำได้`<br>
`ช่วยให้โค้ดเป็นระเบียบ อ่านง่าย ลดความซ้ำซ้อน และปลอดภัยด้วยระบบตรวจสอบชนิดข้อมูล (Type System) ที่เข้มงวด`<br>
`แก้ปัญหาโค้ดซ้ำซ้อน (Duplication), โค้ดซับซ้อนแก้ไขยาก และข้อผิดพลาดจากการส่งชนิดข้อมูลผิดพลาดตั้งแต่ตอนคอมไพล์`

---

## 4. Key Concepts

### 4.1 `[การประกาศฟังก์ชัน (Function Declaration)]`

**คำอธิบาย**

`ใช้คีย์เวิร์ด fn ตามด้วยชื่อฟังก์ชัน` <br>
`ต้องระบุชนิดข้อมูล (Data Type) ของพารามิเตอร์ทุกตัวอย่างชัดเจนเสมอ` <br>
`หากฟังก์ชันมีการคืนค่า (Return value) ต้องใส่เครื่องหมาย -> ตามด้วยชนิดข้อมูลที่คืนค่า`

**ตัวอย่าง**

```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

**Explanation**

`fn add_numbers(a: i32, b: i32) -> i32: ประกาศฟังก์ชันชื่อ add_numbers รับพารามิเตอร์ a และ b ชนิด i32 และกำหนดให้คืนค่า (Return) ออกมาเป็นชนิด i32` <br>
`a + b: เป็นการคืนค่าผลบวกของตัวแปร a และ b`

---

### 4.2 `Statement vs Expression (หัวใจสำคัญของ Rust)`

`ภาษา Rust แยกแยะระหว่าง Statement และ Expression อย่างชัดเจน ซึ่งส่งผลต่อการคืนค่าของฟังก์ชัน:` <br><br>

`Statement: คือคำสั่งที่ทำงานบางอย่างแต่ ไม่คืนค่า มักจะลงท้ายด้วยเครื่องหมายเซมิโคลอน ;`<br>
`Expression: คือนิพจน์ที่ ประเมินผลและคืนค่าออกมา (เช่น 5 + 6, การเรียกฟังก์ชัน, หรือแม้แต่บล็อกโค้ด { let x = 3; x + 1 })` <br><br>

`กฎการ Return: บรรทัดสุดท้ายของฟังก์ชันใน Rust หาก ไม่มีเซมิโคลอน (;) ต่อท้าย จะถือว่าเป็น Expression และถูกใช้เป็นค่า Return ของฟังก์ชันนั้นทันที (ไม่ต้องใช้คำว่า return) แต่ถ้าใส่ ; จะกลายเป็น Statement ที่ไม่คืนค่า (ซึ่งถ้าฟังก์ชันต้องการคืนค่า i32 แต่ดันใส่ ; จะเกิด Error ทันที)`

---

### 4.3 `[การคืนค่าหลายค่า (Multiple Return Values)]`

`Rust ไม่มี Syntax พิเศษสำหรับการคืนค่าหลายค่าโดยตรง แต่เราสามารถใช้ Tuple หรือ Struct เพื่อคืนค่ามากกว่าหนึ่งค่าได้อย่างง่ายดาย`

```rust
fn get_user_info() -> (String, u32) {

    ("Alice".to_string(), 30)
}
```

---

### 4.4 `[การจัดการ Ownership และ Borrowing กับ Function]`

`เนื่องจาก Rust ใช้ระบบ Memory Management แบบ Ownership การส่งตัวแปร (เช่น String หรือ Vector) เข้าไปในฟังก์ชัน จะมีผลต่อสิทธิ์การใช้งานตัวแปรนั้น:` <br><br>

`Move: ถ้าส่งตัวแปรเข้าไปตรงๆ (เช่น print_string(s)) สิทธิ์ความเป็นเจ้าของจะย้าย (Move) เข้าไปในฟังก์ชัน ตัวแปรเดิมข้างนอกจะใช้งานต่อไม่ได้` <br>
`Borrowing (References): หากต้องการให้ฟังก์ชันยืมไปใช้งานเฉยๆ โดยที่ข้างนอกยังใช้ต่อได้ ให้ส่งผ่าน Reference (& หรือ &mut) แทน`

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let my_string = String::from("Hello");
    print_length(&my_string);
    println!("{}", my_string);
}
```

---

### 4.5 `Diverging Functions (ฟังก์ชันที่ไม่เคยคืนค่า)`

`Rust มีฟังก์ชันพิเศษที่เรียกว่า Diverging function ซึ่งใช้เครื่องหมาย ! เป็น Return type หมายความว่าฟังก์ชันนี้ทำงานแล้วไม่มีวันจบปกติ (เช่น ฟังก์ชันที่โปรแกรมจะพังหรือวนลูปไม่รู้จบ)`

```rust
fn panic_error() -> ! {
    panic!("Crash program!");
}
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `fn name(param: Type) -> ReturnType` | `การประกาศฟังก์ชัน กำหนดชื่อ พารามิเตอร์พร้อม Type และชนิดข้อมูลที่ต้องคืนค่า` | `fn add(a: i32, b: i32) -> i32` |
| `บรรทัดสุดท้าย ไม่มี เซมิโคลอน (;)` | `ใช้เป็น Expression เพื่อคืนค่าอัตโนมัติ (Implicit Return) โดยไม่ต้องใช้คำสั่ง return` | `fn square(x: i32) -> i32 { x * x }` |
| `Return Type เป็นเครื่องหมายตกใจ -> !` | `Diverging Function ฟังก์ชันที่ไม่เคยคืนค่าปกติกลับมา (เช่น Panic หรือ Infinite Loop)` | `fn fail() -> ! { panic!("Error"); }` |
| `&mut T` | `Mutable Reference: การส่ง Reference เข้าฟังก์ชันเพื่อให้ฟังก์ชันสามารถแก้ไขค่าของตัวแปรต้นฉบับได้` | `fn update(s: &mut String) { s.push_str("!"); }` |
| `pub fn / pub(crate) fn` | `Visibility (การมองเห็น): กำหนดสิทธิ์การเข้าถึงฟังก์ชันจากภายนอกโมดูลหรือ crate` | `pub fn calculate() {}` |
| `dyn Trait / impl Trait` | `Generics & Traits: การสร้างฟังก์ชันที่รับหรือคืนค่าได้หลาย Type แบบ Polymorphism` | `fn print_item<T: Display>(item: T) {}` |
| `Closures (\|params\| body)` | `Anonymous Functions: ฟังก์ชันไม่มีชื่อที่สามารถจับตัวแปรจาก Environment รอบข้างได้` | `let add = \|a, b\| a + b;` |

### Important Rules

1. `Type Annotation is Mandatory: พารามิเตอร์ทุกตัวของฟังก์ชันในภาษา Rust ต้องระบุ Data Type เสมอ (คอมไพเลอร์จะไม่ช่วยเดา Type ให้เหมือนกับตัวแปรทั่วไปที่ใช้ let)`
2. `Statement vs Expression: ห้ามใส่เซมิโคลอน (;) ที่บรรทัดสุดท้ายของฟังก์ชันหากต้องการให้บรรทัดนั้นเป็นค่า Return ถ้าใส่จะกลายเป็น Statement และทำให้เกิดคอมไพล์เออร์เออร์เรอร์หากฟังก์ชันนั้นกำหนด Return Type ไว้`
3. `Ownership Transfer by Default: การส่งค่าตัวแปรประเภทที่ไม่ใช่ Primitive (เช่น String หรือ Vector) เข้าไปในฟังก์ชันแบบปกติจะทำให้เกิดการย้ายสิทธิ์ (Move) และไม่สามารถนำตัวแปรนั้นกลับมาใช้ซ้ำข้างนอกได้ เว้นแต่จะใช้ Reference (&) เพื่อยืมค่าแทน`
4. `Method vs Function: ฟังก์ชันที่ถูกผูกไว้กับ Struct หรือ Enum (ประกาศภายในบล็อก impl) ซึ่งจะมีพารามิเตอร์ตัวแรกเป็น self, &self, หรือ &mut self`
5. `Higher-Order Functions: ความสามารถในการรับฟังก์ชันอื่นเป็นพารามิเตอร์ หรือการคืนค่าฟังก์ชันออกจากฟังก์ชัน (มักใช้คู่กับ Closures และ Iterator เช่น .map() หรือ .filter())`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `Factorial`

**Purpose:** `การสร้าง Function การคำนวนอะไรบ้างอย่างในที่นี้ขอยกเป็นการหาค่า Factorial`

```rust
fn main() {
    let x = factorial(5);
    print!("{}", x)
}

fn factorial(num: i32) -> i32 {
    let mut result = 1;
    if num == 0 {
        return result;
    }
    else {
        for i in 1..=num {
            result *= i
        }
        return result;
    }
}
```

**Expected Output**

```text
120
```

**Explanation**

`factorial(num): ฟังก์ชันคำนวณแฟกทอรีล`<br>
`ตรวจสอบเงื่อนไขว่าถ้า num เป็น 0 จะคืนค่า 1 ทันที`<br>
`หากไม่ใช่ จะใช้ลูป for วนซ้ำตั้งแต่ 1 ถึง num (รวม num ด้วยผ่าน 1..=num) เพื่อนำค่ามาคูณสะสมในตัวแปร result จนครบ (1 x 2 x 3 x 4 x 5 = 120) แล้วคืนค่าผลลัพธ์กลับไป`

---

### Example 2 — `[discount]`

**Purpose:** `ทำให้รู้ว่า Function สร้าง Function ได้`

```rust
use std::io;
#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

fn create_discount(discount: f64) -> impl Fn(f64) -> f64 {
    move |price| price * (1.0 - discount)
}
fn calculate(
    products: &[Product],
    operation: impl Fn(f64) -> f64,
) {
    for product in products {
        let new_price = operation(product.price);

        println!(
            "{} : {:.2} -> {:.2}",
            product.name,
            product.price,
            new_price
        );
    }
}
fn input(message: &str) -> String {
    let mut value: String = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut value).unwrap();
    value.trim().to_string()
}

fn main() {
    let name1 = input("Product 1 name:");
    let price1: f64 = input("Product 1 price:").parse().unwrap();
    let name2 = input("Product 2 name:");
    let price2: f64 = input("Product 2 price:").parse().unwrap();
    let name3 = input("Product 3 name:");
    let price3: f64 = input("Product 3 price:").parse().unwrap();

    let discount: f64 = input("Discount (%):").parse().unwrap();

    let products = [
        Product {
            name: name1,
            price: price1,
        },
        Product {
            name: name2,
            price: price2,
        },
        Product {
            name: name3,
            price: price3,
        },
    ];

    let discount_fn = create_discount(discount / 100.0);

    calculate(&products, discount_fn);
}


//let mut input = String::new();
//io::stdin().read_line(&mut input).unwrap();
//let age: i32 = input.trim().parse().unwrap();


```
**input**
Product 1 name:
iphone 
Product 1 price:
29990
Product 2 name:
ipad 
Product 2 price:
27900
Product 3 name:
mac
Product 3 price:
44900
Discount (%):
10
**Output**
iphone : 29990.00 -> 26991.00
ipad : 27900.00 -> 25110.00
mac : 44900.00 -> 40410.00
```text
[expected output]
```

**Explanation**

`operation = ฟังก์ชันที่ส่งเข้ามาเป็น Parameter `<br>
`Fn(f64) = รับค่าตัวเลขชนิด f64`<br>
`-> f64 = คืนค่าตัวเลขชนิด f64`<br>

---
## 7. Common Mistakes

### Mistake 1 — Function Overloading

**Problem**

บางครั้งอาจเกิดการสับสนในการใช้ และคิดว่าภาษา rust สามารถทำ `Function Overloading` ได้ (การสร้างฟังก์ชันชื่อเดียวกัน แต่รับ Parameter หรือมี Type ต่างกัน)  แต่ในความจริงเป็นแล้ว rust ไม่รองรับการทำ Function Overloading โดยตรง

**Incorrect Code**

```rust
fn add(x:i32 , y:i32) -> i32{
    x + y
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    let sum1 = add(2,5);
    let sum2 = add(2.2,5.5);
    println!("sum1 = {}",sum1);
    println!("sum2 = {}",sum2);
}
```

**Correct Code**

```rust
fn add_i32(x:i32 , y:i32) -> i32{
    x + y
}

fn add_f64(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    let sum1 = add_i32(2,5);
    let sum2 = add_f64(2.2,5.5);
    println!("sum1 = {}",sum1); // sum1 = 7
    println!("sum2 = {}",sum2); // sum2 = 7.7
}
```

**Why?**

* เพื่อป้องกันความสับสนของ `Type Inference` : หากมี Overloading อาจทำให้ Compiler สับสนหรือคาดเดาประเภทข้อมูลผิดจากที่ผู้เขียนตั้งใจ

* แนวทางใกล้เคียง : ใช้ `Generics` หรือ `Traits` แทน Function Overloading 

---

### Mistake 2 — Default Parameter

**Problem**

ภาษา rust ไม่รองรับการทำ `default parameter`(การกำหนดค่าเริ่มให้กับฟังก์ชัน)

**Incorrect Code**

```rust
fn greet(name: &str, msg: &str = "Hello") {
    println!("{}, {}!",msg,name);
}
fn main() {
    greet("Alice", None);               
    greet("Bob", Some("Good morning")); 
}
```

**Correct Code**

```rust
fn greet(name: &str, msg: Option<&str>) {
    let greeting = match msg {
        Some(msg) => msg,
        None => "Hello",
    };
    println!("{}, {}!", greeting, name);
}
fn main() {
    greet("Alice", None); // Hello, Alice!    
    greet("Bob", Some("Good morning")); // Good morning, Bob!
}
```

**Why?**

* เพื่อความชัดเจน (Explicit over Implicit): Rust เน้นให้ผู้อ่านโค้ดมองเห็นสิ่งที่เกิดขึ้นอย่างชัดเจนที่สุด โดยไม่มีระบบซ่อนการทำงานอยู่เบื้องหลัง

* แนวทางใกล้เคียง : ใช้ `Option<T>` หรือ `Default Trait` แทน Default Parameters

---

## 8. Exercises

### Exercise 1 — Is_Even

**Problem**

จงเขียนฟังก์ชันชื่อ `is_even` ที่รับตัวเลขจำนวนเต็ม `(i32)` เข้ามา 1 ตัว แล้วทำการตรวจสอบว่าตัวเลขนั้นเป็น เลขคู่ (Even Number) หรือไม่
* หากเป็นเลขคู่ ให้พิมพ์คำว่า "`True`"
* หากเป็นเลขคี่ ให้พิมพ์คำว่า "`False`"

**Hint**

* สามารถตรวจสอบเลขคู่ โดยใช้ตัวดำเนินการ Modulo (%)

**Solution**

```rust
fn is_even(number: i32){
    if number % 2 == 0 {
        println!("True");
    } else {
        println!("False");
    }
}
fn main() {
    let num1 = 4;
    let num2 = 7;

    is_even(num1); // True
    is_even(num2); // False
}
```
**Explanation**

* `fn is_even(number: i32)`  : สร้างฟังก์ชันชื่อ `is_even`  ที่รอรับค่าตัวเลขจำนวนเต็ม `(i32)` เข้ามาผ่านพารามิเตอร์ชื่อ `number`
* `if number % 2 == 0`  : นำเลขมาหารด้วย 2 ถ้าหารลงตัว เศษเป็น 0 หมายความว่าตัวเลขนั้นเป็นเลขคู่ ให้พิมพ์ `"True"` 
* `else`  : ถ้าหารไม่ลงตัว หมายความว่าตัวเลขนั้นเป็นเลขคี่ ให้พิมพ์ `"False"` 
* `main()`  : มีการประกาศตัวแปรที่เป็นตัวเลข และส่งตัวแปรไปเช็ค โดยส่ง 4 ได้ผลลัพธ์เป็น True และส่ง 7 ไปเช็ค ได้ผลลัพธ์เป็น `False`

---
### Exercise 2 — Grade_Check

**Problem**

จงเขียนฟังก์ชันชื่อ `grade_check` โดยรับพารามิเตอร์เป็นตัวเลขจำนวนเต็ม  `score (i32)` โดยคะแนนที่รับมาจะไม่เป็นค่าติดลบ หรือมากกว่า 100  จากนั้นจะตัดเกรดโดยใช้เกณฑ์ว่า ถ้าได้ไม่ถึง 40 คะแนนได้เกรดเป็น Fail, ได้ถึง 40 แต่ไม่ถึง 80 คะแนนได้เกรดเป็น Pass, และถ้าได้
ถึง 80 คะแนน จะได้เกรดเป็น Excellent

**Hint**

* ฟังก์ชันนี้ไม่ต้องคืนค่า `(ไม่ต้องใส่ -> ...)` ใช้ `println!` แสดงข้อความในฟังก์ชันได้เลย

**Solution**

```rust
fn grade_check(score: i32){
    if score >= 80 {
        println!("Exellent");
    }else if score >= 40 {
        println!("Pass");
    } else {
        println!("Fail");
    }
}
fn main() {
    grade_check(39); // Fail
    grade_check(40); // Pass
    grade_check(90); // Exellent
}
```

**Explanation**

* `fn grade_check(score: i32)`
    * รับพารามิเตอร์ `score` ชนิดจำนวนเต็ม `i32`
    * ไม่มี `-> ...` แปลว่าฟังก์ชันนี้ไม่คืนค่า ทำหน้าที่แค่แสดงข้อความออกหน้าจอ

* `if score >= 80` : ถ้าคะแนนมากกว่าหรือเท่ากับ 80 ให้แสดง `Excellent` 

* `else if score >= 40` : ถ้า`score` น้อยกว่า 80 และ `>= 40` ให้แสดง `Pass`

* `else` : กรณีที่เหลือทั้งหมด คือ `score` น้อยกว่า 40 ให้แสดง `Fail`

* `grade_check(...)` ส่งค่าตัวเลขไปยังฟังก์ชัน จะแสดงผลออกมาเป็น `Excellent` `Pass` และ `Fail` ตามเงื่อนไขที่เรากำหนดไว้

---


## 9. PPL Perspective

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

ภาษา Rust ประกาศฟังก์ชันด้วย keyword fn ตามด้วยชื่อฟังก์ชัน วงเล็บสำหรับ parameters และ { } สำหรับส่วน body ของฟังก์ชัน
  1) Function พื้นฐาน
     ```rust
     fn function_name() {
        // คำสั่งที่ต้องการให้ function ทำงาน
     }
     ```
  2) Function ที่มี Parameters
  ```rust
      // Create a function
      fn say_hello() {
        println!("Hello from a function!");
      }

      say_hello(); // Call the function
  ```
    ((function parameter ของ Rust เป็นส่วนหนึ่งของ static type system = รู้ตั้งแต่ compile))
  3) Function ที่มี Return Value
     ```rust
     fn add(a: i32, b: i32) -> i32 {
      return a + b;
     }

     let sum = add(3, 4);
     println!("Sum is: {}", sum);
     ```
### 9.2 Semantics

`เมื่อมีการเรียก Function ค่า arguments จะถูกผูกกับ parameters จากนั้นคำสั่งและ expressions ภายใน function body จะถูกประมวลผล และค่าของ expression สุดท้ายที่ไม่มี semicolon (;) สามารถใช้เป็น return value ได้โดยอัตโนมัติ นอกจากนี้สามารถใช้ return เมื่อต้องการคืนค่าออกจาก Function โดยตรงได้เช่นกัน`
ตัวอย่างที่ไม่ได้ใส่ semicolon (;)
```rust
    fn add_one(x: i32) -> i32 {
      x + 1
    }

    fn main() {
      let result = add_one(5);
      println!("{}", result);
    }
```
Expression สุดท้ายจะเป็นค่าที่ Function คืนกลับโดยอัตโนมัติ

ตัวอย่างที่ใส่ semicolon (;)
```rust
    fn add_one(x: i32) -> i32 {
      x + 1;      // ได้ () แต่ต้องการ i32 → Error
    }
```
ผลลัพธ์ของ expression จะไม่ถูกใช้เป็นค่าที่ Function คืน ทำให้ body มีค่าเป็น () และเกิด error หาก Function กำหนดว่าต้องคืน i32
### 9.3 Type System

Rust เป็นภาษาแบบ Statically Typed หมายความว่า ชนิดข้อมูลของ parameters และ return value ของ function จะถูกตรวจสอบตอน Compile ก่อนโปรแกรมทำงาน โดยชนิดของ arguments ที่ส่งเข้า function และค่าที่ function คืนกลับต้องสอดคล้องกับชนิดที่ประกาศไว้ หากชนิดข้อมูลไม่ตรงกันจะเกิด Compile-time Error

### 9.4 Memory / Resource Management

Rust ผ่านระบบ Ownership และ Borrowing เมื่อส่งข้อมูลเข้า Function ค่าอาจถูก Move, Copy หรือ Borrow ขึ้นอยู่กับชนิดข้อมูลและวิธีการส่งค่า เมื่อเจ้าของข้อมูลออกจาก Scope Rust จะทำลายข้อมูลและคืน Resource โดยอัตโนมัติ ซึ่งช่วยลดปัญหาเกี่ยวกับหน่วยความจำ เช่น dangling references และช่วยให้จัดการหน่วยความจำได้อย่างปลอดภัยโดยไม่ต้องใช้ Garbage Collector

### 9.5 Abstraction / Other PPL Concepts

#### Abstraction
Function เป็น **Procedural Abstraction** (การรวมขั้นตอนการทำงานไว้ภายใต้ชื่อเดียว) ผู้เรียกสนใจเพียงว่า Function รับอะไรเข้าไป และคืนอะไรออกมา โดยไม่จำเป็นต้องรู้รายละเอียดภายในทุกขั้นตอน
```rust
    fn average(s: &[f64]) -> f64 {
      s.iter().sum::<f64>() / s.len() as f64
    }
    // เรียกใช้: average(&[80.0, 90.0]) ไม่ต้องรู้ขั้นตอนภายใน
```
#### Scope
Rust ใช้ **Lexical Scope หรือ Static Scope** คือ scope ของตัวแปรพิจารณาได้จากโครงสร้างของ source code
```rust
    let x = 10;
    {
        let y = 5;              // y อยู่แค่ใน block นี้
        println!("{}", x + y);
    }
    let x = x * 2;              // shadowing
    let show = || println!("{}", x); // closure เข้าถึง x ได้ (fn ซ้อนทำไม่ได้)
```
#### Binding
เมื่อมีการเรียก Function ค่า **arguments** จะถูก binding เข้ากับ **parameters** ของ Function เพื่อให้สามารถนำค่าเหล่านั้นไปใช้งานภายใน Function ได้
```rust
    fn add(a: i32, b: i32) -> i32 { a + b } // a, b = parameters
    let r = add(3, 4);                      // 3, 4 = arguments ถูกผูกเข้ากับ a, b

    // Static vs Dynamic binding
    fn f_static<T: Speak>(x: &T) { x.speak() }  // ผูกตอน compile
    fn f_dyn(x: &dyn Speak) { x.speak() }       // ผูกตอนรันไทม์ (vtable)
```
#### Paradigm
Rust เป็นภาษาแบบ Multi-paradigm รองรับทั้ง Imperative, Functional และ Object-oriented บางส่วน (ผ่าน struct, impl, trait) ในส่วนของ Functions นั้น Rust ถือว่า Function เป็น first-class value คือเก็บในตัวแปร ส่งเป็น argument และคืนเป็น return value ได้ รองรับ Closures ที่จับตัวแปรจากบริบทรอบข้างได้ และ Higher-order Functions เช่น map และ filter อย่างไรก็ตาม Rust ไม่ใช่ภาษา Functional แบบ pure เพราะ Function มี side effect ได้
```rust
  // Imperative
  let mut sum = 0;
  for i in 1..=5 { sum += i; }

  // Functional: first-class function, closure, higher-order
  fn make_adder(n: i32) -> impl Fn(i32) -> i32 { move |x| x + n }
  let add5 = make_adder(5);

  let v: Vec<i32> = (1..=5).filter(|x| x % 2 == 1).map(|x| x * x).collect(); // [1, 9, 25]
```
#### Ownership & Borrowing
การเรียกใช้ Function ใน Rust มีความเกี่ยวข้องกับระบบ **Ownership** ของภาษา โดยเมื่อส่งค่าเข้าไปใน Function ค่านั้นอาจถูก ย้ายความเป็นเจ้าของ (Move), คัดลอก (Copy) หรือ ยืมไปใช้ (Borrow) ขึ้นอยู่กับชนิดข้อมูลและวิธีการส่งค่า โดยไม่จำเป็นต้องใช้ Garbage Collector.
```rust
  fn take(s: String) {
    println!("take: {}", s);
  }

  fn len(s: &String) -> usize {
    s.len()
  }

  fn append(s: &mut String) {
    s.push('!');
  }

  fn main() {
    let a = String::from("hi");
    let n = 5;

    take(a);          // Move: a ใช้ต่อไม่ได้

    let m = n;        // Copy: n ยังใช้ได้ เพราะ i32 เป็น Copy
    println!("n = {}, m = {}", n, m);

    let mut b = String::from("hi");

    let length = len(&b);     // Immutable Borrow
    println!("length = {}", length);

    append(&mut b);           // Mutable Borrow
    println!("b = {}", b);
  }
```

### 9.6 Why Rust?

Rust ออกแบบ Functions ให้ทำงานร่วมกับ **Type System และ Ownership & Borrowing** เพื่อเพิ่มความปลอดภัย ความน่าเชื่อถือ และประสิทธิภาพของโปรแกรม
#### Safety
Rust ตรวจสอบชนิดข้อมูลของ **parameters และ return values** ตั้งแต่ Compile Time รวมถึงตรวจสอบกฎ Ownership และ Borrowing เมื่อมีการส่งข้อมูลระหว่าง Functions จึงช่วยป้องกันข้อผิดพลาดด้านชนิดข้อมูลและปัญหาการจัดการหน่วยความจำ
#### Reliability
กฎที่ชัดเจนเกี่ยวกับ **Scope, Type และ Ownership** ช่วยให้พฤติกรรมของ Function คาดเดาได้มากขึ้น และตรวจพบข้อผิดพลาดหลายประเภทก่อนโปรแกรมทำงานจริง
#### Performance
Rust สามารถส่งข้อมูลเข้า Function ได้ทั้งแบบ **Move, Copy และ Borrow** โดยการ Borrow ผ่าน reference ช่วยให้ Function เข้าถึงข้อมูลได้โดยไม่จำเป็นต้องคัดลอกข้อมูลทั้งหมด และ Rust สามารถจัดการหน่วยความจำได้โดยไม่ต้องพึ่ง Garbage Collector

---

## 10. Rust vs. Other Language

**Comparison Language:** `Python`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `ใช้ { } กำหนด block และมักใช้ ; ปิดท้าย statement ประกาศตัวแปรด้วย let` | `ใช้ indentation กำหนด block ไม่ใช้ { } และไม่จำเป็นต้องใช้ ; การประกาศตัวแปรทำได้โดยกำหนดค่าโดยตรง` |
| Semantics / Behavior | `ตรวจสอบข้อผิดพลาดตั้งแต่ Compile Time` | `ตรวจสอบข้อผิดพลาดขณะ Runtime` |
| Type System | `Statically Typed และ Strongly Typed` | `Dynamically Typed และ Strongly Typed` |
| Memory Management | `Ownership, Borrowing, Lifetimes และไม่ใช้ Garbage Collector` | `Automatic Memory Management โดยหลักผ่าน Reference Counting ร่วมกับ Garbage Collector` |
| Safety | `Memory Safety และ Thread Safety โดย compiler ตรวจสอบ ownership, borrowing และ lifetime ช่วยป้องกันปัญหา` | `จัดการ raw memory โดยตรงในโค้ด Python ทั่วไป` |

### Rust Example

```rust
  fn main() {
    let name: String = String::from("Rust");
    let length: usize = get_length(&name);

    println!("Language: {}", name);
    println!("Length: {}", length);
  }

  fn get_length(text: &String) -> usize {
    text.len()
  }
```

### `[Other Language]` Example

```python
  def get_length(text):
    return len(text)

  name = "Python"
  length = get_length(name)

  print("Language:", name)
  print("Length:", length)
```

### Analysis

Rust เน้นความปลอดภัยและประสิทธิภาพ ด้วย Static Typing และระบบ Ownership ที่ตรวจสอบตั้งแต่ Compile Time จึงช่วยลดข้อผิดพลาดด้าน Memory ได้โดยไม่ต้องใช้ Garbage Collector ส่วน Python เน้นความเรียบง่ายและยืดหยุ่น ด้วย Dynamic Typing และ Automatic Memory Management ทำให้เขียนและพัฒนาโปรแกรมได้ง่ายกว่า แต่ข้อผิดพลาดบางอย่างอาจตรวจพบเมื่อ Runtime

---

**Comparison Language:** `Java`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `ใช้ { } กำหนด block และ ; ปิดท้าย statement ประกาศตัวแปรด้วย let` | `ใช้ { } กำหนด block และ ; ปิดท้าย statement โดยทั่วไปต้องระบุชนิดข้อมูล` |
| Semantics / Behavior | `ตรวจสอบข้อผิดพลาดตั้งแต่ Compile Time` | `Compile เป็น Bytecode และโดยทั่วไปทำงานผ่าน JVM` |
| Type System | `Statically Typed และ Strongly Typed` | `Statically Typed และ Strongly Typed และรองรับ Type Inference ใน local variables ด้วย var` |
| Memory Management | `Ownership, Borrowing, Lifetimes และไม่ใช้ Garbage Collector` | `ใช้ Garbage Collector (GC) จัดการ Memory ของ Object ที่ไม่ถูกใช้งานโดยอัตโนมัติ` |
| Safety | `Memory Safety และ Thread Safety โดย compiler ตรวจสอบ ownership, borrowing และ lifetime ช่วยป้องกันปัญหา` | `JVM และ GC ช่วยลดปัญหาการจัดการ Memory โดยตรง แต่ยังสามารถเกิดข้อผิดพลาดขณะ Runtime` |

### Rust Example

```rust
  fn main() {
    let name = String::from("Rust");
    print_name(&name);
    println!("{}", name);
  }

  fn print_name(name: &String) {
    println!("{}", name);
  }
```

### `[Other Language]` Example

```java
  public class Main {
      static void printName(String name) {
        System.out.println(name);
      }

      public static void main(String[] args) {
        String name = "Java"; printName(name);
        System.out.println(name);
      }
  }
```

### Analysis

Rust และ Java เป็นภาษาแบบ Statically Typed เหมือนกัน แต่แตกต่างกันชัดเจนด้านการจัดการ Memory
Rust ใช้ Ownership และ Borrowing เพื่อตรวจสอบและจัดการ Memory ตั้งแต่ Compile Time โดยไม่ใช้ Garbage Collector ส่วน Java ใช้ Garbage Collector จัดการ Memory ขณะ Runtime

---

**Comparison Language:** `C++`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `ใช้ { } กำหนด block และ ; ปิดท้าย statement ประกาศตัวแปรด้วย let` | `ใช้ { } กำหนด block และ ; ปิดท้าย statement ระบุชนิดข้อมูลตอนประกาศ` |
| Semantics / Behavior | `ตรวจสอบข้อผิดพลาดตั้งแต่ Compile Time` | `Programmer ต้องระมัดระวังข้อผิดพลาดเกี่ยวกับ Memory ด้วยตนเอง` |
| Type System | `Statically Typed และ Strongly Typed` | `Statically Typed และรองรับ Type Inference ผ่าน auto` |
| Memory Management | `Ownership, Borrowing, Lifetimes และไม่ใช้ Garbage Collector` | `รองรับทั้ง Automatic Storage, RAII, Smart Pointers และการจัดการ Dynamic Memory โดยตรง` |
| Safety | `Memory Safety และ Thread Safety โดย compiler ตรวจสอบ ownership, borrowing และ lifetime ช่วยป้องกันปัญหา` | `มีความยืดหยุ่นสูง แต่การใช้ Raw Pointer หรือจัดการ Memory ไม่ถูกต้องอาจทำให้เกิด Dangling Pointer, Use-after-free หรือ Memory Leak ได้` |

### Rust Example

```rust
  fn main() {
    let name = String::from("Rust");
    print_name(&name);
    println!("{}", name);
  }

  fn print_name(name: &String) {
    println!("{}", name);
  }
```

### `[Other Language]` Example

```c++
  #include <iostream>
  #include <string>
  using namespace std;

  void printName(const string& name) {
    cout << name << endl;
  }

  int main() {
    string name = "C++";
    printName(name); cout << name << endl;
    return 0;
  }
```

### Analysis

Rust และ C++ เป็นภาษาที่เน้น ประสิทธิภาพและการควบคุมทรัพยากร เช่นเดียวกัน แต่มีแนวทางด้าน Memory Safety แตกต่างกัน
Rust ใช้ Ownership, Borrowing และ Lifetime ให้ Compiler ตรวจสอบความปลอดภัยของ Memory ตั้งแต่ Compile Time ส่วน C++ ให้อิสระแก่ Programmer ในการจัดการ Memory และ Pointer มากกว่า จึงมีความยืดหยุ่นสูง แต่ต้องระมัดระวังข้อผิดพลาดด้าน Memory มากกว่า

---


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

`Concept + Short Code Illustration (ช่วยทำในส่วนเนื้อหา เเต่เขาถอนไไปเเล้ว)`

**Member 2**

`Detailed Code + Live Demo`

**Member 3**

`Rust vs Other Language + PPL Analysis`

**Member 4**

`Exercises + Common Mistakes + Challenge`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `https://www.w3schools.com/rust/rust_functions.php`
2. `https://www.geeksforgeeks.org/blogs/rust-vs-python/`
3. `https://youtu.be/g2qJh-7AiW4?si=fOgnEv-wkoUpePpO`
4. `https://webserv.cp.su.ac.th/lecturer/pinyotae/compro1/labs/lab_problem_set.pdf`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `Chat` | `ใช้เพื่อช่วยร่างโค้ด และเปรียบเทียบเนื้อหากับเว็บอื่น ๆ` | `นำโค้ดไปทดลองรันและดูผลลัพธ์ รวมถึงตรวจสอบว่าตรงกับโจทย์และไม่มีข้อผิดพาด` |
| `Gemini` | `เกี่ยวกับเรียนเนื้อหาเกี่ยวกับ ภาษา rust เเละ การใช้ git` | `เปิดเทียบกับ website ต่างที่มีการสอน syntax เเละ ทดลอง` |
| `cloud AI` | `ใช้เพื่อช่วยอธิบาย concept ในเรื่องการเขียน function ของภาษา rust ` | `เปิดเทียบกับ website อื่น ๆ ที่เกี่ยวข้อง ทดลองเขียนและรันเพื่อดูผลลัพธ์` |

### Declaration

- [✓] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [✓] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [✓] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [✓] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`ใช้ AI ในการช่วยค้นหาข้อมูลร่วมกัน เปรียบเทียบเนื้อหากับเว็บไซต์อื่น ๆ และใช้ออกแบบโค้ดตัวอย่าง และทดลองรันพร้อมดูผลลัพธ์ `

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 `ถอน`| `---------` | `---------` | `---------`| `---------` | `------` |
| Member 2 | `[3]` | `[8]` | `[1]` | `[3]` | `[Detailed Code + Live Demo ]` |
| Member 3 | `[4]` | `[8]` | `[1]` | `[4]` | `[Rust vs Other Language + PPL Analysis]` |
| Member 4 | `[2]` | `[1]` | `[1]` | `[2]` | `[Exercises + Common Mistakes + Challenge ]` |

### Teamwork Reflection

**How did your team collaborate?**

`นัดประชุมหาวัน Deadline วันส่งงาน สมาชิกทุกคนทำงานตามหัวข้อที่ได้รับมอบหมายตามส่วนที่ได้รับผิดชอบ โดยแต่ละคนทำงานบน Branch ของตัวเอง เมื่อทำงานเสร็จจะ commit และ Push ขึ้น GitHub แล้วสร้าง Pull Request เพื่อให้สมาชิกตรวจสอบโค้ด ก่อนที่จะ Merge เข้าสู่ Branch Main ส่วนของสไลด์ Presentation แชร์ลิงก์ไฟล์ Canva ให้สมาชิกคนอื่นเข้าไปทำงานร่วมกัน แต่ละคนรับผิดชอบเนื้อหาในส่วนของตัวเอง ช่วยกันตกแต่งและตรวจเช็คความเรียบร้อยก่อนส่งงาน
`

**Problems encountered**

`เกิด Conflict Request`

**How did you solve them?**

`ให้สมาชิกที่ต้อง Pull Request ลบ Branch เก่าแล้วสร้าง Branch ใหม่ และ commit ใหม่ แล้วถึงจะส่ง Pull Request ไปใน Main`

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

**Repository:** `https://github.com/670710121/rust_function.git`

**Chapter Path:** `04-functions/`

**Final PR:** `#30`

**Submitted by:** `04`

**Date:** `2026-10-03`









