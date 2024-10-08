use sum_check;

#[test]
fn test_verify_process() {
    let equation = sum_check::equation::equation::Equation::from_string("2*x1^3+x1*x2+x2*x3");
    sum_check::sum_check::proving_process_interactive::run_proving_process(equation, 100);
}

#[test]
fn test_verify_process2() {
    let equation = sum_check::equation::equation::Equation::from_string("x1+x2+x3");
    sum_check::sum_check::proving_process_interactive::run_proving_process(equation, 100);
}

#[test]
fn test_verify_process3() {
    let equation = sum_check::equation::equation::Equation::from_string("2*x1^3+x1*x2+x2*x3");
    sum_check::sum_check::proving_process_interactive::run_proving_process(equation, 1000);
}
