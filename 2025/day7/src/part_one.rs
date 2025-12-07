use std::fs;

fn print_grid(grid: &Vec<Vec<char>>){
    for i in 0..grid.len(){
        for j in 0..grid[0].len(){
            print!("{}", grid[i][j]);
        }
        println!("");
    }
}

fn main() {
    let file_path = "input.txt";

    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let mut lc : Vec<char> = Vec::new();
        let s = line.to_string();
        for c in s.chars(){
            lc.push(c);
        }
        grid.push(lc.clone());
    }

    let mut pos_j : Vec<usize> = Vec::new();
    // Find initial position
    for j in 0..grid[0].len(){
        if grid[0][j] == 'S'{
            pos_j.push(j);
            break;
        }
    }

    println!(">> {}", pos_j[0]);

    let mut n_splits = 0;

    for i in 0..grid.len(){
        let mut next_pos_j : Vec<usize> = Vec::new();
        for j in 0..grid[0].len(){
            for j_beam in pos_j.iter(){
                if *j_beam == j{
                    if grid[i][j] == '^'{
                        next_pos_j.push(j-1);
                        next_pos_j.push(j+1);
                        n_splits += 1;
                    }
                    else{
                        next_pos_j.push(j);
                    }
                }
            }
            // Display
            for j_beam in pos_j.iter(){
                if *j_beam == j{
                    if grid[i][j] == '.'{
                        grid[i][j] = '|';
                    }
                }
            }
        }
        next_pos_j.dedup(); // Remove duplictes
        pos_j = next_pos_j.clone();
    }

    print_grid(&grid);
    println!("n_splits: {n_splits}");
}
