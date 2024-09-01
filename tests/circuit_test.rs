use rand::Rng;
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

#[test]
fn test_multi_layer_circuit() {
    let circuit = Circuit::from_strings(vec![
        "ADD,0 1,a;SUB,2 3,b;MUL,0 3,c", // a = 5 + 3, b = 7 - 4, c = 5 * 4 , a = 8, b = 3, c = 20
        "ADD,a b,d;MUL,b c,e;NEG,c,f",   // d = 8 + 3, e = 3 * 20, f = -20 , d = 11, e = 60, f = -20
        "ADD,d e,g;SUB,f e,h",           // g = 11 + 60, h = -20 - 60, g = 71, h = -80
        "MUL,g h,i",                     // i = 71 * -80, i = -5680
    ]);
    assert_eq!(circuit.evaluate(vec![5, 3, 7, 4]), -5680);
}

#[test]
fn test_multi_layer_circuit_2() {
    let circuit = Circuit::from_strings(vec![
        "ADD,0 1,a;SUB,2 3,b", // a = 5 + 3, b = 7 - 4, a = 8, b = 3
        "MUL,a b,c;NEG,b,d",   // c = 8 * 3, d = -3, c = 24, d = -3
        "ADD,c d,e",           // e = 24 + (-3), e = 21
    ]);
    assert_eq!(circuit.evaluate(vec![5, 3, 7, 4]), 21);
}

#[test]
fn test_pass_neg_circuit() {
    let circuit = Circuit::from_strings(vec![
        "PASS,0,a;NEG,1,b", // a = 10, b = -5
        "ADD,a b,c",        // c = 10 + (-5), c = 5
        "NEG,c,d",          // d = -5
    ]);
    assert_eq!(circuit.evaluate(vec![10, 5]), -5);
}

#[test]
fn test_complex_circuit() {
    let circuit = Circuit::from_strings(vec![
        "MUL,0 1,a;ADD,2 3,b", // a = 2 * 4, b = 6 + 8, a = 8, b = 14
        "SUB,b a,c;MUL,a b,d", // c = 14 - 8, d = 8 * 14, c = 6, d = 112
        "ADD,c d,e",           // e = 6 + 112, e = 118
    ]);
    assert_eq!(circuit.evaluate(vec![2, 4, 6, 8]), 118);
}

#[test]
fn test_sequential_operations() {
    let circuit = Circuit::from_strings(vec![
        "ADD,0 1,a", // a = 3 + 5, a = 8
        "MUL,a a,b", // b = 8 * 8, b = 64
        "NEG,b,c",   // c = -64
    ]);
    assert_eq!(circuit.evaluate(vec![3, 5]), -64);
}

#[test]
fn test_large_circuit() {
    let circuit = Circuit::from_strings(vec![
        // Layer 1: Basic operations using inputs
        "ADD,0 1,a1;MUL,0 1,a2;SUB,0 1,a3;ADD,0 1,a4",
        // a1 = 3 + 5 = 8, a2 = 3 * 5 = 15, a3 = 3 - 5 = -2, a4 = 3 + 5 = 8

        // Layer 2: Operations using results from Layer 1
        "MUL,a1 a2,b1;ADD,a2 a3,b2;SUB,a3 a4,b3",
        // b1 = 8 * 15 = 120, b2 = 15 + (-2) = 13, b3 = -2 - 8 = -10

        // Layer 3: Operations using results from Layer 2
        "ADD,b1 b2,c1;MUL,b2 b3,c2",
        // c1 = 120 + 13 = 133, c2 = 13 * -10 = -130

        // Layer 4: Operations using results from Layer 3
        "SUB,c1 c2,d1",
        // d1 = 133 - (-130) = 263

        // Layer 5: Operations using results from Layer 4
        "ADD,d1 d1,e1;NEG,d1,e2",
        // e1 = 263 + 263 = 526, e2 = -263

        // Layer 6: Operations using results from Layer 5
        "MUL,e1 e2,f1",
        // f1 = 526 * (-263) = -138338

        // Layer 7: Operations using results from Layer 6
        "SUB,f1 f1,g1;ADD,f1 f1,g2",
        // g1 = -138338 - (-138338) = 0, g2 = -138338 + (-138338) = -276676

        // Layer 8: Single output gate using results from Layer 7
        "MUL,g1 g2,h1",
        // h1 = 0 * (-276676) = 0
    ]);
    assert_eq!(circuit.evaluate(vec![3, 5]), 0);
}

pub fn generate_random_circuit(layers_count: usize, max_gates_per_layer: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut circuit = Vec::new();
    let mut prev_layer_gates = 2; // Starting with 2 inputs

    for layer_idx in 0..layers_count {
        let gate_count = if layer_idx == layers_count - 1 {
            1 // Ensure the last layer has only one output gate
        } else {
            rng.gen_range(1..=max_gates_per_layer)
        };

        let mut layer_gates = Vec::new();

        for gate_idx in 0..gate_count {
            let gate_type = match rng.gen_range(0..=2) {
                0 => "ADD",
                1 => "MUL",
                _ => "SUB",
            };

            let input1 = rng.gen_range(0..prev_layer_gates);
            let input2 = rng.gen_range(0..prev_layer_gates);

            let gate_name = format!("g{}_{}", layer_idx + 1, gate_idx + 1);
            let gate_str = format!("{},{} {},{}", gate_type, input1, input2, gate_name);

            layer_gates.push(gate_str);
        }

        prev_layer_gates = gate_count;
        circuit.push(layer_gates.join(";"));
    }

    circuit
}

#[test]
fn test_random_circuit() {
    let layers_count = 5;
    let max_gates_per_layer = 10;
    let layers: Vec<String> = generate_random_circuit(layers_count, max_gates_per_layer);
    let vec_of_strs: Vec<&str> = layers.iter().map(|s| s.as_str()).collect();
    let circuit = Circuit::from_strings(vec_of_strs);
    let inputs = vec![3, 5];
    let expected_output = circuit.evaluate(inputs.clone());
    assert_eq!(circuit.evaluate(inputs), expected_output);
    println!("Random circuit output: {}", expected_output);
}
