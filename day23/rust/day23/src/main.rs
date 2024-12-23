use std::fs;
use regex::Regex;
use std::collections::HashMap;
//use indicatif::ProgressBar;
//use std::collections::HashSet;

/*
fn add_edge(graph : &mut HashMap<String, Vec<String>>, n0 : &String, n1 : &String){

    if n0 < n1{
        match graph.get_mut(n0) {
            Some(vec) => vec.push(n1.to_string()),
            None => {
                let mut newvec : Vec<String> = Vec::new();
                newvec.push(n1.to_string());
                graph.insert(n0.to_string(), newvec);
            },
        }
    }
    else{
        add_edge(graph, n1, n0);
    }
}

fn part_1(graph : &HashMap<String, Vec<String>>){

    let mut count = 0;

    for (node1, neigh) in graph{

        let has_t_1 = node1.chars().nth(0) == Some('t');

        for node2 in neigh{

            let has_t_2 = node2.chars().nth(0) == Some('t');

            for (node3, neigh3) in graph {

                let has_t_3 = node3.chars().nth(0) == Some('t');

                if has_t_1 || has_t_2 || has_t_3{

                    if node3 == node1{ continue; }
                    if node3 == node2{ continue; }

                    let mut found_node1 = false;
                    let mut found_node2 = false;

                    for node4 in neigh3{
                        if node4 == node1{ found_node1 = true; }
                        if node4 == node2{ found_node2 = true; }
                    }

                    if found_node1 && found_node2{
                        count += 1;
                        //println!("{node1}, {node2}, {node3}");
                    }
                }
            }
        }
    }

    println!("Found: {count}");
}
 */

fn add_edge(graph : &mut HashMap<String, Vec<String>>, n0 : &String, n1 : &String){

    match graph.get_mut(n0) {
        Some(vec) => vec.push(n1.to_string()),
        None => {
            let mut newvec : Vec<String> = Vec::new();
            newvec.push(n1.to_string());
            graph.insert(n0.to_string(), newvec);
        },
    }

    // Add empty list for new nodes
    match graph.get_mut(n1) {
        Some(_vec) => {},
        None => {
            let newvec : Vec<String> = Vec::new();
            graph.insert(n1.to_string(), newvec);
        },
    }
}

fn part_2(graph : &HashMap<String, Vec<String>>){

    let mut biggest_clique_size = 0;
    let mut biggest_clique : Vec<String> = Vec::new();

    fn is_nei_of(graph : &HashMap<String, Vec<String>>, n1 : &String, n2 : &String) -> bool {
        match graph.get(n1) {
            Some(neigh) => {
                for n in neigh{
                    if n == n2 { return true; }
                }
            },
            None => {},
        }

        match graph.get(n2) {
            Some(neigh) => {
                for n in neigh{
                    if n == n1 { return true; }
                }
            },
            None => {},
        }
        return false;
    }

    fn grow_clique(graph : &HashMap<String, Vec<String>>, clique : &Vec<String>) -> Option<String>{
        for (node, _nei) in graph{

            // Pass nodes already in clique
            for nc in clique{
                if node == nc{
                    continue;
                }
            }

            let mut in_clique = true;
            for nc in clique{
                if !is_nei_of(graph, node, nc){
                    in_clique = false;
                    break;
                }
            }

            if in_clique{
                return Some(node.to_string());
            }
        }

        return None;
    }

    //let mut nodes : Vec<String> = <HashMap<String, Vec<String>> as Clone>::clone(&graph).into_keys().collect();
    //nodes.sort();

    for(node1, _neigh1) in graph{

        let mut clique : Vec<String> = Vec::new();
        clique.push(node1.to_string());

        let mut growing = true;
        while growing{
            match grow_clique(graph, &clique){
                Some(to_add) => { clique.push(to_add); },
                None => { growing = false; },
            }
        }

        if clique.len() > biggest_clique_size{
            biggest_clique_size = clique.len();
            biggest_clique = clique.clone();
        }
    }

    println!("Found: {biggest_clique_size}");
    biggest_clique.sort();
    println!("Biggest clique: {:?}", biggest_clique);
    print!("{}", biggest_clique[0]);
    for s in &biggest_clique[1..]{
        print!(",{s}");
    }
    println!("");
}

fn main() {
    let file_path = "../../input/input.txt";
    //let file_path = "../../input/test.txt";

    let re_split = Regex::new(r"-").unwrap();

    let mut graph : HashMap<String, Vec<String>> = HashMap::new();

    for line in fs::read_to_string(file_path).unwrap().lines(){
        let link : Vec<&str> = re_split.split(&line).collect();
        add_edge(&mut graph, &link[0].to_string(), &link[1].to_string());
    }

    for(_node, neigh) in &mut graph{ neigh.sort(); }

    println!("{:?}", graph);

    //part_1(&graph);
    part_2(&graph);
}
