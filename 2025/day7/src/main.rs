use std::fs;

fn main() {
    let start_time = std::time::Instant::now();

    let file_path = "doc.txt";

    // Load grid
    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines(){
        let mut lc : Vec<char> = Vec::new();
        for c in line.to_string().chars(){
            lc.push(c);
        }
        grid.push(lc.clone());
    }

    // Find initial position
    let pos_j = grid[0].iter().position(|&x| x == 'S').expect("Could not find 'S' on first line!");

    let size_i = grid.len();
    let size_j = grid[0].len();

    // At each node of the tree, keep the number of subtrees
    let mut cache_matrix : Vec<Vec<usize>> = vec![vec![0; size_j]; size_i];
    cache_matrix[0][pos_j] = 1;

    for i in 0..size_i-1{
        for j in 0..size_j{
            if grid[i+1][j] == '^'{
                cache_matrix[i+1][j+1] += cache_matrix[i][j];
                cache_matrix[i+1][j-1] += cache_matrix[i][j];
            }
            else{
                cache_matrix[i+1][j] += cache_matrix[i][j];
            }
        }
    }

    //println!("");
    //for i in 0..cache_matrix.len(){
    //    for j in 0..cache_matrix[0].len(){
    //        if cache_matrix[i][j] == 0{
    //            print!("-- ");
    //        }
    //        else{
    //            print!("{:0>2} ", cache_matrix[i][j]);
    //        }
    //    }
    //    println!("");
    //}

    let mut n_timelines : u64 = 0;
    for j in 0..grid[0].len(){
        n_timelines += cache_matrix[grid[0].len() - 1][j] as u64;
    }

    println!("n_timelines: {n_timelines}");
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
}
