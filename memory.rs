fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    // println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // We need clone method after move the s1 variable to avoid error

    println!("{}, world!", s1);
    println!("{}, world!", s2);
}
