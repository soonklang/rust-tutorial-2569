fn main() {
    //Exercise 2 : ใช้ if Expression เพื่อสร้างค่า
    let score = 75;

    let grade = if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else {
        "C"
    };

    println!("{}", grade);
}