fn main() {
    let arr = [4, 5, 6, 7, 8];

    println!("My array = {:?}", arr);

    for element in arr.iter() {
        println!("A double value of array = {}", element * 2);
    }
}
