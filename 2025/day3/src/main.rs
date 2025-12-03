use std::fs;
use num_traits::pow;

fn part_one(num_vect: &Vec<u32>) -> u32{
    let mut joltage = 0;
    for i in 0..num_vect.len(){
        for j in i+1..num_vect.len(){
            let num = num_vect[i] * 10 + num_vect[j];
            if num > joltage{
                joltage = num;
            }
        }
    }
    return joltage;
}

fn part_two(expo: u32, num_vect: &Vec<u32>, first: usize) -> u64{

    let mut max = 0;
    let mut pos: usize = 0;
    let last = num_vect.len() - expo as usize;

    for i in first..last{
        if num_vect[i] > max{
            max = num_vect[i];
            pos = i as usize;
        }
    }

    if expo == 0{
        return max.into();
    }

    let power_of_tens: u64 = pow(10, expo.try_into().unwrap());
    let max_cast: u64 = max as u64;

    return max_cast * power_of_tens + part_two(expo - 1, num_vect, pos + 1);
}

fn main() {
    let file_path = "input.txt";

    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;
    const RADIX: u32 = 10;

    for line in fs::read_to_string(file_path).unwrap().lines(){
        let s = line.to_string();

        //println!("{s}");

        let num_vect: Vec<u32> = s.chars()
            .into_iter()
            .map(|p| p.to_digit(RADIX).unwrap() )
            .collect();

        let joltage_1 = part_one(&num_vect);
        let joltage_2 = part_two(11, &num_vect, 0);

        sum_1 += joltage_1 as u64;
        sum_2 += joltage_2 as u64;
    }

    println!("Part 1 total joltage: {sum_1}");
    println!("Part 2 total joltage: {sum_2}");
}
