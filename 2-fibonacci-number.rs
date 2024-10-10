use std::io;

// 1 1 2 3 5 8
fn fibonacci_loop(n : i32) -> i64 {
    let mut result = 1i64;
    let mut previous = 0i64;
    for _i in 1..n {
        let temp : i64 = result;
        result = result + previous;
        previous = temp;
    }
    return result;
}

fn fibonacci_recursion(n : i32) -> i64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fibonacci_recursion(n - 1) + fibonacci_recursion(n - 2);
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);
    input = input.split_whitespace().collect();
    let n = input.parse::<i32>().unwrap();

    println!("input: {}", input);
    println!("n: {}", n);

    let result_loop = fibonacci_loop(n);
    println!("Result from Loop: {}", result_loop);

    let result_recursion = fibonacci_recursion(n);
    println!("Result from Recursion: {}", result_recursion);
}