fn main() {
    let mut str = String::from("Simple string");

    take_ownership(&mut str);

    println!("{}", str);
}

fn take_ownership(some_string: &mut String) {
    println!("{}", some_string);
    some_string.push_str(" - mutated");
}
