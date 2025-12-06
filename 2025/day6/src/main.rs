use std::fs;
use itertools::rev;

fn main() {
    let file_path = "input.txt";

    let mut data : Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let mut lc : Vec<char> = Vec::new();
        let s = line.to_string();
        for c in s.chars(){
            lc.push(c);
        }
        data.push(lc.clone());
    }

    let mut nums : Vec<Vec<u64>> = Vec::new();

    let n_lines = data.len() - 1;
    let mut n_cols = 1;
    let mut col_breaks : Vec<usize> = Vec::new();

    col_breaks.push(0);

    // Find column breaks
    for j in 0..data[0].len(){
        let mut is_break = true;
        for i in 0..data.len(){
            if data[i][j] != ' '{
                is_break = false;
                break;
            }
        }
        if is_break{
            col_breaks.push(j);
            n_cols += 1;
        }
    }

    col_breaks.push(data[0].len());

    println!("{n_lines} x {n_cols}");

    for jcol in 0..n_cols{
        let jmax = col_breaks[jcol+1];
        let jmin = col_breaks[jcol];
        let mut col_nums : Vec<u64> = Vec::new();
        for j in rev(jmin..jmax){
            let mut num = 0;
            let mut power = 1;
            for i in rev(0..=(n_lines - 1)){
                if data[i][j] != ' '{
                    num = num + data[i][j].to_digit(10).expect("Error parsing num") * power;
                    power = power * 10;
                }
            }
            if num != 0{
                col_nums.push(num.into());
            }
        }
        nums.push(col_nums.clone());
    }

    let mut opes : Vec<char> = Vec::new();
    for j in 0..data[0].len(){
        if data[n_lines][j] != ' '{
            opes.push(data[n_lines][j]);
        }
    }

    //println!("{:#?}", opes);
    //println!("{:#?}", nums);

    let mut sum : u64 = 0;
    for num_op in 0..n_cols{
        let op = opes[num_op];
        let mut current : u64 = if op == '+' { 0 } else { 1 };

        for c in nums[num_op].clone(){
            if op == '+'{
                current = current + c;
            }
            else{
                current = current * c;
            }
        }

        //println!("{current}");
        sum = sum + current;
    }

    println!("sum: {sum}");
}
