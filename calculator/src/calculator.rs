use crate::operations::{Add, Division, Multiplication, Subtraction};
#[derive(PartialEq, PartialOrd)]
pub struct Calculator {
    result: Option<i32>,
}

impl Calculator {
    pub fn new() -> Self {
        Self { result: None }
    }

    pub fn get_result(&self) -> i32 {
        return self.result.unwrap();
    }

    pub fn parse_operation(&mut self, first: i32, second: i32, operation: char) {
        self.result = match operation {
            '+' => Some(Calculator::add(first, second)),
            '-' => Some(Calculator::sub(first, second)),
            '*' => Some(Calculator::multiply(first, second)),
            '/' => Some(Calculator::divide(first, second)),
            _ => todo!(),
        }
    }
}

impl<T> Add<T> for Calculator {
    type Result = T;

    fn add(first: T, other: T) -> Self::Result {
        return first + other;
    }
}

impl Subtraction<i32> for Calculator {
    type Result = i32;

    fn sub(first: i32, second: i32) -> Self::Result {
        return first - second;
    }
}

impl Multiplication<i32> for Calculator {
    type Result = i32;

    fn multiply(first: i32, second: i32) -> Self::Result {
        return first * second;
    }
}

impl Division<i32> for Calculator {
    type Result = i32;

    fn divide(first: i32, second: i32) -> Self::Result {
        return first / second;
    }
}
