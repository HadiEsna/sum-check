use crate::equation::Equation;

pub struct Sum {
    equation: Equation,
    pub sum: i128,
    F: i128,
}
pub struct Prover {
    pub sum: Sum,
}

impl Prover {
    pub fn new(equation: Equation, F: i128) -> Prover {
        Prover {
            sum: Sum {
                equation: equation,
                sum: 0,
                F: F,
            },
        }
    }

    pub fn calculate_sum(&mut self) -> i128 {
        let mut sum = 0;
        let inputs = self.sum.equation.inputs.clone();
        let inputs_len = inputs.len();
        for i in 0..2_i32.pow(inputs_len.try_into().unwrap()) {
            let mut current_values: Vec<i128> = vec![1; inputs_len.try_into().unwrap()];
            for j in 0..inputs_len {
                let bit: i128 = ((i >> j) & 1).try_into().unwrap();
                current_values[j] = current_values[j] * bit;
            }
            let result = self
                .sum
                .equation
                .evaluate_over_f(current_values.clone(), self.sum.F);
            sum += result;
            sum = sum % self.sum.F;
        }
        self.sum.sum = sum;
        sum
    }

    pub fn first_round(&mut self) -> Equation {
        let inputs = self.sum.equation.inputs.clone();
        let mut final_equation = Equation::new();
        let inputs_len = inputs.len();
        for i in 0..2_i32.pow(inputs_len.try_into().unwrap()) {
            let mut current_values: Vec<i128> = vec![1; inputs_len.try_into().unwrap()];
            for j in 0..inputs_len {
                let bit: i128 = ((i >> j) & 1).try_into().unwrap();
                current_values[j] = current_values[j] * bit;
            }
            if current_values[0] == 1 {
                continue;
            }
            let e = self.sum.equation.evaluate_over_f_with_variable(
                current_values.clone(),
                self.sum.F,
                inputs[0].clone(),
            );
            final_equation = final_equation.add(&e);
        }
        final_equation
    }
}
