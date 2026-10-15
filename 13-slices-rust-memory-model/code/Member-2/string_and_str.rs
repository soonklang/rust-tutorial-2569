// pub fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];}}
//     &s[..]}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];}}
    &s[..]}

fn main() {
    let string = String::from("Silpakorn University");
    let literal = "Silpakorn University";
    let slice = &string[..13];

    // string.clear();

    println!("first word String: {}", first_word(&string));
    println!("first word literal: {}", first_word(&literal));
    println!("first word slice: {}", first_word(&slice));
    
    let th = String::from("รองเท้าเนรคุณ");
        println!("---------- {} ----------", th);
    println!("len : {} bytes", th.len());
    println!("char count : {} chars", th.chars().count());
    // println!("first letter : {}", &th[0..1]); // panic

    println!("---------- is char boundary ----------");
    println!("1 byte : {}", th.is_char_boundary(1));
    println!("3 bytes : {}", th.is_char_boundary(3));
    println!("first letter : {}", &th[0..3]);

    println!("---------- fat pointer ownership ----------");
    let mut s1 = String::from("ญี่ปุ่น");
    let s2 = "มาแล้ว";
    s1.push_str(s2);
    println!("{}", s1);
    println!("s2 is {s2}");
}
