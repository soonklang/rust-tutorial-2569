//Incorrect
enum Payment {
    Cash,
    CreditCard,
    PromptPay,
}
fn main() {
    let payment = Payment::Cash;

    match payment {
        Payment::Cash => println!("ชำระด้วยเงินสด"),
    }
}
//correct code
/*
enum Payment {
    Cash,
    CreditCard,
    PromptPay,
}
fn main() {
    let payment = Payment::Cash;

    match payment {
        Payment::Cash => println!("ชำระด้วยเงินสด"),
        _ => println!("ชำระด้วยวิธีอื่น"),
    }
}


*/