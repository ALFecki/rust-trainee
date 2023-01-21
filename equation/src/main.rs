use regex::Regex;
use quadratic::Quadratic;

mod quadratic;


fn main() {
    let a = Quadratic::new("2x^2 + 3x + 1");
    let b = Quadratic::new("3y^2 + 6 + 5y^2 + 8y");
    a.print();
    b.print();
    let mut c = a + b;
    c.print();
    c.add_number(5);
    c.print();
}
