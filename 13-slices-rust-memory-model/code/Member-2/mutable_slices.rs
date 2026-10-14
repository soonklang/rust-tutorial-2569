pub fn transform_even_odd(slice: &mut [i32]) {
    for x in slice.iter_mut() {
        if *x % 2 == 0 {
            *x *= 2;
        } else {
            *x -= 1;
}}}

fn main() {
    let mut num = [1, 2, 3, 4, 5, 6];
    println!("before:  {:?}", num);
    transform_even_odd(&mut num);
    println!("after: {:?}", num);

    // let s1 = &mut num[..2];
    // let s2 = &mut num[1..];   // error

    // transform_even_odd(s1);
    // transform_even_odd(s2);

    let (left, right) = num.split_at_mut(3);
    // println!("numbers : {:?}", num); // error
    left[0] = 111;
    right[0] = 999;
    println!("left : {:?}", left);
    println!("right : {:?}", right);
    // println!("numbers : {:?}", num);
}
