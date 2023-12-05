use std::env;
use std::fs;
use std::thread::Thread;

#[derive(Debug)]
struct Holding {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

#[derive(Debug, Clone)]
struct Map {
    entrys: Vec<MapEntry>,
}

impl Map {
    fn source_to_destiny(&self, seed: Option<usize>) -> Option<usize> {
        let values = self
            .entrys
            .iter()
            .map(|x| x.source_to_destiny(seed))
            .filter(|x| match x {
                Some(_) => true,
                None => false,
            })
            .last();
        match values {
            Some(x) => return x,
            None => return None,
        };
    }
}

#[derive(Debug, Clone)]
struct MapEntry {
    source: usize,
    destination: usize,
    range: usize,
}

impl MapEntry {
    // Checks if source is in range
    fn source_in_range(&self, seed: usize) -> bool {
        let max = self.source + self.range - 1;
        if (seed >= self.source && seed <= max) {
            return true;
        } else {
            return false;
        }
    }
    // Calculates the new value of the seed, if there is one
    fn source_to_destiny(&self, seed: Option<usize>) -> Option<usize> {
        match seed {
            Some(x) => {
                if self.source_in_range(x) {
                    if self.destination >= self.source {
                        let diff = self.destination - self.source;
                        return Some(x + diff);
                    } else {
                        let diff = self.source - self.destination;
                        return Some(x - diff);
                    };
                } else {
                    return None;
                }
            }
            None => return None,
        };
    }
}

fn read_input(path: &str) -> Holding {
    let string = fs::read_to_string(path).unwrap();
    let map_string: Vec<String> = string.split("\n\n").map(|s| s.to_owned()).collect();
    let seed_string: Vec<usize> = map_string
        .first()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    return Holding {
        seeds: seed_string,
        maps: map_string
            .iter()
            .skip(1)
            .map(|x| string_to_map(x.to_owned()))
            .collect(),
    };
}

fn expand_seed(seed: usize, range: usize) -> Vec<usize> {
    let mut new_vec: Vec<usize> = Vec::new();
    for n in seed..=(seed + range - 1) {
        new_vec.push(n);
    }
    return new_vec;
}

fn string_to_map(string: String) -> Map {
    return Map {
        entrys: string
            .lines()
            .skip(1)
            .map(|x| string_to_entry(x.to_owned()))
            .collect(),
    };
}

fn string_to_entry(string: String) -> MapEntry {
    return MapEntry {
        destination: string
            .split_whitespace()
            .take(1)
            .map(|x| x.to_owned().parse::<usize>().expect("destination"))
            .sum(),
        source: string
            .split_whitespace()
            .skip(1)
            .take(1)
            .map(|x| x.to_owned().parse::<usize>().unwrap())
            .sum(),
        range: string
            .split_whitespace()
            .skip(2)
            .take(1)
            .map(|x| x.to_owned().parse::<usize>().unwrap())
            .sum(),
    };
}

fn soil_to_location(all: Holding) -> usize {
    let mut min: usize = usize::MAX;

    for i in (0..all.seeds.iter().count()).step_by(2) {
        let seeds = expand_seed(
            *all.seeds.get(i).to_owned().unwrap(),
            *all.seeds.get(i + 1).to_owned().unwrap(),
        );

        for element in seeds.iter() {
            min = min.min(seed_to_seed(&all.maps, element.to_owned()));
            println!("New minimum: {}", min);
        }
    }

    return min;
}

fn seed_to_seed(maps: &Vec<Map>, seed: usize) -> usize {
    let mut value = seed;
    for map in maps.iter() {
        value = seed_in_map(map, value);
    }
    return value;
}

fn seed_in_map(map: &Map, seed: usize) -> usize {
    match map.source_to_destiny(Some(seed)) {
        Some(x) => return x,
        None => return seed,
    };
}

fn main() {
    let map = read_input("input.data");
    let soil = soil_to_location(map);

    println!("{:?}", soil);
}
