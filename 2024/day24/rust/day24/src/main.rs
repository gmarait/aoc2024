use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Clone)]
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

#[derive(Clone)]
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
    let mut list_outputs : HashSet<String> = HashSet::new();

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
            list_outputs.insert(caps[4].to_string());
        }
    }

    //println!("{:?}", states);
    //println!("{:?}", instr);

    let x_wires : Vec<String> = get_wires(&states, 'x');
    let bin_num_x = get_bin_from_wires(&states, &x_wires);
    //println!("x -> {:?}", bin_num_x);
    let xval = bin_to_num(&bin_num_x);
    //println!("xval -> {xval}");

    let y_wires : Vec<String> = get_wires(&states, 'y');
    let bin_num_y = get_bin_from_wires(&states, &y_wires);
    //println!("y -> {:?}", bin_num_y);
    let yval = bin_to_num(&bin_num_y);
    //println!("yval -> {xval}");

    let expected_zval = xval + yval;
    //println!("expected zval -> {expected_zval}");
    let bin_num_z = num_to_bin(expected_zval);
    //println!("expected zval binary -> {:?}", bin_num_z);

    let mut swaps : Vec<(String, String)> = Vec::new();
    let bin_num_comp = compute_with_swaps(&mut states.clone(), &mut instr.clone(), &swaps);

    //println!("{:?}", bin_num_comp);
    let result_part_1 = bin_to_num(&bin_num_comp);
    println!("Result part 1: {result_part_1}");

    // Part 2
    swaps.push(("dhg".to_string(), "z06".to_string()));
    swaps.push(("brk".to_string(), "dpd".to_string()));
    swaps.push(("bhd".to_string(), "z23".to_string()));
    //swaps.push(("z45".to_string(), "jrm".to_string()));
    let bin_num_comp = compute_with_swaps(&mut states.clone(), &mut instr.clone(), &swaps);

    let num_ob = bin_to_num(&bin_num_comp);
    println!("Now getting: {num_ob}");

    swap_outputs(&mut instr, &swaps);

    let mut suspects : HashSet<String> = HashSet::new();

    print!("Z expected: {expected_zval} = ");
    print_bin(&bin_num_z);
    print!("Z found: {num_ob} = ");
    print_bin(&bin_num_comp);

    for i in 0..bin_num_comp.len(){
        if bin_num_comp[i] != bin_num_z[i]{
            suspects.insert(format!("z{:02}", i).to_string());
            println!("Diff in bit {i} --> {} - {}", bin_num_comp[i], bin_num_z[i]);
        }
    }

    for i in 0..(bin_num_z.len() - bin_num_comp.len()){
        let k = bin_num_comp.len() + i - 1;
        suspects.insert(format!("z{:02}", k).to_string());
        println!("Diff in bit {k} --> . - {}", bin_num_z[k]);
    }

    let mut growing = true;
    let mut n_suspects = suspects.len();

    while growing{
        for ins in &instr{
            let mut new_sus : Vec<String> = Vec::new();
            for sus in &suspects{
                if ins.output == *sus{

                    if list_outputs.contains(&ins.in1){
                        new_sus.push(ins.in1.to_string());
                    }
                    if list_outputs.contains(&ins.in2){
                        new_sus.push(ins.in2.to_string());
                    }
                }
            }

            for n in new_sus{
                suspects.insert(n.to_string());
            }
        }

        if suspects.len() > n_suspects{
            n_suspects = suspects.len();
        }
        else{
            growing = false;
        }
        println!("number of suspects: {}", suspects.len());
    }

    println!("Suspects: {:?}", suspects);

    write_to_dot(&instr, &suspects);

    let mut sol_part2 : Vec<String> = Vec::new();
    for (w1, w2) in swaps{
        sol_part2.push(w1.to_string());
        sol_part2.push(w2.to_string());
    }
    sol_part2.sort();
    println!("Sol part 2:");
    for s in sol_part2{
        print!("{},", s);
    }
    println!("");
}

fn get_wires(states : &HashMap<String, bool>, c : char) -> Vec<String>{
    let mut c_wires : Vec<String> = Vec::new();

    for (wire, _st) in states{
        if wire.chars().nth(0) == Some(c){
            c_wires.push(wire.to_string());
        }
    }

    c_wires.sort();
    return c_wires;
}

fn get_bin_from_wires(states : &HashMap<String, bool>, wires : &Vec<String>) -> Vec<char>{
    let mut bin_num : Vec<char> = Vec::new();
    for w in wires{
        let val = states.get(w).unwrap();
        if *val { bin_num.push('1'); }
        else { bin_num.push('0'); }
    }
    return bin_num;
}

// Could not find a proper power function in std
// Computes 2^exp
fn pow_2(exp : usize) -> usize {
    let mut res = 1;
    for _i in 0..exp{
        res = 2 * res;
    }
    return res;
}

fn bin_to_num(bin : &Vec<char>) -> usize{
    let mut power = 0;
    let mut num = 0;
    for c in bin{
        if *c == '1'{
            num = num + pow_2(power);
        }
        power = power + 1;
    }
    return num;
}

fn num_to_bin(num : usize) -> Vec<char>{
    let mut bin : Vec<char> = Vec::new();

    let mut cur_num = num;
    while cur_num >= 1{
        if cur_num % 2 == 1{
            bin.push('1');
        }
        else{
            bin.push('0');
        }
        cur_num = cur_num / 2;
    }

    bin.push('1');

    return bin;
}

fn print_bin(bin : &Vec<char>){
    for i in (0..bin.len()).rev(){
        print!("{}", bin[i]);
    }
    println!("");
}

fn compute_circuit(states : &mut HashMap<String, bool>, instr: &Vec<Instruction>){
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
}

fn swap_outputs(instr: &mut Vec<Instruction>, swaps : &Vec<(String, String)>){
    for (out1, out2) in swaps{
        for i in &mut *instr{
            if i.output == *out1{
                i.output = out2.to_string();
            }
                else if i.output == *out2{
                    i.output = out1.to_string();
                }
        }
    }
}

fn compute_with_swaps(states : &mut HashMap<String, bool>, instr: &mut Vec<Instruction>, swaps : &Vec<(String, String)>) -> Vec<char> {

    swap_outputs(instr, swaps);

    compute_circuit(states, instr);
    //println!("{:?}", states);

    let z_wires : Vec<String> = get_wires(&states, 'z');
    let bin_num = get_bin_from_wires(&states, &z_wires);

    return bin_num;
}

fn write_to_dot(instr: &Vec<Instruction>, suspects : &HashSet<String>){

    let f = File::create("graph.dot").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all("digraph G {\n".as_bytes()).expect("Unable to write data");

    for i in instr{
        let gate_name = format!("{}_{}_{}", i.in1, op_to_str(&i.op), i.in2);
        let edge1 = format!("  {} -> {};\n", i.in1, gate_name);
        f.write_all(edge1.as_bytes()).expect("Unable to write data");
        let edge2 = format!("  {} -> {};\n", i.in2, gate_name);
        f.write_all(edge2.as_bytes()).expect("Unable to write data");

        if suspects.contains(&i.output){
            let color_sus = format!("  {} [style=bold, color=red];\n", i.output);
            f.write_all(color_sus.as_bytes()).expect("Unable to write data");
        }

        let edge3 = format!("  {} -> {};\n", gate_name, i.output);
        f.write_all(edge3.as_bytes()).expect("Unable to write data");
    }

    f.write_all("}".as_bytes()).expect("Unable to write data");
}
