#[derive(PartialEq)]
pub enum GateType {
    Add,
    Mul,
    Sub,
    Pass,
    Neg,
}

pub struct Gate {
    pub gate_type: GateType,
    pub inputs: Vec<usize>,
    pub name: String,
}

pub struct Layer {
    pub gates: Vec<Gate>,
}

pub struct Circuit {
    pub layers: Vec<Layer>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit { layers: vec![] }
    }

    pub fn from_strings(layers: Vec<&str>) -> Circuit {
        let mut circuit = Circuit::new();
        for layer in layers {
            circuit.add_layer_from_string(layer);
        }
        circuit
    }

    pub fn add_layer_from_string(&mut self, layer: &str) {
        let mut gates = vec![];
        let gates_str = layer.split(";");
        for gate_str in gates_str {
            let gate_parts: Vec<&str> = gate_str.split(",").collect();
            let gate_type = match gate_parts[0] {
                "ADD" => GateType::Add,
                "MUL" => GateType::Mul,
                "SUB" => GateType::Sub,
                "PASS" => GateType::Pass,
                "NEG" => GateType::Neg,
                _ => panic!("Invalid gate type"),
            };
            let inputs: Vec<&str> = gate_parts[1].split(" ").collect();
            let mut inputs_int = vec![];
            for input in inputs {
                if self.layers.len() == 0 {
                    inputs_int.push(input.parse::<usize>().unwrap());
                } else {
                    let last_layer = &self.layers[0];
                    let last_layer_gates = &last_layer.gates;
                    let mut found = false;
                    // loop with index to get the index of the gate
                    for i in 0..last_layer_gates.len() {
                        if last_layer_gates[i].name == input {
                            inputs_int.push(i);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        inputs_int.push(input.parse::<usize>().unwrap());
                    }
                }
            }
            let name = gate_parts[2].to_string();
            gates.push(Gate {
                gate_type: gate_type,
                inputs: inputs_int,
                name: name,
            });
        }
        self.layers.insert(0, Layer { gates: gates });
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn evaluate(&self, inputs: Vec<i128>) -> i128 {
        let mut outputs = inputs.clone();
        let reversed_layers = self.layers.iter().rev();
        for layer in reversed_layers {
            let mut new_outputs = vec![];
            for gate in layer.gates.iter() {
                if gate.gate_type == GateType::Add {
                    let mut sum = 0;
                    for input in gate.inputs.iter() {
                        sum += outputs[*input];
                    }
                    new_outputs.push(sum);
                } else if gate.gate_type == GateType::Mul {
                    let mut mul = 1;
                    for input in gate.inputs.iter() {
                        mul *= outputs[*input];
                    }
                    new_outputs.push(mul);
                } else if gate.gate_type == GateType::Sub {
                    let mut sub = outputs[gate.inputs[0]];
                    sub -= outputs[gate.inputs[1]];
                    new_outputs.push(sub);
                } else if gate.gate_type == GateType::Pass {
                    new_outputs.push(outputs[gate.inputs[0]]);
                } else if gate.gate_type == GateType::Neg {
                    new_outputs.push(-outputs[gate.inputs[0]]);
                }
            }
            outputs = new_outputs;
        }
        return outputs[0];
    }
}
