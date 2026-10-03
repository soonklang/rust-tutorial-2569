// it's not incorrect, it just isn't recommeded to always use clone
struct User {
    name: String,
    email: String,
    bio: String,
}

fn print_name(name: String) {   // รับแบบ owned โดยไม่จำเป็น
    println!("{}", name);
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@su.ac.th"),
        bio: String::from("no bio added"),
    };

    print_name(user.name.clone());   // clone() ทั้งที่แค่จะ print เฉยๆ
    println!("{}", user.name);       // ต้องใช้ user.name ต่อ เลย clone ไปก่อน
}