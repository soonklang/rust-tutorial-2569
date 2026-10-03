/*
#Excercise 2 : Stock Checker
Problem : เขียน enum ชื่อ Product มี 2 variants ได้แก่ Snack(ใช้ Struct-variant) และ Drink(ใช้ Tuple-variant) ทั้งคู่เก็บชื่อ,ราคา,จำนวนคงเหลือ จากนั้นเขียนฟังก์ชัน check_availbility ที่คืนราคาถ้าสินค้ายังมีสต็อก และคืน None ถ้าสินค้าหมด แล้วแสดงผลลัพธ์ทั้งหมดโดยใช้ if let
Hint : ให้ price เป็นทศนิยม(f64) และ stock เป็นจำนวนเต็ม(u32)
*/
enum Product {
    Snack{name:String ,price: f64 ,stock: u32},   //struct-like
    Drink(String ,f64 ,u32),   //tuple-like
}
fn check_availability(product: &Product) -> Option<f64> {
    match product {
        Product::Snack{price , stock , ..} => {   //ดึง price และ stock จาก field โดยใช้ชื่อ และไม่สนใจค่าที่เหลือ
            if *stock > 0 {
                Some(*price)
            }
            else {
                None
            }
        }
        Product::Drink(_ , price , stock) => {   //ดึงค่าตามตำแหน่ง และใช้_เพื่อไม่สนใจค่าตัวแรก
            if *stock > 0 {
                Some(*price)
            }
            else {
                None
            }
        }
    }
}
fn main() {
    let items = vec![
        Product::Snack{name:String::from("Chips"), price: 20.0, stock: 5},
        Product::Snack{name:String::from("Cookie"), price: 22.5, stock: 0},
        Product::Drink(String::from("Cocoa"), 39.0, 1),
        Product::Drink(String::from("Water"), 7.0, 4),
    ];
    for item in &items {
        let name = match item {
            Product::Snack{name, ..} => name,   //ใน struct-like ใช้ .. หมายถึงไม่สนใจค่าที่เหลือ
            Product::Drink(name ,_ ,_) => name,   //ใน tuple-like ใช้ _ หมายถึงไม่สนใจค่าที่ตำแหน่งนั้น
        };
        if let Some(price) = check_availability(item) {
            println!("{}: There are items price at {} baht", name, price);
        } else {
            println!("{}: Out of stock", name);
        }
    }
}