fn once<F, A, R>(mut f: F) -> impl FnMut(A) -> R
where
    F: FnMut(A) -> R,
    R: Clone,
{
    let mut result: Option<R> = None;

    move |arg| {
        if let Some(value) = &result {
            return value.clone();
        }

        let value = f(arg);
        result = Some(value.clone());

        value
    }
}

fn main() {
    let mut initialize_app = once(|app_name: &str| {
        println!("Initializing {}...", app_name);
        format!("{} is ready", app_name)
    });

    println!("{}", initialize_app("MySystem"));
    println!("{}", initialize_app("OtherSystem"));
}
