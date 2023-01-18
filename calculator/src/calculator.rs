use crate::operations::{Add, Division, Multiplication, Subtraction, self};
use crate::Operations;

pub struct Calculator<T: num::Num> {
    result: Option<T>,
}

impl<T: num::Num> Calculator<T> {
    pub fn new() -> Self {
        Self { result: None }
    }

    pub fn get_result(self) -> T {
        return self.result.unwrap();
    }

    pub fn parse_operation(&mut self, first: T, second: T/*, operation: T*/) {
    // where T: Add + Subtraction + Multiplication + Division {
        self.result = Some(first.add(second));
        // self.result = match operation {
        //      => Some(first.add(second)),
        //     Subtraction => Some(Calculator::sub(first, second)),
        //     Multiplication => Some(Calculator::multiply(first, second)),
        //     Division => Some(Calculator::divide(first, second)),
        //     _ => todo!(),
        // }
    }
}

impl<T: num::Num> Add for T {
    type Result = T;

    fn add(self, other: T) -> Self::Result {
        return self + other;
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
