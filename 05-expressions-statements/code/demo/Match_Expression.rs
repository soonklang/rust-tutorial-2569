fn check_user_role(level: u32) -> &'static str {
    // Early Return: ใช้คำสั่ง return เพื่อคืนค่าและออกจากฟังก์ชันทันที
    if level == 0 {
        return "Guest";
    }

    // match ในฐานะ Expression: คืนค่า String slice ออกจากฟังก์ชันโดยไม่ต้องใช้คำสั่ง return
    match level {
        1 => "Member",
        2 => "Moderator",
        3 => "Admin",
        _ => "Unknown Role",
    }
}

fn main() {
    let user_level = 2;
    let role = check_user_role(user_level);

    println!("User role is: {}", role);
}