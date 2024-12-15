use std::fs;

fn find_initial_pos(grid : &Vec<Vec<char>>) -> (usize, usize){
    for i in 0..grid[0].len(){
        for j in 0..grid.len(){
            if grid[j][i] == '^'{
                return (i, j);
            }
        }
    }

    println!("Initial position not found !");
    return (0, 0);
}

fn oob(xx : i32, yy : i32, xmax : usize, ymax : usize) -> bool{
    if xx < 0 { return true; }
    if yy < 0 { return true; }
    if xx >= xmax as i32 { return true; }
    if yy >= ymax as i32 { return true; }
    return false;
}

fn cur(grid : &mut Vec<Vec<char>>, xx : i32, yy : i32) -> &mut char {
    let i = xx as usize;
    let j = yy as usize;
    return &mut grid[j][i];
}

fn grid_exes(mut grid  : Vec<Vec<char>>) -> Vec<Vec<char>>{
    let (x0, y0) = find_initial_pos(&grid);

    let n_lines = grid.len();
    let n_cols = grid[0].len();

    println!("{n_lines} x {n_cols}");

    let mut exes = 0;
    let mut x : i32 = x0 as i32;
    let mut y : i32 = y0 as i32;

    let mut dir_x : i32 = 0;
    let mut dir_y : i32 = -1;

    loop {
        if *cur(&mut grid, x, y) != 'X'{
            *cur(&mut grid, x, y) = 'X';
            exes += 1;
        }
        //println!("x: {x} y {y} grid[x][y] {}", *cur(&mut grid, x, y));

        x += dir_x;
        y += dir_y;

        if oob(x, y, n_cols, n_lines){
            break;
        }
        else{
            if *cur(&mut grid, x, y) == '#'{
                x -= dir_x;
                y -= dir_y;

                if dir_x == 1{
                    dir_x = 0;
                    dir_y = 1;
                }
                else if dir_x == -1{
                    dir_x = 0;
                    dir_y = -1;
                }
                else if dir_y == 1{
                    dir_x = -1;
                    dir_y = 0;
                }
                else if dir_y == -1{
                    dir_x = 1;
                    dir_y = 0;
                }

                x += dir_x;
                y += dir_y;
            }
        }
    }

    //for j in 0..grid.len(){
    //    println!("{:?}", grid[j]);
    //}

    println!("X on the map: {exes}");

    return grid
}

fn is_grid_loop(mut grid : Vec<Vec<char>>, obs_x : usize, obs_y : usize, x0 : usize, y0 : usize) -> bool {

    if grid[obs_x][obs_y] != '.'{
        return false;
    }

    let n_lines = grid.len();
    let n_cols = grid[0].len();

    grid[obs_x][obs_y] = '#';

    let mut x : i32 = x0 as i32;
    let mut y : i32 = y0 as i32;

    let mut dir_x : i32 = 0;
    let mut dir_y : i32 = -1;

    loop {

        if dir_x == 1{
            *cur(&mut grid, x, y) = '>';
        }
        else if dir_x == -1{
            *cur(&mut grid, x, y) = '<';
        }
        else if dir_y == 1{
            *cur(&mut grid, x, y) = 'v';
        }
        else if dir_y == -1{
            *cur(&mut grid, x, y) = '^';
        }

        //println!("x: {x} y {y} grid[x][y] {}", *cur(&mut grid, x, y));

        x += dir_x;
        y += dir_y;

        if oob(x, y, n_cols, n_lines){
            return false;
        }

        if *cur(&mut grid, x, y) == '#'{
            x -= dir_x;
            y -= dir_y;

            if dir_x == 1{
                dir_x = 0;
                dir_y = 1;
            }
            else if dir_x == -1{
                dir_x = 0;
                dir_y = -1;
            }
            else if dir_y == 1{
                dir_x = -1;
                dir_y = 0;
            }
            else if dir_y == -1{
                dir_x = 1;
                dir_y = 0;
            }

            x += dir_x;
            y += dir_y;
        }
        else if *cur(&mut grid, x, y) == '>' && dir_x == 1{
            return true;
        }
        else if *cur(&mut grid, x, y) == '<' && dir_x == -1{
            return true;
        }
        else if *cur(&mut grid, x, y) == 'v' && dir_y == 1{
            return true;
        }
        else if *cur(&mut grid, x, y) == '^' && dir_y == -1{
            return true;
        }
    }
}

fn main() {
    let file_path = "../../input/input.txt";
    //let file_path = "../../input/input_test.txt";

    let mut grid : Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){
        grid.push(line.chars().map(|b| b as char).collect::<Vec<_>>());
    }

    //println!("{n_lines} x {n_cols}");
    println!("Initial pos: {:?}", find_initial_pos(&grid));

    let (x0, y0) = find_initial_pos(&grid);

    let grid_with_x = grid_exes(grid.clone());

    let mut n_loops = 0;
    for i in 0..grid[0].len(){
        for j in 0..grid.len(){
            //println!("{i} {j}");
            if grid_with_x[i][j] == 'X'{
                if is_grid_loop(grid.clone(), i, j, x0, y0){
                    n_loops += 1;
                    //println!("Loop with obstacle in: {i} {j}");
                }
            }
        }
    }

    println!("Number of possible loops : {n_loops}");
}
