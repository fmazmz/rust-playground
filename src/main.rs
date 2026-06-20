mod calculator;

use std::io;
use std::io::Write;
use crate::calculator::Calculator;

fn main() {
    print_ascii();
    let calculator = Calculator::new();

    let operands = calculator.get_operands();

    let x = read_number("Enter first number: ");
    let y = read_number("Enter second number: ");
    println!("\nAvailable operations: {:?}", operands.as_slice());
    let op = read_line("Enter operand: ");

    println!("= {}", calculator.handle_operand(&op, x, y));
}

fn print_ascii() {
    let art = r#"
    __________                __   _________        .__
\______   \__ __  _______/  |_ \_   ___ \_____  |  |   ____
 |       _/  |  \/  ___/\   __\/    \  \/\__  \ |  | _/ ___\
 |    |   \  |  /\___ \  |  |  \     \____/ __ \|  |_\  \___
 |____|_  /____//____  > |__|   \______  (____  /____/\___  >
        \/           \/                \/     \/          \/
    "#;

    println!("{}", art);
}

fn read_number(prompt: &str) -> i32 {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Not a valid integer")
}

fn read_line(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

