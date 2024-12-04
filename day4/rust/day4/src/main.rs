use std::fs;

// Check 8 neighbor cells, bound checking
// Return true if the character c if found at position (pos_x + offx, pos_y + offy),
// with the coordinates of the found character in the retuned tuple
fn find_in_direction(grid : &Vec<Vec<char>>, c : char, pos_x : i32, pos_y : i32, offx : i32, offy : i32)
-> (bool, i32, i32)
{

    let max_rows = grid.len();
    let max_cols = grid[0].len();
    let test_x : i32 = pos_x + offx;
    let test_y : i32 = pos_y + offy;

    if test_x < 0 || test_y < 0 || test_x as usize >= max_rows || test_y as usize >= max_cols {
        return (false, pos_x, pos_y);
    }

    let xx = test_x as usize;
    let yy = test_y as usize;

    if grid[xx][yy] == c{
        return (true, xx as i32, yy as i32);
    }

    return (false, pos_x, pos_y);
}

// Look for the the character 'c' at position (pos_x + offx, pos_y + offy),
// and the opposite character at position (pos_x - offx, pos_y - offy)
// Opposite characters are 'M' <-> 'S'
// Return true of both characters were found
fn look_for_opposites(grid : &Vec<Vec<char>>, c : char, pos_x : i32, pos_y : i32, offx : i32, offy : i32) -> bool {

    let (f, _, _) = find_in_direction(grid, c, pos_x, pos_y, offx, offy);

    if f{
        let op_c = if c == 'M' { 'S' } else { 'M' };
        let (f_op, _, _) = find_in_direction(grid, op_c, pos_x, pos_y, -offx, -offy);
        return f_op;
    }

    return false;
}

fn main() {

    let file_path = "../../input/input.txt";
    //let file_path = "../../input/test.txt";
    //let file_path = "../../input/test_p2.txt";

    let mut grid : Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){
        grid.push(line.chars().map(|b| b as char).collect::<Vec<_>>());
    }

    let n_lines = grid.len();
    let n_cols = grid[0].len();

    println!("{n_lines} x {n_cols}");

    let mut found_xmas = 0;

    //println!("First line: {:?}", grid[0]);
    //println!("Last line: {:?}", grid[n_lines - 1]);

    for i in 0..n_lines{
        for j in 0..n_cols{

            if grid[i][j] == 'X'{
                let x0 = i as i32;
                let y0 = j as i32;

                for offx in -1..=1{
                    for offy in -1..=1{

                        if offx == 0 && offy == 0{ continue };

                        let (f1, x1, y1) = find_in_direction(&grid, 'M', x0, y0, offx, offy);
                        if f1{
                            let (f2, x2, y2) = find_in_direction(&grid, 'A', x1, y1, offx, offy);
                            if f2{
                                let (f3, _, _) = find_in_direction(&grid, 'S', x2, y2, offx, offy);
                                if f3{
                                    found_xmas += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 1 -> found XMAS: {found_xmas}");

    for i in 0..n_lines{
        for j in 0..n_cols{

            if grid[i][j] == 'A'{

                let x0 = i as i32;
                let y0 = j as i32;

                let f1 = look_for_opposites(&grid, 'M', x0, y0, -1, -1);
                let f2 = look_for_opposites(&grid, 'S', x0, y0, -1, -1);

                if f1 || f2{
                    let f3 = look_for_opposites(&grid, 'M', x0, y0, -1, 1);
                    let f4 = look_for_opposites(&grid, 'S', x0, y0, -1, 1);

                    if f3 || f4{
                        found_xmas += 1;
                    }
                }

            }
        }
    }

    println!("Part 2 -> found X - MAS: {found_xmas}");
}
