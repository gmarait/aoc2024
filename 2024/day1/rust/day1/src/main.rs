use std::fs;
use regex::Regex;

fn main() {

    let file_path = "../../C++/input.txt";

    let re = Regex::new(r"^(\d+)\s+(\d+)$").unwrap();

    let mut list1 : Vec<i32> = vec![];
    let mut list2 : Vec<i32> = vec![];

    for line in fs::read_to_string(file_path).unwrap().lines(){
        let s = line.to_string();

        let Some(caps) = re.captures(&s) else {
            println!("no match!");
            return;
        };

        let n1: &i32 = &caps[1].parse().expect("Cound not find a number");
        let n2: &i32 = &caps[2].parse().expect("Cound not find a number");

        list1.push(*n1);
        list2.push(*n2);
    }

    list1.sort();
    list2.sort();

    let mut diff = 0;
    for (n1, n2) in list1.iter().zip(list2.iter()){
        diff += (n1 - n2).abs();
    }

    let mut similarity = 0;
    for n1 in &list1{
        let mut count = 0;
        for n2 in &list2{
            if *n1 == *n2 {
                count += 1;
            }
        }
        similarity += *n1 * count;
    }

    println!("Difference {diff}");
    println!("Similarity {similarity}");
}
