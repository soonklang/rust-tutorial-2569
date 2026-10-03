use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== MPSC Channel Example ===");

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("ข้อความที่ 1 จาก Thread 1"),
            String::from("ข้อความที่ 2 จาก Thread 1"),
        ];
        for msg in messages {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("ข้อความที่ 1 จาก Thread 2"),
            String::from("ข้อความที่ 2 จาก Thread 2"),
        ];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(30));
        }
    });

    for received in rx {
        println!("[Consumer Received]: {}", received);
    }
}