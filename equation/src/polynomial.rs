use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"(?P<first>[+|-]*\s*\d*\w\^\d)|(?P<second>[+|-]*\s*\d*[[:alpha:]])|(?P<third>[+|-]*\s*\d+)"
    )
    .unwrap();
}

#[derive(Debug)]
pub struct Polynomial {
    terms: BTreeMap<i32, f64>,
}

impl Polynomial {
    pub fn new(expr: &str) -> Result<Self, &str> {
        Ok( Polynomial {
            terms: match Self::parse_terms(expr) {
                Ok(res) => res,
                Err(str) => {
                    return Err(str);
                }
            },
        })
    }

    fn parse_terms(expr: &str) -> Result<BTreeMap<i32, f64>, &str> {
        let mut coeffs = BTreeMap::<i32, f64>::new();
        for caps in REGEX.captures_iter(expr) {
            if let Some(cap) = caps.name("first") {
                let number = match Self::get_term_number(cap.as_str(), false) {
                    Ok(res) => res,
                    Err(str) => return Err(str),
                };
                coeffs
                    .entry(match Self::get_power(cap.as_str()) {
                        Ok(res) => res,
                        Err(str) => return Err(str),
                    })
                    .and_modify(|num| *num += number)
                    .or_insert(number);
            } else if let Some(cap) = caps.name("second") {
                let number = match Self::get_term_number(cap.as_str(), false) {
                    Ok(res) => res,
                    Err(str) => return Err(str),
                };
                coeffs
                    .entry(1)
                    .and_modify(|num| *num += number)
                    .or_insert(number);
            } else if let Some(cap) = caps.name("third") {
                let number = match Self::get_term_number(cap.as_str(), true) {
                    Ok(res) => res,
                    Err(str) => return Err(str),
                };
                coeffs
                    .entry(0)
                    .and_modify(|num| *num += number)
                    .or_insert(number);
            }
        }

        Ok(coeffs)
    }

    fn get_power(cap: &str) -> Result<i32, &str> {
        let power_start = cap.find(|c: char| c == '^').unwrap_or(0) + 1;
        let power_end = cap.chars().count();
        return match cap
            .get(power_start..power_end)
            .unwrap_or("1")
            .parse::<i32>()
        {
            Ok(p) => Ok(p),
            Err(_) => return Err("Parse error"),
        };
    }

    fn get_term_number(cap: &str, c_flag: bool) -> Result<f64, &str> {
        let mut res = 0.0;
        let digit_start = cap.find(|c: char| c.is_ascii_digit()).unwrap_or(0);
        let digit_end = if c_flag {
            cap.trim().chars().count()
        } else {
            cap.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(0)
        };
        let coeff = match cap
            .get(digit_start..digit_end)
            .unwrap_or("1")
            .parse::<f64>()
        {
            Ok(c) => c,
            Err(_) => return Err("Parse error"),
        };
        if cap.contains('-') {
            res -= coeff;
        } else {
            res += coeff;
        }
        Ok(res)
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_map = self.terms;
        for term in rhs.terms {
            new_map.entry(term.0).and_modify(|num| *num += term.1);
        }
        Polynomial { terms: new_map }
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_map = self.terms;
        for term in rhs.terms {
            new_map.entry(term.0).and_modify(|num| *num -= term.1);
        }
        Polynomial { terms: new_map }
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_map = BTreeMap::<i32, f64>::new();
        for term in &self.terms {
            for term_rhs in &rhs.terms {
                new_map
                    .entry(*term.0 + *term_rhs.0)
                    .and_modify(|num| *num += *term.1 * *term_rhs.1)
                    .or_insert(*term.1 * *term_rhs.1);
            }
        }
        Polynomial { terms: new_map }
    }
}

impl Div for Polynomial {
    type Output = (Polynomial, Option<(Polynomial, Polynomial)>);

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Display for Polynomial { // one huge crutch
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.terms.iter();
        let mut is_first = true;
        while let Some(term) = iter.next_back() {
            if *term.1 == 0.0 {
                is_first = false;
                continue;
            }
            if *term.0 == 1 {
                if *term.1 == 1.0 && is_first {
                    write!(f, "x ")?;
                } else if *term.1 == 1.0 {
                    write!(f, "+x ")?;
                } else if *term.1 == -1.0 {
                    write!(f, "-x ")?;
                } else {
                    write!(f, "{:+}x ", term.1)?;
                }
            } else if *term.0 == 0 {
                write!(f, "{:+} ", term.1)?;
            } else if *term.1 == 1.0 && is_first {
                write!(f, "x^{} ", term.0)?;
            } else if *term.1 == 1.0 {
                write!(f, "+x^{} ", term.0)?;
            } else if *term.1 == -1.0 {
                write!(f, "-x^{} ", term.0)?;
            } else {
                write!(f, "{:+}x^{} ", term.1, term.0)?;
            }
            is_first = false;
        }
        write!(f, "")
    }
}
