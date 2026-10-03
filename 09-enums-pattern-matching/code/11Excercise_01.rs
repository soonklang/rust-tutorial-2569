/*
#Excercise 1 : Calculate Shape Area
Problem : เขียน enum ชื่อ Shape มี variant 3 ตัว ได้แก่ Circle, Rectangle, Triangle จากนั้นเขียนฟังก์ชัน area ที่รับ Shape และคำนวณพื้นที่แต่ละรูปโดยใช้ match
Hint :  พื้นที่วงกลม = π × r²
        พื้นที่สี่เหลี่ยม = กว้าง × ยาว
        พื้นที่สามเหลี่ยม = 0.5 × ฐาน × สูง
*/
enum Shape {
    Circle(f64),   //tuple-like
    Rectangle(f64, f64),   //tuple-like
    Triangle { base: f64, height: f64 },   //struct-like
}
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,   //std::f64::consts::PI หมายถึง ค่าคงที่ของ π (ประมาณ 3.14)
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}
fn main() {
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle { base: 5.0, height: 6.0 };
    println!("Circle = {:.2}", area(&circle));   //แสดงผลลัพธ์เป็นทศนิยม 2 ตำแหน่ง
    println!("Rectangle = {:.2}", area(&rectangle));
    println!("Triangle = {:.2}", area(&triangle));
}