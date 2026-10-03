// Mistake 3 (ถูก): ประกาศ mod utils;  -> พิมพ์ สวัสดีจาก utils
mod utils;
use utils::greet;

fn main() {
    greet();
}
