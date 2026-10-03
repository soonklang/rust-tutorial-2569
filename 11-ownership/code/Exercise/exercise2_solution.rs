struct Book {
    title: String,
    author: String,
}
 
fn make_label(title: String) -> String {
    format!("[{}]", title)
}
 
fn main() {
    let books = vec![
        Book { title: String::from("Rust programming"), author: String::from("Graydon Hoare") },
        Book { title: String::from("C programming"), author: String::from("Dennis Ritchie") },
    ];
 
    for book in books {
        let label = make_label(book.title.clone());
        println!("{}", label);
        println!("by {} - full title: {}", book.author, book.title);
    }
}