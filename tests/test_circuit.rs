use sum_check::gkr::circuit::*;
#[test]
fn test_init_and_evaluate_circuit() {
    let layer1 = Layer {
        gates: vec![Gate {
            gate_type: GateType::Add,
            inputs: vec![0, 1],
            name: "x".to_string(),
        }],
    };
    let circuit = Circuit {
        layers: vec![layer1],
    };
    assert_eq!(circuit.evaluate(vec![1, 2]), 3);
}

#[test]
fn test_init_and_evaluate_circuit2() {
    let layer1 = Layer {
        gates: vec![
            Gate {
                gate_type: GateType::Add,
                inputs: vec![0, 1],
                name: "x".to_string(),
            },
            Gate {
                gate_type: GateType::Mul,
                inputs: vec![2, 3],
                name: "y".to_string(),
            },
        ],
    };
    let layer2 = Layer {
        gates: vec![Gate {
            gate_type: GateType::Add,
            inputs: vec![0, 1],
            name: "z".to_string(),
        }],
    };
    let circuit = Circuit {
        layers: vec![layer1, layer2],
    };
    assert_eq!(circuit.evaluate(vec![1, 2, 2, 2]), 7);
}

#[test]
fn test_init_from_string() {
    let circuit = Circuit::from_strings(["ADD,0 1,x", "NEG,x,y"].to_vec());
    assert_eq!(circuit.evaluate(vec![1, 2]), -3);
}

#[test]
fn test_init_from_string2() {
    let circuit = Circuit::from_strings(["MUL,0 1,x"].to_vec());
    assert_eq!(circuit.evaluate(vec![1, 2]), 2);
}
