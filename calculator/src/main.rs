use std::io::Stdin;

use calculator::Calculator;

mod calculator;
mod operations;

pub enum Operations {
    Add,
    Subtraction,
    Multiplication,
    Division,
}

pub enum NumberType {
    Integer(i32),
    Float(f64),
}

impl NumberType {
    fn get_integer(self) -> i32 {
        if let NumberType::Integer(num) = self {
            num
        } else {
            panic!("No value!")
        }
    }

    fn get_float(self) -> f64 {
        if let NumberType::Float(num) = self {
            num
        } else {
            panic!("No value!")
        }
    }
}

fn input_data(input: Stdin) -> Result<(NumberType, NumberType), &'static str> {
    let mut buf = String::new();
    let mut float_flag = false;
    println!("Please enter the first number: ");

    input
        .read_line(&mut buf)
        .expect("Failed to read the string!");

    let mut first = match read_number(&buf.clone(), float_flag) {
        Ok(res) => res,
        Err(str) => return Err("Reading string error"),
    };

    buf.clear();
    println!("Please enter the second number: ");

    input
        .read_line(&mut buf)
        .expect("Failed to read the string!");

    if let NumberType::Float(x) = first {
        float_flag = true;
    }

    let mut second = match read_number(&buf.clone(), float_flag) {
        Ok(res) => res,
        Err(str) => return Err("Reading string error"),
    };

    return Ok((first, second));
}

fn read_number(buf: &String, float_flag: bool) -> Result<NumberType, &str> {
    let result;
    if buf.contains('.') || buf.contains(',') || float_flag {
        result = match buf.trim().parse::<f64>() {
            Ok(n) => Ok(NumberType::Float(n)),
            Err(_) => Err("Error parsing float"),
        };
    } else {
        result = match buf.trim().parse::<i32>() {
            Ok(n) => Ok(NumberType::Integer(n)),
            Err(_) => Err("Error parsing int"),
        };
    }
    return result;
}

fn main() {
    let mut a: Calculator<f64> = Calculator::new();

    println!("Please welcome to Calculator on Rust!");

    let input = std::io::stdin();

    let numbers = input_data(input);
    // let first = match numbers {
    //     Err(str) => {
    //         println!("Error: {str}");
    //     }
    //     Ok((left, right)) => {
           
    //     }
    // };

    a.parse_operation(2.0, 3.2);
    println!("Result is {}", a.get_result());
}
