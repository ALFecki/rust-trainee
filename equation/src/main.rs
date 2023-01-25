
use quadratic::Quadratic;

mod quadratic;
mod biquadratic;


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
    e.print()

}
