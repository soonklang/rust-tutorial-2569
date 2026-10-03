fn main() {
    let x = 4;

    let result = {
        let y = x + 2;

        if y > 5 {
            y * 2
        } else {
            y - 1
        }
    };

    println!("{}", result);
}
