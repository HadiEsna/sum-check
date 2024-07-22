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
        layers: vec![layer2, layer1],
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

#[test]
fn test_init_from_string_more_complex() {
    let circuit = Circuit::from_strings(
        [
            "ADD,0 1,x;MUL,2 3,y;ADD,0 3,z", // x = 1 + 2, y = 2 * 4, z = 1 + 4 , x = 3, y = 8, z = 5
            "ADD,x y,k;MUL,y z,j",           // k = 3 + 8, j = 8 * 5, k = 11, j = 40
            "ADD,j k,s",                     // s = 11 + 40, s = 51
        ]
        .to_vec(),
    );
    assert_eq!(circuit.evaluate(vec![1, 2, 2, 4]), 51);
}
