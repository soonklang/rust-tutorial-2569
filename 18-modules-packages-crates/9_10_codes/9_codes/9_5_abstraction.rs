mod food {
    pub fn order() {
        prepare();
        println!("Order: Pizza");
    }

    fn prepare() {
        println!("Preparing...");
    }
}

fn main() {
    food::order();
}