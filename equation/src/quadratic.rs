use std::{
    ops::{Add, Mul, Sub, Div}, result, fmt::Error,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::biquadratic::Biquadratic;

#[derive(Clone, Copy)]
pub struct Quadratic {
    a: f64,
    b: f64,
    c: f64,
    x1: Option<f64>,
    x2: Option<f64>
}

impl Quadratic {
    pub fn new(expr: &str) -> Self {
        let terms_vec = Self::split_string(expr);

        let coeffs = match Self::parse_terms(terms_vec) {
            Ok(result) => result,
            Err(str) => {
                panic!("Error: {}", str);
            }
        };
        let mut res = Quadratic {
            a: coeffs.0,
            b: coeffs.1,
            c: coeffs.2,
            ..Default::default()
        };

        res.find_solves();
        return res;
    }

    fn split_string(expr: &str) -> Vec<&str> {
        let mut terms_vec = Vec::new();

        let mut splitted_expr: (&str, &str) = (expr, "");

        for _i in 0..expr.matches(|c| c == '+' || c == '-').count() + 1 {
            splitted_expr = splitted_expr
                .0
                .split_at(splitted_expr.0.rfind(['+', '-']).unwrap_or(0));

            terms_vec.push(splitted_expr.1);
        }
        return terms_vec;
    }

    fn parse_terms(terms_vec: Vec<&str>) -> Result<(f64, f64, f64), &str> {
        let mut coeffs = (0.0, 0.0, 0.0);

        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"(?P<first>\d*\w\^2)|(?P<second>\d*[[:alpha:]])|(?P<third>\d+)").unwrap();
        }

        for term in terms_vec {

            let caps = match REGEX.captures(term) {
                Some(captures) => captures,
                None => return Err("Cannot read equation terms"),
            };

            if caps.name("first").is_some() {

                let coeff = term
                    .get(
                        term.find(|c: char| c.is_ascii_digit()).unwrap()
                            ..term.find(|c: char| c.is_ascii_alphabetic()).unwrap(),
                    )
                    .unwrap_or("1")
                    .parse::<f64>()
                    .unwrap();
                if term.contains('-') {
                    coeffs.0 -= coeff;
                } else {
                    coeffs.0 += coeff;
                }
            } else if caps.name("second").is_some() {
                let coeff = term
                    .get(
                        term.find(|c: char| c.is_ascii_digit()).unwrap()
                            ..term.find(|c: char| c.is_ascii_alphabetic()).unwrap(),
                    )
                    .unwrap_or("1")
                    .parse::<f64>()
                    .unwrap();
                if term.contains('-') {
                    coeffs.1 -= coeff;
                } else {
                    coeffs.1 += coeff;
                }
            } else if caps.name("third").is_some() {
                let coeff = term
                    .get(
                        term.find(|c: char| c.is_ascii_digit()).unwrap()
                            ..term.trim().chars().count(),
                    )
                    .unwrap()
                    .parse::<f64>()
                    .unwrap(); // todo if expression is ending with space

                if term.contains('-') {
                    coeffs.2 -= coeff;
                } else {
                    coeffs.2 += coeff;
                }
            }
        }
        return Ok(coeffs);
    }

    fn new_from_coeffs(a: f64, b: f64, c: f64) -> Self {
        return Quadratic { a, b, c, ..Default::default() };
    }

    pub fn add_number(&mut self, number: i32) {
        self.c += &f64::from(number);
    }

    fn find_solves(&mut self) {
        if self.a < 0.0 {
            self.x1 = Some(-self.c);
            self.x2 = None;
        }
        let desc = self.b.powi(2) - 4.0*self.a*self.c;
        if desc > 0.0 {
            self.x1 = Some((-self.b + desc.sqrt())/(2.0 * self.a).clone());
            self.x2 = Some((-self.b - desc.sqrt())/(2.0 * self.a).clone());
        } else if desc == 0.0 {
            self.x1 = Some((-self.b/(2.0 * self.a)).clone());
            self.x2 = None;
        } else {
            self.x1 = None;
            self.x2 = None;
        }
    }

    pub fn print(&self) {
        if self.a != 0.0 {
            print!("{:+}x^2 ", self.a);
        }
        if self.b != 0.0 {
            print!("{:+}x ", self.b);
        }
        if self.c != 0.0 {
            println!("{:+}", self.c);
        }
    }
}

impl Add for Quadratic {
    type Output = Quadratic;
    fn add(self, rhs: Self) -> Self::Output {
        Quadratic::new_from_coeffs(self.a + rhs.a, self.b + rhs.b, self.c + rhs.c)
    }
}

impl Sub for Quadratic {
    type Output = Quadratic;
    fn sub(self, rhs: Self) -> Self::Output {
        Quadratic::new_from_coeffs(self.a - rhs.a, self.b - rhs.b, self.c - rhs.c)
    }
}

impl Mul for Quadratic {
    type Output = Biquadratic;
    fn mul(self, rhs: Self) -> Self::Output {
        Biquadratic::new_from_coeffs(
            self.a * rhs.a,
            self.a * rhs.b + self.b * rhs.a,
            self.a * rhs.c  + self.b * rhs.b + self.c * rhs.a,
            self.b * rhs.c + self.c * rhs.b,
            self.c * rhs.c
        )
    }
}

impl Div for Quadratic {
    type Output = QuadraticDivision;

    fn div(mut self, rhs: Self) -> Self::Output {
        let mut res = QuadraticDivision {
            first_equation: self,
            second_equation: rhs
        };
        if self.x1.is_none() && self.x2.is_none()
            || rhs.x1.is_none() && rhs.x2.is_none() {
            return res;
        }
        if self.x1 == rhs.x1 {
            res.first_equation.x1 = None;
            res.second_equation.x1 = None;
        } else if self.x1 == rhs.x2 {
            res.first_equation.x1 = None;
            res.second_equation.x2 = None;
        } else if self.x2 == rhs.x1 {
            res.first_equation.x2 = None;
            res.second_equation.x1 = None;
        } else if self.x2 == rhs.x2 {
            res.first_equation.x2 = None;
            res.second_equation.x2 = None;
        }
        return res;

    }
}

impl Default for Quadratic {
    fn default() -> Self {
        Quadratic {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            x1: None,
            x2: None
        }
    }
}


pub struct QuadraticDivision { // a(x - x1)(x - x2) / a1(x - x3)(x - x4) otherwise a(x - x1) / a1(x - x3)
    first_equation: Quadratic,
    second_equation: Quadratic
}

impl QuadraticDivision {
    pub fn print(&self) {
        if self.first_equation.x1.is_none() && self.first_equation.x2.is_none()
            || self.second_equation.x1.is_none() && self.second_equation.x2.is_none() {
            println!("{:+}x^2 {:+}x {:+} / {:+}x^2 {:+}x {:+}",
                     self.first_equation.a, self.first_equation.b, self.first_equation.c,
                        self.second_equation.a, self.second_equation.b, self.second_equation.c
            );
        } else {
            print!("{}", self.first_equation.a);
            if self.first_equation.x1.is_some() {
                print!("( x {:+} )", -self.first_equation.x1.unwrap());
            }
            if self.first_equation.x2.is_some() {
                print!("( x {:+} )", -self.first_equation.x2.unwrap());
            }
            print!(" / {}", self.second_equation.a);
            if self.second_equation.x1.is_some() {
                print!("( x {:+} )", -self.second_equation.x1.unwrap());
            }
            if self.second_equation.x2.is_some() {
                print!("( x {:+} )", -self.second_equation.x2.unwrap());
            }
        }
    }
}
