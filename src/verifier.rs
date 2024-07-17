use crate::equation::Equation;
use rand::Rng;

pub struct Verifier {
    pub sum: i128,
    pub f: i128,
    pub max_degree: u32,
    pub selected_values: Vec<i128>,
}

impl Verifier {
    pub fn new(sum: i128, f: i128, max_degree: u32) -> Verifier {
        Verifier {
            sum: sum,
            f: f,
            max_degree: max_degree,
            selected_values: vec![],
        }
    }

    pub fn verify_first_round(&mut self, equation: &Equation) -> i128 {
        let degree = equation.degree();
        if degree > self.max_degree {
            panic!("degree is larger the degree of the equation");
        }
        let mut sum = 0;
        let mut inputs = [0];
        sum += equation.evaluate_over_f(inputs.to_vec(), self.f);
        inputs[0] = 1;
        sum += equation.evaluate_over_f(inputs.to_vec(), self.f);
        sum = sum % self.f;
        let random_value = rand::thread_rng().gen_range(0..self.f);
        if sum != self.sum {
            panic!("sum is not equal to the sum of the equation");
        }
        self.selected_values.push(random_value);
        random_value
    }

    pub fn verify_next_rounds(
        &mut self,
        previous_equation: &Equation,
        new_equation: &Equation,
        value: i128,
    ) -> i128 {
        let degree = new_equation.degree();
        if degree > self.max_degree {
            panic!("degree is larger the degree of the equation");
        }
        let mut sum = 0;
        let mut inputs = [0];
        sum += new_equation.evaluate_over_f(inputs.to_vec(), self.f);
        inputs[0] = 1;
        sum += new_equation.evaluate_over_f(inputs.to_vec(), self.f);
        sum = sum % self.f;
        inputs[0] = value;
        if sum != previous_equation.evaluate_over_f(inputs.try_into().unwrap(), self.f) {
            println!("sum: {}", sum);
            println!(
                "previous: {}",
                previous_equation.evaluate_over_f(inputs.try_into().unwrap(), self.f)
            );
            panic!("sum is not equal to the sum of the equation");
        }
        let random_value = rand::thread_rng().gen_range(0..self.f);
        self.selected_values.push(random_value);
        random_value
    }

    pub fn verify_last_round(&mut self, origianl_equation: &Equation, latest_equation: &Equation) {
        let inputs = [self.selected_values[self.selected_values.len() - 1]];
        let latest_evaluation = latest_equation.evaluate_over_f(inputs.to_vec(), self.f);
        let original_evaluation =
            origianl_equation.evaluate_over_f(self.selected_values.to_vec(), self.f);
        if latest_evaluation != original_evaluation {
            panic!("sum is not equal to the sum of the equation");
        }
    }
    pub fn choose_random_input_and_set_value(&self, lenght: usize) -> (i128, i128) {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..lenght);
        let random_value = rng.gen_range(0..self.f);
        (random_index as i128, random_value)
    }
}
