// ========================================
//            key 1: iterator
// ========================================
fn key_concept_1() {
    let numbers = [1, 2, 3];

    // Create an iterator for the array
    let mut iter = numbers.iter();

    // Get the next element
    println!("{:?}", iter.next()); //Some(1)
    println!("{:?}", iter.next()); //Some(2)
    println!("{:?}", iter.next()); //Some(3)
    println!("{:?}", iter.next()); //None
}

// ========================================
//             key 2: .map
// ========================================
fn key_concept_2() {
    let numbers = [1, 2, 3];

    // Multiply every element by 2
    let result: Vec<i32> = numbers
        // Create an iterator
        .iter()
        // Transform each element by multiplying it by 2
        .map(|x| x * 2)
        // Collect the results into a Vec<i32>
        .collect();

    println!("{:?}", result);
}

// ========================================
//            key 3: .filter
// ========================================
fn key_concept_3() {
    let numbers = [1, 2, 3, 4, 5];

    // Keep only even numbers
    let result: Vec<i32> = numbers
        // Create an iterator
        .iter()
        // keep only even number
        .filter(|x| **x % 2 == 0)
        // Convert &i32 references into i32 values
        .copied()
        // Collect the results into a Vec<i32>
        .collect();

    println!("{:?}", result);
}

// ========================================
//            key 4: closure
// ========================================
fn key_concept_4() {
    // Create a closure that adds 10
    let add_ten = |x| x + 10;

    println!("{}", add_ten(5));
}

// ========================================
//      key 5: High order programming
// ========================================
fn apply<F>(n: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(n) // Call the function passed as an argument
}

// ========================================
// iterator and High order programming
// ========================================


fn main() {
    println!("========================================");
    println!("key concept 1: iterator");
    key_concept_1();
    println!("========================================");
    println!("key concept 2: .map");
    key_concept_2();
    println!("========================================");
    println!("key concept 3: .filter");
    key_concept_3();
    println!("========================================");
    println!("key concept 4: closure");
    key_concept_4();
    println!("========================================");
    println!("key concept 5: Higher-Order Function");
    //apply() รับ function/closure เป็น parameter → จึงเป็น Higher-Order Function.
    let result = apply(5, |x| x + 1); // Pass a closure to another function
    println!("{}", result); // 6
    println!("========================================");
}
