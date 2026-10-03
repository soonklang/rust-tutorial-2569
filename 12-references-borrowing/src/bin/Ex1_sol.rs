fn main() {
    let mut names = vec![String::from("Noah"), String::from("Oliver"), String::from("James")];

    let first = &names[0];
    println!("Hosted by: {}", first);
    names.push(String::from("Quinn"));
    
    let mut add = false;
    for name in &names {
        if name == "Oliver" {
            add = true;
        }
    }

    if add {
        names.push(String::from("Charlotte"));
    }
    println!("{:?}", names);
}