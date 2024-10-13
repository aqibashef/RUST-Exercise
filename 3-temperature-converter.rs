use std::io;

// fn process_input(input : &str) -> Vec<&str> {
    
// }

fn main() {
    println!("Input Structure: <Value> <From Unit> <To Unit>. i.e. 100 c f");
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input);
    let input_parts = input.split_whitespace().collect::<Vec<&str>>();
    println!("{:?}", input_parts);

    let value : f32 = input_parts[0].parse::<f32>().unwrap();
    println!("{}", value);
    let mut result : f32 = value;

    if input_parts[1] == "c" || input_parts[1] == "C" {
        if input_parts[2] == "f" || input_parts[2] == "F" {
            result = value * 1.8f32 + 32f32;
        } 
    }
    if input_parts[1] == "f" || input_parts[1] == "F" {
        if input_parts[2] == "c" || input_parts[2] == "C" {
            result = (value - 32f32) / 1.8f32;
        }
    }

    println!("Result: {}", result);
}