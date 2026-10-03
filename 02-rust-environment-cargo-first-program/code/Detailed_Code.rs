use std::io;

fn main() {
    println!("========================================");
    println!("    Student Score Recorder PPL");
    println!("========================================");

    let mut name = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let mut midterm = String::new();
    println!("Enter Midterm score (35):");
    io::stdin().read_line(&mut midterm).expect("Failed to read line");

    let mut final_exam = String::new();
    println!("Enter Final score (35):");
    io::stdin().read_line(&mut final_exam).expect("Failed to read line");

    let mut kahoot = String::new();
    println!("Enter Kahoot score (12):");
    io::stdin().read_line(&mut kahoot).expect("Failed to read line");

    let mut project = String::new();
    println!("Enter Project score (12):");
    io::stdin().read_line(&mut project).expect("Failed to read line");

    let mut attendance = String::new();
    println!("Enter Attendance score (3):");
    io::stdin().read_line(&mut attendance).expect("Failed to read line");

    let mut typing = String::new();
    println!("Enter Typing score (3):");
    io::stdin().read_line(&mut typing).expect("Failed to read line");

    println!();
    println!("========================================");
    println!("Name       : {}", name.trim());
    println!("Midterm    : {}/35", midterm.trim());
    println!("Final      : {}/35", final_exam.trim());
    println!("Kahoot     : {}/12", kahoot.trim());
    println!("Project    : {}/12", project.trim());
    println!("Attendance : {}/3", attendance.trim());
    println!("Typing     : {}/3", typing.trim());
    println!("========================================");
}