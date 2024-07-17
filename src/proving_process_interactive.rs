use crate::equation::Equation;
use crate::prover::Prover;
use crate::verifier::Verifier;

pub fn run_proving_process(equation: Equation, f: i128) {
    let mut prover = Prover::new(equation.clone(), f);
    let sum = prover.calculate_sum();
    let mut verifier = Verifier::new(sum, f, equation.clone().degree());
    assert_eq!(sum, 12);
    let mut e = prover.first_round();
    let mut next_element = verifier.verify_first_round(&e);
    for _ in 1..equation.inputs.len() {
        let new_equation = prover.later_round(next_element);
        next_element = verifier.verify_next_rounds(&e, &new_equation, next_element);
        e = new_equation;
    }

    verifier.verify_last_round(&equation, &e);
}
