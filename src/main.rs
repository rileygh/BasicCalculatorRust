use std::io;

fn main() {
    println!("Welcome to the calculator!");
    println!("Enter an expression using +, -, *, or /, then press Enter:");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input == "q" || input == "quit" {
            break;
        }

        let result = match calculate(&input) {
            Ok(val) => val,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        println!("Result: {}", result);
    }
}

fn calculate(input: &str) -> Result<f64, &'static str> {
    let mut accumulator = 0.0;
    let mut operator = '+';

    for token in input.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => operator = token.chars().next().unwrap(),
            _ => {
                let operand = match token.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => return Err("Invalid number"),
                };

                accumulator = match operator {
                    '+' => accumulator + operand,
                    '-' => accumulator - operand,
                    '*' => accumulator * operand,
                    '/' => accumulator / operand,
                    _ => return Err("Invalid operator"),
                };
            }
        }
    }

    Ok(accumulator)
}