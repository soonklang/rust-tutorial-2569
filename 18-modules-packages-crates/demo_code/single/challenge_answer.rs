// Challenge (เฉลยโจทย์เติมคำ ___ = pub)  -> พิมพ์ 12.56
mod shapes {
    pub mod circle {
        pub fn area(r: f64) -> f64 {
            3.14 * r * r
        }
    }
}

use shapes::circle;

fn main() {
    println!("{:.2}", circle::area(2.0));
}
