// Result Enum ถูก Build-in มาในภาษา หน้าตาเป็นแบบนี้:
// enum Result<T, E> { Ok(T), Err(E), }

fn main() {
    // การแปลง String เป็นตัวเลข อาจเกิด Error ได้
    let parse_result: Result<i32, _> = "100a".parse(); 

    match parse_result {
        Ok(number) => println!("แปลงสำเร็จ ได้เลข: {}", number),
        Err(e) => println!("แปลงไม่สำเร็จ เกิดข้อผิดพลาด: {}", e),
    }
}