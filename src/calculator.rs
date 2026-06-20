pub struct Calculator {
    operands: Vec<String>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator { operands: vec!["+".to_string(), "*".to_string(), "/".to_string(), "-".to_string()] }
    }

    pub fn handle_operand(&self, operand: &str, x: i32, y: i32) -> i32 {
        match operand {
            "+" => add(x, y),
            "-" => sub(x, y),
            "*" => mul(x, y),
            "/" => div(x, y),
            _ => panic!("Invalid operand: {}", operand),
        }
    }

    pub fn get_operands(&self) -> Vec<String> {
        self.operands.clone()
    }
}

fn add(x: i32, y: i32) -> i32 { x + y }
fn sub(x: i32, y: i32) -> i32 { x - y }
fn mul(x: i32, y: i32) -> i32 { x * y }
fn div(x: i32, y: i32) -> i32 { x / y }