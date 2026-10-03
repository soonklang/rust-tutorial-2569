fn main() {
    let score = 85;

    // Block Expression: คืนค่า String slice เข้าตัวแปร grade โดยตรง
    let grade = {
        let bonus = 5;
        let total_score = score + bonus;

        // ไม่ใส่ Semicolon (;) เพื่อให้เป็น Expression คืนค่าออกไป
        if total_score >= 80 {
            "A"
        } else if total_score >= 70 {
            "B"
        } else {
            "F"
        }
    };

    println!("Total calculated grade: {}", grade);
}