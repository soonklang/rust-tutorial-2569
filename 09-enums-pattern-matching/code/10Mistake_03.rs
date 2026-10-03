//Incorrect code
enum Status {
    Success,
    Error,
    Pending,
}
fn show_status(status: Status) {
    match status {
        Status::Success => println!("สถานะสำเร็จ"),
        _ => println!("สถานะอื่น"),
        Status::Error => println!("เกิดข้อผิดพลาด"),
    }
}
fn main() {
    show_status(Status::Error);
}
/* correct code
enum Status {
    Success,
    Error,
    Pending,
}
fn show_status(status: Status) {
    match status {
        Status::Success => println!("สถานะสำเร็จ"),
        Status::Error => println!("เกิดข้อผิดพลาด"),
        _ => println!("สถานะอื่น"),
    }
}
fn main() {
    show_status(Status::Error);
}
*/