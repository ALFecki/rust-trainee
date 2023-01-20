


pub trait Subtraction<Rhs = Self> {
    type Result;

    fn sub(self, second: Rhs) -> Self::Result;
}

pub trait Multiplication<Rhs = Self> {
    type Result;

    fn multiply(self, second: Rhs) -> Self::Result;
}

pub trait Division<Rhs = Self> {
    type Result;

    fn divide(self, second: Rhs) -> Self::Result;
}
