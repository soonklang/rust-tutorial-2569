mod student;
mod utils;

use student::{calculate_grade, display_result};
use utils::{print_header, print_sep};

fn main() {
    print_header("Student Grade Demo");

    let Achiraya_score = 85.0;
    let Somchai_score = 45.0;

    display_result("Achiraya", Achiraya_score);

    print_sep();

    display_result("Somchai", Somchai_score);
}
