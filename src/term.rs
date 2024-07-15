use num::BigInt;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq)]
pub struct Term {
    pub exponents: HashMap<String, u32>,
}

impl Term {
    pub fn new() -> Self {
        Term {
            exponents: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, var: &str, exp: u32) {
        if self.exponents.contains_key(var) {
            let current_exp = self.exponents.get(var).unwrap();
            self.exponents.insert(var.to_string(), current_exp + exp);
        } else {
            self.exponents.insert(var.to_string(), exp);
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut sorted_exponents: Vec<_> = self.exponents.iter().collect();
        sorted_exponents.sort_by(|a, b| a.0.cmp(b.0));
        for (key, value) in sorted_exponents {
            if *value == 1 {
                result.push_str(&key);
            } else {
                result.push_str(&format!("{}^{}", key, value));
            }
            result.push_str("*");
        }
        if result.ends_with("*") {
            result.pop();
        }
        result
    }

    pub fn compare(&self, other: &Term) -> Ordering {
        let mut sum_exponents_self = 0;
        let mut sum_exponents_other = 0;
        for (_, value) in &self.exponents {
            sum_exponents_self += value;
        }
        for (_, value) in &other.exponents {
            sum_exponents_other += value;
        }
        if sum_exponents_self < sum_exponents_other {
            return Ordering::Less;
        } else if sum_exponents_self > sum_exponents_other {
            return Ordering::Greater;
        }
        return self.to_string().cmp(&other.to_string());
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Clone for Term {
    fn clone(&self) -> Self {
        let mut new_term = Term::new();
        for (key, value) in &self.exponents {
            new_term.exponents.insert(key.clone(), value.clone());
        }
        new_term
    }
}

impl Hash for Term {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Sort the HashMap by keys to ensure consistent hashing
        let mut sorted_exponents: Vec<_> = self.exponents.iter().collect();
        sorted_exponents.sort_by(|a, b| a.0.cmp(b.0));

        for (key, value) in sorted_exponents {
            key.hash(state);
            value.hash(state);
        }
    }
}
