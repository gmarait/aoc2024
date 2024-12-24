use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

enum Operation{
    OR,
    XOR,
    AND
}

fn str_to_op(s : &str) -> Operation{
    match s{
        "OR" => return Operation::OR,
        "XOR" => return Operation::XOR,
        "AND"  => return Operation::AND,
        _ => {
            println!("Error str_to_op");
            return Operation::OR;
        }
    }
}

fn op_to_str(s : &Operation) -> String{
    match s{
        Operation::OR  => return "OR".to_string(),
        Operation::XOR => return "XOR".to_string(),
        Operation::AND  => return "AND".to_string(),
    }
}

struct Instruction{
    in1: String,
    in2: String,
    op: Operation,
    output: String
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Op [{} {} {} -> {}]", self.in1, op_to_str(&self.op), self.in2, self.output)
    }
}

fn main() {
    let file_path = "../../input/input.txt";
    //let file_path = "../../input/test.txt";

    let re_state = Regex::new(r"([\w|\d]{3}): (0|1)").unwrap();
    let re_instr = Regex::new(r"([\w|\d]{3}) (OR|AND|XOR) ([\w|\d]{3}) -> ([\w|\d]{3})").unwrap();

    let mut states : HashMap<String, bool> = HashMap::new();
    let mut instr : Vec<Instruction> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){

        if re_state.is_match(line){
            let Some(caps) = re_state.captures(line) else { return };
            let wire = &caps[1].to_string();
            let state = if &caps[2] == "1" { true } else { false };
            states.insert((&wire).to_string(), state);
        }

        else if re_instr.is_match(line){
            let Some(caps) = re_instr.captures(line) else { return };

            let i = Instruction {
                in1: caps[1].to_string(),
                in2: caps[3].to_string(),
                op: str_to_op(&caps[2]),
                output: caps[4].to_string(),
            };

            instr.push(i);
        }

    }

    //println!("{:?}", states);
    //println!("{:?}", instr);

    part_1(&mut states, &mut instr);
}

fn part_1(states : &mut HashMap<String, bool>, instr: &Vec<Instruction>){

    let mut done = HashSet::new();

    while done.len() < instr.len(){
        for i in 0..instr.len(){

            if done.contains(&i){
                continue;
            }

            let ins = &instr[i];

            let st1 = match states.get(&ins.in1){
                Some(s) => s,
                None => { continue; },
            };

            let st2 = match states.get(&ins.in2){
                Some(s) => s,
                None => { continue; },
            };

            let result : bool = match ins.op{
                Operation::OR => { *st1 || *st2 },
                Operation::AND => { *st1 && *st2 },
                Operation::XOR => { (*st1 && !*st2) || (!*st1 && *st2) },
            };

            match states.get_mut(&ins.output){
                Some(s) => { *s = result; },
                None => {
                    states.insert(ins.output.to_string(), result);
                },
            };

            done.insert(i);
        }
    }

    println!("{:?}", states);

    let mut z_wires : Vec<String> = Vec::new();

    for (wire, _st) in &mut *states{
        if wire.chars().nth(0) == Some('z'){
            z_wires.push(wire.to_string());
        }
    }

    z_wires.sort();

    println!("{:?}", z_wires);

    let mut bin_num : Vec<char> = Vec::new();
    for zw in z_wires{
        let val = states.get(&zw).unwrap();
        if *val { bin_num.push('1'); }
        else { bin_num.push('0'); }
    }

    println!("{:?}", bin_num);

    // Could not find a proper power function in std
    fn pow_2(exp : usize) -> usize {
        let mut res = 1;
        for _i in 0..exp{
            res = 2 * res;
        }
        return res;
    }

    let mut power = 0;
    let mut result = 0;
    for c in bin_num{
        if c == '1'{
            result = result + pow_2(power);
        }
        power = power + 1;
    }

    println!("Result: {result}");
}
