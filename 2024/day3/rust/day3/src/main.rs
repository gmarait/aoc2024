use std::fs;
use regex::Regex;

fn main() {

    let file_path = "../../input/input.txt";

    //let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    let mut execute = true;

    for line in fs::read_to_string(file_path).unwrap().lines(){

        let instructions : Vec<&str> = re.captures_iter(line).map(|caps| {
            let (_, [s]) = caps.extract();
            s
        }).collect();

        for i in instructions{
            if i == "do()" {
                execute = true;
            }
            else if i == "don't()"{
                execute = false;
            }
            else{

                for (_, [n1, n2]) in re_mul.captures_iter(i).map(|c| c.extract()){
                    if execute{
                        let n1: i32 = n1.parse().unwrap();
                        let n2: i32 = n2.parse().unwrap();
                        println!("{i} -> {n1} * {n2}");
                        sum += n1 * n2;
                    }
                }

            }
        }
    }

    println!("Sum of multiplications: {sum}");
}
