use sum_check;


#[test]
fn test_sum() {
    let equation = sum_check::equation::Equation::from_string("x".to_string());
    let mut prover = sum_check::prover::Prover::new(equation, 5);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 1);
    println!("summ {}", prover.sum.sum);
}

#[test]
fn test_sum2() {
    let equation = sum_check::equation::Equation::from_string("x+y".to_string());
    let mut prover = sum_check::prover::Prover::new(equation, 5);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 4);
    println!("summ {}", prover.sum.sum);
}