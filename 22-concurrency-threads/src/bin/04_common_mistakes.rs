use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Common Concurrency Mistakes & Solutions ===");

    // Mistake 1 Solution: ใช้ `move` Closure
    let v = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("Mistake 1 Solution: {:?}", v);
    });
    handle1.join().unwrap();

    // Mistake 2 Solution: ใช้ Arc<T> แทน Rc<T>
    let arc_val = Arc::new(Mutex::new(5));
    let arc_clone = Arc::clone(&arc_val);
    let handle2 = thread::spawn(move || {
        let mut guard = arc_clone.lock().unwrap();
        *guard += 10;
    });
    handle2.join().unwrap();
    println!("Mistake 2 Solution: Value = {}", *arc_val.lock().unwrap());
}