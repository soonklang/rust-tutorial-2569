mod food {
    pub struct Menu {
        pub name: String,
    }

    pub fn order(menu: &Menu) {
        println!("Order: {}", menu.name);
    }
}

fn main() {
    let menu = food::Menu {
        name: String::from("Pizza"),
    };

    food::order(&menu);
}