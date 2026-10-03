// Challenge (ข้อต่อ): ใส่ pub แค่ที่ mod แต่ลืมที่ fn  -> คาดว่า error E0603
mod shapes {
    pub mod circle {
        fn area(r: f64) -> f64 {
            3.14 * r * r
        }
    }
}

use shapes::circle;

fn main() {
    println!("{:.2}", circle::area(2.0));
}
