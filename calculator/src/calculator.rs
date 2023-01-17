use crate::operations::{Add, Division, Multiplication, Subtraction};
use crate::Operations;

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

    pub fn parse_operation(&mut self, first: i32, second: i32, operation: Operations) {
        self.result = match operation {
            Add => Some(Calculator::add(first, second)),
            Subtraction => Some(Calculator::sub(first, second)),
            Multiplication => Some(Calculator::multiply(first, second)),
            Division => Some(Calculator::divide(first, second)),
            _ => todo!(),
        }
    }
}

impl Add<i32> for Calculator {
    type Result = i32;

    fn add(first: i32, other: i32) -> Self::Result {
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
