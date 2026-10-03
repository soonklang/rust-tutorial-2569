use std::io;
#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

fn create_discount(discount: f64) -> impl Fn(f64) -> f64 {
    move |price| price * (1.0 - discount)
}
fn calculate(
    products: &[Product],
    operation: impl Fn(f64) -> f64,
) {
    for product in products {
        let new_price = operation(product.price);

        println!(
            "{} : {:.2} -> {:.2}",
            product.name,
            product.price,
            new_price
        );
    }
}
fn input(message: &str) -> String {
    let mut value: String = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut value).unwrap();
    value.trim().to_string()
}

fn main() {
    let name1 = input("Product 1 name:");
    let price1: f64 = input("Product 1 price:").parse().unwrap();
    let name2 = input("Product 2 name:");
    let price2: f64 = input("Product 2 price:").parse().unwrap();
    let name3 = input("Product 3 name:");
    let price3: f64 = input("Product 3 price:").parse().unwrap();

    let discount: f64 = input("Discount (%):").parse().unwrap();

    let products = [
        Product {
            name: name1,
            price: price1,
        },
        Product {
            name: name2,
            price: price2,
        },
        Product {
            name: name3,
            price: price3,
        },
    ];

    let discount_fn = create_discount(discount / 100.0);

    calculate(&products, discount_fn);
}


//let mut input = String::new();
//io::stdin().read_line(&mut input).unwrap();
//let age: i32 = input.trim().parse().unwrap();

