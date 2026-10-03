//std::fmt::Display
fn main(){
    //ปิดการแสดง warning ของ Rust
    #![allow(unused_must_use)]

    //Mistake 1: ใส่ Semicolon หลัง Expression สุดท้าย
    
    //Incorrect Code
    /*
    let x = {
        5 + 3;
    };
    println!("The value of x is: {:?}", x);
    */

    //correct Code
    /*
    let y = {
        5 + 3
    };
    println!("The value of y is: {:?}", y);
    */

    //Mistake 2: ให้ค่าจาก if แต่ละ branch เป็นคนละชนิด

    //Incorrect Code
    /*
    let condition = true;
    
    let result = if condition {
        10
    } else {
        "ten"
    };
    println!("The value of result is: {:?}", result);
    */

    //correct Code
    /*
    let condition = true;

    let result = if condition {
        10
    } else {
        20
    };
    println!("The value of result is: {:?}", result);
    */
}