
use quadratic::Quadratic;
use regex::Regex;

mod quadratic;


fn main() {
    let a = Quadratic::new("2x^2 + 3x + 1");
    let b = Quadratic::new("3y^2 + 6 + 5y^2 + 8y");
    a.print();
    b.print();
    let mut c = a - b;
    c.print();
    c.add_number(4);
    c.print();


    // let reg = Regex::new(r"(?P<f>\d+\w\^2)|(?P<s>\d+\w)|(?P<t>\d+)").unwrap();
    // // let names = reg.capture_names();
    // let g = reg.captures("2x").unwrap();
    
    // println!("{:?}", &g["s"]);
    // println!("{}", reg.is_match("+ 3x"));

}
