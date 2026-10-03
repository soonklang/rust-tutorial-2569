fn longest_line(text: &String) -> String {
    let mut best = "";
    for line in text.lines() {
        if line.len() > best.len() {
            best = line;
        }
    }
    best.to_string()
}

fn get_report() -> String {
    let text = String::from("short\na much longer line here\nmid");
    longest_line(&text)
}

fn main() {
    let report = get_report();
    println!("{}", report);
}