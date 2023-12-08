pub mod wasteland {
    use std::collections::{HashMap, VecDeque};
    #[derive(Debug, Clone, Copy)]
    pub enum Direction {
        Left,
        Right,
        No_direction,
    }

    #[derive(Debug, Clone)]
    pub struct Walker {
        pub steps: usize,
        pub akt_node: String,
        node_map: HashMap<String, (String, String)>,
        directions: Box<VecDeque<Direction>>,
    }

    impl Walker {
        pub fn new(start_node: String, map: HashMap<String, (String, String)>, dirs: String) -> Walker {
            return Walker {
                steps: 0,
                akt_node: start_node,
                node_map: map,
                directions: Box::new(Direction::new_vec(dirs)),
            };
        }
        
        pub fn walk_a_step(&mut self) -> Walker {
            let direction = self.directions.pop_front().unwrap();
            self.steps += 1;
            let tuple: (String, String) = self.node_map.get(&self.akt_node).unwrap().clone();
            self.akt_node = 
                match direction {
                    Direction::Left => tuple.0,
                    Direction::Right => tuple.1,
                    Direction::No_direction => String::from("AAA"),
                };

            self.directions.push_back(direction);
            return Walker {
                steps: self.steps,
                akt_node: self.akt_node.to_owned(),
                node_map: self.node_map.clone(),
                directions: self.directions.clone(),
            };
        }

        pub fn walk_to_destination(&mut self) -> usize {
            let mut iter = self.directions.iter();

            while self.akt_node != String::from("ZZZ") {
                self.steps += 1;
                let next_move = match iter.next() {
                    Some(x) => x,
                    None => {
                        iter = self.directions.iter();
                        iter.next().unwrap()
                    }
                };
                let tuple: (String, String) = self.node_map.get(&self.akt_node).unwrap().clone();
                self.akt_node = match next_move {
                    Direction::Left => tuple.0,
                    Direction::Right => tuple.1,
                    Direction::No_direction => String::from("AAA"),
                };
            }

            return self.steps;
        }
    }

    impl Direction {
        pub fn new(dir: char) -> Direction {
            match dir {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => Direction::No_direction,
            }
        }

        pub fn new_vec(directions: String) -> VecDeque<Direction> {
            return directions.chars().map(|x| Direction::new(x)).collect();
        }
    }
}
