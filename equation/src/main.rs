
use crate::polynomial::Polynomial;

mod polynomial;


fn main() {

    let g = Polynomial::new("x^2 - 2x - 24");
    println!("{g}");
    let b = Polynomial::new("y^2 + 6 - 7y");
    println!("{b}");
    let c = g * b;

    println!("{c}");

}
