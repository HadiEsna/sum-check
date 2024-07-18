use num::BigInt;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::equation::term::Term;

pub struct Equation {
    pub inputs: Vec<String>,
    pub terms: HashMap<Term, i128>,
}

impl Equation {
    pub fn new() -> Self {
        Equation {
            terms: HashMap::new(),
            inputs: Vec::new(),
        }
    }

    pub fn from_string(function_f: &str) -> Equation {
        let oprators = ['+'];
        let term_oprators = ['*', '/'];
        let pow_oprator = ['^'];
        let function_f = function_f.replace(" ", "");
        let function_f = function_f.replace("-", "+-");
        let (terms_strings, _) = split_string_based_on_list(&function_f, &oprators);
        let mut equation = Equation::new();
        for term_string in terms_strings {
            let mut term = Term::new();
            let mut coefficient: i128 = 1;
            if term_string == "" {
                continue;
            }
            let term_strings2: Vec<_> = term_string.split(|c| term_oprators.contains(&c)).collect();
            for term_string2 in term_strings2 {
                if is_integer(term_string2) {
                    coefficient = coefficient * term_string2.parse::<i128>().unwrap();
                } else {
                    let mut exponent = 1;
                    let mut name = term_string2.to_string();
                    if term_string2.contains("^") {
                        let (parts, _) = split_string_based_on_list(&term_string2, &pow_oprator);
                        name = parts[0].to_string();
                        exponent = parts[1].parse::<u32>().unwrap();
                    }
                    term.add_variable(&name, exponent);
                }
            }
            equation.add_term(term, coefficient);
        }
        for term in &equation.terms {
            for (var, _) in term.0.exponents.iter() {
                if !equation.inputs.contains(&var) {
                    equation.inputs.push(var.clone());
                }
            }
        }
        equation.inputs.sort();
        equation
    }

    pub fn add_term(&mut self, term: Term, coefficient: i128) {
        if self.terms.contains_key(&term) {
            let current_coeff = self.terms.get(&term).unwrap();
            self.terms.insert(term, current_coeff + coefficient);
        } else {
            self.terms.insert(term, coefficient);
        }
        // loop through all inputs
        for (t, _) in self.terms.iter() {
            for (var, _) in t.exponents.iter() {
                if !self.inputs.contains(&var) {
                    self.inputs.push(var.clone());
                }
            }
        }
        self.inputs.sort();
    }

    pub fn evaluate(&self, values: Vec<i128>) -> i128 {
        let mut result: i128 = 0;
        for (term, coeff) in self.terms.iter() {
            let mut term_result = coeff.clone();
            for (var, exp) in term.exponents.iter() {
                let index = self.inputs.iter().position(|x| x == var).unwrap();
                term_result *= values[index].pow(*exp);
            }
            result += term_result;
        }
        result
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let sorted_terms: Vec<_> = self.sorted_terms();
        for (term, coeff) in sorted_terms {
            if (term.exponents.len() == 0) && (*coeff == 0) {
                continue;
            }
            if term.exponents.len() == 0 {
                result.push_str(&coeff.to_string());
                result.push_str("+");
                continue;
            }
            if *coeff == 1 {
                result.push_str(&term.to_string());
            } else {
                if *coeff <= -1 && result.ends_with("+") {
                    result.pop();
                }
                result.push_str(&format!("{}*{}", coeff, term.to_string()));
            }
            result.push_str("+");
        }
        if result.ends_with("+") {
            result.pop();
        }
        if result.starts_with("+") {
            result.remove(0);
        }
        result
    }

    pub fn evaluate_over_f(&self, values: Vec<i128>, f: i128) -> i128 {
        let mut result: i128 = 0;
        for (term, coeff) in self.terms.iter() {
            let mut term_result = coeff.clone();
            for (var, exp) in term.exponents.iter() {
                let index = self.inputs.iter().position(|x| x == var).unwrap();
                term_result *= values[index].pow(*exp);
                term_result = term_result % f;
            }
            result += term_result;
            result = result % f;
        }
        result
    }

    pub fn evaluate_over_f_with_variable(
        &self,
        values: Vec<i128>,
        f: i128,
        variable: String,
    ) -> Equation {
        let mut result: i128 = 0;
        let mut equation = Equation::new();
        for (terms, coef) in &self.terms {
            let mut variable_power = 0;
            let mut term_result = coef.clone();
            for (variable_name, exponent) in &terms.exponents {
                if *variable_name == variable {
                    variable_power += exponent;
                } else {
                    let index = self
                        .inputs
                        .iter()
                        .position(|x| *x == *variable_name)
                        .unwrap();
                    term_result *= values[index].pow(*exponent);
                    term_result = term_result % f;
                }
            }
            if variable_power > 0 {
                let mut term = Term::new();
                term.add_variable(&variable, variable_power);
                equation.add_term(term, term_result);
            } else {
                result += term_result;
                result = result % f;
            }
        }
        let mut ee = equation.to_string();
        if result > 0 {
            let term = Term::new();
            equation.add_term(term, result);
        }
        ee = equation.to_string();
        equation
    }

    pub fn sorted_terms(&self) -> Vec<(&Term, &i128)> {
        let mut sorted_terms: Vec<_> = self.terms.iter().collect();
        sorted_terms.sort_by(|a, b| {
            if a.0.exponents.len() == 0 {
                return Ordering::Greater;
            } else if b.0.exponents.len() == 0 {
                return Ordering::Less;
            }
            a.0.cmp(&b.0).reverse()
        });
        sorted_terms
    }

    pub fn degree(&self) -> u32 {
        let mut max_degree: u32 = 0;
        for (term, _) in &self.terms {
            for (_, exp) in &term.exponents {
                if *exp > max_degree {
                    max_degree = *exp;
                }
            }
        }
        max_degree
    }

    pub fn add(&self, other: &Equation) -> Equation {
        let mut result = Equation::new();
        for (term, coef) in &self.terms {
            result.add_term(term.clone(), coef.clone());
        }
        for (term, coef) in &other.terms {
            result.add_term(term.clone(), coef.clone());
        }
        result
    }

    pub fn clone(&self) -> Equation {
        let mut result = Equation::new();
        for (term, coef) in &self.terms {
            result.add_term(term.clone(), coef.clone());
        }
        result
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn split_string_based_on_list<'a>(
    input: &'a str,
    delimiters: &'a [char],
) -> (Vec<&'a str>, Vec<usize>) {
    let indexes = input
        .match_indices(|c| delimiters.contains(&c))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let result = input.split(|c| delimiters.contains(&c)).collect::<Vec<_>>();
    return (result, indexes);
}

fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}
