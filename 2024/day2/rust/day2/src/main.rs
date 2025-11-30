use std::fs;
use regex::Regex;

fn is_seq_ok(sequence: &Vec<i32>) -> bool {
    let mut diff : Vec<i32> = vec![];
    for idx in 0..sequence.len()-1{
        diff.push(sequence[idx+1] - sequence[idx]);
    }

    // Check monotony
    let sign = diff[0] > 0;

    for d in diff{
        let cur_sign = d > 0;

        if (sign && !cur_sign) || (!sign && cur_sign){
            return false;
        }

        if d.abs() == 0 || d.abs() > 3{
            return false;
        }
    }

    return true;
}

fn seq_without_one(sequence: &Vec<i32>, idx_removed : i32) -> Vec<i32>{
    let mut outseq : Vec<i32> = vec![];
    for idx in 0..sequence.len(){
        if idx != idx_removed.try_into().unwrap(){
            outseq.push(sequence[idx]);
        }
    }
    return outseq;
}

fn main() {

    let file_path = "../../input/input.txt";

    let re = Regex::new(r" ").unwrap();

    let mut n_safe = 0;

    for line in fs::read_to_string(file_path).unwrap().lines(){

        let splitted : Vec<&str> = re.split(line).collect();

        let mut sequence : Vec<i32> = vec![];
        for s in splitted{
            sequence.push(s.parse().expect("Cound not find a number"));
        }

        let mut safe = false;
        for idx in 0..sequence.len(){
            let cidx = idx as i32;
            let vw = seq_without_one(&sequence, cidx);
            if is_seq_ok(&vw){
                safe = true;
                break;
            }
        }

        if safe{ n_safe += 1; }

    }

    println!("Number of safe reports: {n_safe}");

    //let v = vec![1, 2, 3, 4, 5];
    //let v_w_2 = seq_without_one(&v, 2);
    //println!("{:?} without 2 is {:?}", v, v_w_2);
}
