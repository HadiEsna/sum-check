pub struct Variable{
    pub name: String,
    pub exponent: u32,
}

pub struct Term{
    pub coefficient: i128,
    pub variables: Vec<Variable>,
}

pub struct Equation{
    pub inputs: Vec<String>,
    pub terms: Vec<Term>,
}

impl Equation{
    pub fn new(inputs: Vec<String>, terms: Vec<Term>) -> Equation{
        Equation{
            inputs: inputs,
            terms: terms,
        }
    }

    pub fn from_string(mut function_f: String) -> Equation {
        let mut inputs = Vec::new();
        let oprators = ['+'];
        let term_oprators = ['*', '/'];
        let pow_oprator = ['^'];
        let mut terms = Vec::new();
        function_f = function_f.replace(" ", "");
        function_f = function_f.replace("-", "+-");
        let (terms_strings, inds) = split_string_based_on_list( &function_f, &oprators);
        let mut i = 0;
        for term_string in terms_strings{

            let mut coefficient: i128 = 1;
            if term_string == ""{
                continue;
            }
            let mut variables = Vec::new();
            let mut term_strings2: Vec<_> = term_string.split(|c| term_oprators.contains(&c)).collect();
            for term_string2 in term_strings2{
                if is_integer(term_string2){
                    coefficient = coefficient * term_string2.parse::<i128>().unwrap();
                }else{
                    let mut exponent = 1;
                    let mut name = term_string2.to_string();
                    if term_string2.contains("^"){
                        let (parts, indexs) = split_string_based_on_list( &term_string2, &pow_oprator); 
                        name = parts[0].to_string();
                        exponent = parts[1].parse::<u32>().unwrap();
                    }
                    if !inputs.contains(&name){
                        inputs.push(name.clone());
                    }
                    variables.push(Variable{
                        name: name,
                        exponent: exponent,
                    });
                }
            }
            terms.push(Term{
                coefficient: coefficient,
                variables: variables,
            });
            i += 1;
        }
        Equation{
            inputs: inputs,
            terms: terms,
        }
    }

    pub fn evaluate(&self, values: Vec<i128>) -> i128{
        let mut result:i128 = 0;
        for term in &self.terms{
            let mut term_result = term.coefficient;
            for variable in &term.variables{
                let index = self.inputs.iter().position(|x| x == &variable.name).unwrap();
                term_result *= values[index].pow(variable.exponent);
            }
            result += term_result;
        }
        result
    }



    pub fn evaluateOverFieldF(&self, values: Vec<i128>, F: i128) -> i128{
        let mut result:i128 = 0;
        for term in &self.terms{
            let mut term_result = term.coefficient;
            for variable in &term.variables{
                let index = self.inputs.iter().position(|x| x == &variable.name).unwrap();
                term_result *= values[index].pow(variable.exponent);
                term_result = term_result % F;
            }
            result += term_result;
            result = result % F;
        }
        result
    }
}


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


pub fn split_string_based_on_list<'a>(input: &'a str, delimiters: &'a [char]) -> (Vec<&'a str>,  Vec<usize>){
    let indexes = input.match_indices(|c| delimiters.contains(&c)).map(|(index, _)| index).collect::<Vec<_>>();
    let result = input.split(|c| delimiters.contains(&c)).collect::<Vec<_>>();
    return (result, indexes);
}

fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}