use std::env;
use std::fs;
use std::usize;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}


#[derive(Debug, PartialEq, Eq)]
struct Set{
    blue: usize,
    green: usize,
    red: usize
}


fn read_puzzle_input(path: String) -> Vec<Game> {
    let input = fs::read_to_string(path).unwrap();
    let games: Vec<Game> = input.lines().map(|game| read_game(game.to_owned())).collect(); 

    return games;
}

fn read_game(game_string: String) -> Game {
    let halves : Vec<String> = game_string.split(": ").map(|x| x.to_owned()).collect();
    let game = Game {
        id: halves.first().unwrap().strip_prefix("Game ").unwrap().parse::<usize>().expect("Game id went wrong"),
        sets: read_sets(halves.get(1).unwrap().to_owned()),
    };
    return game;
}

fn read_sets(set_strings: String) -> Vec<Set> {
    return set_strings.split(";").map(|x| choose_colors(x.to_owned())).collect();
}

fn choose_colors(color_string: String) -> Set {
    let seperated : Vec<String> = color_string.split(", ").map(|x| x.to_owned()).collect();
    let mut set = Set{
        blue:  0,
        green: 0,
        red: 0,
        }; 
    for color in seperated {
        match color {
            x if color.contains("blue") => {set.blue = split_color(x)},
            x if color.contains("green") => {set.green = split_color(x)},
            x if color.contains("red") => {set.red = split_color(x)},
            _ => {}
            
        }
    }
    return set;
}

fn split_color(string: String) -> usize {
    return string.split_whitespace().next().unwrap().to_owned().parse::<usize>().unwrap();
}


fn compare_sets(set: &Set, max: &Set) -> bool{
    return  set.blue <= max.blue && set.green <= max.green && set.red <= max.red;
}

fn game_readout(game: &Game, configuration: &Set) -> usize{
    match game.sets.iter().all(|s| compare_sets(&s, &configuration)) {
        true => {return game.id;}
        false => {return 0;}
    }
}



fn max_colors(set_a: &Set, set_b: &Set) -> Set{
    return Set{blue: set_a.blue.max(set_b.blue), green: set_a.green.max(set_b.green), red: set_a.red.max(set_b.red)};    
}

fn game_max(game: &Game) -> usize{
    let max = game.sets.iter().fold(Set{blue: 0, green: 0, red: 0}, |acc, set| max_colors(&acc, set));
    println!("{:?}", max);
    return max.blue * max.green * max.red;
    
}

fn main() {
    let configuration = Set {
        blue: 14,
        green: 13,
        red: 12,
    };
    let games = read_puzzle_input(String::from("input.data"));
    //part1
    //let sum: usize = games.iter().map(|g| game_readout(g, &configuration)).sum();
    let sum: usize = games.iter().map(|g| game_max(g)).sum();
    println!("{}", sum);
}
