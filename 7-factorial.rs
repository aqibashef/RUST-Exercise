use std::io;

fn factorial_loop(n: i64) -> i64 {
    let mut result = 1i64;
    for i in 1..=n {
        result = result * i;
    }
    return result;
}

fn factorial_recursion(n:i64) -> i64 {
    return if n <= 0 {1} else {n * factorial_recursion(n-1)};
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);
    input = input.split_whitespace().collect();
    let n = input.parse::<i64>().unwrap();
    println!("input: {}", input);
    println!("n: {}", n);

    let result_loop = factorial_loop(n);
    println!("Factorial by Loop: {}", result_loop);

    let result_recursion = factorial_recursion(n);
    println!("Factorial by Recursion: {}", result_recursion);
}