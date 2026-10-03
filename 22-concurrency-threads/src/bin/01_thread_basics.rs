use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 1. Basic Thread Spawn & Join ===");
    
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("[Spawned Thread] ลำดับที่: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("[Main Thread] ลำดับที่: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();
    println!("[Main Thread] Spawned Thread ทำงานเสร็จสิ้นแล้ว\n");

    println!("=== 2. Thread with `move` Closure ===");
    let data = vec![10, 20, 30];

    let join_handle = thread::spawn(move || {
        println!("[Data Thread] รับ Vector มาใช้งาน: {:?}", data);
    });

    join_handle.join().unwrap();
}