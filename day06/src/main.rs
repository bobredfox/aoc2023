use std::env;
use std::fs;

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn new(time: &str, record: &str) -> Race {
        return Race {
            time: time.to_owned().parse::<usize>().unwrap(),
            record: record.to_owned().parse::<usize>().unwrap(),
        };
    }
    // Calculates the distance by a determined hold time
    fn calculate_distance(&self, hold_time: usize) -> usize {
        let speed = hold_time;
        return speed * (self.time - hold_time);
    }
    //
    fn calculate_max_distance(&self) -> Vec<usize> {
        return (1..self.time).map(|x| self.calculate_distance(x)).collect();
    }

    fn ways_to_win(&self) -> usize {
        return self
            .calculate_max_distance()
            .iter()
            .filter(|x| x > &&self.record)
            .count();
    }
}

fn read_input(path: &str) -> Vec<Race> {
    let input_string = fs::read_to_string(path).unwrap();
    let lines = input_string.lines().take(1).last().unwrap().to_owned();
    let pair: Vec<(&str, &str)> = lines
        .split_whitespace()
        .zip(
            input_string
                .lines()
                .skip(1)
                .take(1)
                .last()
                .unwrap()
                .split_whitespace(),
        )
        .collect();
    let races: Vec<Race> = pair.iter().map(|(x, y)| Race::new(x, y)).collect();
    return races;
}

fn main() {
    let races = read_input("input.data");
    let wins = races.iter().map(|x| x.ways_to_win()).fold(1, |acc, x| acc * x);
    println!("{:?}", wins);
}
