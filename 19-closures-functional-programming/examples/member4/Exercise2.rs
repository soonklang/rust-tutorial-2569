fn create_filter<T, V, P>(
    property: impl Fn(&T) -> V,
    condition: P,
) -> impl Fn(&T) -> bool
where
    P: Fn(V) -> bool,
{
    move |item| {
        let value = property(item);
        condition(value)
    }
}

fn main() {
    let products = vec![
        ("Laptop", 1200),
        ("Mouse", 25),
        ("Keyboard", 75),
    ];

    let is_price_over_50 =
        create_filter(|product: &(&str, i32)| product.1, |price| price > 50);

    let result: Vec<_> = products
        .iter()
        .filter(|product| is_price_over_50(product))
        .collect();

    println!("{:?}", result);
}
