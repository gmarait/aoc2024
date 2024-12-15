use std::fs;
use regex::Regex;

fn print_grid(grid : &Vec<Vec<char>>){
    for i in 0..grid.len(){
        let mut line = String::new();
        for j in 0..grid[i].len(){
            line.push_str(&grid[i][j].to_string());
        }
        println!("{line}");
    }
}

fn find_robot(grid : &Vec<Vec<char>>) -> (i32, i32){
    for i in 0..grid.len(){
        for j in 0..grid[i].len() {
            if grid[i][j] == '@'{
                return (i as i32, j as i32);
            }
        }
    }

    return (0,0)
}

fn find_next_in_grid(grid : &Vec<Vec<char>>, robot : (i32, i32), dir : (i32, i32), next : char) -> ((i32, i32), usize){

    let mut cur  = (robot.0 + dir.0, robot.1 + dir.1);
    let mut gc   = grid[cur.0 as usize][cur.1 as usize];
    let mut distance = 0;

    while gc != next{
        if cur.0 < 1 ||
            cur.1 < 1 ||
            cur.0 >= (grid.len() - 1).try_into().unwrap() ||
            cur.1 >= (grid[0].len() - 1).try_into().unwrap() {
            return ((0, 0), 1000000);
        }
        cur  = (cur.0 + dir.0, cur.1 + dir.1);
        gc   = grid[cur.0 as usize][cur.1 as usize];
        distance += 1;
    }
    return (cur, distance);
}

fn compute_score(grid : &Vec<Vec<char>>) -> usize{
    let mut score = 0;
    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if grid[i][j] == 'O'{
                score += 100 * i + j;
            }
        }
    }
    return score;
}

fn iteration(grid : &mut Vec<Vec<char>>, robot : &mut (i32, i32), instr : char){
    let dir : (i32, i32) = match instr {
        '^'=> (-1, 0),
        'v'=> ( 1, 0),
        '>'=> ( 0, 1),
        '<'=> ( 0,-1),
        _ =>  ( 0, 0)
    };

    //println!("Direction {instr}");

    let (_next_wall, dist_wall) = find_next_in_grid(&grid, robot.clone(), dir, '#');
    let (next_space, dist_space) = find_next_in_grid(&grid, robot.clone(), dir, '.');

    //println!("Wall {:?} {:?}", next_wall, dist_wall);
    //println!("Space {:?} {:?}", next_space, dist_space);

    if dist_wall <= dist_space{
        return;
    }

    let mut tmp = '.';
    let mut cur = (robot.0, robot.1);
    loop{
        let moving : char = grid[cur.0 as usize][cur.1 as usize];
        grid[cur.0 as usize][cur.1 as usize] = tmp;
        tmp = moving;
        cur.0 += dir.0;
        cur.1 += dir.1;
        if moving == '@'{
            robot.0 = cur.0;
            robot.1 = cur.1;
        }
        if cur.0 == next_space.0 && cur.1 == next_space.1{
            grid[next_space.0 as usize][next_space.1 as usize] = tmp;
            return;
        }
    }
    //for _i in 0..dist_space + 1{
    //    let moving : char = grid[cur.0 as usize][cur.1 as usize];
    //    grid[cur.0 as usize][cur.1 as usize] = tmp;
    //    tmp = moving;
    //    cur.0 += dir.0;
    //    cur.1 += dir.1;
    //}
}

fn main() {
    let file_path = "../../input/input.txt";
    //let file_path = "../../input/test.txt";

    let re_grid = Regex::new(r"^(#|O|\.|@)+$").unwrap();
    let re_instr = Regex::new(r"^(<|>|\^|v)+$").unwrap();

    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut instr : Vec<char> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){

        if re_grid.is_match(line){
            grid.push(line.chars().map(|b| b as char).collect::<Vec<_>>());
        }
        else if re_instr.is_match(line){
            for c in line.chars(){
                instr.push(c);
            }
        }
    }

    print_grid(&grid);

    //println!("{:?}", instr);
    let mut robot = find_robot(&grid);
    println!("{:?}", robot);

    for ins in instr{
        iteration(&mut grid, &mut robot, ins);
    }
    print_grid(&grid);

    println!("Score {:?}", compute_score(&grid));
}
