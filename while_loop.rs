fn main() {
    loop {
        println!("Again");
        break;
    }

    let mut number = 8;
    while number != 0 {
        if number > 3 {
            println!("Number {}", number);
        }

        number = number - 1;
    }
}
