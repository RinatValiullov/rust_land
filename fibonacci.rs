fn main() {
    let result_5 = fib(5);
    let result_9 = fib(9);
    let result_3 = fib(3);
    let result_11 = fib(11);

    println!("Fibonacci sequence of 5 = {}", result_5);
    println!("Fibonacci sequence of 9 = {}", result_9);
    println!("Fibonacci sequence of 3 = {}", result_3);
    println!("Fibonacci sequence of 11 = {}", result_11);
}

fn fib(num: i32) -> i32 {
    if num <= 1 {
        return num;
    } else {
        let res = fib(num - 1) + fib(num - 2);
        return res;
    }
}
