pub fn calculate_grade(score: f32) -> &'static str {
    if score >= 80.0 {
        "A"
    } else if score >= 70.0 {
        "B"
    } else if score >= 60.0 {
        "C"
    } else if score >= 50.0 {
        "D"
    } else {
        "F"
    }
}

fn get_message(score: f32) -> &'static str {
    if score >= 80.0 {
        "Excellent!"
    } else if score >= 50.0 {
        "Passed"
    } else {
        "Failed"
    }
}

pub fn display_result(name: &str, score: f32) {
    println!("Student: {}", name);
    println!("Score: {}", score);
    println!("Grade: {}", calculate_grade(score));
    println!("Status: {}", get_message(score));
}
