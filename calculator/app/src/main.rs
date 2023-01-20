use std::io::Stdin;

use calculator::Calculator;

mod calculator;
mod operations;

pub enum Operation {
    Add,
    Subtraction,
    Multiplication,
    Division,
    Clear,
}

fn input_data(
    input: &Stdin,
    result: f64,
    result_clear: bool,
) -> Result<(f64, Operation, f64), &'static str> {
    let mut buf = String::new();
    let first;
    if result_clear {
        println!("Please enter the first number: ");

        input
            .read_line(&mut buf)
            .expect("Failed to read the string!");

        first = match read_number(&buf.clone()) {
            Ok(res) => res,
            Err(_) => return Err("Reading string error"),
        };
    } else {
        first = result.clone();
        println!("Note: if you want to clear the calculator result type \'c\'");
    }

    buf.clear();
    println!("Please enter the operation: ");

    input
        .read_line(&mut buf)
        .expect("Failed to read the string!");

    if buf.chars().count() > 3 {
        return Err("To much symbols in string");
    }

    let operation = match buf.trim().chars().next().unwrap_or('0') {
        '+' => Operation::Add,
        '-' => Operation::Subtraction,
        '*' => Operation::Multiplication,
        '/' => Operation::Division,
        'c' => return Ok((first, Operation::Clear, 0.0)),
        _ => return Err("Unknown operation"),
    };

    buf.clear();
    println!("Please enter the second number: ");

    input
        .read_line(&mut buf)
        .expect("Failed to read the string!");

    let second = match read_number(&buf.clone()) {
        Ok(res) => res,
        Err(_) => return Err("Reading string error"),
    };

    return Ok((first, operation, second));
}

fn read_number(buf: &String) -> Result<f64, ()> {
    let result;
    // if buf.contains('.') || buf.contains(',') || float_flag {
    result = match buf.trim().parse::<f64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(()),
    };

    // } else {
    //     result = match buf.trim().parse::<i32>() {
    //         Ok(n) => Ok(NumberType::Integer(n)),
    //         Err(_) => Err("Error parsing int"),
    //     };
    // }

    return result;
}

fn main() {
    let mut calc: Calculator<f64> = Calculator::new();

    println!("//////////////////////////////////////////////////////");
    println!("\nPlease welcome to Calculator on Rust!\n");
    println!("//////////////////////////////////////////////////////\n");

    let input = std::io::stdin();
    let mut result = 0.0;
    let mut result_clear = true;
    loop {
        let numbers = input_data(&input, result, result_clear);

        match numbers {
            Err(str) => {
                println!("Error: {str}");
                continue;
            }
            Ok((first, operation, second)) => {
                calc.parse_operation(first, &operation, second);
                if let Operation::Clear = operation {
                    result_clear = true;
                    result = 0.0;
                    continue;
                }
            }
        };
        
        result_clear = false;

        if !result_clear {
            result = calc.get_result().clone();
        }
        println!("Result is {}", calc.get_result());
    }
}
