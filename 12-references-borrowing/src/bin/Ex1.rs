fn main() {}

/* 
 * This program demonstrates a compile error that occurs when trying to modify a vector while holding an immutable reference to one of its elements.
 * The error arises because Rust's borrowing rules prevent mutable and immutable references from coexisting.
 */

 /*
fn main() {
    let mut names = vec![String::from("Noah"), String::from("Oliver"), String::from("James")];

    let first = &names[0];
    names.push(String::from("Quinn"));
    println!("Hosted by: {}", first);

    for name in &names {
        if name == "Oliver" {
            names.push(String::from("Charlotte"));
        }
    }
    println!("{:?}", names);
}

*/

