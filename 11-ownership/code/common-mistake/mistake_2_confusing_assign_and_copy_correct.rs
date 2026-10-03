fn main() {
    let s1 = String::from("move not copy");
    let s2 = s1.clone();           
    println!("{}", s1);       
}