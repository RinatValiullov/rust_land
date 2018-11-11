fn main() {
    let str = String::from("Hello, world!");

    let hello = &str[0..9];

    println!("{}", hello);

    let arr = [1, 2, 3, 4, 5, 6];

    let slice_arr = &arr[2..5];

    println!("{:?}", slice_arr);

    let arr_len = arr.len(); // get the length of an array
    println!("{}", arr_len);
}
