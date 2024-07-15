use sum_check;

#[test]
fn test_add() {
    let equation = sum_check::equation::Equation::from_string("2*x1^3+x1*x2+x2*x3");
    let mut prover = sum_check::prover::Prover::new(equation, 100);
    let sum = prover.calculate_sum();
    assert_eq!(sum, 12);
    let e = prover.first_round();
    assert_eq!(e.to_string(), "8*x1^3+2*x1+1".to_string());
}
