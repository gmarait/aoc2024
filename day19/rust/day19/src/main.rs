use std::fs;
use regex::Regex;
use std::collections::HashMap;
use indicatif::ProgressBar;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn towel_possible(avail_map : &HashMap<char, Vec<String>>, towel : &String, first_idx : usize) -> bool {
    //println!("towel: {:?}", towel);
    //println!("first_idx: {:?}", first_idx);

    let sletter = towel.chars().nth(first_idx).unwrap();

    match avail_map.get(&sletter) {
        Some(poss) => {
            for p in poss{
                let l = p.len();
                let next_idx = first_idx + l;

                if next_idx == towel.len(){
                    //println!("{}", towel[first_idx..next_idx].to_string());
                    if *p == towel[first_idx..next_idx]{
                        return true;
                    }
                    //return *p == towel[first_idx..next_idx];
                }
                else if next_idx < towel.len(){
                    //println!("{}", towel[first_idx..next_idx].to_string());
                    if *p == towel[first_idx..next_idx]{
                        if  towel_possible(avail_map, towel, next_idx){
                            return true;
                        }
                    }
                }
            }
            return false;
        },
        None => {
            //println!("Found: false");
            return false;
        },
    }
}

fn main() {
    let file_path = "../../input/input.txt";
    //let file_path = "../../input/test.txt";

    let re_split = Regex::new(r", ").unwrap();

    let lines = read_lines(file_path);

    let avail : Vec<&str> = re_split.split(&lines[0]).collect();
    //println!("{:?}", avail);

    let mut hash_first_letter : HashMap<char, Vec<String>> = HashMap::new();

    for a in &avail{
        let sletter = a.chars().nth(0).unwrap();
        match hash_first_letter.get_mut(&sletter) {
            Some(vec) => vec.push(a.to_string()),
            None => {
                let mut newvec : Vec<String> = Vec::new();
                newvec.push(a.to_string());
                hash_first_letter.insert(sletter, newvec);
            },
        }
    }

    //println!("{:?}", avail);
    println!("{:?}", hash_first_letter);

    let towels = &lines[2..];
    //let mut towels : Vec<String> = Vec::new();
    //towels.push("brwrr".to_string());
    //println!("{:?}", towels);

    let mut possibles = 0;
    let bar = ProgressBar::new(towels.len().try_into().unwrap());
    for t in towels{
        if towel_possible(&hash_first_letter, &t, 0){
            possibles += 1;
        }
        //else{
        //    println!("Impossible {t}");
        //}
        bar.inc(1);
    }
    bar.finish();

    println!("Number of possible towels: {possibles}");
}
