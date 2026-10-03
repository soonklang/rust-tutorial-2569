fn describe(item: String) -> String {
    format!("Item: {}", item)
}
 
fn main() {
    let item = String::from("Book");
 
    let result = describe(item.clone());   // ส่งสำเนาไป เก็บตัวจริงไว้
    println!("{}", result);
 
    println!("{}", item);   // ยังใช้งานได้ เพราะตัวจริงไม่เคยถูกย้าย
}