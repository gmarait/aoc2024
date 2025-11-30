use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn add_edge(graph : &mut HashMap<String, Vec<String>>, n0 : &String, n1 : &String){

    // Add edge in one direction only

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
    println!("Part 1");

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


fn add_empty_nodes(graph : &mut HashMap<String, Vec<String>>){
    // Make sure all nodes are in the keys of the graph

    let mut nodes_to_add : Vec<String> = Vec::new();

    for(_node1, neigh1) in & *graph{

        for nei in neigh1{
            match graph.get(nei) {
                Some(_n) => {},
                None => {
                    nodes_to_add.push(nei.to_string());
                },
            }
        }
    }

    for nei in nodes_to_add{
        let newvec : Vec<String> = Vec::new();
        graph.insert(nei.to_string(), newvec);
    }
}

fn part_2(graph : &HashMap<String, Vec<String>>){
    println!("Part 2");

    let mut biggest_clique_size = 0;
    let mut biggest_clique : Vec<String> = Vec::new();

    fn is_nei_of(graph : &HashMap<String, Vec<String>>, n1 : &String, n2 : &String) -> bool {
        for n in graph.get(n1).unwrap(){
            if n == n2 { return true; }
        }

        for n in graph.get(n2).unwrap(){
            if n == n1 { return true; }
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

    //println!("{:?}", graph);
    part_1(&graph);

    add_empty_nodes(&mut graph);

    part_2(&graph);
}
