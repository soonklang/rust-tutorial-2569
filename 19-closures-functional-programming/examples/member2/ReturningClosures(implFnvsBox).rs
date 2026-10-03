// Static Dispatch (Zero-cost): Compiler รู้ type ตอน compile
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// Dynamic Dispatch (Heap Allocated): รองรับ Closure หลาย type ในค่าคืน
fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    if op == "add" {
        Box::new(|a, b| a + b)
    } else {
        Box::new(|a, b| a * b)
    }
}

fn demo_returning_closures() {
    println!("\n--- [Demo 3] Returning Closures ---");

    let add_five = create_adder(5);
    println!("create_adder(5)(10) = {}", add_five(10));

    let calc = make_operation("multiply");
    println!("make_operation('multiply')(4, 5) = {}", calc(4, 5));
}

fn main() {
    demo_returning_closures();
}
