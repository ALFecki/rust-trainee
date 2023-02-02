
use quadratic::Quadratic;
use crate::polynomial::Polynomial;

mod quadratic;
mod biquadratic;
mod polynomial;


fn main() {
    let a = Quadratic::new("x^2 - 2x - 24");
    let b = Quadratic::new("y^2 + 6 - 7y");
    a.print();
    b.print();

    let f = a + 4;
    f.print();
    
    let c = a - b;
    c.print();

    c.print();

    let d = a * b;
    d.print();

    let e = a / b;
    e.print();

    let g = Polynomial::new("x^2 - 2x - 24");
    g.print();
    let b = Polynomial::new("y^2 + 6 - 7y");
    b.print();
    let c = g * b;
    c.print()

}
