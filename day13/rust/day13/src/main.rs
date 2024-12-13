use std::fs;
use regex::Regex;
use itertools::izip;
use nalgebra::{U2,OMatrix,OVector};

// Retuns -1 if too far (> 0.001) from an integer
fn closer_int(val: f64) -> i64{
    let out = val.round();
    if (val - out).abs() > 0.001{
        return -1;
    }
    return out as i64;
}

fn main() {
    let file_path = "../../input/input.txt";
    //let file_path = "../../input/test.txt";

    let re_button = Regex::new(r"Button (?<aorb>A|B): X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

    let mut button_a : Vec<(i32,i32)> = Vec::new();
    let mut button_b : Vec<(i32,i32)> = Vec::new();
    let mut prizes   : Vec<(i32,i32)> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){

        if re_button.is_match(line){
            let Some(caps) = re_button.captures(line) else { println!("no match!"); return; };
            let letter = &caps["aorb"];

            let x: &i32 = &caps["x"].parse().expect("Cound not find a number");
            let y: &i32 = &caps["y"].parse().expect("Cound not find a number");

            if letter == "A"{
                button_a.push((*x,*y));
            }
            else{
                button_b.push((*x,*y));
            }

            //println!("Found {letter} : {x} - {y}");
        }
        else if re_prize.is_match(line){
            let Some(caps) = re_prize.captures(line) else { println!("no match!"); return; };

            let x: &i32 = &caps["x"].parse().expect("Cound not find a number");
            let y: &i32 = &caps["y"].parse().expect("Cound not find a number");

            prizes.push((*x, *y));
            //println!("P R I Z E : {x} - {y}");
        }
    }

    //println!("A -> {:?}", button_a);
    //println!("B -> {:?}", button_b);
    //println!("Prizes -> {:?}", prizes);

    type Matrix2x2 = OMatrix<f64, U2, U2>;
    type Vector2 = OVector<f64, U2>;

    let mut tokens : i64 = 0;

    let cost_a : i64 = 3;
    let cost_b : i64 = 1;

    for (a, b, p) in izip!(button_a, button_b, prizes){
        let a11: f64 = f64::from(a.0);
        let a12: f64 = f64::from(b.0);
        let a21: f64 = f64::from(a.1);
        let a22: f64 = f64::from(b.1);

        let b1: f64 = f64::from(p.0);
        let b2: f64 = f64::from(p.1);

        let mat = Matrix2x2::new(a11, a12,
            a21, a22);
        let rhs = Vector2::new(b1 + 10000000000000.0, b2 + 10000000000000.0);

        let x = mat.lu().solve(&rhs).expect("Linear resolution failed.");

        let nb_tok_a: i64 = closer_int(x[0]) as i64;
        let nb_tok_b: i64 = closer_int(x[1]) as i64;

        if nb_tok_a > 0 && nb_tok_b > 0{
            tokens += nb_tok_a * cost_a + nb_tok_b * cost_b;
            //println!("Found {nb_tok_a} a and {nb_tok_b} b");
        }
        //println!("A {:?} -- b {:?} -- > x {:?}", mat, rhs, x);
    }

    println!("Number of tokens: {tokens}");

}
