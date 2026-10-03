# 🦀 Rust Tutorial Project — Principles of Programming Languages

> **Topic No.:** `22`  
> **Topic Name:** `Concurrency & Threads`  
> **Group No.:** `22`

> Tutorial ภาษาไทยสำหรับรายวิชา **517321 Principles of Programming Languages**

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายกรวิชญ์ บุญชู | `630710836` | `@[630710836]` | Concept + Short Code |
| 2 | นายผกาย เมืองแมน | `640710542` | `@[640710542]` | Detailed Code + Live Demo |
| 3 | นางสาวศศิธร โตนาม | `640710572` | `@[username]` | Rust vs Other Language + PPL |
| 4 | นายธนภัทร คงหอม | `640710845` | `@[640710845]` | Exercises + Common Mistakes |

> สมาชิกทุกคนต้องสามารถอธิบายเนื้อหาและ Code ของกลุ่มได้ทั้งหมด ไม่ใช่เฉพาะส่วนที่ตนเองรับผิดชอบ

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. อธิบายแนวคิด **Concurrency & Threads** ในภาษา Rust ได้
2. สร้างและจัดการ Thread ด้วย `std::thread::spawn()` และ `.join()` ได้
3. ใช้ **Message Passing** ผ่าน `mpsc::channel()` และ **Shared State** ผ่าน `Arc<Mutex<T>>` ได้
4. วิเคราะห์ Concurrency ของ Rust ในมุมมองของ **Syntax, Semantics, Type System และ Memory Management**
5. เปรียบเทียบแนวทาง Concurrency ของ Rust กับภาษาโปรแกรมอื่นได้
6. อธิบายบทบาทของ `Send` และ `Sync` ต่อ Concurrency Safety ได้

---

## 3. Introduction

**Concurrency** คือแนวทางการออกแบบโปรแกรมที่ทำให้หลายส่วนของโปรแกรมสามารถดำเนินงานไปพร้อมกันหรือสลับการทำงานกันได้

ใน Rust การทำงานแบบ Concurrent มีแนวคิดสำคัญคือ **Fearless Concurrency** ซึ่งใช้ Ownership System, Borrowing, Type System และ Traits เช่น `Send` และ `Sync` เพื่อช่วยตรวจสอบความปลอดภัยของการทำงานข้าม Thread ตั้งแต่ Compile-time

Concurrency ช่วยให้โปรแกรมสามารถใช้ทรัพยากรของระบบได้อย่างมีประสิทธิภาพ แต่ก็มีปัญหาที่ต้องระวัง เช่น:

- **Data Race** — หลาย Thread เข้าถึงและแก้ไขข้อมูลเดียวกันอย่างไม่ปลอดภัย
- **Deadlock** — Thread รอ Resource ซึ่งกันและกันจนไม่สามารถทำงานต่อได้
- **Race Condition** — ผลลัพธ์ขึ้นอยู่กับลำดับหรือจังหวะการทำงานของ Thread

Rust จึงออกแบบกลไกด้าน Ownership และ Type System เพื่อช่วยป้องกันปัญหาเหล่านี้ตั้งแต่ก่อน Runtime

---

## 4. Key Concepts

### 4.1 Thread Creation & Lifecycle

**คำอธิบาย**

Rust ใช้ `std::thread::spawn()` เพื่อสร้าง Thread ใหม่ และใช้ `JoinHandle` สำหรับจัดการ Thread ที่ถูกสร้างขึ้น

`.join()` ใช้สำหรับรอให้ Thread ทำงานเสร็จสิ้นก่อนที่ Main Thread จะดำเนินการต่อ

**ตัวอย่าง**

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
    });

    handle.join().unwrap();
}
```

**Explanation**

`thread::spawn()` รับ Closure สำหรับให้ Thread ใหม่ทำงาน ส่วน `.join()` ทำให้ Main Thread รอ Thread ที่สร้างขึ้นจนเสร็จ

---

### 4.2 Message Passing with `mpsc`

Rust รองรับการสื่อสารระหว่าง Thread ผ่าน Channel โดยใช้ `std::sync::mpsc`

`mpsc` ย่อมาจาก:

> **Multiple Producer, Single Consumer**

ประกอบด้วย `Sender` สำหรับส่งข้อมูล และ `Receiver` สำหรับรับข้อมูล

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("Hello from thread!"))
            .unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Received: {}", received);
}
```

---

### 4.3 Shared State with `Arc<Mutex<T>>`

เมื่อหลาย Thread จำเป็นต้องเข้าถึงข้อมูลเดียวกัน สามารถใช้ `Arc<Mutex<T>>`

- `Arc<T>` — Atomic Reference Counting ใช้แชร์ Ownership ระหว่าง Thread
- `Mutex<T>` — Mutual Exclusion ใช้ควบคุมการเข้าถึงข้อมูล
- `Arc<Mutex<T>>` — ใช้ร่วมกันเพื่อสร้าง Shared State ที่ปลอดภัย

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}
```

---

### 4.4 `Send` and `Sync`

`Send` และ `Sync` เป็น Traits ที่มีบทบาทสำคัญต่อ Thread Safety

- **`Send`** — Ownership ของ Type สามารถส่งข้าม Thread ได้อย่างปลอดภัย
- **`Sync`** — Reference ของ Type สามารถแชร์ข้าม Thread ได้อย่างปลอดภัย

Compiler จะใช้ข้อมูลจาก Type System เพื่อช่วยตรวจสอบว่าการใช้งานประเภทข้อมูลข้าม Thread เป็นไปตามข้อกำหนดหรือไม่

---

### 4.5 Fearless Concurrency

แนวคิด **Fearless Concurrency** คือแนวคิดที่ Rust พยายามทำให้การเขียน Concurrent Program มีความปลอดภัยมากขึ้น โดยให้ Compiler ช่วยตรวจสอบกฎด้าน Ownership, Borrowing และ Type System

```text
Ownership
    ↓
Borrowing
    ↓
Type System
    ↓
Send / Sync
    ↓
Concurrency Safety
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `thread::spawn()` | สร้าง Thread ใหม่ | `thread::spawn(|| {})` |
| `.join()` | รอ Thread ให้ทำงานเสร็จ | `handle.join().unwrap()` |
| `move` | ย้าย Ownership เข้า Closure | `spawn(move || {})` |
| `mpsc::channel()` | สร้าง Channel สำหรับ Message Passing | `let (tx, rx) = mpsc::channel()` |
| `Arc<T>` | แชร์ Ownership แบบ Atomic | `Arc::clone(&data)` |
| `Mutex<T>` | ควบคุมการเข้าถึง Shared State | `data.lock().unwrap()` |
| `Send` | ส่ง Ownership ข้าม Thread | `Send` |
| `Sync` | แชร์ Reference ข้าม Thread | `Sync` |

### Important Rules

1. Thread ที่ต้องใช้ข้อมูลจาก Scope ภายนอกอาจต้องใช้ `move` เพื่อย้าย Ownership เข้า Closure
2. Shared State ที่ถูกแก้ไขจากหลาย Thread ต้องมีการ Synchronization ที่เหมาะสม เช่น `Mutex`
3. `MutexGuard` จะคืน Lock เมื่อหลุดออกจาก Scope
4. Type ที่ถูกส่งหรือแชร์ข้าม Thread ต้องเป็นไปตามข้อกำหนดของ `Send` / `Sync`
5. `Rc<T>` ไม่เหมาะสำหรับการแชร์ Ownership ข้าม Thread ควรใช้ `Arc<T>` ในกรณีดังกล่าว

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — Thread Creation & Join

**Purpose:** สาธิตการสร้าง Thread และรอ Thread ด้วย `.join()`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("[Spawned Thread] {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=2 {
        println!("[Main Thread] {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();

    println!("Spawned Thread finished.");
}
```

**Expected Output**

```text
[Main Thread] 1
[Spawned Thread] 1
[Main Thread] 2
[Spawned Thread] 2
[Spawned Thread] 3
Spawned Thread finished.
```

> ลำดับของ Output อาจแตกต่างกันได้ตาม Thread Scheduling

**Explanation**

Main Thread และ Spawned Thread สามารถทำงานสลับกันได้ และ `.join()` ทำให้ Main Thread รอ Spawned Thread ให้เสร็จ

---

### Example 2 — Shared State with `Arc<Mutex<T>>`

**Purpose:** สาธิตการแชร์ข้อมูลและแก้ไขข้อมูลร่วมกันระหว่างหลาย Thread

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();

            *num += 1;

            println!(
                "Thread {} -> Counter = {}",
                i, *num
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Final Counter = {}",
        *counter.lock().unwrap()
    );
}
```

**Expected Output**

```text
Thread 0 -> Counter = 1
Thread 1 -> Counter = 2
Thread 2 -> Counter = 3
...
Thread 9 -> Counter = 10
Final Counter = 10
```

**Explanation**

`Arc` ทำให้หลาย Thread สามารถถือ Reference ไปยัง Counter เดียวกัน ส่วน `Mutex` ทำหน้าที่ป้องกันไม่ให้หลาย Thread แก้ไข Counter พร้อมกันในช่วงเวลาเดียวกัน

---

## 7. Common Mistakes

Concurrency เป็นหัวข้อที่มีความซับซ้อน เนื่องจากโปรแกรมสามารถมีหลาย Thread ที่ทำงานพร้อมกันและเข้าถึง Resource ร่วมกันได้ ดังนั้นการเข้าใจข้อผิดพลาดที่พบบ่อยจึงเป็นส่วนสำคัญของการเรียนรู้ Rust Concurrency

---

### Mistake 1 — Forget `move`

**Problem**

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    thread::spawn(|| {
        println!("{:?}", v);
    });
}
```

Code ลักษณะนี้อาจไม่ Compile เนื่องจาก Closure พยายาม Borrow `v` จาก Scope ภายนอก ในขณะที่ Thread ที่สร้างขึ้นอาจมี Lifetime ยาวนานกว่า Scope ดังกล่าว

**Correct Code**

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    thread::spawn(move || {
        println!("{:?}", v);
    });
}
```

**Why?**

Keyword `move` ทำให้ Closure รับ Ownership ของ `v` เข้าไปเอง

```text
Before:

Main Scope
    │
    └── v


After move:

Main Scope
    │
    └── v ──move──> Thread Closure
```

อย่างไรก็ตาม `move` ไม่ได้หมายความว่า Type นั้นสามารถใช้ข้าม Thread ได้เสมอไป เพราะ Closure ที่ส่งให้ `thread::spawn()` ยังต้องเป็นไปตามข้อกำหนดด้าน `Send` และ Lifetime ของ API ด้วย

> **Key Idea:** `move` เกี่ยวข้องกับการถ่ายโอน Ownership เข้า Closure ส่วน `Send` เกี่ยวข้องกับความปลอดภัยของการส่ง Type ข้าม Thread

---

### Mistake 2 — Using `Rc<T>` Across Threads

**Problem**

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(5);

    thread::spawn(move || {
        println!("{}", data);
    });
}
```

Code นี้ไม่สามารถใช้ `Rc<T>` ข้าม Thread ได้ เนื่องจาก `Rc<T>` ไม่ได้ออกแบบมาให้ Reference Counting ทำงานแบบ Atomic

**Correct Code**

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(5);

    thread::spawn(move || {
        println!("{}", data);
    });
}
```

`Arc<T>` หรือ **Atomic Reference Counting** เหมาะสำหรับการแชร์ Ownership ของข้อมูลระหว่าง Thread

```text
Single Thread
     │
    Rc<T>
     │
     ▼
Non-Atomic Reference Counting


Multiple Threads
     │
    Arc<T>
     │
     ▼
Atomic Reference Counting
```

> **Key Idea:** `Rc<T>` เหมาะกับ Single-threaded Context ส่วน `Arc<T>` เหมาะกับการแชร์ Ownership ระหว่าง Thread

---

### Mistake 3 — Deadlock จากการล็อค Mutex หลายตัวสลับลำดับ

**Deadlock** เกิดขึ้นเมื่อ Thread ตั้งแต่สองตัวขึ้นไปต่างรอ Resource ที่อีกฝ่ายถืออยู่ ทำให้ไม่มี Thread ใดสามารถดำเนินการต่อได้

ตัวอย่างสถานการณ์:

```text
Thread A                 Thread B
   │                        │
   ├── Lock A               │
   │                        ├── Lock B
   │                        │
   ├── รอ Lock B            ├── รอ Lock A
   │                        │
   └──────── DEADLOCK ──────┘
```

ตัวอย่าง Code ที่มีความเสี่ยง:

```rust
// Thread A
let a_guard = a.lock().unwrap();
let b_guard = b.lock().unwrap();

// Thread B
let b_guard = b.lock().unwrap();
let a_guard = a.lock().unwrap();
```

หาก Thread A ถือ `A` แล้วรอ `B` ในขณะที่ Thread B ถือ `B` แล้วรอ `A` จะเกิด Circular Wait

### แนวทางป้องกัน

กำหนด **Lock Ordering** ให้เหมือนกันในทุก Thread

```text
Thread A: Lock A → Lock B

Thread B: Lock A → Lock B
```

หรือหลีกเลี่ยงการถือหลาย Lock พร้อมกันหากไม่จำเป็น

สามารถพิจารณา `try_lock()` ในสถานการณ์ที่เหมาะสมเพื่อตรวจสอบว่า Lock พร้อมใช้งานหรือไม่ แทนการรอแบบ Blocking

> **Best Practice:** ถือ `MutexGuard` ให้นานเท่าที่จำเป็น และหลีกเลี่ยงการล็อคหลายตัวพร้อมกันโดยไม่มีเหตุผลที่จำเป็น

---

### Mistake 4 — Mutex Poisoning

หาก Thread ที่กำลังถือ `Mutex` เกิด `panic!` ขึ้นมาในขณะที่ถือ Lock อยู่ Mutex อาจเข้าสู่สถานะ **Poisoned**

Rust ใช้กลไกนี้เพื่อแจ้งให้ Thread อื่นทราบว่า Thread ก่อนหน้าเกิด Panic ขณะถือ Lock ซึ่งอาจหมายความว่าข้อมูลภายใน Mutex อยู่ในสถานะที่ผู้พัฒนาอาจต้องตรวจสอบก่อนใช้งานต่อ

ตัวอย่าง:

```rust
use std::sync::Mutex;

fn main() {
    let lock = Mutex::new(10);

    let result = lock.lock();

    match result {
        Ok(value) => {
            println!("Value = {}", *value);
        }
        Err(poisoned) => {
            println!("Mutex was poisoned.");
            let value = poisoned.into_inner();
            println!("Recovered Value = {}", *value);
        }
    }
}
```

รูปแบบการทำงาน:

```text
Thread A
   │
   ├── lock()
   │
   ├── modify data
   │
   └── panic!
          │
          ▼
    Mutex Poisoned
          │
          ▼
Thread B
   │
   └── lock()
          │
          ├── Ok(...)
          │
          └── Err(PoisonError)
```

`into_inner()` สามารถใช้เพื่อดึงข้อมูลภายในออกมาได้ แต่ผู้พัฒนาควรพิจารณาก่อนว่าข้อมูลยังอยู่ในสถานะที่สามารถใช้งานต่อได้อย่างถูกต้องหรือไม่

> **Key Idea:** Mutex Poisoning เป็นกลไกแจ้งเตือนเกี่ยวกับ Panic ที่เกิดขึ้นขณะถือ Lock ไม่ใช่การป้องกัน Data Race โดยตรง

---

## 8. Exercises

ส่วนนี้ใช้ทดสอบความเข้าใจเกี่ยวกับ Concurrency โดยเน้น 2 รูปแบบสำคัญของ Rust ได้แก่ **Message Passing** และ **Shared State**

---

### Exercise 1 — Parallel Processing ด้วย MPSC Channel

#### 🎯 Problem

กำหนด Vector:

```text
[1, 2, 3, 4, 5]
```

แบ่งข้อมูลออกเป็น 2 ส่วน และสร้าง Thread อย่างน้อย 2 Thread เพื่อคำนวณค่ากำลังสอง `x²`

ผลลัพธ์จากแต่ละ Thread ต้องถูกส่งกลับมายัง Main Thread ผ่าน **MPSC Channel**

#### Requirements

โปรแกรมต้อง:

1. สร้าง `mpsc::channel()`
2. สร้าง Thread อย่างน้อย 2 Thread
3. แบ่งข้อมูลให้แต่ละ Thread รับผิดชอบ
4. คำนวณ `x²`
5. ส่งผลลัพธ์ผ่าน `Sender`
6. Main Thread รับข้อมูลผ่าน `Receiver`
7. ใช้ `drop(tx)` เพื่อปิด Sender ตัวหลัก
8. รวบรวมผลลัพธ์และแสดงผล

#### 💡 Concept

```text
              numbers
                 │
        ┌────────┴────────┐
        ▼                 ▼
    Thread 1           Thread 2
    [1, 2, 3]           [4, 5]
        │                 │
     x² │              x² │
        │                 │
        └───────┬─────────┘
                ▼
          MPSC Channel
                │
                ▼
           Main Thread
                │
                ▼
        Squared Results
```

#### ✅ Solution

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let numbers = vec![1, 2, 3, 4, 5];
    let chunk_size = (numbers.len() + 1) / 2;

    for chunk in numbers.chunks(chunk_size) {
        let tx_clone = tx.clone();
        let chunk_vec = chunk.to_vec();

        thread::spawn(move || {
            for num in chunk_vec {
                tx_clone.send(num * num).unwrap();
            }
        });
    }

    // ปิด Sender ตัวหลัก
    // เพื่อให้ Receiver สามารถทราบได้ว่าไม่มี Sender เหลืออยู่
    drop(tx);

    let mut results = Vec::new();

    for squared in rx {
        results.push(squared);
    }

    println!("Squared Results: {:?}", results);
}
```

#### Expected Result

ค่าที่ได้ควรเป็นสมาชิกของ:

```text
[1, 4, 9, 16, 25]
```

ลำดับอาจแตกต่างกันได้ เช่น:

```text
Squared Results: [1, 4, 9, 16, 25]
```

หรือ:

```text
Squared Results: [1, 4, 16, 25, 9]
```

เนื่องจาก Thread ทำงานแบบ Concurrent และลำดับการส่ง Message ไม่ได้ถูกกำหนดให้เหมือนลำดับของ Input เสมอไป

#### 🔎 PPL Analysis

ตัวอย่างนี้แสดงแนวคิดสำคัญหลายด้าน:

| PPL Concept             | การใช้งาน                                          |
| ----------------------- | -------------------------------------------------- |
| **Concurrency**         | มีหลาย Thread ทำงานพร้อมกัน                        |
| **Ownership**           | `chunk_vec` ถูกย้ายเข้า Thread                     |
| **Closure**             | ใช้ `move` Closure                                 |
| **Message Passing**     | ใช้ `mpsc::channel()`                              |
| **Type System**         | Compiler ตรวจสอบข้อกำหนดของข้อมูลที่ส่งข้าม Thread |
| **Resource Management** | `drop(tx)` จัดการ Lifetime ของ Sender              |

---

### Exercise 2 — Safe Concurrent Counter ด้วย `Arc<Mutex<T>>`

#### 🎯 Problem

สร้างระบบ **Request Counter** สำหรับนับจำนวน Request ที่เกิดขึ้นพร้อมกัน

ให้สร้าง Thread จำนวน **5 Threads**

แต่ละ Thread ต้องเพิ่ม Counter:

```text
100 ครั้ง
```

ดังนั้นค่าที่คาดหวังคือ:

```text
5 Threads × 100 Requests = 500
```

#### Requirements

โปรแกรมต้อง:

1. ใช้ `Arc<Mutex<i32>>`
2. สร้าง Thread จำนวน 5 Threads
3. แต่ละ Thread เพิ่ม Counter 100 ครั้ง
4. ใช้ `Mutex` ควบคุมการเข้าถึง Counter
5. เก็บ `JoinHandle` ของแต่ละ Thread
6. ใช้ `.join()` รอทุก Thread
7. ตรวจสอบว่าค่าสุดท้ายเท่ากับ `500`

#### 💡 Concept

```text
                  Counter = 0
                      │
                Arc<Mutex<i32>>
                      │
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
    Thread 1      Thread 2       Thread 3
     +100           +100           +100
        │             │             │
        └─────────────┼─────────────┘
                      │
                Thread 4 +100
                      │
                Thread 5 +100
                      │
                      ▼
                 Counter = 500
```

#### ✅ Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    // รอให้ทุก Thread ทำงานเสร็จ
    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = *counter.lock().unwrap();

    println!("Final Counter Value: {}", final_value);

    assert_eq!(final_value, 500);
}
```

#### Expected Output

```text
Final Counter Value: 500
```

หากโปรแกรมทำงานถูกต้อง:

```rust
assert_eq!(final_value, 500);
```

จะไม่เกิด Panic

#### 🔎 PPL Analysis

ตัวอย่างนี้แสดงการทำงานร่วมกันของหลายแนวคิด:

```text
        Arc
         │
         ▼
   Shared Ownership
         │
         ▼
       Mutex
         │
         ▼
   Exclusive Access
         │
         ▼
   Shared Counter
```

**`Arc<T>`**

ช่วยให้หลาย Thread สามารถถือ Ownership ของ Object เดียวกันได้ โดยใช้ Atomic Reference Counting

**`Mutex<T>`**

ควบคุมการเข้าถึงข้อมูลภายใน เพื่อให้ Thread ที่กำลังแก้ไขข้อมูลได้รับ Exclusive Access

**`move`**

ย้าย `counter_clone` เข้าไปใน Closure ของ Thread

**`join()`**

ทำให้ Main Thread รอจน Worker Threads ทำงานเสร็จ

---

### 🧠 Message Passing vs Shared State

Exercises ทั้งสองข้อแสดง Concurrency 2 รูปแบบหลัก:

| รูปแบบ              | Exercise   | กลไกหลัก          |
| ------------------- | ---------- | ----------------- |
| **Message Passing** | Exercise 1 | `mpsc::channel()` |
| **Shared State**    | Exercise 2 | `Arc<Mutex<T>>`   |

```text
                 Concurrency
                     │
          ┌──────────┴──────────┐
          ▼                     ▼
   Message Passing         Shared State
          │                     │
     mpsc::channel()       Arc<Mutex<T>>
          │                     │
          ▼                     ▼
     ส่งข้อมูลระหว่าง       แชร์ข้อมูลร่วมกัน
        Thread                อย่างปลอดภัย
```

ทั้งสองแนวทางมีจุดประสงค์เพื่อให้หลาย Thread ทำงานร่วมกัน แต่มีวิธีจัดการข้อมูลแตกต่างกัน

> **หลักสำคัญ:** ควรเลือก Message Passing หรือ Shared State ตามลักษณะของปัญหาและรูปแบบการไหลของข้อมูล

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

การวิเคราะห์ Rust Concurrency ในมุมมองของ PPL ไม่ควรพิจารณาเพียง Syntax ของ `thread::spawn()` หรือ `Mutex` เท่านั้น แต่ควรพิจารณาถึงความสัมพันธ์ระหว่าง **Syntax, Semantics, Type System, Ownership, Memory Management และ Concurrency Safety**

---

### 9.1 Syntax

Concurrency ใน Rust ใช้ Syntax หลายส่วน เช่น:

```rust
thread::spawn(move || {
    // concurrent code
});
```

Construct ที่เกี่ยวข้องได้แก่:

* Function Call
* Closure
* `move`
* Method Call
* Generic Type
* Smart Pointer
* Trait
* Pattern Matching

ตัวอย่าง:

```rust
let handle = thread::spawn(move || {
    println!("Hello from thread");
});
```

สามารถวิเคราะห์ Syntax ได้เป็น:

```text
thread::spawn()
      │
      └── Function Call
              │
              ▼
          move Closure
              │
              ▼
        Concurrent Code
```

---

### 9.2 Semantics

**Semantics** อธิบายว่า Syntax เหล่านั้นมีพฤติกรรมอย่างไรเมื่อโปรแกรมทำงาน

ตัวอย่าง:

```rust
thread::spawn(|| {
    println!("Hello");
});
```

Semantics ของ `thread::spawn()` คือสร้าง Thread ใหม่เพื่อดำเนินการ Closure แยกจาก Thread ที่เรียกใช้งาน

ส่วน:

```rust
handle.join().unwrap();
```

มีความหมายว่าผู้เรียกจะรอให้ Thread ที่เกี่ยวข้องทำงานเสร็จ และผลลัพธ์ของ `join()` จะบอกว่าการทำงานของ Thread สำเร็จหรือเกิด Panic

สำหรับ:

```rust
mutex.lock()
```

Semantics คือพยายามขอ Lock เพื่อเข้าถึงข้อมูลภายใน `Mutex`

เมื่อ Lock ถูกปล่อย เช่น `MutexGuard` หลุดออกจาก Scope ระบบจะคืนสิทธิ์ในการเข้าถึงให้ Thread อื่น

---

### 9.3 Type System: `Send` และ `Sync`

Rust ใช้ **Type System** เป็นกลไกสำคัญในการควบคุมความปลอดภัยของ Concurrent Programs

Traits ที่สำคัญคือ:

```text
Send
Sync
```

ทั้งสองเป็น **Auto Traits** ซึ่ง Compiler สามารถอนุมานคุณสมบัติจากองค์ประกอบของ Type ได้ในหลายกรณี

---

#### `Send`

`Send` หมายถึง Type นั้นสามารถ **ย้าย Ownership ไปยัง Thread อื่นได้อย่างปลอดภัย**

ตัวอย่าง:

```rust
let message = String::from("Hello");

std::thread::spawn(move || {
    println!("{}", message);
});
```

Ownership ของ `message` ถูกย้ายเข้าไปใน Closure ที่ทำงานบน Thread ใหม่

Type ทั่วไปที่เป็น `Send` ได้แก่:

```text
i32
String
Vec<T> เมื่อ T เป็น Send
```

อย่างไรก็ตาม ไม่ใช่ทุก Type จะเป็น `Send`

ตัวอย่าง:

```rust
use std::rc::Rc;

let data = Rc::new(10);
```

`Rc<T>` ไม่เหมาะสำหรับการส่ง Ownership ข้าม Thread

หากต้องการ Reference Counting สำหรับหลาย Thread สามารถใช้:

```rust
use std::sync::Arc;

let data = Arc::new(10);
```

> **Key Idea:** `Send` เกี่ยวข้องกับ **การย้าย Ownership ข้าม Thread**

---

#### `Sync`

`Sync` หมายถึง Type สามารถถูก **แชร์ผ่าน Reference (`&T`) ระหว่าง Thread ได้อย่างปลอดภัย**

ความสัมพันธ์สำคัญคือ:

```text
T: Sync
   ↓
&T เป็น Send
```

กล่าวในเชิงนิยามได้ว่า:

> `T` เป็น `Sync` ก็ต่อเมื่อ `&T` เป็น `Send`

ตัวอย่างเช่น:

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
```

`Mutex<T>` ทำหน้าที่ควบคุมการเข้าถึงข้อมูลภายใน ทำให้หลาย Thread สามารถแชร์ `Mutex` ได้ตามข้อกำหนดของ Type System

> **Key Idea:** `Sync` เกี่ยวข้องกับ **การแชร์ Reference ข้าม Thread**

---

### 9.4 `Send` vs `Sync`

| Trait  | ความหมาย                                   | คำถามที่ใช้ทำความเข้าใจ                             |
| ------ | ------------------------------------------ | --------------------------------------------------- |
| `Send` | ย้าย Ownership ข้าม Thread ได้อย่างปลอดภัย | **ส่งค่าไป Thread อื่นได้หรือไม่?**                 |
| `Sync` | แชร์ Reference ข้าม Thread ได้อย่างปลอดภัย | **หลาย Thread ใช้ Reference นี้ร่วมกันได้หรือไม่?** |

สามารถจำง่าย ๆ:

```text
Send
 ↓
Move Ownership
 ↓
Thread อื่น


Sync
 ↓
Share Reference (&T)
 ↓
หลาย Thread
```

---

### 9.5 Ownership & Closure Semantics: `move`

Rust ใช้ **Ownership** ร่วมกับ Closure เพื่อควบคุม Lifetime ของข้อมูลที่ถูกนำไปใช้ใน Thread

ตัวอย่าง:

```rust
use std::thread;

fn main() {
    let message = String::from("Hello from Rust!");

    thread::spawn(move || {
        println!("{}", message);
    });
}
```

Keyword:

```rust
move
```

บังคับให้ Closure รับ Ownership ของตัวแปรที่ถูก Capture ตามกฎของ Closure

สามารถแสดงได้ดังนี้:

```text
Main Thread
    │
    │ Ownership
    ▼
  message
    │
    │ move
    ▼
Thread Closure
```

### ทำไมเรื่องนี้สำคัญ?

Thread ที่สร้างขึ้นอาจมี Lifetime ยาวนานกว่า Scope ของ Function ที่สร้าง Thread

หาก Closure ถือ Reference ที่มี Lifetime ไม่เพียงพอ อาจเกิดปัญหาเช่น:

* Dangling Reference
* Use-After-Free
* Invalid Memory Access

Rust จึงใช้ Borrow Checker ตรวจสอบ Lifetime และ Ownership ก่อน Compile

> **Key Idea:** `move` ทำให้ความสัมพันธ์ระหว่าง **Closure, Ownership และ Thread Lifetime** ชัดเจนขึ้น

---

### 9.6 Memory Model & Compile-Time Data Race Prevention

#### Data Race คืออะไร?

Data Race เกิดขึ้นเมื่อมีเงื่อนไขสำคัญครบ 3 ข้อ:

1. มี Thread อย่างน้อย 2 Thread เข้าถึง Memory Location เดียวกัน
2. มีอย่างน้อย 1 Thread ทำการเขียนข้อมูล
3. ไม่มี Synchronization ที่เหมาะสม

แนวคิด:

```text
Thread A ── Write ──┐
                    ├──> Shared Memory
Thread B ── Read ───┘
                    ↑
             ไม่มี Synchronization
```

---

### Ownership Rule กับ Concurrency

Rust ใช้ Ownership และ Borrowing Rules เพื่อควบคุมการเข้าถึง Memory

หลักสำคัญของ Borrowing คือแนวคิด:

> **Aliasing XOR Mutability**

กล่าวอย่างง่ายคือ โดยทั่วไปไม่สามารถมี Mutable Reference พร้อมกับ Shared References ที่ขัดแย้งกันในช่วงเวลาเดียวกันได้

ตัวอย่าง:

```rust
let mut value = 10;

let reference = &mut value;
*reference += 1;
```

เมื่อแนวคิดนี้ทำงานร่วมกับ `Send` และ `Sync` จึงช่วยป้องกัน Concurrent Access Patterns ที่ไม่ปลอดภัยจำนวนมากตั้งแต่ Compile Time

---

### 9.7 Fearless Concurrency

แนวคิดนี้นำไปสู่สิ่งที่ Rust มักเรียกว่า:

> **Fearless Concurrency**

ภาพรวม:

```text
Ownership
     +
Borrowing
     +
Lifetime
     +
Type System
     +
Send / Sync
     ↓
Compile-Time Safety
     ↓
Safer Concurrency
```

Rust จึงไม่ได้อาศัยเพียง Runtime Synchronization เท่านั้น แต่ใช้ **Type System และ Compiler** เป็นส่วนหนึ่งของกลไกด้านความปลอดภัย

---

### 9.8 Abstraction

Rust สร้าง Abstraction สำหรับ Concurrency ผ่านหลายรูปแบบ:

```text
Concurrency
│
├── Threads
│
├── Message Passing
│   └── mpsc
│
└── Shared State
    ├── Arc
    └── Mutex
```

Abstraction เหล่านี้ช่วยให้ Programmer สามารถจัดการ Concurrent Program ในระดับที่เหมาะสม โดยไม่จำเป็นต้องจัดการ Hardware Synchronization โดยตรง

---

### 9.9 Resource Management

Concurrency เชื่อมโยงกับ Memory และ Resource Management ผ่าน:

* Ownership
* Borrowing
* Scope
* Smart Pointers
* RAII
* `Drop`

ตัวอย่างเช่น:

```rust
let mut data = mutex.lock().unwrap();

*data += 1;
```

ตัวแปร `data` ในที่นี้เป็น `MutexGuard`

เมื่อ `data` หลุดออกจาก Scope ระบบจะเรียกกลไก `Drop` และคืน Lock ให้ Mutex

```text
lock()
  │
  ▼
MutexGuard
  │
  │ use data
  ▼
Scope ends
  │
  ▼
Drop
  │
  ▼
Unlock
```

จึงช่วยลดความจำเป็นในการจัดการ Lock ด้วยตนเอง

---

### 9.10 PPL Concept Summary

สามารถสรุปความสัมพันธ์ของแนวคิดต่าง ๆ ได้ดังนี้:

```text
                    Rust Concurrency
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
      Type System      Ownership       Synchronization
          │               │                │
     Send / Sync       move / Borrow    Mutex / Channel
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                  Memory Safety
                          │
                          ▼
                  Safer Concurrency
```

ดังนั้น Rust Concurrency จึงเป็นตัวอย่างที่ดีของการนำแนวคิดจาก Programming Language Concepts หลายด้านมาทำงานร่วมกัน ได้แก่:

```text
Syntax
  +
Semantics
  +
Type System
  +
Ownership
  +
Borrowing
  +
Lifetime
  +
Resource Management
  +
Abstraction
  ↓
Concurrency Safety
```

---

## 10. Rust vs Other Languages

Concurrency ไม่ได้มีเพียงรูปแบบเดียวในแต่ละภาษา แต่แต่ละภาษามี Execution Model, Type System, Memory Management และ Synchronization Mechanism ที่แตกต่างกัน

การเปรียบเทียบนี้พิจารณา Rust, Go, Java และ C++

| มิติ                        | 🦀 Rust                                                                            | 🐹 Go                                                                                             | ☕ Java                                                           | ⚙️ C++                                                                     |
| --------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- | -------------------------------------------------------------------------- |
| **Execution Model**         | `std::thread` ใช้ OS Threads ตาม Platform                                          | Goroutines จัดการโดย Go Runtime และ Multiplex ไปยัง OS Threads                                    | Platform Threads และ Virtual Threads ใน Java รุ่นใหม่            | `std::thread` ใช้ OS Threads ตาม Platform                                  |
| **Data Race Prevention**    | Ownership + Borrow Checker + `Send`/`Sync` ช่วยป้องกันหลายกรณีตั้งแต่ Compile Time | Compiler ไม่ได้ป้องกัน Data Race ทั้งหมด ต้องใช้ Synchronization และเครื่องมือ เช่น Race Detector | ต้องออกแบบ Synchronization ให้ถูกต้อง และมีเครื่องมือช่วยตรวจสอบ | ต้องออกแบบ Synchronization อย่างถูกต้อง มิฉะนั้นอาจเกิด Undefined Behavior |
| **Synchronization**         | `Arc<Mutex<T>>`, `mpsc::channel` และ Primitive ใน `std::sync`                      | Channels, `sync.Mutex`, `sync.WaitGroup` เป็นต้น                                                  | `synchronized`, Locks, Concurrent Collections เป็นต้น            | `std::mutex`, `std::atomic`, `std::condition_variable` เป็นต้น             |
| **Memory Management**       | Ownership / Borrowing / RAII ไม่มี GC                                              | Garbage Collector                                                                                 | Garbage Collector                                                | RAII / Manual Resource Management ไม่มี GC                                 |
| **Thread Safety**           | ฝังอยู่ใน Type System หลายส่วน                                                     | ใช้ Language + Runtime + Synchronization Tools                                                    | ใช้ Type System + Synchronization API + Runtime                  | พึ่งพาการออกแบบและการใช้ Synchronization อย่างถูกต้อง                      |
| **Concurrency Abstraction** | Threads, Channels, Mutex และ Library อื่น                                          | Goroutines + Channels                                                                             | Threads, Virtual Threads, Executors, Concurrent Collections      | Threads, Atomics, Mutex และ Library อื่น                                   |

> **หมายเหตุ:** ตารางนี้เป็นการเปรียบเทียบเชิงแนวคิด ไม่ได้หมายความว่าภาษาใดมีความสามารถด้าน Concurrency เพียงรูปแบบเดียว เพราะแต่ละภาษามี Runtime, Standard Library และ Abstraction หลายระดับ

---

### 10.1 Rust

Rust ใช้แนวคิด:

```text
Ownership
+
Borrowing
+
Type System
+
Send / Sync
```

เพื่อช่วยให้ Compiler ตรวจสอบความปลอดภัยของ Concurrent Program หลายประเภทตั้งแต่ Compile Time

ตัวอย่าง:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from Rust thread!");
    });

    handle.join().unwrap();
}
```

---

### 10.2 Go

Go มีแนวคิด **Goroutine** ซึ่งเป็นหน่วยการทำงานที่เบากว่า OS Thread และถูกจัดการโดย Go Runtime

ตัวอย่าง:

```go
package main

import (
    "fmt"
    "sync"
)

func main() {
    var wg sync.WaitGroup

    wg.Add(1)

    go func() {
        defer wg.Done()
        fmt.Println("Hello from Go goroutine!")
    }()

    wg.Wait()
}
```

Go ยังมี Channel สำหรับ Message Passing:

```text
Goroutine
    │
    ▼
 Channel
    │
    ▼
Goroutine
```

อย่างไรก็ตาม Go Compiler ไม่ได้ป้องกัน Data Race ทุกกรณี จึงยังต้องใช้ Synchronization และเครื่องมือช่วยตรวจสอบตามความเหมาะสม

---

### 10.3 Java

Java รองรับ Concurrency ผ่านหลายระดับ เช่น:

* Platform Threads
* Virtual Threads
* `synchronized`
* Locks
* Executors
* Concurrent Collections
* Atomic Classes

ตัวอย่างพื้นฐาน:

```java
public class Main {
    public static void main(String[] args)
            throws InterruptedException {

        Thread thread = new Thread(() -> {
            System.out.println("Hello from Java thread!");
        });

        thread.start();
        thread.join();
    }
}
```

Java ใช้ Garbage Collector สำหรับ Memory Management ดังนั้น Programmer ไม่ต้องจัดการ Memory แบบ Manual ในลักษณะเดียวกับ C++

---

### 10.4 C++

C++ มี Thread และ Synchronization Primitive ผ่าน Standard Library เช่น:

```text
std::thread
std::mutex
std::atomic
std::condition_variable
```

ตัวอย่าง:

```cpp
#include <iostream>
#include <thread>

void task() {
    std::cout << "Hello from C++ thread!";
}

int main() {
    std::thread t(task);

    t.join();

    return 0;
}
```

C++ ให้ Programmer ควบคุม Resource และ Memory ได้อย่างยืดหยุ่น แต่ความปลอดภัยหลายส่วนขึ้นอยู่กับการออกแบบและการใช้งานของ Programmer

---

### 10.5 Rust vs C++ ในมุมมอง PPL

| Aspect                   | Rust                                                        | C++                                                                   |
| ------------------------ | ----------------------------------------------------------- | --------------------------------------------------------------------- |
| **Syntax**               | `thread::spawn`, `Arc`, `Mutex`, `mpsc`                     | `std::thread`, `std::mutex`, `std::atomic`                            |
| **Semantics / Behavior** | Concurrency เชื่อมโยงกับ Ownership และ Type System          | Programmer ควบคุม Thread และ Resource โดยตรงมากกว่า                   |
| **Type System**          | มี Ownership, Borrowing, `Send`, `Sync`                     | มี Static Type System แต่ไม่มีกลไก Ownership/Borrow Checking แบบ Rust |
| **Memory Management**    | Ownership + Borrowing + RAII                                | RAII + Manual Resource Management                                     |
| **Safety**               | Compiler ช่วยตรวจสอบ Memory และ Concurrency Safety หลายกรณี | ความปลอดภัยหลายส่วนขึ้นอยู่กับการออกแบบและการใช้งานของ Programmer     |
| **Abstraction**          | Ownership-based concurrency abstractions                    | Library-based concurrency abstractions                                |

### Analysis

Rust และ C++ ต่างสามารถสร้าง Thread และใช้ Synchronization Primitive ได้ แต่แนวทางการออกแบบภาษาแตกต่างกัน

Rust เชื่อมโยงแนวคิดด้าน Memory Safety และ Concurrency Safety เข้ากับ **Ownership, Borrowing และ Type System**

ในขณะที่ C++ ให้อิสระแก่ Programmer ในการจัดการ Memory และ Resource มากกว่า โดยมี Library และ RAII เป็นกลไกสำคัญ

ดังนั้นความแตกต่างเชิง PPL ที่สำคัญไม่ได้อยู่เพียง Syntax ของการสร้าง Thread แต่รวมถึง **วิธีที่ภาษาใช้ Type System และ Resource Model เพื่อกำหนดพฤติกรรมที่ปลอดภัยของโปรแกรม**

---

### 10.6 Comparison Summary

สามารถสรุปแนวคิดหลักได้ดังนี้:

```text
Rust
│
├── Ownership
├── Borrowing
├── Send / Sync
└── Compile-Time Safety


Go
│
├── Goroutines
├── Channels
└── Runtime + Synchronization


Java
│
├── Threads
├── Virtual Threads
├── Executors
└── Runtime / GC


C++
│
├── std::thread
├── std::mutex
├── std::atomic
└── Programmer-managed Resource
```

จุดประสงค์ของการเปรียบเทียบไม่ใช่การระบุว่าภาษาใดดีกว่าภาษาอื่น แต่เพื่อแสดงให้เห็นว่า **แต่ละภาษาใช้ Language Design และ Runtime Model ที่แตกต่างกันในการแก้ปัญหา Concurrency**

---

### 📌 Overall PPL Relationship

```text
                  Programming Language Concepts
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
      Syntax              Semantics            Type System
        │                     │                     │
   spawn / move          Thread Behavior       Send / Sync
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              ▼
                         Ownership
                              │
                         Borrowing
                              │
                           Lifetime
                              │
                              ▼
                      Memory Management
                              │
                              ▼
                       Synchronization
                              │
                 ┌────────────┴────────────┐
                 ▼                         ▼
          Message Passing            Shared State
                 │                         │
            mpsc::channel()          Arc<Mutex<T>>
                 │                         │
                 └────────────┬────────────┘
                              ▼
                    Concurrent Programs
                              │
                              ▼
                     Safer Concurrency
```

> **Final Key Idea:** Rust Concurrency เป็นตัวอย่างที่ชัดเจนของการนำแนวคิดจาก Programming Language Concepts มาผสานกัน โดยเฉพาะ **Type System, Ownership, Borrowing, Lifetime, Resource Management และ Abstraction** เพื่อช่วยให้ Compiler สามารถตรวจสอบปัญหาด้าน Memory และ Thread Safety ได้ตั้งแต่ก่อน Runtime

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 — นายกรวิชญ์ บุญชู | Concept + Short Code Illustration | 5 min |
| Member 2 — นายผกาย เมืองแมน | Detailed Code + Live Demo | 5 min |
| Member 3 — นางสาวศศิธร โตนาม | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 — นายธนภัทร คงหอม | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

Concept, Introduction และ Short Code Illustration

**Member 2**

Detailed Code และ Live Demo ของ `mpsc` / `Arc<Mutex<T>>`

**Member 3**

Rust vs Other Language และ PPL Analysis

**Member 4**

Exercises, Common Mistakes และ Challenge

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

1. [The Rust Programming Language — Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
2. [Rust Documentation — `std::thread`](https://doc.rust-lang.org/std/thread/)
3. [Rust Documentation — `std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/)
4. [Rust Documentation — `std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
5. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | ช่วยจัดโครงสร้างเอกสารและอธิบายแนวคิด | ตรวจสอบกับ Rust Documentation และทดลอง Compile / Run |
| `Claude` | ช่วยเรียบเรียงและตรวจสอบเนื้อหา | สมาชิกตรวจสอบเนื้อหาและ Code ด้วยตนเอง |

### Declaration

- [x] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [x] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [x] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [x] ระบุการใช้ AI อย่างโปร่งใส

### รายละเอียดการใช้ AI

AI ถูกใช้เพื่อช่วยจัดระเบียบโครงสร้างเอกสารตาม Template ช่วยเรียบเรียงคำอธิบาย และช่วยตรวจสอบรูปแบบของ Code และ Markdown

สมาชิกกลุ่มเป็นผู้ตรวจสอบผลลัพธ์ด้วยตนเอง และทดสอบ Code ด้วย Cargo ก่อนนำมาใช้ใน Tutorial และการนำเสนอ

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| นายกรวิชญ์ บุญชู | 2 | 3 | 1 | 2 | Concept + Short Code |
| นายผกาย เมืองแมน | 2 | 4 | 1 | 2 | Detailed Code + Live Demo |
| นางสาวศศิธร โตนาม | 2 | 3 | 1 | 2 | Rust vs Other Language + PPL |
| นายธนภัทร คงหอม | 2 | 3 | 1 | 2 | Exercises + Common Mistakes |

### Teamwork Reflection

**How did your team collaborate?**

สมาชิกแบ่งงานผ่าน GitHub Issues และพัฒนาใน Branch ของตนเอง จากนั้นรวมผลงานผ่าน Pull Request

**Problems encountered**

ปัญหาหลักคือการจัดโครงสร้าง Tutorial ให้ครอบคลุมทั้งเนื้อหา Rust และการวิเคราะห์ในมุมมอง PPL รวมถึงการตรวจสอบ Code ตัวอย่างให้สามารถ Compile และ Run ได้จริง

**How did you solve them?**

สมาชิกช่วยกันตรวจสอบ Code, ทบทวนเนื้อหา และทำ Code Review ผ่าน Pull Request ก่อน Merge เข้า Branch หลัก

---

## 15. Final Checklist

### Tutorial

- [x] Learning Objectives
- [x] Introduction
- [x] Key Concepts
- [x] Important Syntax / Rules
- [x] Runnable Code Examples
- [x] Common Mistakes
- [x] Exercises **2 ข้อ**
- [x] PPL Perspective
- [x] Rust vs Other Language
- [x] Teach Your Topic
- [x] References
- [x] AI Usage Declaration
- [x] GitHub Contribution

### Code

- [x] Code อยู่ใน Cargo Project
- [x] Code สามารถ Compile ได้
- [x] Code สามารถ Run ได้
- [x] ทดสอบ Runnable Examples แล้ว

### GitHub

- [x] Issues
- [x] Branches
- [x] Commits
- [x] Pull Requests
- [x] Code Reviews
- [x] Merge เข้า Branch หลัก

### Presentation

- [x] Member 1 — Concept + Short Code
- [x] Member 2 — Detailed Code + Live Demo
- [x] Member 3 — Rust vs Other Language + PPL Analysis
- [x] Member 4 — Exercises + Common Mistakes + Challenge
- [ ] เตรียม Live Demo
- [ ] เตรียม Q&A

---

## 📌 Submission Information

| Item | Information |
|---|---|
| **Topic** | 22 — Concurrency & Threads |
| **Course** | 517321 Principles of Programming Languages |
| **Group** | 22 |
| **Submission** | 4 October 2026, 23:59 |
| **Repository** | `[https://github.com/soonklang/rust-tutorial-2569/tree/main/22-concurrency-threads]` |
| **Pull Request** | `#[26]` |

---

<div align="center">

### 🦀 Concurrency & Threads in Rust

**517321 Principles of Programming Languages**

*Fearless Concurrency through Ownership, Types and Abstraction.*

</div>