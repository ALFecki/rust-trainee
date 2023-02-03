use std::str::FromStr;
use crate::polynomial::Polynomial;

mod polynomial;


fn main() {

    let g = Polynomial::from_str("x^2 - 2x + 3x - 24").unwrap();
    println!("{g}");
    let b = Polynomial::from_str("y^3 + y^2 + 6 - 7y").unwrap();
    println!("{b}");
    let c = g.clone() * b;
    println!("{c}");


    let a = Polynomial::new(vec![1.0, 4.0]);
    println!("{}", a);
    if let Ok(d) = g / a {
        println!("{d}");
    };

}
