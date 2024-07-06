
use sum_check;

#[test]
fn test_add() {
    assert_eq!(sum_check::equation::add(2, 3), 5);
}

#[test]
fn test_add_negative() {
    assert_eq!(sum_check::equation::add(-2, -3), -5);
}

#[test]
fn test_add_zero() {
    assert_eq!(sum_check::equation::add(0, 0), 0);
}

#[test]
fn test_create_equation() {
    let inputs = vec!["x".to_string(), "y".to_string()];
    let terms = vec![
        sum_check::equation::Term{
            coefficient: 1.0,
            variables: vec![
                sum_check::equation::Variable{
                    name: "x".to_string(),
                    exponent: 1,
                },
                sum_check::equation::Variable{
                    name: "y".to_string(),
                    exponent: 3,
                },
            ],
        },
        sum_check::equation::Term{
            coefficient: 2.0,
            variables: vec![
                sum_check::equation::Variable{
                    name: "y".to_string(),
                    exponent: 1,
                },
            ],
        },
    ];
    let equation = sum_check::equation::Equation::new(inputs, terms);
    assert_eq!(equation.inputs, vec!["x".to_string(), "y".to_string()]);
    assert_eq!(equation.terms[0].coefficient, 1.0);
    assert_eq!(equation.terms[0].variables[0].name, "x".to_string());
    assert_eq!(equation.terms[0].variables[0].exponent, 1);
    assert_eq!(equation.terms[0].variables[1].name, "y".to_string());
    assert_eq!(equation.terms[0].variables[1].exponent, 3);
    assert_eq!(equation.terms[1].coefficient, 2.0);
    assert_eq!(equation.terms[1].variables[0].name, "y".to_string());
    assert_eq!(equation.terms[1].variables[0].exponent, 1);
}


#[test]
fn test_evaluate_equation() {
    let inputs = vec!["x".to_string(), "y".to_string()];
    let terms = vec![
        sum_check::equation::Term{
            coefficient: 1.0,
            variables: vec![
                sum_check::equation::Variable{
                    name: "x".to_string(),
                    exponent: 1,
                },
                sum_check::equation::Variable{
                    name: "y".to_string(),
                    exponent: 3,
                },
            ],
        },
        sum_check::equation::Term{
            coefficient: 2.0,
            variables: vec![
                sum_check::equation::Variable{
                    name: "y".to_string(),
                    exponent: 1,
                },
            ],
        },
    ];
    let equation = sum_check::equation::Equation::new(inputs, terms);
    let values = vec![2.0, 3.0];
    assert_eq!(equation.evaluate(values), 60.0);
}