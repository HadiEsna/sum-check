use sum_check;

#[test]
fn test_sum() {
    let equation = sum_check::equation::equation::Equation::from_string("x");
    let mut prover = sum_check::sum_check::prover::Prover::new(equation, 5);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 1);
}

#[test]
fn test_sum2() {
    let equation = sum_check::equation::equation::Equation::from_string("x+y");
    let mut prover = sum_check::sum_check::prover::Prover::new(equation, 5);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 4);
}

#[test]
fn test_sum3() {
    let equation =
        sum_check::equation::equation::Equation::from_string("x*y^3 + 2*y^1 + 3*y^3 + y + z+j");
    let mut prover = sum_check::sum_check::prover::Prover::new(equation, 67);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 1);
}

#[test]
fn test_sum4() {
    let equation = sum_check::equation::equation::Equation::from_string("x");
    let mut prover = sum_check::sum_check::prover::Prover::new(equation, 100);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 1);
    let e = prover.first_round();
    assert_eq!(e.to_string(), "x".to_string());
}

#[test]
fn test_sum5() {
    let equation = sum_check::equation::equation::Equation::from_string("x + 1");
    let mut prover = sum_check::sum_check::prover::Prover::new(equation, 100);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 3);
    let e = prover.first_round();
    assert_eq!(e.to_string(), "x+1".to_string());
}

#[test]
fn test_sum6() {
    let equation = sum_check::equation::equation::Equation::from_string("x + y");
    let mut prover = sum_check::sum_check::prover::Prover::new(equation, 100);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 4);
    let e = prover.first_round();
    assert_eq!(e.to_string(), "2*x+1".to_string());
}

#[test]
fn test_sum7() {
    let equation = sum_check::equation::equation::Equation::from_string("x^3 + 3*x*y^3 + y");
    let mut prover = sum_check::sum_check::prover::Prover::new(equation, 100);
    prover.calculate_sum();
    assert_eq!(prover.sum.sum, 7);
    let e = prover.first_round();
    assert_eq!(e.to_string(), "2*x^3+3*x+1".to_string());
}
