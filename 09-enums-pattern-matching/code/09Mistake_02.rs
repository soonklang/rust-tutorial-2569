//Incorrect code
enum Message {
    Write(String),
}
fn main() {
    let msg = Message::Write(String::from("have a nice day")) ;
    match msg {
        Message::Write(text) => println!("{}",text),
    }
    match msg {
        Message::Write(text) => println!("{}",text),
    }
}
/* correct code
enum Message {
    Write(String),
}
fn main() {
    let msg = Message::Write(String::from("have a nice day")) ;
    match &msg {
        Message::Write(text) => println!("{}",text),
    }
    match &msg {
        Message::Write(text) => println!("{}",text),
    }
}
*/