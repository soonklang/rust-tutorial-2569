// rust
// ประกาศ enum 
enum PaymentStatus {
    Pending,                 // กำหนดสถานะ
    Success(u32),            
    Failed(String),          
}

// ประกาศฟังก์ชัน รับค่า status 
fn check_payment(status: PaymentStatus) {
    
    // ตรวจสอบค่าของ status ว่าตรงกับสถานะไหน
    match status {
        PaymentStatus::Pending => println!("กำลังตรวจสอบชำระเงิน..."),
        PaymentStatus::Success(id) => println!("ชำระเงินสำเร็จ! รหัสสลิป: {id}"),
        PaymentStatus::Failed(reason) => println!("ชำระเงินไม่สำเร็จ: {reason}"),
    }
}


fn main() {
    
    let status = PaymentStatus::Success(98765);
    
    // เรียกใช้ฟังก์ชัน 
    check_payment(status);
}