use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input.txt";

    let re_digits = Regex::new(r"(\d+)-(\d+)").unwrap();

    let mut buff_reader = BufReader::new(File::open(file_path)?);
    let mut line = String::new();
    buff_reader.read_line(&mut line)?;

    let matches: Vec<_> = re_digits.find_iter(line.as_str()).map(|m| m.as_str()).collect();

    let mut sum_part_one: i64 = 0;
    let mut sum_part_two: i64 = 0;

    for m in matches{
        let Some(caps) = re_digits.captures(&m) else {
            println!("no match!");
            return Ok(());
        };
        let start: &i64 = &caps[1].parse().expect("Cound not find a number");
        let end: &i64 = &caps[2].parse().expect("Cound not find a number");

        println!("{start} to {end}");

        for i in *start..=*end{
            let num = i.to_string();
            let half = num.len() / 2;

            // Part 1
            if num.len() % 2 == 0{
                if &num[0..half] == &num[half..num.len()]{
                    //println!("num: {num}");
                    sum_part_one += i;
                }
            }

            // Part 2
            for k in 1..=half{
                // Search for sequence of size k
                if num.len() % k != 0{
                    continue;
                }
                //println!("Search size: {k}");

                let n_seq = num.len() / k;

                let mut found: bool = true;
                for w in 1..n_seq{
                    if &num[w*k..(w+1)*k] != &num[0..k]{
                        found = false;
                        break;
                    }
                }

                if found{
                    println!("Found: {i}");
                    sum_part_two += i;
                    break;
                }
            }
        }
    }

    println!("sum_part_one: {sum_part_one}");
    println!("sum_part_two: {sum_part_two}");

    return Ok(());
}
