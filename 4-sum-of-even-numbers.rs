use std::io;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);
    let input_parts = input.split_whitespace().collect::<Vec<&str>>();
    println!("{:?}", input_parts);

    let mut input_values = Vec::<i32>::new();
    for i in input_parts {
        input_values.push(i.parse::<i32>().unwrap());
    }

    println!("{:?}", input_values);

    let mut result : i32 = 0i32;

    for i in input_values {
        if i % 2 == 0 {
            result = result + i;
        }
    }

    println!("Result: {}", result);
}