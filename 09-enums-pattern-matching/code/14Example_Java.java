// java code
// ประกาศ interface 
interface PaymentStatus {
    void check();
}

// สร้างคลาส Pending 
class Pending implements PaymentStatus {
    
    public void check() {
        System.out.println("กำลังตรวจสอบชำระเงิน...");
    }
}


class Success implements PaymentStatus {
    private int transactionId; 
    public Success(int id) { this.transactionId = id; } // คอนสตรัคเตอร์รับค่า id มาบันทึกไว้

    
    public void check() {
        System.out.println("ชำระเงินสำเร็จ! รหัสสลิป: " + transactionId);
    }
}

// สร้างคลาส
class Failed implements PaymentStatus {
    private String reason; 
    public Failed(String reason) { this.reason = reason; } // คอนสตรัคเตอร์รับข้อความสาเหตุมาบันทึกไว้

    
    public void check() {
        System.out.println("ชำระเงินไม่สำเร็จ: " + reason);
    }
}


public class Main {
    
    public static void main(String[] args) {
        
        PaymentStatus status = new Success(98765);
        
        // เรียกใช้เมธอด 
        status.check();
    }
}