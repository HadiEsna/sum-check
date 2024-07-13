use sum_check;


#[test]
fn test_sum() {
    let equation = sum_check::equation::Equation::from_string("x".to_string());
    let mut prover = sum_check::prover::Prover::new(equation, 5);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 1);
}

#[test]
fn test_sum2() {
    let equation = sum_check::equation::Equation::from_string("x+y".to_string());
    let mut prover = sum_check::prover::Prover::new(equation, 5);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 4);
}

#[test]
fn test_sum3() {
    let equation = sum_check::equation::Equation::from_string("x*y^3 + 2*y^1 + 3*y^3 + y + z+j".to_string());
    let mut prover = sum_check::prover::Prover::new(equation, 67);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 1);
}