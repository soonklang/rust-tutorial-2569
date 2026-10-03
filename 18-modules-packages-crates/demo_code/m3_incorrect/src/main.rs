// Mistake 3 (ผิด): ลืม mod utils;  -> คาดว่า error E0432
use crate::utils::greet;

fn main() {
    greet();
}
