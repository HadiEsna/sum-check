use crate::equation::Equation;

pub struct Sum {
    equation: Equation,
    pub sum: i128,
    f: i128,
}
pub struct Prover {
    pub sum: Sum,
    pub selected_values: Vec<i128>,
}

impl Prover {
    pub fn new(equation: Equation, f: i128) -> Prover {
        Prover {
            sum: Sum {
                equation: equation,
                sum: 0,
                f: f,
            },
            selected_values: vec![],
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
                .evaluate_over_f(current_values.clone(), self.sum.f);
            sum += result;
            sum = sum % self.sum.f;
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
                self.sum.f,
                inputs[0].clone(),
            );
            final_equation = final_equation.add(&e);
        }
        final_equation
    }

    pub fn later_round(&mut self, selected_value: i128) -> Equation {
        let inputs = self.sum.equation.inputs.clone();
        let mut final_equation = Equation::new();
        let inputs_len: u32 = inputs.len().try_into().unwrap();
        self.selected_values.push(selected_value);
        let selected_values_len: u32 = self.selected_values.len().try_into().unwrap();
        for i in 0..2_i32.pow(inputs_len - selected_values_len - 1) {
            let mut current_values: Vec<i128> = vec![1; inputs_len.try_into().unwrap()];
            // write the selected values to the current values
            for j in 0..selected_values_len {
                current_values[j as usize] = self.selected_values[j as usize];
            }
            // write the remaining values to the current values
            for j in 0..inputs_len - selected_values_len - 1 {
                let bit: i128 = ((i >> j) & 1).try_into().unwrap();
                current_values[(j + selected_values_len + 1) as usize] =
                    current_values[(j + selected_values_len + 1) as usize] * bit;
            }
            let e = self.sum.equation.evaluate_over_f_with_variable(
                current_values.clone(),
                self.sum.f,
                inputs[selected_values_len as usize].clone(),
            );
            final_equation = final_equation.add(&e);
        }
        final_equation
    }
}
