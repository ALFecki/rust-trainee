use std::process::Output;


pub trait Add<Rhs> where Rhs: std::ops::Add<Output = Rhs> {
    type Result;

    fn add(first: Rhs, second: Rhs) -> Self::Result;
}

pub trait Subtraction<Rhs = i32> {
    type Result;

    fn sub(first: Rhs, second: Rhs) -> Self::Result;
}

pub trait Multiplication<Rhs = i32> {
    type Result;

    fn multiply(first: Rhs, second: Rhs) -> Self::Result;
}

pub trait Division<Rhs = i32> {
    type Result;

    fn divide(first: Rhs, second: Rhs) -> Self::Result;
}
