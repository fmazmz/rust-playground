use std::io;
use std::io::BufRead;

fn main() {
    let x: i32 = read_number();
    let y: i32 = read_number();
    let op: &str = &read_line();

    println!("= {}", handle_operand(op, x, y));
}


fn read_number() -> i32 {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let trimmed = line.trim().to_string();

    match trimmed.parse::<i32>() {
        Ok(..) => trimmed.parse().unwrap(),
        Err(..) => panic!("input is not an integer: {}", trimmed),
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let trimmed = line.trim().to_string();

    match trimmed.parse::<String>() {
        Ok(..) => trimmed.parse().unwrap(),
        Err(..) => panic!("input is not an integer: {}", trimmed),
    }
}
fn handle_operand(operand: &str, x: i32, y: i32) -> i32{
    match operand{
        "+" => add(x, y),
        "-" => sub(x, y),
        "*" => mul(x, y),
        "/" => div(x, y),
        _ => panic!("Invalid operand: {}", operand),
    }
}

fn add(x: i32, y: i32) -> i32{
    x + y
}

fn sub(x: i32, y: i32) -> i32{
    x - y
}

fn mul(x: i32, y: i32) -> i32{
    x * y
}

fn div(x: i32, y: i32) -> i32{
    x / y
}

