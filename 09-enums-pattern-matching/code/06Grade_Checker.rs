use std::collections::HashMap;
// [1] แปลงคะแนนเป็นเกรด 
fn get_grade(score: i32) -> Option<char> {
    match score {
        80..=100 => Some('A'),
        70..=79 => Some('B'),
        60..=69 => Some('C'),
        0..=59 => Some('D'),
        _ => None, //คะแนนติดลบหรือเกินช่วง
    }
}
//[2] รายงานผลคะแนน
fn report(db: &HashMap<&str, i32>, name: &str) {
    // ชั้นที่ 1: ค้นหาคะแนนจากรายชื่อนักศึกษา
    match db.get(name) { 
        None => println!("{:<11} : Error! Not found", name), //ไม่พบชื่อ = ไม่มีเกรด (none)
        //ขั้นที่ 2 : พบรายชื่อตรวจสอบเกรดที่่ได้
        Some(&score) => { match get_grade(score) {
                Some(g) => println!("{:<11} : {} points [grade {}]", name, score, g),
                None => println!("{:<11} : Warning! {} points --> Incorrect scores", name, score),
            }
        }
    }
}
//[3] การเรียกใช้งานจริง
fn main() {
    println!("=========== Grade Checker ===========");
//รายชื่อที่เก็บไว้พร้อมคะแนนรายบุคคล
    let db = HashMap::from([
        ("Sanruethai", 85),
        ("Sakaodeuan", 50),
        ("Pancheewa", 67),
        ("Thaenrak", 150),
    ]);
 //รายชื่อที่จะทำการค้นหา
    let names = ["Sanruethai", "Sakaodeuan", "Pancheewa", "Thaenrak", "Mether"];
    for name in names {
        report(&db, name); //รายงานผล
    }
}
