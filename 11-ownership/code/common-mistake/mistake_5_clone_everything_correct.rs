struct User {
    name: String,
    email: String,
    bio: String,
}

fn print_name(name: &str) {      // ยืมแค่ &str แทนที่จะรับ owned String
    println!("{}", name);
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@su.ac.th"),
        bio: String::from("no bio added"),
    };

    print_name(&user.name);      // แค่ยืม ไม่ต้อง clone
    println!("{}", user.name);   // ยังใช้งานได้ปกติ เพราะไม่มีอะไรถูกย้ายหรือลบไปไหน
}