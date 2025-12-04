use std::fs;

fn print_grid(grid : &Vec<Vec<char>>){
    for i in 0..grid.len(){
        let mut line = String::new();
        for j in 0..grid[i].len(){
            line.push_str(&grid[i][j].to_string());
        }
        println!("{line}");
    }
}

fn main() {
    let file_path = "input.txt";

    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut grid_out : Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){
        grid.push(line.chars().map(|b| b as char).collect::<Vec<_>>());
        grid_out.push(line.chars().map(|b| b as char).collect::<Vec<_>>());
    }

    //print_grid(&grid_out);

    let imax = grid.len();
    let jmax = grid[0].len();


    let mut total_removed = 0;
    let mut removed = 1;

    while removed > 0{
        removed = 0;
        for i in 0..imax{
            for j in 0..jmax{

                if grid[i][j] != '@'{
                    continue;
                }

                let mut n_nei = 0;

                for x in [-1 as i32, 0, 1]{
                    for y in [-1 as i32, 0, 1]{

                        if x == 0 && y == 0{
                            continue;
                        }
                        if x == -1 && i == 0{
                            continue;
                        }
                        if x == 1 && i == (imax - 1){
                            continue;
                        }
                        if y == -1 && j == 0{
                            continue;
                        }
                        if y == 1 && j == (jmax - 1){
                            continue;
                        }

                        let ipos : usize = (i as i32 + x) as usize;
                        let jpos : usize = (j as i32 + y) as usize;
                        if grid[ipos][jpos] == '@'{
                            n_nei += 1;
                        }
                    }
                }

                if n_nei < 4{
                    removed += 1;
                    grid_out[i][j] = 'x';
                }
            }
        }

        for i in 0..imax{
            for j in 0..jmax{
                if grid_out[i][j] == 'x'{
                    total_removed += 1;
                    grid[i][j] = '.';
                    grid_out[i][j] = '.';
                }
            }
        }
    }

    println!("Total removed: {total_removed}");

    print_grid(&grid_out);
}
