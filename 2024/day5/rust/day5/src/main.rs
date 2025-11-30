use std::fs;
use regex::Regex;

fn order_list(order : &Vec<(usize, usize)>, list : &mut Vec<usize>){

    let mut modif = false;
    for i in 0..list.len(){
        for j in i..list.len(){
            for (f, l) in order{

                // Swap elements if not properly ordered
                if list[j] == *f && list[i] == *l{
                    let tmp = list[j];
                    list[j] = list[i];
                    list[i] = tmp;
                    modif = true;
                    break;
                }

            }
        }
    }

    println!("{:?}", list);
    if modif{
        order_list(order, list);
    }
}

fn is_list_ordered(order : &Vec<(usize, usize)>, list : & Vec<usize>) -> bool {

    for i in 0..list.len(){
        for j in i..list.len(){
            for (f, l) in order{

                if list[j] == *f && list[i] == *l{
                    return false;
                }

            }
        }
    }

    return true;
}

fn main() {
    let file_path = "../../input/input.txt";
    //let file_path = "../../input/test_input.txt";

    let re_order = Regex::new(r"^(\d+\|\d+)$").unwrap();
    let re_list = Regex::new(r"^((\d+),)+\d+$").unwrap();

    let mut order : Vec<(usize, usize)> = Vec::new();
    let mut lists : Vec<Vec<usize>> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){

        //let m1 = re_order.find(line).unwrap();
        if re_order.is_match(line){
            let m = re_order.find(line).unwrap();
            let splitted : Vec<&str> = m.as_str().split("|").collect();
            let n1: &usize = &splitted[0].parse().expect("Cound not find a number");
            let n2: &usize = &splitted[1].parse().expect("Cound not find a number");

            order.push((*n1, *n2));
        }
        else if re_list.is_match(line){
            let m = re_list.find(line).unwrap();

            let vals : Vec<usize> = m.as_str().split(",").map(|v| v.parse::<usize>().unwrap()).collect();
            lists.push(vals);
        }
    }

    //println!("{:?}", order);
    //println!("{:?}", lists);

    let mut sum_ordered = 0;
    let mut sum_incorrect = 0;
    for mut ll in &mut lists{
        if is_list_ordered(&order, &ll){
            sum_ordered +=  ll[ll.len()/2];
        }
        else{
            order_list(&order, &mut ll);
            sum_incorrect +=  ll[ll.len()/2];
        }
    }

    println!("Sum of middle elements or ordered list {sum_ordered}");
    println!("Sum of middle elements or incorrect list {sum_incorrect}");
}
