fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7];
    
    match &nums[..] {
        [first, .., last] => println!("first / last : {first} / {last}"),
        [only] => println!("only one: {only}"),
        [] => println!("empty"),
    }
    // println!("first / last : {:?} / {:?}", nums.first(), nums.last());
    match &nums[..] {
        [first, rest @ ..] => println!("first / rest : {first} / {rest:?}"),
        [] => {}
    }
    println!(".get(10) : {:?}", nums.get(10));
    println!(".split_at(3) : {:?}", nums.split_at(3));
    println!(".chunks(3) : {:?}", nums.chunks(3).collect::<Vec<_>>());
    println!(".windows(3) : {:?}", nums.windows(3).collect::<Vec<_>>());
    println!(".contains(&4) : {}", nums.contains(&4));
 
    let owned: Vec<i32> = nums[..3].to_vec();
    println!("to_vec : {:?}", owned);
}
