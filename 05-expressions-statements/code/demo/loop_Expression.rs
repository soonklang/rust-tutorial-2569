fn main() {
    let mut counter = 0;

    // loop เป็น Expression ที่ส่งค่ากลับมาเข้าตัวแปร result ได้โดยตรง
    let result = loop {
        counter += 1;

        if counter == 3 {
            // คืนค่า counter * 10 ออกไปให้ตัวแปร result แล้วหยุด loop ทันที
            break counter * 10;
        }
    };

    println!("The result from loop execution is: {}", result);
}