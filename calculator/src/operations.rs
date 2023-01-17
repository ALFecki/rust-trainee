
pub trait Add<Rhs = Self> {
    type Result;

    fn add(first: Rhs, second: Rhs) -> Self::Result;
}

pub trait Subtraction<Rhs = Self> {
    type Result;

    fn sub(first: Rhs, second: Rhs) -> Self::Result;
}

pub trait Multiplication<Rhs = Self> {
    type Result;

    fn multiply(first: Rhs, second: Rhs) -> Self::Result;
}

pub trait Division<Rhs = Self> {
    type Result;

    fn divide(first: Rhs, second: Rhs) -> Self::Result;
}
