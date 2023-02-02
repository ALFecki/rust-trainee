
use crate::polynomial::Polynomial;

mod polynomial;


fn main() {

    let g = Polynomial::new("x^2 - 2x + 3x - 24").unwrap();
    println!("{g}");
    let b = Polynomial::new("y^3 + y^2 + 6 - 7y").unwrap();
    println!("{b}");
    let c = g * b;

    println!("{c}");

}
