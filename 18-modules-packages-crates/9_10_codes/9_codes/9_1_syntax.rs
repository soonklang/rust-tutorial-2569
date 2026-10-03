mod food {
    pub fn order() {
        println!("Order: Pizza");
    }
}

use crate::food::order;

fn main() {
    order();
}