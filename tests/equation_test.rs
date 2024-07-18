use sum_check;

#[test]
fn test_add() {
    assert_eq!(sum_check::equation::equation::add(2, 3), 5);
}

#[test]
fn test_add_negative() {
    assert_eq!(sum_check::equation::equation::add(-2, -3), -5);
}

#[test]
fn test_add_zero() {
    assert_eq!(sum_check::equation::equation::add(0, 0), 0);
}

#[test]
fn test_init_from_string() {
    let equation = sum_check::equation::equation::Equation::from_string("x");
    assert_eq!(equation.evaluate(vec![1]), 1);
    assert_eq!(equation.inputs, vec!["x".to_string()]);
    let equation2 = sum_check::equation::equation::Equation::from_string("x + y");
    assert_eq!(equation2.evaluate(vec![1, 1]), 2);
    assert_eq!(equation2.inputs, vec!["x".to_string(), "y".to_string()]);
    let equation3 = sum_check::equation::equation::Equation::from_string("x^2");
    assert_eq!(equation3.evaluate(vec![2]), 4);
}

#[test]
fn test_init_from_string2() {
    let equation = sum_check::equation::equation::Equation::from_string("x*y^3 + 2*y^1 + 3*y^3");
    assert_eq!(equation.evaluate(vec![1, 1]), 6);
    assert_eq!(equation.inputs, vec!["x".to_string(), "y".to_string()]);
    assert_eq!(equation.to_string(), "x*y^3+3*y^3+2*y".to_string());
}

#[test]
fn test_init_from_string3() {
    let equation = sum_check::equation::equation::Equation::from_string("3*x+2*x^2*y");
    assert_eq!(equation.evaluate(vec![1, 1]), 5);
    assert_eq!(equation.inputs, vec!["x".to_string(), "y".to_string()]);
    assert_eq!(equation.to_string(), "2*x^2*y+3*x".to_string());
}

#[test]
fn test_init_from_string_with_minus() {
    let equation = sum_check::equation::equation::Equation::from_string("x*y^3 + 2*y^1 - 3*y^3");
    assert_eq!(equation.evaluate(vec![2, 3]), -21);
    assert_eq!(equation.to_string(), "x*y^3-3*y^3+2*y".to_string());
}

#[test]
fn test_degree() {
    let equation = sum_check::equation::equation::Equation::from_string("x*y^3 + 2*y^1 - 3*y^3");
    assert_eq!(equation.degree(), 3);
}

#[test]
fn test_degree2() {
    let equation =
        sum_check::equation::equation::Equation::from_string("x*y^10 + 2*y^1 - 3*y^3 + x^2");
    assert_eq!(equation.degree(), 10);
}
