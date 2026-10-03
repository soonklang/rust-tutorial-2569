fn main() {
    let x = factorial(5);

    print!("{}", x);
}

fn factorial(num: i32) -> i32 {
    let mut result = 1;

    if num == 0 {
        return result;
    } else {
        for i in 1..=num {
            result *= i;
        }

        return result;
    }
}