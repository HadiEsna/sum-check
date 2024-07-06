pub struct Variable{
    pub name: String,
    pub exponent: i32,
}

pub struct Term{
    pub coefficient: f64,
    pub variables: Vec<Variable>,
}

pub struct Equation{
    pub inputs: Vec<String>,
    pub terms: Vec<Term>,
}

impl Equation{
    pub fn new(inputs: Vec<String>, terms: Vec<Term>) -> Equation{
        Equation{
            inputs: inputs,
            terms: terms,
        }
    }

    pub fn evaluate(&self, values: Vec<f64>) -> f64{
        let mut result = 0.0;
        for term in &self.terms{
            let mut term_result = term.coefficient;
            for variable in &term.variables{
                let index = self.inputs.iter().position(|x| x == &variable.name).unwrap();
                term_result *= values[index].powi(variable.exponent);
            }
            result += term_result;
        }
        result
    }
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
