use std::fs;
use std::cmp;

fn part_one(file_path: &str) -> u64 {

    let mut ranges : Vec<[u64; 2]> = Vec::new();
    let mut vals : Vec<u64> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){

        if line.find("-").is_some(){
            let bounds : Vec<_> = line.split('-').collect();
            let beg : u64 = bounds[0].parse().expect("Could not parse value");
            let end : u64 = bounds[1].parse().expect("Could not parse value");
            ranges.push([beg, end]);
        }
        else if line.len() > 0{
            let v : u64 = line.parse().expect("Could not parse value");
            vals.push(v);
        }
    }

    //println!("{:#?}", ranges);
    //println!("{:#?}", vals);

    let mut n_fresh : u64 = 0;
    for v in vals{
        for [beg, end] in ranges.iter(){
            if *beg <= v && *end >= v{
                n_fresh += 1;
                break;
            }
        }
    }

    return n_fresh;
}

fn part_two(file_path: &str) -> u64 {

    let mut ranges : Vec<[u64; 2]> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){
        if line.find("-").is_some(){
            let bounds : Vec<_> = line.split('-').collect();
            let beg : u64 = bounds[0].parse().expect("Could not parse value");
            let end : u64 = bounds[1].parse().expect("Could not parse value");
            ranges.push([beg, end]);
        }
    }

    //println!("{:?}", ranges);

    let mut comp_ranges : Vec<[u64; 2]> = Vec::new();

    for [beg, end] in ranges.iter(){
        comp_ranges.push([*beg, *end]);
    }

    let mut modifs = true;
    while modifs{
        modifs = false;

        let mut to_merge : [usize; 2] = [0, 0];

        for inew in 0..comp_ranges.len(){
            for iold in 0..comp_ranges.len(){
                if iold == inew{ continue; }
                let beg_new = comp_ranges[inew][0];
                let end_new = comp_ranges[inew][1];
                let beg_old = comp_ranges[iold][0];
                let end_old = comp_ranges[iold][1];

                if beg_new <= beg_old && beg_old <= end_new{
                    to_merge[0] = iold;
                    to_merge[1] = inew;
                    modifs = true;
                    break;
                }

                if beg_new <= end_old && end_old <= end_new{
                    to_merge[0] = iold;
                    to_merge[1] = inew;
                    modifs = true;
                    break;
                }
            }

            if modifs{ break; }
        }

        if modifs{
            let merged_beg = cmp::min(comp_ranges[to_merge[0]][0], comp_ranges[to_merge[1]][0]);
            let merged_end = cmp::max(comp_ranges[to_merge[0]][1], comp_ranges[to_merge[1]][1]);
            //println!("Merge {} - {} and {} - {} into {} - {}",
            //    comp_ranges[to_merge[0]][0],
            //    comp_ranges[to_merge[0]][1],
            //    comp_ranges[to_merge[1]][0],
            //    comp_ranges[to_merge[1]][1],
            //    merged_beg,
            //    merged_end);

            let first_idx = cmp::min(to_merge[0], to_merge[1]);
            let last_idx = cmp::max(to_merge[0], to_merge[1]);
            comp_ranges.remove(last_idx);
            comp_ranges.remove(first_idx);
            comp_ranges.push([merged_beg, merged_end]);
        }
    }

    let mut sum : u64 = 0;

    for [beg, end] in comp_ranges.iter(){
        sum += *end - *beg + 1;
    }

    //println!("{:?}", comp_ranges);

    return sum;
}

fn main() {
    let file_path = "input.txt";

    let start_time_1 = std::time::Instant::now();
    let res1 = part_one(&file_path);
    println!("Part 1 result: {res1}");
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time_1);

    let start_time_2 = std::time::Instant::now();
    let res2 = part_two(&file_path);
    println!("Part 2 result: {res2}");
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time_2);
}
