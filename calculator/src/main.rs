use calculator::Calculator;

mod calculator;
mod operations;

pub enum Operations {
    Add,
    Subtraction,
    Multiplication,
    Division
}

fn main() {
    let mut a = Calculator::new();
    a.parse_operation(2, 3, Operations::Add);
    println!("Result is {}", a.get_result());

}
