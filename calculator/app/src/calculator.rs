use crate::operations::{Division, Multiplication, Subtraction};
use crate::Operation;

use add::Add;

pub struct Calculator<T: num::Num> {
    first: Option<T>,
    second: Option<T>,
    result: Option<T>,
}

impl<T: num::Num + Clone> Calculator<T> {
    pub fn new() -> Self {
        Self { first: None, second: None, result: None }
    }

    pub fn get_result(&self) -> T {
        return self.result.as_ref().unwrap().clone();
    }

    pub fn parse_operation(&mut self, first: T, operation: &Operation, second: T) {
        // where T: Add + Subtraction + Multiplication + Division {

        self.result = match operation {
            Operation::Add => Some(first.add(second)),
            Operation::Subtraction => Some(first.sub(second)),
            Operation::Multiplication => Some(first.multiply(second)),
            Operation::Division => Some(first.divide(second)),
            Operation::Clear => None
        }
    }
}

impl<T: num::Num> Add for Option<T> {
    type Result = Option<T>;

    fn add(self, other: Option<T>) -> Self::Result {
        return self.unwrap() + other;
    }
}

impl<T: num::Num> Subtraction for T {
    type Result = T;

    fn sub(self, second: T) -> Self::Result {
        return self - second;
    }
}

impl<T: num::Num> Multiplication for T {
    type Result = T;

    fn multiply(self, second: T) -> Self::Result {
        return self * second;
    }
}

impl<T: num::Num> Division for T {
    type Result = T;

    fn divide(self, second: T) -> Self::Result {
        return self / second;
    }
}
