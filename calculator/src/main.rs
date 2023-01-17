use calculator::Calculator;
use operations::Add;

mod calculator;
mod operations;

fn main() {
    let mut a = calculator::Calculator::new();
    a.parse_operation(2, 3, '+');
    println!("Result is {}", a.get_result());

}
