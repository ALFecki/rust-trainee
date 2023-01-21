use std::ops::{Add, Mul, Sub};

use regex::Regex;

pub struct Quadratic {
    a: f64,
    b: f64,
    c: f64,
}

impl Quadratic {


    pub fn new(expr: &str) -> Self {
        let terms_vec = Self::split_string(expr);

        let coeffs = Self::parse_terms(terms_vec);

        return Quadratic {
            a: coeffs.0,
            b: coeffs.1,
            c: coeffs.2,
        };
    }

    fn split_string(expr: &str) -> Vec<&str> {
        let mut terms_vec = Vec::new();

        let mut splitted_expr: (&str, &str) = (expr, "");

        for i in 0..expr.matches(|c| c == '+' || c == '-').count() + 1 {
            splitted_expr = splitted_expr
                .0
                .split_at(splitted_expr.0.rfind(['+', '-']).unwrap_or(0));

            terms_vec.push(splitted_expr.1);
        }
        return terms_vec;
    }

    fn parse_terms(terms_vec: Vec<&str>) -> (f64, f64, f64) {
        let mut coeffs = (0.0, 0.0, 0.0);
        let regex = (
            Regex::new(r"\d+\w\^2").unwrap(),
            Regex::new(r"\d+\w").unwrap(),
        );
        for term in terms_vec {
            if regex.0.is_match(term) {
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
            } else if regex.1.is_match(term) {
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
            } else {
                // println!("{term}: {left} - {right}");
                let coeff = term
                    .get(term.find(|c: char| c.is_ascii_digit()).unwrap()..term.trim().chars().count())
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
            return coeffs;
    }

    fn new_from_coeffs(a: f64, b: f64, c: f64) -> Self {
        return Quadratic { a, b, c };
    }

    pub fn add_number(&mut self, number: i32) {
        self.c += &f64::from(number);
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

// impl Mul for Quadratic {
//     type Output = Quadratic;
//     fn mul(self, rhs: Self) -> Self::Output {
//         Quadratic::new_from_coeffs(self.a * rhs.a, self.b * rhs.b, self.c * rhs.c)

//     }
// }
