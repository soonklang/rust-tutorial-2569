# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม
> **Topic No.:** `8`
> **Topic Name:** `[Structs & Methods]`
> **Group No.:** `8`

---

## 1. Members

| #   | Name                    | Student ID    | GitHub Username | Main Responsibility          |
| --- | ----------------------- | ------------- | --------------- | ---------------------------- |
| 1   | `[นายภัทรดนัย คนชม]` | `[670710140]` | `@[670710140]`   | Concept + Code               |
| 2   | `[นายพชรพล อาจม่วง]`     | `[670710138]` | `@[670710138]`  | Code + Demo                  |
| 3   | `[นายพิชัยพร พลบเนียม]`    | `[670710139]` | `@[670710139]`   | Rust vs Other Language + PPL |
| 4   | `[นายภัทรดนัย คนชม]`      | `[670710140]` | `@[670710140]`  | Exercises + Common Mistakes  |

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

`[Structs & Methods ในภาษา Rust ใช้สำหรับจัดกลุ่มข้อมูลและกำหนดการทำงานของข้อมูล โดย struct ใช้เก็บข้อมูลที่เกี่ยวข้องกัน และ method ใช้จัดการหรือทำงานกับข้อมูลนั้น ช่วยให้โค้ดเป็นระเบียบ อ่านง่าย และจัดการข้อมูลได้สะดวกขึ้น]`

---

## 4. Key Concepts

### 4.1 `[Structs and Fields]`

`[Struct คือ ประเภทของข้อมูลที่เราสามารถใช้เพื่อจัดกลุ่มของข้อมูลจำนวนหนึ่งที่เกี่ยวข้องกันมาจัดรูปเป็นformat โดยข้อมูลย่อยเหล่านั้นจะถูกเรียกว่า field]`

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
    is_available: bool,
}

fn main() {
    let my_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik"),
        pages: 552,
        is_available: true,
    };

    println!("Book: {} by {}", my_book.title, my_book.author);
    println!("Pages: {}, Available: {}", my_book.pages, my_book.is_available);
}

```

`[จากโค้ดด้านบนจะเห็นว่าเราสร้าง Book ที่มีชื่อ field และชนิดข้อมูลของมัน (u32 คือ Unsigned 32-bit Integer หรือก็คือจำนวนต็มบวกที่ไม่เกิน32bit) ต่อมาในส่วนของ main จะเป็นการกำหนดค่าลงใน struct ตามรูปแบบที่สร้างไว้ และสุดท้ายในส่วนของ println เป็นการ print ค่าออกมาทาง output โดยจะมี {} เป็นตัวกำหนดตำแหน่งแทนค่า]`

---

### 4.2 `[Modifying Structs]`

`[โดยปกติแล้ว structs ในภาษา Rust จะเปลี่ยนแปลงค่าภายใน fields ไม่ได้แต่ถ้าเราต้องการที่จะเปลี่ยนแปลงค่าตัวแปรนั้นๆต้องใช้คำสั่ง mut]`

```rust
struct User {
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: String) -> User {
    User {
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(String::from("rust_coder"));
    user1.active = false;

    let user2 = User {
        username: String::from("new_user"),
        ..user1 
    };

    println!("User2 active state: {}", user2.active);
}

```

`[ส่วนของ fn build_user เป็นการสร้าง function และ return ค่ากลับไปเป็น User หรือก็คือตัว struct ที่เราสร้างไว้ในตอนแรก ต่อมาในส่วนของ main ตัวคำว่า mut คือคำสั่ง Mutable ทำหน้าที่อนุญาติให้เราแก้ค่าที่ถูกตั้งไปแล้วใน struct ได้ ส่วน ..user1 เป็นคำสั่งที่บอกให้ field ส่วนที่เหลือที่เราไม่ได้กำหนดค่าให้ user2 ไปเอามาจาก user1 ได้เลย]`

---

### 4.3 `[Tuple Structs & Unit-Like Structs]`

`[Tuple Struct : มีแต่ขื่อชนิด struct ไม่มีชื่อของ fields || Unit-Like Struct : มีแต่ชื่อ struct ไม่มี fields]`

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);

    let _subject = AlwaysEqual;
}

```

`[ในส่วนแรก Color,Point คือการประกาศ struct แบบที่ไม่ได้กำหนดชื่อตัวแปรใน fields เรียกว่า Tuple Struct โดยจะมีแค่ชนิดตัวแปรแล้วเวลาจะเรียกใช้ก็ค่อยสร้างและใส่ค่่าลงไปทีเดียว ส่วนต่อมา AlwaysEqual เป็นการประกาศ struct ที่ไม่มี fields ใดๆเลยโดย struct รูปแบบนี้เรียกว่า Unit-Like Struct เอาไว้สร้าง type ขึ้นมาเพื่อนำไปสืบทอดหรือกำหนดกลุ่มชนิดข้อมูลบางอย่าง]`

---

### 4.4 `[Methods and the self Parameter`

`[Methods คือ fnc ที่ถูกประกาศภายใต้เนื้อหาของ struct จะต่างจาก fnc ปกติ ตรงที่ต้องใช้ self มาเป็น parameter ตัวแรกในรูปแบบต่างๆเสมอ]`

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.", rect.area());
}

```

`[impl Rectangle คือการ implements ในรูปของภาษา rust มีไว้สำหรับเขียน methods ส่วน self จะทำหน้าที่คล้ายๆ this ใน java แต่จะต่างกันตรงที่การใส่ & ด้านหน้า self เป็นการยืมมาเพียงค่าไม่ได้ยึด ownership มาด้วย]`

---

### 4.5 `[Associated Functions]`

`[Associated Functions เป็น fnc ที่ถูกสร้างไว้ภายใต้ impl ที่ไม่มี parameter เป็น self เนื่องจากว่ามันไม่ได้เชื่อมโยงกับ object หรือ instance ตัวใดตัวหนึ่ง]`

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(r: f64) -> Circle {
        Circle { radius: r }
    }
}

fn main() {
    let my_circle = Circle::new(5.5);

    println!("Created a circle with radius: {}", my_circle.radius);
}

```

`[ในส่วนของ fn new จะคล้ายๆกับการสร้าง constructor ใน java จะสักเกตุได้จากการที่ parameter ไม่มี self เป็นตัวบ่งบอกว่า fnc นี้เป็น Associated Functions ซึ่งจะไม่ได้ผูกกับ obj ไหนแต่จะผูกกับโครงสร้าง Circle แทน]`

---

## 5. Important Syntax / Rules

| Syntax / Rule   | Meaning      | Example    |
| --------------- | ------------ | ---------- |
| `[struct Name { field: Type }]` | `[Classic Struct: ประกาศโครงสร้างข้อมูลแบบมีชื่อฟิลด์ชัดเจน]` | `[struct Rectangle { width: u32, height: u32 }]` |
| `[struct Name(Type, Type);]` | `[Tuple Struct: โครงสร้างข้อมูลที่ไม่ระบุชื่อฟิลด์ แต่ใช้อ้างอิงด้วยดัชนีตำแหน่ง]` | `[struct Point(i32, i32);]` |
| `[struct Name;]` | `[Unit-like Struct: โครงสร้างข้อมูลที่ไม่มีฟิลด์ นิยมใช้จัดกลุ่ม behavior หรือ trait]` | `[struct AlwaysEqual;]` |
| `[impl Name { ... }]` | `[Implementation Block: บล็อกสำหรับนิยาม Method และ Associated Function ให้กับ Struct]` | `[impl Rectangle { ... }]` |
| `[&self]` | `[Immutable Borrow: Method อ่านข้อมูลได้อย่างเดียว ไม่สามารถแก้ไขค่าใน Struct ได้]` | `[fn area(&self) -> u32 { self.width * self.height }]` |
| `[&mut self]` | `[Mutable Borrow: Method สามารถแก้ไขข้อมูลใน Struct ได้]` | `[fn scale(&mut self, factor: u32) { self.width *= factor; }]` |
| `[self]` | `[Take Ownership: Method รับยึด Ownership ไป ทำให้ instance เดิมใช้ต่อไม่ได้]` | `[fn destroy(self) { ... }]` |
| `[Name::fn_name()]` | `[Associated Function: ฟังก์ชันภายใน impl ที่ไม่มี self (มักใช้ทำ Constructor)]` | `[let r = Rectangle::new(10, 20);]` |

### Important Rules

1. `[การแก้ไขค่าต้องประกาศ mut (Mutability) หากต้องการแก้ไขค่าในฟิลด์ หรือเรียกใช้ Method ที่รับพารามิเตอร์เป็น &mut self ตัวแปรที่สร้าง instance นั้นจะต้องถูกประกาศด้วย let mut เสมอ]`

2. `[การย้าย Ownership ด้วย self หาก Method รับพารามิเตอร์เป็น self (ไม่มี &) Ownership ของ instance จะถูกย้าย (Move) เข้าไปใน Method นั้นทันที และจะถูกทำลายเมื่อจบ Method ทำให้ไม่สามารถนำตัวแปรเดิมมาใช้งานได้อีก]`
   
3. `[กฎความเป็นเอกสิทธิ์ของการอ้างอิง (Borrowing Rules) ภาษา Rust ไม่อนุญาตให้สร้าง &mut self (Mutable Reference) ซ้ำกันในเวลาเดียวกัน และไม่สามารถมี &mut self ร่วมกับ &self (Immutable Reference) ในขอบเขตเวลาเดียวกันได้]`

4. `[Associated Functions ต้องเรียกผ่าน :: ฟังก์ชันในบล็อก impl ที่ไม่มีพารามิเตอร์ self (เช่น new()) ถือเป็น Associated Function ไม่ใช่ Method จึงต้องเรียกใช้ผ่านชื่อ Struct ร่วมกับสัญลักษณ์ :: ไม่ใช่การใช้จุด .]`

5. `[สามารถแยกบล็อก impl ออกเป็นหลายบล็อกได้ (Multiple impl blocks) ภาษา Rust อนุญาตให้เขียนบล็อก impl แยกกันหลายๆ บล็อกสำหรับ Struct เดียวกันได้ เพื่อช่วยในการจัดหมวดหมู่โค้ดให้เป็นระเบียบ]`

6. `[ความปลอดภัยในการเข้าถึงข้อมูล (Field Visibility) ฟิลด์ภายใน Struct และตัว Method จะเป็น Private โดยสัญชาตญาณ (Default) หากต้องการให้โมดูลอื่นเข้าถึงได้ จะต้องเติมคีย์เวิร์ด pub ข้างหน้าฟิลด์หรือ Method นั้นๆ]`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ การสร้าง Struct และการเรียกระหว่าง Associated Function กับ Methods]`

**Purpose:** `[สาธิตการประกาศ Struct, การสร้าง Associated Function (new), การใช้ Method อ่าน/แก้ไขข้อมูล (&self, &mut self), และการใช้ Method ที่ย้าย Ownership (self)]`

```rust
#[derive(Debug)]
struct UserAccount {
    username: String,
    balance: f64,
    active: bool,
}

impl UserAccount {
    fn new(username: &str, initial_balance: f64) -> Self {
        Self {
            username: username.to_string(),
            balance: initial_balance,
            active: true,
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("ฝากเงินสำเร็จ: +${:.2} | ยอดคงเหลือปัจจุบัน: ${:.2}", amount, self.balance);
        } else {
            println!("จำนวนเงินฝากต้องมากกว่า 0");
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            Err("จำนวนเงินถอนต้องมากกว่า 0".to_string())
        } else if amount > self.balance {
            Err("ยอดเงินคงเหลือไม่เพียงพอ".to_string())
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }

    fn close_account(self) -> f64 {
        println!("ปิดบัญชีของ {} สำเร็จ คืนเงินคงเหลือ: ${:.2}", self.username, self.balance);
        self.balance
    }
}

fn main() {
    let mut my_account = UserAccount::new("Alice", 100.0);
    println!("เริ่มต้นบัญชี: {:?}", my_account);

    println!("ยอดเงินเริ่มต้น: ${:.2}", my_account.get_balance());

    my_account.deposit(50.0);

    match my_account.withdraw(30.0) {
        Ok(new_balance) => println!("ถอนเงินสำเร็จ ยอดคงเหลือ: ${:.2}", new_balance),
        Err(e) => println!("เกิดข้อผิดพลาด: {}", e),
    }

    let refunded = my_account.close_account();
    println!("เงินคืนเข้ามือ: ${:.2}", refunded);
}
```

**Expected Output**

```text
เริ่มต้นบัญชี: UserAccount { username: "Alice", balance: 100.0, active: true }
ยอดเงินเริ่มต้น: $100.00
ฝากเงินสำเร็จ: +$50.00 | ยอดคงเหลือปัจจุบัน: $150.00
ถอนเงินสำเร็จ ยอดคงเหลือ: $120.00
ปิดบัญชีของ Alice สำเร็จ คืนเงินคงเหลือ: $120.00
เงินคืนเข้ามือ: $120.00
```

**Explanation**

- **`struct UserAccount`**: การนิยามโครงสร้างข้อมูลเพื่อเก็บสเตตของผู้ใช้ ประกอบด้วย `username`, `balance`, และ `active`
- **`#[derive(Debug)]`**: ช่วยให้สามารถพิมพ์ค่าของ Struct ออกมาทางหน้าจอโดยใช้ฟอร์แมต `{:?}` เพื่อการ Debugging ได้สะดวก
- **`impl UserAccount`**: บล็อกสำหรับเขียนฟังก์ชันเฉพาะของ Struct
- **`fn new(...)`**: เป็น **Associated Function** (ไม่ได้รับ `self`) ทำหน้าที่เป็น Constructor เพื่อสร้าง Instance ใหม่ของ Struct
- **`fn get_balance(&self)`**: เป็น **Immutable Method** ยืมอ่านค่าฟิลด์ `balance` โดยไม่มีการเปลี่ยนแปลงข้อมูลภายใน
- **`fn deposit(&mut self, ...)` & `fn withdraw(&mut self, ...)`**: เป็น **Mutable Methods** ยืมสิทธิ์เข้าถึงเพื่อปรับแต่งแก้ไขค่า `balance` ภายใน Struct
- **`fn close_account(self)`**: เป็น **Ownership Consumer Method** โดยรับ `self` ไปตรง ๆ ทำให้ Instance นั้นถูกย้าย Ownership (Move) และย้อนกลับมาใช้งานอีกไม่ได้หลังจบคำสั่ง เพื่อความปลอดภัยด้านหน่วยความจำ

---

### Example 2 — `[ชื่อ การคำนวณพื้นที่และการปรับขนาดรูปทรงสี่เหลี่ยม (Rectangle Structure)]`

**Purpose:** `[สาธิตการใช้ Struct เก็บขนาดวัตถุ, การสร้าง Associated Function (new), การใช้ Method คำนวณค่า (&self), การใช้ Method ปรับเปลี่ยนข้อมูลภายใน (&mut self), และการย้าย Ownership เพื่อคืนค่ากลับ (self)]`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    fn square_up(self) -> Self {
        let max_side = if self.width > self.height {
            self.width
        } else {
            self.height
        };
        Self {
            width: max_side,
            height: max_side,
        }
    }
}

fn main() {
    let mut rect = Rectangle::new(10, 5);
    println!("เริ่มต้นรูปทรง: {:?}", rect);

    println!("พื้นที่รูปทรง: {} ตารางหน่วย", rect.area());

    rect.scale(2);
    println!("หลังขยายสเกล 2 เท่า: {:?}", rect);
    println!("พื้นที่ใหม่: {} ตารางหน่วย", rect.area());

    let square = rect.square_up();
    println!("ปรับรูปทรงเป็นจัตุรัส: {:?}", square);
    println!("พื้นที่จัตุรัส: {} ตารางหน่วย", square.area());
}
```

**Expected Output**

```text
เริ่มต้นรูปทรง: Rectangle { width: 10, height: 5 }
พื้นที่รูปทรง: 50 ตารางหน่วย
หลังขยายสเกล 2 เท่า: Rectangle { width: 20, height: 10 }
พื้นที่ใหม่: 200 ตารางหน่วย
ปรับรูปทรงเป็นจัตุรัส: Rectangle { width: 20, height: 20 }
พื้นที่จัตุรัส: 400 ตารางหน่วย
```

**Explanation**

- **`struct Rectangle`**: การนิยามโครงสร้างข้อมูลสำหรับเก็บขนาดสี่เหลี่ยม โดยมีฟิลด์ width และ height เป็นชนิดข้อมูล u32

- **`#[derive(Debug)]`**: คำสั่งอนุญาตให้แสดงผลข้อมูลใน Struct ออกทางหน้าจอผ่านฟอร์แมต {:?} ได้

- **`impl Rectangle`**: บล็อกสำหรับนิยามการทำงานทั้งหมดของ Struct

- **`fn new(...)`**: เป็น Associated Function ทำหน้าที่เป็น Constructor สำหรับสร้าง Instance ใหม่

- **`fn area(&self)`**: เป็น Immutable Method ดึงค่า width และ height จาก Instance มาคำนวณพื้นที่โดยไม่มีการแก้ไขข้อมูล

- **`fn scale(&mut self, ...)`**: เป็น Mutable Method ทำการปรับเปลี่ยนค่าฟิลด์ภายใน Instance เดิมโดยการคูณขยายขนาด

- **`fn square_up(self)`**: เป็น Ownership Consumer Method รับ self เพื่อทำลาย/แปลง Instance เดิม แล้วส่งคืน Instance ของสี่เหลี่ยมจัตุรัสรูปใหม่กลับไป

---

## 7. Common Mistakes

### Mistake 1 — `[ลืม &self ใน Method]`

**Problem**

`[สร้าง Method ที่ต้องการเข้าถึงข้อมูลของ Struct แต่ไม่ได้ใส่ &self]`

**Incorrect Code**

```rust
struct Student {
    name: String,
    age: i32,
}
    impl Student {
        fn show_info() {
            println!("ชื่อ: {}", self.name);
            println!("อายุ: {}", self.age);
        }
    }
fn main() {
    // สร้าง instance ของ Student
    let student = Student {
        name: String::from("สมชาย ใจดี"),
        age: 20,
    };
    student.show_info();
}
```

**Correct Code**

```rust
struct Student {
    name: String,
    age: i32,
}
    impl Student {
        fn show_info(&self) {
            println!("ชื่อ: {}", self.name);
            println!("อายุ: {}", self.age);
        }
    }
fn main() {
    // สร้าง instance ของ Student
    let student = Student {
        name: String::from("สมชาย ใจดี"),
        age: 20,
    };
    student.show_info();
}
```

**Why?**

`[self หมายถึง ข้อมูลของ Struct ตัวที่กำลังเรียก Method เมื่อเราเขียน:`

`student.show_info();`

`Rust จะส่ง student เข้ามาให้ Method ผ่าน self`

`ดังนั้นถ้าต้องการใช้:`

`self.name`
`self.age`

`เราต้องประกาศ:`

`fn show_info(&self)`

`&self หมายถึง Method สามารถ อ่านข้อมูลของ Struct ได้ โดยไม่ต้องเป็นเจ้าของข้อมูล]`
---

### Mistake 2 — `[ลืม mut เมื่อ Method ต้องการแก้ไขข้อมูล]`

**Problem**

`[สร้าง Method ที่ต้องการเปลี่ยนแปลงข้อมูลภายใน Struct แต่ไม่ได้ใช้ &mut self หรือไม่ได้ประกาศตัวแปรด้วย mut]`

**Incorrect Code**

```rust
struct Student {
    name: String,
    age: i32,
}
impl Student {
    fn birthday(&mut self) {
        self.age += 1;
    }
}
fn main() {
    let student = Student { // <-- ไม่มี mut บอกว่า Method นี้ต้องการแก้ไขข้อมูลของ student
        name: "John".to_string(),
        age: 20,
    };
student.birthday(); }

```

**Correct Code**

```rust
struct Student {
    name: String,
    age: i32,
}
impl Student {
    fn birthday(&mut self) {
        self.age += 1;
    }
}
fn main() {
    let mut student = Student { //<-- มี mut อนุญาตให้ student ถูกแก้ไข
        name: "John".to_string(),
        age: 20,
    };
student.birthday(); }
```

**Why?**

`[ถ้า Method ต้องการ แก้ไขข้อมูลของ Struct ต้องใช้ &mut self`

`fn birthday(&mut self)`

`และตัวแปรที่นำไปเรียก Method ก็ต้องประกาศด้วย mut`

`let mut student = Student { ... };`

`จำง่าย ๆ:`

`อ่านข้อมูล ใช้ &self`

`แก้ไขข้อมูล ใช้ &mut self หรือ mut ตัวแปร]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[เก็บข้อมูลนักเรียน]`

**Problem**

`[ให้สร้าง struct Student สำหรับเก็บข้อมูลนักเรียน โดยมีข้อมูลดังนี้`

`- ชื่อ (name) เป็น String`
`- อายุ (age) เป็น i32`

`จากนั้นสร้าง Method ชื่อ show_info() เพื่อแสดงข้อมูลของนักเรียน`

`ผลลัพธ์ที่ต้องการ:`
    `ชื่อ: John`
    `อายุ: 20]`


**Hint**

`[- ใช้ struct Student`
`- สร้าง Method ภายใน impl Student`
`- Method show_info() ให้ใช้ &self`
`- ใน main() ให้สร้าง Student 1 คน แล้วเรียกใช้ show_info()]`


**Solution**

```rust
struct Student {
    name: String,
    age: i32,
}
impl Student {
    fn show_info(&self) {
        println!("ชื่อ: {}", self.name);
        println!("อายุ: {}", self.age);
    }
}
fn main() {
    let student = Student {
        name: "John".to_string(),
        age: 20,
    };
    student.show_info();
}
```

**Explanation**

`[สร้าง struct Student เพื่อเก็บข้อมูลของนักเรียน:`

`struct Student {`
    `name: String,`
    `age: i32,`
`}`

`จากนั้นใช้ impl Student เพื่อสร้าง Method ให้กับ Student`

`impl Student {`
    `fn show_info(&self) {`
        `// ...`
    `}`
`}`

`&self ใช้สำหรับให้ Method เข้าถึงข้อมูลของ Student ตัวที่เรียก Method`

`เช่น:`

`self.name`
`self.age`

`ใน main() สร้าง Student:`

`let student = Student {`
    `name: String::from("John"),`
    `age: 20,`
`};`

`แล้วเรียก Method:`

`student.show_info();]`


---

### Exercise 2 — `[ข้อมูลหนังสือ]`

**Problem**

`[ให้สร้าง struct Book สำหรับเก็บข้อมูลหนังสือ โดยมีข้อมูลดังนี้`

`title เป็น String`
`author เป็น String`
`price เป็น f64`

`จากนั้นสร้าง Method ชื่อ show_info() เพื่อแสดงข้อมูลหนังสือ`

`ผลลัพธ์ที่ต้องการ:`

`ชื่อหนังสือ: Rust Programming`
`ผู้เขียน: John`
`ราคา: 599]`


**Hint**

`[- ใช้ struct Book`
`- สร้าง Method ภายใน impl Book`
`- Method show_info() ให้ใช้ &self`
`- ใช้ println!() เพื่อแสดงข้อมูล`
`- ใน main() ให้สร้างหนังสือ 1 เล่ม แล้วเรียก show_info()]`


**Solution**

```rust
struct Book {
    title: String,
    author: String,
    price: f64,
    }
impl Book {
    fn show_info(&self) {
        println!("ชื่อหนังสือ: {}", self.title);
        println!("ผู้เขียน: {}", self.author);
        println!("ราคา: {}", self.price);
        }
}
fn main() {
    let book = Book {
        title: "Rust Programming".to_string(),
        author: "John".to_string(),
        price: 599.0,
        };
    book.show_info();
}
```

**Explanation**

`[สร้าง struct Book เพื่อเก็บข้อมูล 3 อย่าง:`

`struct Book {`
    `title: String,`
    `author: String,`
    `price: f64,`
`}`

`จากนั้นใช้ impl Book เพื่อสร้าง Method:`

`impl Book {`
    `fn show_info(&self) {`
        `// ...`
    `}`
`}`

`&self ทำให้ Method สามารถอ่านข้อมูลของ book ได้ เช่น:`

`self.title`
`self.author`
`self.price`

`ใน main() สร้างหนังสือ:`

`let book = Book {`
    `title: "Rust Programming".to_string(),`
    `author: "John".to_string(),`
    `price: 599.0,`
`};`

`แล้วเรียก Method:`

`book.show_info();`

`ผลลัพธ์:`

`ชื่อหนังสือ: Rust Programming`
`ผู้เขียน: John`
`ราคา: 599]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

ในภาษา Rust ไวยากรณ์ของ **Structs & Methods** สะท้อนแนวคิด **Separation of Concerns (การแยกความรับผิดชอบ)** โดยแยกการนิยามโครงสร้างข้อมูลออกจากพฤติกรรมการทำงานอย่างเด็ดขาด:

1. **Separation of Definition and Implementation:**
   * โครงสร้างข้อมูลถูกประกาศด้วยคีย์เวิร์ด `struct` ทำหน้าที่เป็นเพียงพิมพ์เขียวของหน่วยความจำ (Memory Layout)
   * เมธอดและการทำงานทั้งหมดถูกแยกไปเขียนไว้ในบล็อก `impl` (Implementation) ต่างจากภาษา OOP ทั่วไป (เช่น Java, C++, Python) ที่รวมฟิลด์และเมธอดไว้ในบล็อกคลาสเดียวกัน
2. **Explicit Receiver Parameter:**
   * ไวยากรณ์ของ Rust บังคับให้เมธอดต้องประกาศพารามิเตอร์ตัวแรกเป็น `&self`, `&mut self`, หรือ `self` อย่างชัดเจน เพื่อบอกสิทธิ์การเข้าถึงหน่วยความจำ
   * หากไม่มี `self` ในพารามิเตอร์ตัวแรก จะถือว่าเป็น **Associated Function** (ทำงานเหมือน Static Method หรือ Constructor) และเรียกใช้งานผ่านโอเปอเรเตอร์ Scope Resolution (`::`)

```rust
// นิยามเฉพาะโครงสร้างข้อมูล (Data Abstraction)
struct UserAccount {
    username: String,
    balance: f64,
}

// นิยามพฤติกรรมและการทำงาน (Behavior Implementation)
impl UserAccount {
    // 1. Associated Function (Constructor): ไม่มี self เรียกด้วย UserAccount::new(...)
    fn new(username: &str, initial_balance: f64) -> Self {
        Self {
            username: username.to_string(),
            balance: initial_balance,
        }
    }

    // 2. Method ที่มี Explicit Receiver (&self): ขอยืมอ่านข้อมูล
    fn get_balance(&self) -> f64 {
        self.balance
    }
}
```

### 9.2 Semantics

ความหมายเชิงพฤติกรรม (Semantics) ของการเรียกใช้เมธอดใน Rust ถูกผูกมัดเข้ากับระบบ **Ownership & Borrowing** โดยตรงผ่านชนิดของ Receiver (`self`):

1. **Borrowing Semantics (`&self` และ `&mut self`):**
   * **`&self` (Shared Borrow):** เมธอดมีพฤติกรรมขอ **ยืมอ่านข้อมูล** ได้พร้อมกันหลายจุด (Aliasing) โดยห้ามแก้ไขค่าฟิลด์ใดๆ ภายใน Struct การันตีความปลอดภัยในการเข้าถึงพร้อมกัน
   * **`&mut self` (Exclusive Borrow):** เมธอดมีพฤติกรรมขอ **ยืมแก้ไขข้อมูล** โดยคอมไพเลอร์จะบังคับกฎ *Aliasing XOR Mutability* ว่าขณะเรียกเมธอดนี้ จะต้องไม่มีพอยน์เตอร์หรือการอ้างอิงอื่นชี้มาที่ Struct นี้พร้อมกันเด็ดขาด จึงช่วยป้องกันปัญหา Data Race ใน Safe Rust
2. **Move Semantics (`self` - Consuming Method):**
   * **`self` (Ownership Transfer):** เมื่อเมธอดรับ `self` ตรงๆ จะเกิดการโอนย้ายกรรมสิทธิ์ (Move) เข้ามาในเมธอด และเมื่อเมธอดทำงานเสร็จสิ้น อินสแตนซ์เดิมจะถูกทำลายคืนทรัพยากรทันทีตามหลัก RAII ทำให้ตัวแปรต้นทางไม่สามารถถูกนำมาเรียกใช้ซ้ำได้อีก (เช่น เมธอด `close_account(self)`) ช่วยสร้าง State Machine ที่ป้องกัน Use-After-Free ได้ในระดับภาษา
3. **Syntactic Sugar (Automatic Referencing and Dereferencing):**
   * ในการเรียกใช้งาน เช่น `account.deposit(50.0)` ภาษา Rust มีกลไก Dot Operator Dereferencing ช่วยแปลงเป็น `UserAccount::deposit(&mut account, 50.0)` ให้อัตโนมัติ โปรแกรมเมอร์จึงไม่ต้องเขียนเครื่องหมาย `&` หรือ `*` ด้วยตนเองเหมือนในภาษา C/C++


### 9.3 Type System

ในมิติของ **ระบบชนิดข้อมูล (Type System)** โครงสร้าง Structs & Methods ใน Rust มีคุณลักษณะเชิงทฤษฎีที่สำคัญ 4 ประการ:

1. **Nominal Typing (ระบบชนิดข้อมูลแบบระบุชื่อ):**
   * Rust ใช้ระบบ **Nominal Typing** ในการตรวจสอบความถูกต้องของ Structs ไม่ใช่ Structural Typing
   * ตัวอย่างเช่น หากเรานิยาม `struct UserAccount { username: String, balance: f64 }` และ `struct CompanyAccount { username: String, balance: f64 }` แม้ทั้งสองจะมีฟิลด์ชนิดเดียวกันและลำดับเดียวกันเป๊ะ แต่ Type System จะถือว่าเป็นคนละชนิดข้อมูลกันโดยสิ้นเชิง ไม่สามารถนำมาใช้งานแทนกันหรือกำหนดค่าข้ามกันได้ ช่วยป้องกันข้อผิดพลาดเชิงความหมาย (Semantic Errors) ตั้งแต่ขั้นตอน Compile-time
2. **Product Type ใน Algebraic Data Types (ADTs):**
   * ในทางทฤษฎีภาษาโปรแกรม Struct ทำหน้าที่เป็น **Product Type ($A \times B$)** ซึ่งหมายความว่าเซตของสถานะที่เป็นไปได้ของ Struct เกิดจากผลคูณคาร์ทีเซียน (Cartesian Product) ของโดเมนข้อมูลของฟิลด์ทั้งหมดรวมกัน (เช่น ใน `UserAccount` คือ โดเมนของ `username` $\times$ โดเมนของ `balance`)
   * แตกต่างจาก `enum` ของ Rust ที่ทำหน้าที่เป็น **Sum Type ($A + B$)** ที่เก็บค่าได้เพียงตัวเลือกใดตัวเลือกหนึ่ง ณ ขณะหนึ่ง
3. **Affine Type System (การบังคับใช้ Ownership ผ่าน Receiver Types):**
   * ระบบ Type ของ Rust ทำงานบนพื้นฐานของ **Affine Substructural Type System** ซึ่งกำหนดว่าค่าของข้อมูลสามารถถูกใช้งานได้ *อย่างมากที่สุดหนึ่งครั้ง (at most once)*
   * ชนิดของพารามิเตอร์ `self` ในบล็อก `impl` ทำหน้าที่เป็นตัวบังคับใช้กฎนี้:
     * หากเมธอดรับ `self` (Move): Type System จะเปลี่ยนสถานะของตัวแปรต้นทางเป็น "ถูกใช้งานแล้ว (Moved)" และบล็อกไม่ให้โค้ดส่วนอื่นนำตัวแปรเดิมมาอ้างอิงซ้ำ
     * หากเมธอดรับ `&self` หรือ `&mut self` (Borrow): Type System จะตรวจสอบ Lifetime ให้การยืมคืนเสร็จสิ้นก่อนที่เจ้าของเดิมจะหมดอายุ
   * ทำให้การันตีเรื่อง **Memory Safety** และ **Data-Race Freedom** ได้อย่างเบ็ดเสร็จผ่าน Type Checker โดยไม่ต้องพึ่งพา Runtime Garbage Collector
4. **Traits & Ad-hoc Polymorphism (แทนที่ Class Inheritance):**
   * Rust ไม่สนับสนุนการสืบทอดคลาส (Subtyping Inheritance) แต่ใช้ระบบ **Traits** (เทียบเคียงได้กับ Typeclasses ในภาษาเชิงฟังก์ชัน) เพื่อกำหนดและแชร์พฤติกรรมร่วม
   * **Static Dispatch (Monomorphization):** เมื่อเขียนฟังก์ชันหรือเมธอดที่ใช้ Generic Bound เช่น `fn print_account<T: Summary>(acc: &T)` คอมไพเลอร์จะสร้างสำเนาโค้ดเฉพาะสำหรับแต่ละชนิดข้อมูลขึ้นมาตอนคอมไพล์ ทำให้เรียกใช้ได้เร็วเทียบเท่าฟังก์ชันตรงๆ โดยไร้ Overhead
   * **Dynamic Dispatch (`dyn Trait`):** หากต้องการ Polymorphism แบบพลวัตตอนรันไทม์ Rust จะบังคับให้ใช้ผ่าน **Fat Pointer** (พอยน์เตอร์ชี้ข้อมูล 8 ไบต์ + พอยน์เตอร์ชี้ VTable 8 ไบต์) ทำให้เห็นต้นทุนหน่วยความจำอย่างโปร่งใสในระดับ Type System

---

### 9.4 Memory / Resource Management

ในเชิงภาษาระบบ (Systems Programming Language) โครงสร้าง Structs & Methods ของ Rust มีบทบาทสำคัญในการบริหารจัดการหน่วยความจำและทรัพยากรของเครื่อง ดังนี้:

1. **Stack-First Allocation (Value Semantics):**
   * Struct ในภาษา Rust มีคุณสมบัติเป็น **Value Type** เมื่อประกาศตัวแปร เช่น `let my_account = UserAccount::new("Alice", 100.0);` ตัวอินสแตนซ์ของ Struct จะถูกจัดสรรพื้นที่บน **Stack** เป็นค่าเริ่มต้นโดยตรง (เว้นแต่จะระบุให้จัดเก็บบน Heap ผ่าน Smart Pointer เช่น `Box<T>`)
   * ทำให้เข้าถึงข้อมูลได้เร็วระดับคำสั่งฮาร์ดแวร์ (CPU Register/Cache) และข้อมูลจะถูกนำออกจาก Stack ทันทีที่จบขอบเขตฟังก์ชันโดยไม่มีภาระการกวาดหน่วยความจำ แตกต่างจากภาษา OOP อย่าง Java หรือ Python ที่โดยทั่วไปอ็อบเจกต์จะถูกจัดสรรบน **Heap Memory** (Reference Semantics)
2. **Zero Object Header Overhead:**
   * อ็อบเจกต์ในภาษา Java มีข้อมูลแฝงที่เรียกว่า Object Header (ขนาดประมาณ 12–16 ไบต์) สำหรับเก็บคลาสพอยน์เตอร์และข้อมูลการล็อก ซึ่งกินพื้นที่หน่วยความจำมากกว่าข้อมูลจริง
   * ในทางตรงกันข้าม Struct ของ Rust มีคุณสมบัติ **Zero-Metadata Overhead** คือไม่มี Object Header หรือข้อมูลเสริมแอบแฝง ขนาดของ `UserAccount` ในหน่วยความจำจึงมีค่าเท่ากับผลรวมของขนาดฟิลด์จริงเพียวๆ (String ขนาด 24 ไบต์ + f64 ขนาด 8 ไบต์ = 32 ไบต์บนระบบ 64-bit) เทียบเท่ากับ Struct ในภาษา C
3. **Field Alignment & Dynamic Reordering:**
   * สถาปัตยกรรม CPU กำหนดให้ข้อมูลต้องวางเรียงตรงตามแนวความกว้างของคำสั่ง (Memory Alignment) ซึ่งในภาษา C หากประกาศฟิลด์สลับไปมาจะเกิดช่องว่างที่สูญเปล่า (**Padding Space**)
   * แต่คอมไพเลอร์ Rust (`rustc`) มีกลไก **Dynamic Field Reordering** คอยจัดเรียงลำดับฟิลด์ภายในหน่วยความจำใหม่ให้อัตโนมัติ เพื่อบีบอัด Padding ให้เหลือศูนย์หรือน้อยที่สุด ช่วยประหยัดพื้นที่ RAM และเพิ่มอัตรา Cache Hit ใน CPU
4. **Deterministic Destruction ผ่าน RAII (`Drop` Trait):**
   * ภาษา Rust ปฏิเสธการใช้ Garbage Collector (GC) แต่ใช้ปรัชญา **RAII (Resource Acquisition Is Initialization)** ควบคู่กับ Trait ชื่อ `Drop`
   * เมื่อ Struct หลุดออกนอกขอบเขตการทำงาน (Scope) คอมไพเลอร์จะแทรกคำสั่งเรียก `Drop::drop(&mut self)` เพื่อคืนทรัพยากรและหน่วยความจำของฟิลด์ทั้งหมดโดยอัตโนมัติ และมี Lifetime ที่คาดเดาได้อย่างแม่นยำ (Deterministic Lifetime) ผ่านกลไก RAII
5. **Method Dispatch Efficiency (ประสิทธิภาพการเรียกใช้เมธอด):**
   * การเรียกเมธอดบน Struct เช่น `my_account.get_balance()` มีรูปแบบเป็น **Direct Function Call** ในระดับ Machine Code เสมือนการเรียกฟังก์ชันธรรมดา
   * รองรับการทำ **Inlining** (นำโค้ดในเมธอดมาแทรกแทนจุดที่เรียกใช้งาน) ทำให้ไม่มีต้นทุนการกระโดดข้ามฟังก์ชัน และไม่มีความหน่วงของตาราง Virtual Method Table (VTable) จึงบรรลุเป้าหมายหลักการ **Zero-Cost Abstraction** อย่างสมบูรณ์

---


### 9.5 Abstraction / Other PPL Concepts

โครงสร้าง **Structs & Methods** ในภาษา Rust เชื่อมโยงกับมิติเชิงกระบวนทัศน์และทฤษฎีภาษาโปรแกรม ดังนี้:

1. **Data Abstraction & Encapsulation:**
   * **Data Abstraction:** ตัว `struct` ทำหน้าที่เป็น Abstract Data Type (ADT) ที่รวบรวมกลุ่มข้อมูลที่สัมพันธ์กันไว้เป็นหน่วยเดียว
   * **Information Hiding:** Rust ไม่ใช้คีย์เวิร์ด Access Modifier ระดับคลาสอย่าง `private`/`protected` แบบ Java/C++ แต่ใช้ระบบ **Module-level Encapsulation** โดยฟิลด์และเมธอดจะเป็น private ตามค่าเริ่มต้น (เข้าถึงได้เฉพาะในโมดูลเดียวกัน) และต้องระบุคีย์เวิร์ด `pub` อย่างชัดเจนหากต้องการเปิดให้ภายนอกเข้าถึง ช่วยลดการรั่วไหลของ Implementation Details
2. **Composition over Inheritance:**
   * ในการออกแบบภาษา OOP ดั้งเดิม การขยายความสามารถมักพึ่งพา **Class Inheritance** ซึ่งนำไปสู่ปัญหาคลาสฐานเปราะบาง (*Fragile Base Class Problem*) และปัญหาเพชรมรณะ (*Diamond Problem*) จากการสืบทอดหลายสาย
   * Rust เลือกตัดระบบ Inheritance ทิ้งไปโดยสิ้นเชิง และใช้หลักการ **Composition over Inheritance** โดยนำ Struct ย่อยมารวมกัน (Composition) และใช้ **Traits** เพื่อแชร์พฤติกรรมร่วม ทำให้ระบบมีความยืดหยุ่นสูงและปลอดภัยกว่า
3. **Data-Driven Design (Multi-paradigm Programming):**
   * การแยกข้อมูล (`struct`) ออกจากพฤติกรรม (`impl`) ทำให้สถาปัตยกรรมของโปรแกรมโน้มเอียงไปทาง **Data-Oriented Design** ซึ่งข้อมูลถูกจัดเรียงเป็นก้อนที่ต่อเนื่องกันในหน่วยความจำ เป็นมิตรกับ CPU Cache และสามารถผสมผสานกระบวนทัศน์แบบ Functional Programming (เช่น Method Chaining, Iterator, Closure) เข้ากับ Imperative Programming ได้อย่างลงตัว
4. **Zero-Cost Abstraction:**
   * การห่อหุ้มตรรกะไว้ใน Struct และ Method ไม่ก่อให้เกิดต้นทุนด้านความเร็วหรือหน่วยความจำในตอนทำงาน (No Runtime Cost) เนื่องจากคอมไพเลอร์ LLVM จะทำการ Optimize และ Inlining เมธอดส่วนใหญ่ให้กลายเป็น Machine Instructions เสมือนเขียนฟังก์ชันระดับภาษา C ตรงๆ

---

### 9.6 Why Rust?

ทำไมภาษา Rust จึงเลือกออกแบบระบบ Structs & Methods ในรูปแบบนี้ แทนที่จะใช้ Class เหมือนภาษา OOP กระแสหลักทั่วไป? คำตอบเชื่อมโยงกับเกณฑ์การประเมินภาษาโปรแกรม (Language Evaluation Criteria) ดังนี้:

1. **Maximum Reliability (ความน่าเชื่อถือและความปลอดภัยสูงสุด):**
   * กำจัดปัญหาหลักของภาษา C/C++ เช่น **Dangling Pointer**, **Double Free**, และ **Data Race** โดยการตรวจเช็กสิทธิ์ผ่าน Borrow Checker ตั้งแต่ Compile-time
   * กำจัดปัญหา **Null Pointer Exception** ของ Java โดยไม่มีค่า `null` ในระดับภาษา หากค่าอาจไม่มีอยู่จะถูกบังคับให้ใช้ `Option<T>` ร่วมกับ Struct แทน
2. **Predictable Bare-metal Performance (ประสิทธิภาพระดับฮาร์ดแวร์):**
   * ไม่มีระบบ **Garbage Collector (GC)** มาคอยหยุดการทำงานของโปรแกรมแบบสุ่ม (No Stop-the-world Latency)
   * ข้อมูลใน Struct วางบน Stack เป็นค่าเริ่มต้น และขนาดในหน่วยความจำมีขนาดเท่ากับผลรวมของข้อมูลจริงเพียวๆ (Zero Object Header Overhead) ไม่เปลือง RAM และเข้าถึงได้รวดเร็ว
3. **High Readability & Maintainability (ความอ่านง่ายและง่ายต่อการดูแลรักษา):**
   * การบังคับระบุ Receiver (`&self`, `&mut self`, `self`) ทำให้ผู้อ่านโค้ดเข้าใจเจตนาของฟังก์ชันได้ทันทีว่าเมธอดนี้จะ "แค่อ่าน", "ขอแก้", หรือ "กลืนทำลาย" ข้อมูล โดยไม่ต้องเปิดเข้าไปดูโค้ดข้างในฟังก์ชัน


---

## 10. Rust vs. Other Language

**Comparison Languages:** **C++, Java, Python** (เปรียบเทียบการจัดการโครงสร้างข้อมูล พฤติกรรมเมธอด และการบริหารหน่วยความจำกับ Rust)

| มิติ (Aspect) | ภาษา Rust (`struct` + `impl`) | ภาษา C++ (`class` / `struct`) | ภาษา Java (`class`) | ภาษา Python (`class`) |
| :--- | :--- | :--- | :--- | :--- |
| **1. Syntax & Declaration** | แยกนิยาม Data (`struct`) ออกจาก Behavior (`impl`) อย่างเด็ดขาด ไม่มีคลาส | สามารถรวม Data และ Method ใน struct/class ก้อนเดียวกัน | ทุกอย่างต้องรวมอยู่ใน `class` ไม่มีอิสระ | รวมข้อมูลและเมธอดไว้ใน `class` นิยามฟิลด์ใน `__init__` |
| **2. Semantics & Receiver** | Receiver ชัดเจน (`&self`, `&mut self`, `self`) ระบุสิทธิ์ Borrow/Move ชัดแจ้ง | ใช้ `this` เป็น pointer และใช้คีย์เวิร์ด `const` แยกเมธอดอ่าน | ใช้ `this` เป็น reference ทุกเมธอดสามารถแก้ไขสถานะอ็อบเจกต์ได้ | ใช้ `self` เป็นพารามิเตอร์ตัวแรก เพื่ออ้าง Object ปัจจุบัน |
| **3. Type System** | Static Typing, Nominal Typing, **ไม่มี Inheritance** (ใช้ Trait + Composition) | Static Typing, Nominal Typing, Multiple Inheritance, Virtual Function | Static Typing, Nominal Typing, Single Inheritance มี Object เป็น Root | Dynamic Typing, Duck Typing, Multiple Inheritance ตรวจสอบไทป์ตอน Runtime |
| **4. Memory Management** | Stack เป็นค่าเริ่มต้น, คืน Memory อัตโนมัติด้วย Ownership + Borrowing + Scope ไร้ GC | Stack / Heap + RAII + new/delete หรือ Smart Pointer เสี่ยง Dangling Pointer | ทุกอ็อบเจกต์จองบน Heap ผ่าน `new`, พึ่งพา Garbage Collector | ทุกอ็อบเจกต์จองบน Heap, พึ่งพา Reference Counting และ Cyclic GC |
| **5. Safety & Guarantees** | Compile-time Safety ผ่าน Ownership & Borrowing (ป้องกัน Null Pointer และ Data Race) | ยืดหยุ่นสูง แต่โปรแกรมเมอร์ต้องคุมเอง เสี่ยง Dangling Pointer และ Undefined Behavior | GC ช่วย Memory Safety แต่ยังอาจเกิด `NullPointerException` ตอน Runtime | จัดการ Memory อัตโนมัติ แต่เสี่ยงเกิด `AttributeError` / `TypeError` ตอน Runtime |

---

### Rust Example

```rust
#[derive(Debug)]
struct UserAccount {
    username: String,
    balance: f64,
}

impl UserAccount {
    fn new(username: &str, initial_balance: f64) -> Self {
        Self {
            username: username.to_string(),
            balance: initial_balance,
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("ฝากเงินสำเร็จ: +${:.2} | ยอดคงเหลือปัจจุบัน: ${:.2}", amount, self.balance);
        } else {
            println!("จำนวนเงินฝากต้องมากกว่า 0");
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            Err("จำนวนเงินถอนต้องมากกว่า 0".to_string())
        } else if amount > self.balance {
            Err("ยอดเงินคงเหลือไม่เพียงพอ".to_string())
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }

    // Consuming method: รับกรรมสิทธิ์ self (Move) ทำให้ account เดิมถูกทำลาย ป้องกันการใช้งานซ้ำ
    fn close_account(self) -> f64 {
        println!("ปิดบัญชีของ {} สำเร็จ คืนเงินคงเหลือ: ${:.2}", self.username, self.balance);
        self.balance
    }
}

fn main() {
    let mut my_account = UserAccount::new("Alice", 100.0);
    println!("เริ่มต้นบัญชี: {:?}", my_account);
    println!("ยอดเงินเริ่มต้น: ${:.2}", my_account.get_balance());

    my_account.deposit(50.0);

    match my_account.withdraw(30.0) {
        Ok(new_balance) => println!("ถอนเงินสำเร็จ ยอดคงเหลือ: ${:.2}", new_balance),
        Err(e) => println!("เกิดข้อผิดพลาด: {}", e),
    }

    let refunded = my_account.close_account();
    println!("เงินคืนเข้ามือ: ${:.2}", refunded);
    // my_account.deposit(10.0); // ❌ คอมไพล์ไม่ผ่านทันที! (use of moved value: `my_account`)
}
```

---

### C++ Example

```cpp
#include <iostream>
#include <string>
#include <iomanip>

struct Result {
    bool is_ok;
    double value;
    std::string error;

    static Result Ok(double val) { return {true, val, ""}; }
    static Result Err(const std::string& err) { return {false, 0.0, err}; }
};

class UserAccount {
private:
    std::string username;
    double balance;

public:
    UserAccount(const std::string& username, double initial_balance)
        : username(username), balance(initial_balance) {}

    double get_balance() const { // const method บ่งบอกการยืมอ่าน
        return balance;
    }

    void deposit(double amount) {
        if (amount > 0.0) {
            balance += amount;
            std::cout << "ฝากเงินสำเร็จ: +$" << std::fixed << std::setprecision(2) << amount
                      << " | ยอดคงเหลือปัจจุบัน: $" << balance << "\n";
        } else {
            std::cout << "จำนวนเงินฝากต้องมากกว่า 0\n";
        }
    }

    Result withdraw(double amount) {
        if (amount <= 0.0) {
            return Result::Err("จำนวนเงินถอนต้องมากกว่า 0");
        } else if (amount > balance) {
            return Result::Err("ยอดเงินคงเหลือไม่เพียงพอ");
        } else {
            balance -= amount;
            return Result::Ok(balance);
        }
    }

    double close_account() {
        std::cout << "ปิดบัญชีของ " << username << " สำเร็จ คืนเงินคงเหลือ: $" 
                  << std::fixed << std::setprecision(2) << balance << "\n";
        double refunded = balance;
        balance = 0.0;
        return refunded;
    }
};

int main() {
    UserAccount my_account("Alice", 100.0);
    std::cout << "ยอดเงินเริ่มต้น: $" << std::fixed << std::setprecision(2) << my_account.get_balance() << "\n";
    my_account.deposit(50.0);

    Result res = my_account.withdraw(30.0);
    if (res.is_ok) {
        std::cout << "ถอนเงินสำเร็จ ยอดคงเหลือ: $" << std::fixed << std::setprecision(2) << res.value << "\n";
    }

    double refunded = my_account.close_account();
    std::cout << "เงินคืนเข้ามือ: $" << std::fixed << std::setprecision(2) << refunded << "\n";
    // my_account ยังคงอยู่และเรียกต่อได้ C++ คอมไพเลอร์ไม่ได้ห้าม ต้องเขียน logic ตรวจเช็กเอง
}
```

---

### Java Example

```java
public class UserAccount {
    private String username;
    private double balance;

    public UserAccount(String username, double initialBalance) {
        this.username = username;
        this.balance = initialBalance;
    }

    public double getBalance() {
        return this.balance;
    }

    public void deposit(double amount) {
        if (amount > 0.0) {
            this.balance += amount;
            System.out.printf("ฝากเงินสำเร็จ: +$%.2f | ยอดคงเหลือปัจจุบัน: $%.2f%n", amount, this.balance);
        } else {
            System.out.println("จำนวนเงินฝากต้องมากกว่า 0");
        }
    }

    public double withdraw(double amount) throws Exception {
        if (amount <= 0.0) {
            throw new Exception("จำนวนเงินถอนต้องมากกว่า 0");
        } else if (amount > this.balance) {
            throw new Exception("ยอดเงินคงเหลือไม่เพียงพอ");
        } else {
            this.balance -= amount;
            return this.balance;
        }
    }

    public double closeAccount() {
        System.out.printf("ปิดบัญชีของ %s สำเร็จ คืนเงินคงเหลือ: $%.2f%n", this.username, this.balance);
        double refunded = this.balance;
        this.balance = 0.0;
        return refunded;
    }

    public static void main(String[] args) {
        UserAccount myAccount = new UserAccount("Alice", 100.0);
        System.out.printf("ยอดเงินเริ่มต้น: $%.2f%n", myAccount.getBalance());
        myAccount.deposit(50.0);

        try {
            double newBalance = myAccount.withdraw(30.0);
            System.out.printf("ถอนเงินสำเร็จ ยอดคงเหลือ: $%.2f%n", newBalance);
        } catch (Exception e) {
            System.out.println("เกิดข้อผิดพลาด: " + e.getMessage());
        }

        double refunded = myAccount.closeAccount();
        System.out.printf("เงินคืนเข้ามือ: $%.2f%n", refunded);
        // อ็อบเจกต์ myAccount ยังลอยอยู่ใน Heap รอ Garbage Collector มาเก็บกวาด
    }
}
```

---

### Python Example

```python
class UserAccount:
    def __init__(self, username: str, initial_balance: float):
        self.username = username
        self.balance = initial_balance

    def get_balance(self) -> float:
        return self.balance

    def deposit(self, amount: float) -> None:
        if amount > 0.0:
            self.balance += amount
            print(f"ฝากเงินสำเร็จ: +${amount:.2f} | ยอดคงเหลือปัจจุบัน: ${self.balance:.2f}")
        else:
            print("จำนวนเงินฝากต้องมากกว่า 0")

    def withdraw(self, amount: float):
        if amount <= 0.0:
            return (False, "จำนวนเงินถอนต้องมากกว่า 0")
        elif amount > self.balance:
            return (False, "ยอดเงินคงเหลือไม่เพียงพอ")
        else:
            self.balance -= amount
            return (True, self.balance)

    def close_account(self) -> float:
        print(f"ปิดบัญชีของ {self.username} สำเร็จ คืนเงินคงเหลือ: ${self.balance:.2f}")
        refunded = self.balance
        self.balance = 0.0
        return refunded


def main():
    my_account = UserAccount("Alice", 100.0)
    print(f"ยอดเงินเริ่มต้น: ${my_account.get_balance():.2f}")
    my_account.deposit(50.0)

    is_ok, result = my_account.withdraw(30.0)
    if is_ok:
        print(f"ถอนเงินสำเร็จ ยอดคงเหลือ: ${result:.2f}")
    else:
        print(f"เกิดข้อผิดพลาด: {result}")

    refunded = my_account.close_account()
    print(f"เงินคืนเข้ามือ: ${refunded:.2f}")
```

---

### Analysis (บทวิเคราะห์เชิงเปรียบเทียบหลักการภาษา)

จากการเปรียบเทียบการทำงานของ `UserAccount` ระหว่าง **Rust**, **C++**, **Java**, และ **Python** พบความแตกต่างเชิงหลักการภาษา (PPL Perspectives) ดังนี้:

1. **Encapsulation & Separation of Concerns:**
   * **C++, Java, Python** จัดวาง Data และ Behavior รวมไว้เป็นก้อนเดียวกันใน `class` ซึ่งส่งผลให้เกิดความผูกพันแน่นหนาระหว่างโครงสร้างหน่วยความจำกับตรรกะของฟังก์ชัน
   * **Rust** แยก `struct` (ที่เก็บเฉพาะ Data Layout) ออกจาก `impl` (Behavior) ทำให้สามารถขยายฟังก์ชันการทำงานหรือเพิ่ม Trait ได้เรื่อยๆ โดยไม่ต้องแตะต้องโครงร่างข้อมูลเดิม
2. **Semantics ของ Receiver และ Mutability Control:**
   * ใน **Java และ C++** มีตัวแปรแฝงคือ `this` ซึ่งใน Java อ็อบเจกต์จะถูกแก้ไขสถานะได้เสมอหากเข้าถึงได้ ส่วน C++ ใช้คีย์เวิร์ด `const` ท้ายเมธอดเพื่อป้องกันการแก้ไข
   * ใน **Python** ใช้ `self` เป็นพารามิเตอร์ตัวแรก แต่ไม่มีการควบคุม Mutability ทำให้ฟิลด์ใดๆ ถูกแก้ไขหรือเพิ่มฟิลด์ใหม่แบบไดนามิกได้ตลอดเวลา ซึ่งลด Reliability ของระบบ
   * ใน **Rust** บังคับระบุ Receiver อย่างชัดเจน: `&self` อนุญาตให้อ่านเท่านั้น, `&mut self` บังคับความเป็นเจ้าของแต่เพียงผู้เดียวเพื่อป้องกัน Data Race และที่โดดเด่นที่สุดคือ `self` (Consuming Method) ที่รับสิทธิ์ความเป็นเจ้าของ (Move) ทำให้เมื่อเรียก `my_account.close_account()` แล้ว อ็อบเจกต์จะถูกทำลายทิ้งทันที คอมไพเลอร์จะบล็อกไม่ให้ใครนำบัญชีที่ปิดไปแล้วมาฝาก-ถอนได้อีก ช่วยแก้ปัญหา Bug เชิงตรรกะได้ตั้งแต่ตอนคอมไพล์
3. **Memory Lifecycle & Overhead:**
   * **Java และ Python** จัดสรรอ็อบเจกต์ส่วนใหญ่บน **Heap Memory** ในลักษณะ Reference Type และมี **Object Header Overhead** (กินพื้นที่หน่วยความจำมากกว่าข้อมูลจริงเพื่อเก็บ Metadata ของคลาส) พร้อมทั้งต้องพึ่งพา **Garbage Collector** ในการคืนเมมโมรี
   * **C++ และ Rust** จัดเก็บข้อมูลอินสแตนซ์บน **Stack** เป็นค่าเริ่มต้น (Zero Object Header Overhead) และมีขนาดในหน่วยความจำเท่ากับผลรวมของข้อมูลจริงเพียวๆ (String pointer + length + capacity + f64) และใช้หลักการ **RAII** คืนหน่วยความจำทันทีที่หลุดขอบเขต โดย Rust แตกต่างตรงที่มี Borrow Checker ช่วยตรวจสอบการอ้างอิงและ Ownership ตั้งแต่ Compile-time คอยป้องกันไม่ให้เกิด Dangling Pointer
4. **Error Handling Paradigm:**
   * **Java** ใช้ Exception (`try-catch`) ซึ่งเป็น Unchecked/Checked Exception ที่สร้าง Runtime Stack Trace Overhead สูง
   * **Rust** ใช้ Monadic Error Handling ผ่าน `Result<T, E>` บังคับให้จัดการข้อผิดพลาดด้วย Pattern Matching (`match`) ตั้งแต่ตอนคอมไพล์ ทำให้โปรแกรมทำงานได้เร็วและไม่มีทางเกิด Unhandled Exception แอบแฝง

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

`[Concept + Short Code Illustration]`

**Member 2**

`[Detailed Code + Live Demo ]`

**Member 3**

`[Rust vs Other Language + PPL Analysis ]`

**Member 4**

`[Exercises + Common Mistakes + Challenge ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`
=======
4. `[https://www.w3schools.com/rust]`
5. `[https://users.rust-lang.org/]`
6. `[https://medium.com/@sathabhronchangchuea/rust-%E0%B8%97%E0%B8%B3%E0%B8%84%E0%B8%A7%E0%B8%B2%E0%B8%A1%E0%B8%A3%E0%B8%B9%E0%B9%89%E0%B8%88%E0%B8%B1%E0%B8%81%E0%B8%81%E0%B8%B1%E0%B8%9A-ownership-references-borrow-%E0%B8%AA%E0%B8%B3%E0%B8%AB%E0%B8%A3%E0%B8%B1%E0%B8%9A%E0%B8%88%E0%B8%B1%E0%B8%94%E0%B8%81%E0%B8%B2%E0%B8%A3-memory-903e42a0e9cd]`


=======
4. `[https://www.w3schools.com/rust]`
5. `[https://users.rust-lang.org/]`
5. `[https://medium.com/@sathabhronchangchuea/rust-%E0%B8%97%E0%B8%B3%E0%B8%84%E0%B8%A7%E0%B8%B2%E0%B8%A1%E0%B8%A3%E0%B8%B9%E0%B9%89%E0%B8%88%E0%B8%B1%E0%B8%81%E0%B8%81%E0%B8%B1%E0%B8%9A-ownership-references-borrow-%E0%B8%AA%E0%B8%B3%E0%B8%AB%E0%B8%A3%E0%B8%B1%E0%B8%9A%E0%B8%88%E0%B8%B1%E0%B8%94%E0%B8%81%E0%B8%B2%E0%B8%A3-memory-903e42a0e9cd]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool         | Purpose       | How the Result Was Verified |
| --------------- | ------------- | --------------------------- |
| `[ChatGPT]`    | `[วิเคราะห์ สรุป และช่วยตรวจสอบเนื้อหาเกี่ยวกับ Rust Structs & Methods และการเปรียบเทียบภาษา]` | `[ใช้ claude ในการตรวจสอบ]`           |
| `[Gemini]`     | `[วิเคราะห์ สรุป และช่วยตรวจสอบเนื้อหาเกี่ยวกับ Rust Structs & Methods และการเปรียบเทียบภาษา]` | `[ใช้ GPT,Claude ในการตรวจสอบ] `           |
| `[claude]`     | `[ใช้เพื่อตรวจสอบการทำงานของ ChatGPT,Gemini]` | `[สมาชิกกลุ่มอ่านทานร่วมกัน แบบ Manual ทุกประโยค และเทียบเคียงกับนิยามอย่างเป็นทางการใน The Rust Reference ก่อนสรุปเนื้อหา] `           |

### Declaration

- [/] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [/] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [/] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [/] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[ใช้ ai เป็นผู้ช่วยคิด และนำทางการทำไฟล์ และCode บางส่วน]`

---

## 14. GitHub Contribution

| Member   |   Issues |  Commits | Pull Requests | Code Reviews | Contribution  |
| -------- | -------: | -------: | ------------: | -----------: | ------------- |
| Member 1 | `[0 ครั้ง]` | `[2 ครั้ง]` |      `[2 ครั้ง]` |     `[0 ครั้ง]` | `[Concept + Short Code Illustration ]` |
| Member 2 | `[0 ครั้ง]` | `[4 ครั้ง]` |      `[4 ครั้ง]` |     `[0 ครั้ง]` | `[Detailed Code + Live Demo ]` |
| Member 3 | `[0 ครั้ง]` | `[1 ครั้ง]` |      `[1 ครั้ง]` |     `[0 ครั้ง]` | `[Rust vs Other Language + PPL Analysis ]` |
| Member 4 | `[5 ครั้ง]` | `[5 ครั้ง]` |      `[4 ครั้ง]` |     `[0 ครั้ง]` | `[Exercises + Common Mistakes + Challenge]` |
`[ข้อมูลด้านบนมาจาก https://github.com/670710140/Project_Topic_Structs-Methods/tree/main ในไฟล์ rust_tutorial_template.md]`

### Teamwork Reflection

**How did your team collaborate?**

`[เราแบ่งงานกันเป็นส่วนๆ และทำงานในส่วนของตัวเอง และเราแบ่งการทำงานส่วนกลาง เป็นงานส่วนรวม เราแยกกันทำตามที่ระบบ Github สามารถทำได้ ใครทำเสร็จแล้วให้อัปโหลด เข้า main ส่วนกลาง]`

**Problems encountered**

`[ปัญหาที่พบบคือ การทำงานร่วมกันบน Github ยังไม่ค่อยคุ้นชิน จึงทำให้งานล่าช้า]`

**How did you solve them?**

`[เรียนรู้วิธีใช้ Github ผ่าน youtube และ AI สอนเบื้องต้นเป็นเพื่อนช่วยคิด]`

---

## 15. Final Checklist

- [/] Learning Objectives ครบ 3–4 ข้อ
- [/] Key Concepts ครบถ้วน
- [/] Syntax / Rules
- [/] Runnable Code Examples
- [/] Code Compile และ Run ได้จริง
- [/] Common Mistakes
- [/] Exercises 2 ข้อ พร้อม Solutions
- [/] PPL Perspective
- [/] Rust vs Other Language
- [/] References อย่างน้อย 4 แหล่ง
- [/] AI Usage Declaration
- [/] GitHub Contribution
- [/] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที `[เนื่องจากเพื่อนในกลุ่มเหลือ 3 คนจึงไม่ครบเงื่อนไข]`
- [/] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[https://github.com/670710140/Project_Topic_Structs-Methods]`

**Chapter Path:** `[08-structs-methods/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group 8]`

**Date:** `[2026-10-03]`
