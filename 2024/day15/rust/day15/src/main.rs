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

/*
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
}
 */

fn dbg_box(grid : &Vec<Vec<char>>) -> bool{
    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if grid[i][j] == '[' && grid[i][j+1] != ']'{
                return false;
            }
            if grid[i][j] == ']' && grid[i][j-1] != '['{
                return false;
            }
        }
    }
    return true;
}

fn new_iteration(grid : &mut Vec<Vec<char>>, robot : &mut (i32, i32), instr : char){
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

    fn find_previous_char(to_move : &Vec<(i32, i32, char)>, cur_pos : (i32, i32)) -> char{
        for (tm_x, tm_y, c) in to_move{
            if *tm_x == cur_pos.0 && *tm_y == cur_pos.1{
                return *c;
            }
        }

        return '.';
    }

    // Up or down
    if dir.0 != 0{
        let mut to_move : Vec<(i32, i32, char)> = Vec::new();

        //let prev_grid = grid.clone();

        if can_move_updown(&grid, *robot, dir.0, &mut to_move){

            for (tm_x, tm_y, c) in &to_move{
                let prev_char = find_previous_char(&to_move, (*tm_x - dir.0, *tm_y));
                grid[(*tm_x + dir.0) as usize][*tm_y as usize] = *c;
                grid[*tm_x as usize][*tm_y as usize] = prev_char;

                //println!("Moving {prev_char} to {}", *c);

                if *c == '@'{
                    robot.0 = *tm_x + dir.0;
                    robot.1 = *tm_y;
                }
            }
        }
    }

    // Left or right
    else{
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
    }
}

fn can_move_updown(grid : &Vec<Vec<char>>, pos : (i32, i32), dir : i32, to_move : &mut Vec<(i32, i32, char)>) -> bool{
    let c = grid[pos.0 as usize][pos.1 as usize];

    //println!("At {:?} -> {c}", pos);

    match c{
        '#' => return false,
        '.' => return true,
        '['  => {
            to_move.push((pos.0, pos.1, c));
            let right_box = (pos.0, pos.1 + 1);
            let next_pos = (pos.0 + dir, pos.1);
            if to_move.iter().find(|&&x| x.0 == right_box.0 && x.1 == right_box.1).is_some(){
                return can_move_updown(grid, next_pos, dir, to_move);
            }
            return can_move_updown(grid, next_pos, dir, to_move) && can_move_updown(grid, right_box, dir, to_move);
        },
        ']'  => {
            to_move.push((pos.0, pos.1, c));
            let left_box = (pos.0, pos.1 - 1);
            let next_pos = (pos.0 + dir, pos.1);
            if to_move.iter().find(|&&x| x.0 == left_box.0 && x.1 == left_box.1).is_some(){
                return can_move_updown(grid, next_pos, dir, to_move);
            }
            return can_move_updown(grid, next_pos, dir, to_move) && can_move_updown(grid, left_box, dir, to_move);
        },
        '@'  => {
            to_move.push((pos.0, pos.1, c));
            let next_pos = (pos.0 + dir, pos.1);
            return can_move_updown(grid, next_pos, dir, to_move);
        }
        _ => {
            println!("Error: char is {c}");
            return false;
        }
    }
}

fn part_2_grid(grid : &Vec<Vec<char>>) -> Vec<Vec<char>>{

    /*
    If the tile is #, the new map contains ## instead.
    If the tile is O, the new map contains [] instead.
    If the tile is ., the new map contains .. instead.
    If the tile is @, the new map contains @. instead.
     */

    let mut newgrid : Vec<Vec<char>> = Vec::new();

    for i in 0..grid.len(){
        newgrid.push(Vec::<char>::new());
        for j in 0..grid[i].len(){
            let newchar : (char, char) = match grid[i][j] {
                '#'=> ('#', '#'),
                'O'=> ('[', ']'),
                '.'=> ('.', '.'),
                '@'=> ('@', '.'),
                _  => ('X', 'X')
            };
            newgrid[i].push(newchar.0);
            newgrid[i].push(newchar.1);
        }
    }

    return newgrid;
}

fn compute_score_2(grid : &Vec<Vec<char>>) -> usize{
    let mut score = 0;

    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if grid[i][j] == '['{
                score += 100 * i + j;
            }
        }
    }
    return score;
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
    //let mut robot = find_robot(&grid);

    // Part 1
    /*
    println!("{:?}", robot);

    for ins in instr{
        iteration(&mut grid, &mut robot, ins);
    }
    print_grid(&grid);

    println!("Score {:?}", compute_score(&grid));
     */

    // Part 2

    let mut newgrid = part_2_grid(&grid);
    print_grid(&newgrid);

    let mut robot = find_robot(&newgrid);

    for ins in instr{
        new_iteration(&mut newgrid, &mut robot, ins);
    }
    print_grid(&newgrid);
    println!("Found bug? {:?}", !dbg_box(&newgrid));

    println!("Score part 2 {:?}", compute_score_2(&newgrid));
    /*
    for ins in instr{
        new_iteration(&mut newgrid, &mut robot, ins);
        print_grid(&newgrid);
        if !dbg_box(&newgrid){
            println!("Error!");
            break;
        }
    }
    */
}
