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

        return Quadratic {
            a: coeffs.0,
            b: coeffs.1,
            c: coeffs.2,
        };
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
                Regex::new(r"(?P<first>\d+\w\^2)|(?P<second>\d+\w)|(?P<third>\d+)").unwrap();
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
                    .unwrap()
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
                    .unwrap()
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
        return Quadratic { a, b, c };
    }

    pub fn add_number(&mut self, number: i32) {
        self.c += &f64::from(number);
    }

    fn find_solves(&self) -> Result<(f64, f64), &str> {
        if self.a < 0.0 {
            return Err("Not a quadratic equation");
        }
        let desc = self.b.powi(2) - 4.0*self.a*self.c;
        if desc > 0.0 {
            return Ok(((-self.b + desc.sqrt())/4.0*self.a, (-self.b + desc.sqrt())/4.0*self.a));
        } else if desc == 0.0 {
            return Ok((-self.b/4.0*self.a, -self.b/4.0*self.a));
        } else {
            return Err("Desc is less then 0");
        }
    }

    pub fn print(&self) {
        println!("{:+}x^2 {:+}x {:+}", self.a, self.b, self.c);
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

    fn div(self, rhs: Self) -> Self::Output {
        let self_solves = match self.find_solves() {
            Ok(res) => res,
            Err(_) => (0.0, 0.0)
        }; 
        let rhs_solves = match rhs.find_solves() {
            Ok(res) => res,
            Err(_) => (0.0, 0.0)
        }; 

        todo!()

    }
}


pub struct QuadraticDivision { // (ax^2 + bx + c) / (a1x^2 + b1x + c) otherwise (ax - x_solve) / (a1x - x1_solve)
    a: f64,
    b: f64,
    c: f64,
    a1: f64,
    b1: f64,
    c1: f64,
    x_solve: f64,
    x1_solve: f64
}
