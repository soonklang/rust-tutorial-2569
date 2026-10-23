# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 21
> **Topic No.:** 21
> **Topic Name:** Smart Pointers & Resource Management
> **ประเด็นหลักที่ควรครอบคลุม:** Box, Rc, Arc, RefCell และ resource management

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายวีรภัทร พิริยะสถิต | 670710653 | `@670710653` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวอาธารดา พรหมแทนสุด | 670710654 | `@670710654` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายณัฐนนท์ อ้นเพ็ชร | 630710128 | `@[กรอก GitHub username]` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายอนุวัฒน์ เส็งเจริญ | 630710136 | `@[กรอก GitHub username]` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

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


`ในระบบการจัดการหน่วยความจำของภาษา Rust นักพัฒนาสามารถเลือกใช้กลไกการอ้างอิงตำแหน่งหน่วยความจำ (Pointer) ได้ 2 รูปแบบหลัก`

`แบบที่1 การอ้างอิงแบบปกติ (Normal References / Raw Pointers)
ตัวชี้วัดหรือการอ้างอิงประเภท &T และ &mut T ทำหน้าที่เป็นเพียงดัชนีระบุตำแหน่งหน่วยความจำมีขนาดคงที่เท่ากับความกว้างของสถาปัตยกรรมหน่วยประมวลผล (8 ไบต์ สำหรับระบบ 64-bit) 
โดยมีบทบาทภายใต้กฎการยืมข้อมูลเท่านั้น ไม่มีสิทธิ์การเป็นเจ้าของทรัพยากรและไม่มีการทำงานส่วนเกินใด ๆ ซ่อนอยู่เบื้องหลัง`

`แบบที่2 Smart Pointers
โครงสร้างข้อมูลประเภท Structs เช่น Box<T>, Rc<T>, และ RefCell<T> ซึ่งนอกจากจะทำหน้าที่ชี้ตำแหน่งหน่วยความจำแล้ว 
ยังทำหน้าที่เป็นผู้ถือครองสิทธิ์ในทรัพยากรนั้นๆ พร้อมทั้งมีกลไกที่ถูกควบคุมผ่านโปรแกรมเบื้องหลัง เพื่อบริหารจัดการLifecycleของข้อมูล และสิทธิ์การเข้าถึงข้อมูลให้เป็นไปอย่างปลอดภัย`

---

## 4. Key Concepts

### 4.1 `[หลักการทำงานของSmart Pointer และ ลักษณะเฉพาะ]`

**คำอธิบาย**

`Smart Pointer ไม่ใช่แค่การชี้ไปยังตำแหน่งหน่วยความจำเหมือนพอยเตอร์ทั่วไปแต่เป็น โครงสร้างข้อมูลที่จำลองพฤติกรรมของพอยเตอร์ โดยมีคุณลักษณะเด่น 3 ประการหลักๆ:
การเป็นเจ้าของทรัพยากร: พอยเตอร์ปกติ (&T) ทำหน้าที่เพียงแค่ "ยืม" ข้อมูลมาใช้งาน แต่ Smart Pointer จะทำหน้าที่เป็น "เจ้าของ" ข้อมูลก้อนนั้นโดยตรง มันมีสิทธิ์ขาดในการควบคุมวงจรชีวิตของข้อมูล
การเพิ่มคุณลักษณะและความสามารถ: มันถูกออกแบบมาเพื่อแก้ข้อจำกัดของระบบหน่วยความจำ เช่น การย้ายข้อมูลไปไว้บน Heap อัตโนมัติ (Box), การแชร์ข้อมูลให้เจ้าของหลายคน (Rc/Arc), หรือการปลดล็อกให้แก้ไขข้อมูลในตัวแปรที่ถูกห้ามแก้ (RefCell)
ผู้ใช้งานสามารถเรียกใช้ข้อมูลผ่าน Smart Pointer ได้เสมือนเป็นตัวแปรธรรมดา โดยไม่ต้องสั่งเปิด-ปิด หรือคำนวณตำแหน่งหน่วยความจำด้วยตัวเองในโค้ด`



---

### 4.2 `[Memory allocation of Smart pointer]`

`โดยทั่วไป Pointer จะถูกออกแบบให้ใช้พื้นที่ 8bytes ในสถาปัตยกรรมของ CPU 64-bit แต่ Smart Pointer นั้นจะทำให้เกิด Overhead แบบหลีกเลี่ยงไม่ได้ในทั้ง Heap และ Stack
Stack Overhead คือพื้นที่หน่วยความจำที่ตัวควบคุม Smart Pointer ใช้จัดเก็บสถานะระบบ ตัวอย่างเช่น RefCell<T> จะใช้หน่วยความจำเพิ่มเติม 8 ไบต์บน Stack 
เพื่อเก็บตัวแปรบ่งชี้สถานะการยืม (Borrow Flag) สำหรับตรวจสอบความปลอดภัยของการเข้าถึงข้อมูลในขณะประมวลผล
Heap Overhead คือหน่วยความจำส่วนเกินที่ถูกจัดสรรควบคู่ไปกับข้อมูลจริงบน Heap เช่น Rc<T> และ Arc<T> 
ซึ่งระบบจะสร้างตัวนับจำนวนการอ้างอิงอิมพูเตชันภายนอก เพื่อประเมินรอบเวลาในการคืนสภาพหน่วยความจำอัตโนมัติ`

```rust
use std::mem::size_of;
use std::cell::RefCell;
fn main() {
    println!("=== Size Comparison of Stack ===");

    let raw_data: i32 = 42;
    println!("Size of i32 :  {} bytes", size_of::<i32>());

    let reference: &i32 = &raw_data;
    println!("Size of &i32 (Normal Pointer): {} bytes", size_of::<&i32>());

    let ref_cell: RefCell<i32> = RefCell::new(42);
    println!("Size of RefCell<i32> (Smart Pointer): {} bytes", size_of::<RefCell<i32>>());
}
```
**Output**
```
=== Size Comparison of Stack ===
Size of i32 :  4 bytes
Size of &i32 (Normal Pointer): 8 bytes
Size of RefCell<i32> (Smart Pointer): 16 bytes
```
**Explanation**

`2 บรรทัดแรกเป็นการเรียกใช้ Library พื้นฐานในการเรียกดูขนาดของข้อมูล
หลักการทำงานในบรรทัด let raw_data กำหนดแปรเป็น int32 ขนาด bit ให้เก็บค่า 42
และทำการ print เพื่อดูขนาดของตัวแปรซึ่งจะให้ขนาด 4bytes
ต่อมาสร้างตัวแปรชื่อ reference ที่เป็นตัว int ให้ชี้ไปที่ raw_data และจะ print เพื่อแสดงขนาดของ pointer(ขนาดของตัวแปรreference) ซึ่งจะให้ผลลัพธ์เป็น 8bytes ตามขนาดสถาปัตยกรรมของ 64-bit เพื่อชี้ทางไปหาข้อมูลตัวอื่น
ตัวแปรตัวสุดท้ายอย่าง ref_cell หรือก็คือตัว Smart pointer ที่จะเก็บค่า int และกำหนดค่าเป็น 42 พอใช้คำสั่ง Print จะให้ขนาดเป็น 16bytes`

**ข้อควรระวัง**

`ข้อที่.1 (เนื่องจากในภาษา Rust ไม่ยอมให้มีค่า Null แบบใน Java เราเลยต้องกำหนดค่าก่อนเพราะ Smart Pointer ไม่ใช่ตัวแปรทั่วไปแต่มันเป็น Object ชนิดนึงแทนที่เราจะประกาศค่าปกติเหมือนตัวแปรแรกเราประกาศให้ Smart Pointer ของเราเป็นตัวที่เก็บค่านั้นเลย
และเวลาใครจะใช้ค่าในนี้ค่อย Point มาหาและเพราะว่า Smart Pointer เป็นเจ้าของข้อมูลมันจริงอัพเดทค่าในตัวมันได้ด้วย แต่ต้องแลกกับการที่ขนาดของ Smart Pointer ในหน่วยความจำนั้นมีขนาดใหญ่มากกว่าการประกาศตัวแปรทั่วไป)`

`ข้อที่.2 (ขนาดจริงๆของ Smart Pointer ในตัวอย่างนี้คือ 12 bytes แบ่งได้ตามนี้ Refcell<i32> = 8 bytes เป็นการจองพื้นที่บน stack , i32 = 4 bytes เป็นการจองพื้นที่บน heap เหตุผลที่ว่าผลลัพธ์เป็น 16bytes เกิดจากการตัวจัดสรรหน่วยความจำมักจะทำงานได้ลำบากหาก byte เหล่านั้นไม่สามารถหารด้วย8ลงตัว)`


---

### 4.3 `[การใช้Box<T>เบื้องต้น]`

**คำอธิบาย**

`Box<T>ถูกหยิบมาใช้เมื่อต้องการจะให้ค่าที่เก็บนั้นไปอยู่บนheapแทนที่จะไปเก็บไว้บนstackแต่ยังคงรักษาความเป็นเจ้าของตัวแปรนั้นเพียงตัวเดียว
ในตัวอย่างนี้จะเป็นการบวกเลข5ที่ประกาศเป็นintegerแบบปกติกับค่าของBoxที่ถูกประกาศไว้และชี้ไปที่ค่า10`

```rust
fn main() {
    let stack_number = 5;
    let heap_pointer = Box::new(10);
    let sum = stack_number + *heap_pointer;
    println!("Total is: {}", sum);
}
```
**Output**
```
Total is: 15
```
**Explanation**
```
เมื่อBox::new(10)ถูกเรียกตัวภาษาจะหาพื้นที่ว่างในheapเพื่อเก็บค่า10และจะเก็บตัวheap_pointerบนstackและจะชี้ไปยังค่า10
ทั้งนี้ทั้งนั้นตัวBoxเป็นเจ้าของค่า10นั้นแต่เพียงผู้เดียวตามกฎของBox<T>ที่ให้มีเจ้าของค่าได้แค่คนเดียว
เมื่อตัวแปรheap_pointerถึงปีกกาปิดheapในส่วนนั้นจะถูกลบโดยทันที
```
---

### 4.4 `[การใช้Rc<T>เบื่องต้น]`

**คำอธิบาย**

`Rc<T>หรือReference Counted จะถูกหยิบมาใช้ตอนหลายส่วนในโปรแกรมต้องการอ่านข้อมูลเดียวกันแต่ไม่อาจทราบว่าส่วนที่อ่านค่าเป็นคนสุดท้ายเป็นใครเพื่อป้องกันData race`

```rust
use std::rc::Rc;

fn main() {
    let original = Rc::new(String::from("Shared Text"));
    {
        let owner_two = Rc::clone(&original);
        
        println!("Active owners inside block: {}", Rc::strong_count(&original));
        println!("Owner two sees: {}", owner_two);
    }
    println!("Active owners after block: {}", Rc::strong_count(&original));
    
}
```
**Output**
```
Active owners inside block: 2
Owner two sees: Shared Text
Active owners after block: 1
```
**Explanation**
```
แทนที่จะคัดลอกค่านั้นตามคำสั่งcloneตัวRcจะให้ตัวที่โคลนนั้นชี้ไปยัง"Shared Text"โดยจะนับจำนวนคนที่โคลนตัวแหน่งนั้นไปใช้
เมื่อจบรอบการทำงานภายในBlockแล้วจำนวนคนที่เป็นเจ้าของค่านั้นจาก2จะถูกลดเหลือ1
ข้อมูลที่ถูกเก็บไว้ในRcจะถูกกำจัดก็ต่อเมื่อการทำงานของโปรแกรมถึงปีกกาปิดหลังจากนั้นจะคืนพื้นที่
```
**ข้อควรระวัง**

`Rc อนุญาตให้อ่านได้อย่างเดียว(Read only)และไม่อนุญาตให้เขียนข้อมูลเพิ่มเติม`

---

### 4.5 `[การใช้Refcell<T>เบื่องต้น]`

`Refcellจะถูกหยิบมาใช้เมื่อมีค่าimmutableและจำเป็นต้องแก้ไขค่านั้นจริงๆSmart pointerตัวนี้จะอนุญาตให้สามารถแก้ไขค่าImmutableนี้โดยการBypassตรวจเช็คการแก้ค่าImmutableตอนCompileและค่าเหล่านั้นจะไปถูกแก้ตอนRuntimeแทน`

```rust
use std::cell::RefCell;

fn main() {
    let safe_data = RefCell::new(50);

    {
        let mut temporary_writer = safe_data.borrow_mut();
        *temporary_writer += 25;
        
    }
    println!("Final value: {}", safe_data.borrow());
}
```
**Output**
```
Final value: 75
```
**Explanation**
```
เมื่อไม่ได้ประกาศว่าค่านั้นสามารถแก้ไขได้หรือไม่ได้ใส่keyword (mut) ตัวRefcellจะมีคำสั่ง.borrow_mut()เพื่อให้เราสามารถแก้ค่าตอนRuntimeได้
โดยตัวแปรใหม่ที่เราสร้างเพื่อมาแก้ค่าตอนRuntimeนั้นจะบวกค่าให้ตัวแปรsafe_dataจากเดิม50บวกเพิ่มไปอีก25
หลังจากจบBlockการทำงานของตัว.borrow_mut()ค่าของsafe_dataจะถูกล็อคไม่ให้แก้ดั้งเดิม
หลังจากนั้นก็จะแสดงค่าผลลัพธ์ของsafe_dataที่ตอนนี้มีค่า75
```
**ข้อควรระวัง**

`แน่นอนว่าตามกฎของRefcellที่ไม่อนุญาตให้คนอื่นมายุ่งกับตัวมันตอนมีคนกำลังใช้งานค่าอยู่หากมีคนต้องการมาใช้ค่าหรืออ่านค่าตัวโปรแกรมจะcrashโดยทันทีเพื่อป้องกันข้อมูลเกิดความเสียหาย`

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `Box<T>` | `สร้างข้อมูลบน Heap แทนการสร้างบน Stack` | `ใช้เมื่อไม่ทราบขนาดของข้อมูลขณะ Runtime หรือต้องการจะส่งต่อความเป็นเจ้าของของข้อมูลโดยไม่ต้อง Copy ค่าเหล่านั้น` |
| `Rc<T>` | `อนุญาตให้มีตัวแปรหลายตัวเป็นเจ้าของค่านี้รวมถึงการอ้างอิงแบบปกติด้วย` | `เมื่อหลายๆส่วนในโปรแกรมต้องการอ่านค่าเดียวกันแต่ไม่ทราบว่าส่วนไหนจะใช้เป็นคนสุดท้าย` |
| `RefCell<T>` | `บังคับใช้กฏการยืมนะตอน Runtime แทนที่จะเป็นตอน compile` | `เมื่อต้องการจะเปลี่ยนแปลงค่าแม้ว่าค่านั้นจะเป็นค่าอ้างอิง(Pointer)ที่เปลี่ยนแปลงไม่ได้` |

### Important Rules

1. `Box<T> บังคับให้มีเจ้าของข้อมูลได้แค่คนเดียวและเมื่อเจ้าของค่านั้นหลุดนอกขอบเขตของค่าที่กำหนดเอาไว้ระบบจะทำการคืนพื้นที่ heap นั้นๆทันที`
   
2. `Rc<T> จะคอยนับว่ามีตัวแปรตัวไหนถือครองค่านี้บ้างและจะมีระบบป้องกัน Data races(การแย่งกันใช้ข้อมูล) หรือการทำให้ข้อมูลส่วนนั้นเกิดความเสียหายด้วยการที่ตัวแปรเหล่านั้นจะทำได้แค่ถือครองค่าแต่จะไม่สามารถแก้ไขค่าใน Rc<T> ได้และจะคืนพื้นที่บนหน่วยความจำเมื่อไม่มีใครอ้างอิงค่าในนี้`
 
3. `Refcell<T> ในภาษา Rust การกำอ้างอิงหรือยืมค่าจะต้องระบุให้ชัดเจนว่าค่านี้เป็นค่าที่เปลี่ยนแปลงได้หรือไม่ได้มิฉะนั้นจะไม่สามารถ Compile ได้ แต่ Refcell<T> สามารถเปลี่ยนแปลงค่าเหล่านั้นขณะ Runtime ได้ด้วยวิธี Interior mutability ข้อควรระวังเพราะ Refcell<T> จะบังคับการยืมค่าตอน Runtime มันจะคอยเช็คเสมอว่าใครกำลังยืม .borrow() หากพยายามจะยืมค่าในขณะที่กำลังมีค่าอื่นใช้งานอยู่ Refcell<T> จะเกิดอาการลกส่งผลให้ Program Crash ในทันที เพื่อป้องกันอาการลกของ Refcell<T> ต้องทำให้มั่นใจว่าไม่มีใครใช้ค่านั้นก่อนจะทำการยืมค่านั้นเสมอ`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `Box<T>`

**Purpose:** `เพื่อสาธิตการใช้ Box<T> ในการจัดเก็บข้อมูลไว้บน Heap แทน Stack และแสดงวิธีใช้ Box<T> ในการสร้าง Recursive Data Structure (โครงสร้างข้อมูลที่เรียกใช้ตัวเอง) เช่น Linked List ซึ่ง Rust บน Stack ไม่สามารถคำนวณขนาด (Size) ในช่วง Compile time ได้หากไม่ใช้ Smart Pointer`

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn sum_list(list: &List) -> i32 {
    match list {
        Cons(value, next) => value + sum_list(next),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("List: {:?}", list);
    println!("Sum of list = {}", sum_list(&list));

    let boxed_number = Box::new(42);
    println!("Boxed number = {}", boxed_number);
}
```

**Expected Output**

```text
List: Cons(1, Cons(2, Cons(3, Nil)))
Sum of list = 6
Boxed number = 42
```

**Explanation**

- enum List { Cons(i32, Box<List>), Nil } — ถ้าไม่มี Box ตรงนี้ Rust จะ error ทันที เพราะ List จะมีขนาดไม่จำกัด (แต่ละ Cons มี List อีกตัวซ้อนอยู่ข้างใน ไม่รู้จบ) การใส่ Box<List> ทำให้ Rust รู้ขนาดที่แน่นอน เพราะ Box คือ pointer ที่มีขนาดคงที่ (ชี้ไปยัง heap)
- sum_list() recursive function เดินไล่ตาม pointer ไปเรื่อย ๆ จนเจอ Nil
- Box::new(42) คือตัวอย่างง่าย ๆ ของการย้ายค่าไปเก็บบน heap แล้วเมื่อ boxed_number หมด scope มันจะถูก deallocate อัตโนมัติ (ผ่าน Drop ที่ Rust ทำให้ built-in)

---

### Example 2 — `Rc<RefCell<T>>`

**Purpose:** `เพื่อสาธิตการก้าวข้ามกฎ Borrow Checker ที่ปกติห้ามแก้ไขข้อมูลหากใช้ Immutable Reference โดยใช้ Interior Mutability Pattern ผ่าน RefCell<T> ซึ่งจะย้ายการตรวจกฎ Borrowing Rules จากช่วง Compile time ไปตรวจช่วง Runtime แทน`

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Counter {
    value: i32,
}

fn increment(shared_counter: &Rc<RefCell<Counter>>) {
    let mut counter = shared_counter.borrow_mut();
    counter.value += 1;
}

fn main() {
    let shared_counter = Rc::new(RefCell::new(Counter { value: 0 }));

    let counter_a = Rc::clone(&shared_counter);
    let counter_b = Rc::clone(&shared_counter);

    increment(&counter_a);
    increment(&counter_b);
    increment(&shared_counter);

    println!("Final value = {}", shared_counter.borrow().value);
    println!("Total owners (strong_count) = {}", Rc::strong_count(&shared_counter));
}
```

**Expected Output**

```text
Final value = 3
Total owners (strong_count) = 3
```

**Explanation**

- Rc<RefCell<Counter>> คือการรวมร่างสองตัว: Rc จัดการเรื่อง "แชร์เจ้าของ", RefCell จัดการเรื่อง "แก้ไขค่าได้แม้ตัวแปรจะดู immutable"
- borrow_mut() คือการขอยืมแบบแก้ไขได้ โดย Rust จะเช็คกฎการยืม (borrowing rules) ตอน runtime แทน compile time — ถ้ามีการ borrow ซ้อนกันผิดกฎ (เช่น borrow_mut สองครั้งพร้อมกัน) โปรแกรมจะ panic ทันที`
- ฟังก์ชัน increment รับ &Rc<RefCell<Counter>> แล้วแก้ค่าข้างในได้ แม้จะไม่ได้เป็นเจ้าของโดยตรง`
- ผลลัพธ์สุดท้าย value = 3 เพราะเรียก increment ผ่าน 3 handle ที่ต่างชี้ไปข้อมูลเดียวกัน

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

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

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

| Member | Responsibility | Time |
|---|---|---:|
| 670710653 | Concept + Short Code Illustration | 5 min |
| 670710654 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**670710653**

`Introduction + Keyconcept + basic code`

**670710654**

`Live demo code + Advanced code`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `https://doc.rust-lang.org/book/ch15-00-smart-pointers.html`
2. `https://dev.to/leapcell/rust-smart-pointers-explained-ownership-memory-and-safety-1ek3`
3. `https://stackoverflow.com/questions/55075458/understand-smart-pointers-in-rust`
4. `https://hackmd.io/@0xdeveloperuche/ryK-pIHyee`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `Claude` | `ออกแบบโค้ดตัวอย่าง` | `ตรวจสอบโดยการนำมา run ผ่านโปรแกรมและเว็บไซต์ Rust Playground` |
| `Gemini` | `ตรวจเช็คเนื้อหาในสิ่งที่ผู้จัดทำเขียน` | `ผู้จัดทำไปทำการตรวจสอบข้อมูลที่AIนำมาใช้ตรวจผู้จัดทำว่าตรงตามเอกสารจากแหล่งอ้างอิงหรือไม่ด้วยตนเอง` |
| `Copilot` | `ช่วยpush file` | `มันทำงานไม่ถูกต้องเลยไม่ได้ใช้ต่อ` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`Geminiถูกนำมาใช้ในส่วนของเนื้อหาหลักผู้จัดทำนำมาช่วยในการตอบคำถามเพื่อเช็คความเข้าใจของผู้จัดทำว่าถูกต้องตามแหล่งอ้างอิงที่ระบุมาหรือไม่เปรียบเสมือนการเรียนรู้แบบSupervisedเพื่อลดเวลาในการเรียนด้วยตนเองโดยยังคงผลลัพธ์ของการเรียนรู้เอาไว้`

`Claudeถูกทำมาสร้างในส่วนของCodeตัวอย่างเนื่องจากความสามารถด้านProgrammingขั้นสูงของAiตัวนี้ทำให้ผู้จัดทำสามารถแสดงการทำงานของSmart pointerได้เห็นภาพมากกว่าการนำโจทย์มาดัดแปลงแล้วฝืนใช้Smart pointerเพราะการใช้งานจริงของSmart pointerนั้นมีหัวข้อที่ทำได้ดีมากๆและหัวข้อที่ไม่จำเป็นต้องนำมาใช้เลยเพราะฉะนั้นการให้Aiมาจัดการในเรื่องของโจทย์ปัญหาที่ต้องการSmart Pointerเข้ามาแก้แบบเฉพาะเจาะจงจึงเป็นสิ่งที่ผู้จัดทำเห็นควรว่าเหมาะสมกว่า`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| 670710653 | `0` | `7` | `4` | `4` | `Concept + introduction + sample code` |
| 670710654 | `0` | `15` | `5` | `3` | `Detail code + Box,Rc,Refcell tutorial` |
| Member 3  | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4  | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`แยกกันทำในส่วนของเนื้อหาที่รับผิดชอบและร่วมกันทำสไลด์นำเสนอ`

**Problems encountered**

`สมาชิกบางส่วนไม่สามารถติดต่อได้และไม่ได้เข้ามาร่วมทำ`

**How did you solve them?**

`ตัดชื่อและปล่อยในส่วนที่ไม่สามารถทำได้ทิ้งตามข้อกำหนดในการทำตามหัวข้อที่ได้รับ`

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

**Repository:** `https://github.com/670710653/Smart-Pointer-Resource-Management`

**Chapter Path:** `src`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group 21]`

**Date:** `[2026-10-03]`


*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
