use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Exercise 1: Parallel Sum ===");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let mid = numbers.len() / 2;
    let left_part = numbers[..mid].to_vec();
    let right_part = numbers[mid..].to_vec();

    let h1 = thread::spawn(move || left_part.iter().sum::<i32>());
    let h2 = thread::spawn(move || right_part.iter().sum::<i32>());

    let sum1 = h1.join().unwrap();
    let sum2 = h2.join().unwrap();

    let total = sum1 + sum2;
    println!("ผลรวมทั้งหมด: {} (Expected: 55)", total);
    assert_eq!(total, 55);

    println!("\n=== Exercise 2: Mutex Data Collector ===");
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for id in 0..5 {
        let results_clone = Arc::clone(&results);
        let h = thread::spawn(move || {
            let mut list = results_clone.lock().unwrap();
            list.push(id);
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("ข้อมูลใน Vector หลังจากทุก Thread ทำงานเสร็จ: {:?}", *results.lock().unwrap());
}