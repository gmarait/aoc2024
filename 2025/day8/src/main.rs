use std::fs;
use std::collections::HashSet;

fn distsq(pos_1 : &[f64;3], pos_2 : &[f64;3]) -> f64{
    let mut normsq : f64 = 0.0;
    for d in 0..3{
        normsq += (pos_1[d] - pos_2[d]) * (pos_1[d] - pos_2[d]);
    }
    return normsq
}

fn part_one(file_path: &str, n_connect: u32, n_maxes_result: usize) -> u64 {

    let coords: Vec<[f64; 3]> = fs::read_to_string(file_path).unwrap()
        .lines()
        .map(|line| {
            let nums: Vec<f64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            [nums[0], nums[1], nums[2]]
        }).collect();

    let n_boxes = coords.len();
    let mut dist_matrix : Vec<Vec<f64>> = vec![vec![0.0; n_boxes]; n_boxes];
    let mut connect_array : Vec<HashSet<usize>> = Vec::new();

    for i in 0..n_boxes{
        connect_array.push(HashSet::from([i]));
        for j in i+1..n_boxes{
            dist_matrix[i][j] = distsq(&coords[i], &coords[j]);
        }
    }

    for _k in 0..n_connect{
        let mut shortest = f64::MAX;
        let mut pos = vec![0, 0];
        for i in 0..n_boxes{
            for j in i+1..n_boxes{
                if dist_matrix[i][j] < shortest{
                    shortest = dist_matrix[i][j];
                    pos[0] = i;
                    pos[1] = j;
                }
            }
        }
        dist_matrix[pos[0]][pos[1]] = f64::MAX;

        let mut circ_1 = n_boxes;
        let mut circ_2 = n_boxes;
        for k in 0..connect_array.len(){
            if connect_array[k].contains(&pos[0]){
                circ_1 = k;
            }
            if connect_array[k].contains(&pos[1]){
                circ_2 = k;
            }
            if circ_1 != n_boxes && circ_2 != n_boxes { break; }
        }

        if circ_1 != circ_2{
            let copy = connect_array[circ_2].clone();
            connect_array[circ_1].extend(&copy);
            connect_array.remove(circ_2);
        }
    }

    let mut maxes = vec![0, 0, 0];
    for k in 0..n_maxes_result{
        let mut max = 0;
        let mut idx = 0;
        for a in 0..connect_array.len(){
            if connect_array[a].len() > max{
                max = connect_array[a].len();
                idx = a;
            }
        }
        maxes[k] = max;
        connect_array.remove(idx);
    }

    let mut result : u64 = 1;
    for m in maxes{ result = result * (m as u64); }

    return result;
}

fn part_two(file_path: &str) -> u64 {

    let coords: Vec<[f64; 3]> = fs::read_to_string(file_path).unwrap()
        .lines()
        .map(|line| {
            let nums: Vec<f64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            [nums[0], nums[1], nums[2]]
        }).collect();

    let n_boxes = coords.len();
    let mut dist_matrix : Vec<Vec<f64>> = vec![vec![0.0; n_boxes]; n_boxes];
    let mut connect_array : Vec<HashSet<usize>> = Vec::new();

    for i in 0..n_boxes{
        connect_array.push(HashSet::from([i]));
        for j in i+1..n_boxes{
            dist_matrix[i][j] = distsq(&coords[i], &coords[j]);
        }
    }

    let mut last_connec_1 = 0;
    let mut last_connec_2 = 0;

    while connect_array.len() > 1{
        let mut shortest = f64::MAX;
        let mut pos = vec![0, 0];
        for i in 0..n_boxes{
            for j in i+1..n_boxes{
                if dist_matrix[i][j] < shortest{
                    shortest = dist_matrix[i][j];
                    pos[0] = i;
                    pos[1] = j;
                }
            }
        }
        last_connec_1 = pos[0];
        last_connec_2 = pos[1];

        dist_matrix[pos[0]][pos[1]] = f64::MAX;

        let mut circ_1 = n_boxes;
        let mut circ_2 = n_boxes;
        for k in 0..connect_array.len(){
            if connect_array[k].contains(&pos[0]){
                circ_1 = k;
            }
            if connect_array[k].contains(&pos[1]){
                circ_2 = k;
            }
            if circ_1 != n_boxes && circ_2 != n_boxes { break; }
        }

        if circ_1 != circ_2{
            let copy = connect_array[circ_2].clone();
            connect_array[circ_1].extend(&copy);
            connect_array.remove(circ_2);
        }
    }

    return (coords[last_connec_1][0] as u64) * (coords[last_connec_2][0] as u64);
}

fn main() {
    let file_path = "input.txt";

    let start_time_1 = std::time::Instant::now();
    let n_connect : u32 = 1000;
    let n_maxes_res : usize = 3;
    let res1 = part_one(&file_path, n_connect, n_maxes_res);
    println!("Part 1 result: {res1}");
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time_1);

    let start_time_2 = std::time::Instant::now();
    let res2 = part_two(&file_path);
    println!("Part 2 result: {res2}");
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time_2);
}
