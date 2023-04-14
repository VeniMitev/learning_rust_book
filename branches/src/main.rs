// Generate the nth Fibonacci number.
fn main() {
    let n = 10;
    println!("The {}th Fibonacci number is {}", n, fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}