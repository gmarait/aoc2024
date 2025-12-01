use std::fs;
use regex::Regex;

fn main() {
    let file_path = "input.txt";

    let re = Regex::new(r"^(L|R)(\d+)$").unwrap();

    let mut dial: i32 = 50;
    let mut counter_stop: i32 = 0;
    let mut counter_pass: i32 = 0;

    println!("  - The dial starts by pointing at {dial}");

    for line in fs::read_to_string(file_path).unwrap().lines(){
        let s = line.to_string();

        let Some(caps) = re.captures(&s) else {
            println!("no match!");
            return;
        };

        let direction: &str = &caps[1];
        let offset: &i32 = &caps[2].parse().expect("Cound not find a number");


        if direction == "L" {
            for _i in 0..*offset{
                dial = dial - 1;
                if dial == 0{
                    counter_pass += 1;
                }
                else if dial < 0{
                    dial = dial + 100;
                }
            }
        }
        else{
            for _i in 0..*offset{
                dial = dial + 1;
                if dial == 100{
                    counter_pass += 1;
                    dial = 0;
                }
            }
        }

        if dial == 0{
            counter_stop += 1;
        }

        println!("  - The dial is rotated {s} to point at {dial}. Passed through 0: {counter_pass} times");
    }

    println!("Number of 0 stopped: {counter_stop}");
    println!("Number of 0 passed: {counter_pass}");
}
