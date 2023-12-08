use crate::wasteland::wasteland::{Direction, Walker};
use std::{collections::HashMap, fs, thread::{self, JoinHandle}};

pub mod wasteland;

fn read_input(input_path: &str) -> Walker {
    let input = fs::read_to_string(input_path).unwrap();
    let directions: String = input.lines().take(1).last().unwrap().to_string();

    let lines: Vec<&str> = input.lines().skip(2).collect();

    Walker::new(String::from("AAA"),create_node_map(lines), directions)
}

fn read_input_part2(input_path: &str) -> Vec<Walker> {
    let input = fs::read_to_string(input_path).unwrap();
    let directions: String = input.lines().take(1).last().unwrap().to_string();

    let lines: Vec<&str> = input.lines().skip(2).collect();

    let node_map = create_node_map(lines);
    
    let keys: Vec<String> = node_map.keys().filter(|x| x.chars().last().unwrap() == 'A').map(|x| x.to_owned()).collect();
    
    let mut vector: Vec<Walker> = Vec::new();

    for key in keys.iter() {
        vector.push(Walker::new(key.to_owned(), node_map.clone(), directions.clone()));
    }

    return vector;
}

fn create_node_map(input: Vec<&str>) -> HashMap<String, (String, String)> {
    let identifier: Vec<String> = input
        .iter()
        .map(|x| (&x.to_string()[0..3]).to_owned())
        .collect();
    let tuple: Vec<(String, String)> = input
        .iter()
        .map(|x| {
            (
                (&x.to_string()[7..10]).to_owned(),
                (&x.to_string()[12..15]).to_owned(),
            )
        })
        .collect();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for (id, x) in identifier.iter().zip(tuple.iter()) {
        nodes.insert(id.to_owned(), x.to_owned());
    }

    return nodes;
}

fn main() {
    //let mut walker = read_input("input.data");
    //let steps = walker.walk_to_destination();
    let mut part2 = read_input_part2("input.data");
    while !part2.iter().all(|w| w.akt_node.chars().last().unwrap() == 'Z') {
        let mut handles: Vec<JoinHandle<Walker>> = Vec::new();
        for element in part2.iter() {
            let mut new_element = element.clone();
            let handle = thread::spawn(move || {
                new_element.walk_a_step()
            });
            handles.push(handle);
            
        };
        let mut walkers: Vec<Walker> = Vec::new();
        for join in handles {
            walkers.push(join.join().unwrap());
        }
        part2 = walkers;
        println!("Steps: {}, Node: {}", part2.first().unwrap().steps, part2.first().unwrap().akt_node);
    }

    
    let steps = part2.first().unwrap().steps;
    println!("Schritte: {}", steps);
}
