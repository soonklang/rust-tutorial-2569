// Exercise 1: module ซ้อนกัน + use ... as
// ผลลัพธ์:
// Circle (r=2): 12.57
// Rectangle (3x4): 12.00
mod shapes {
    pub mod circle {
        pub fn area(r: f64) -> f64 {
            std::f64::consts::PI * r * r
        }
    }

    pub mod rectangle {
        pub fn area(w: f64, h: f64) -> f64 {
            w * h
        }
    }
}

use shapes::circle;
use shapes::rectangle::area as rect_area;

fn main() {
    println!("Circle (r=2): {:.2}", circle::area(2.0));
    println!("Rectangle (3x4): {:.2}", rect_area(3.0, 4.0));
}
