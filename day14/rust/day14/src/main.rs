use std::fs;
use regex::Regex;
use itertools::izip;

fn print_grid(grid : &Vec<Vec<i32>>, sizes : &(i32, i32)){
    for j in 0..sizes.1{
        let mut line = String::new();
            for i in 0..sizes.0{
            let n = grid[i as usize][j as usize];
            if n == 0{
                line.push_str(".");
            }
            else{
                line.push_str(&n.to_string());
            }
        }
        println!("{line}");
    }
}

fn compute_safety_score(grid : &Vec<Vec<i32>>, sizes : &(i32, i32)) -> i32 {

    let mut quadrant_scores = (0, 0, 0, 0);

    for j in 0..sizes.1{
        for i in 0..sizes.0{
            let n = grid[i as usize][j as usize];

            let left =  i < sizes.0 / 2;
            let right = i > sizes.0 / 2;
            let up =    j < sizes.1 / 2;
            let down =  j > sizes.1 / 2;

            if      left && up    { quadrant_scores.0 += n; }
            else if left && down  { quadrant_scores.1 += n; }
            else if right && up   { quadrant_scores.2 += n; }
            else if right && down { quadrant_scores.3 += n; }

        }
    }
    println!("Quadrants {:?}", quadrant_scores);
    let score = quadrant_scores.0 * quadrant_scores.1 * quadrant_scores.2 * quadrant_scores.3;
    return score;
}

// Check if robots have other robots in their diagonal position
fn compute_diagonal_score(grid : &Vec<Vec<i32>>, sizes : &(i32, i32)) -> usize {

    let mut score = 0;

    for j in 1..sizes.1-1{
        for i in 1..sizes.0-1{

            if grid[i as usize][j as usize] > 0{
                if grid[(i + 1) as usize][(j + 1) as usize] > 0 && grid[(i - 1) as usize][(j - 1) as usize] > 0{
                    score += 1;
                }
                if grid[(i - 1) as usize][(j + 1) as usize] > 0 && grid[(i + 1) as usize][(j - 1) as usize] > 0{
                    score += 1;
                }
            }

        }
    }

    return score;
}

fn main() {

    let file_path = "../../input/input.txt";
    let sizes = (101, 103);

    //let file_path = "../../input/test.txt";
    //let sizes = (11, 7);

    let mut positions : Vec<(i32,i32)> = Vec::new();
    let mut velocities : Vec<(i32,i32)> = Vec::new();

    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines(){

        if re.is_match(line){
            let Some(caps) = re.captures(line) else { println!("no match!"); return; };

            let px: &i32 = &caps["px"].parse().expect("Cound not find a number");
            let py: &i32 = &caps["py"].parse().expect("Cound not find a number");
            let vx: &i32 = &caps["vx"].parse().expect("Cound not find a number");
            let vy: &i32 = &caps["vy"].parse().expect("Cound not find a number");

            positions.push((*px,*py));
            velocities.push((*vx,*vy));

            //println!("p ({px}, {py}), v ({vx}, {vy})");
        }
    }

    // Part 1
    let mut niter = 100;
    for _i in 0..niter{
        for (p, v) in izip!(&mut positions, &velocities){
            //println!("Before: p {:?}, v {:?}", p, v);
            p.0 = (p.0 + v.0 + sizes.0) % sizes.0;
            p.1 = (p.1 + v.1 + sizes.1) % sizes.1;
            //println!("After: p {:?}, v {:?}", p, v);
        }
    }

    let mut grid1 = init_grid(&sizes);

    for (x, y) in &positions{
        grid1[*x as usize][*y as usize] += 1;
    }

    print_grid(&grid1, &sizes);
    let safetey_score = compute_safety_score(&grid1, &sizes);

    println!("Safety score: {safetey_score}");

    fn init_grid(sz : &(i32, i32)) -> Vec<Vec<i32>> {
        let mut grid : Vec<Vec<i32>> = Vec::new();
        for _i in 0..sz.0{
            grid.push(vec![0; sz.1 as usize]);
        }
        return grid;
    }

    // Part 2
    niter = 10000;

    let mut victorious_grid = init_grid(&sizes);
    let mut victorious_ite = 0;
    let mut previous_score = 0;

    for i in 0..niter{

        let mut grid = init_grid(&sizes);

        for (p, v) in izip!(&mut positions, &velocities){
            p.0 = (p.0 + v.0 + sizes.0) % sizes.0;
            p.1 = (p.1 + v.1 + sizes.1) % sizes.1;
        }

        for (x, y) in &positions{
            grid[*x as usize][*y as usize] += 1;
        }

        let score = compute_diagonal_score(&grid, &sizes);
        if score > previous_score{
            previous_score = score;
            println!("Iteration {}, score {score}", i + 1);
            victorious_ite = i + 1;
            victorious_grid = grid.clone();
        }
    }

    println!("Best grid found at iteration {victorious_ite}, with grid:");
    print_grid(&victorious_grid, &sizes);
}
