use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Shared State with Arc<Mutex<T>> ===");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread {} เพิ่มค่า Counter เป็น: {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("=== ผลลัพธ์สุดท้ายของ Counter: {} ===", *counter.lock().unwrap());
}