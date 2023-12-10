pub mod maze {
    use std::{collections::HashMap, fs, usize};

    #[derive(Debug, PartialEq, Eq)]
    pub enum Pipe {
        Vertical,   // |
        Horizontal, // -
        Down_Right, // L
        Down_Left,  // J
        Up_Right,   // F
        Up_Left,    // 7
        Ground,     // .
        Start,      // S
    }

    #[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
    pub struct Position {
        x: usize,
        y: usize,
    }

    impl Position {
        fn new(x: usize, y: usize) -> Position {
            Position { x, y }
        }
    }

    pub fn create_map(input_path: &str) -> HashMap<Position, Pipe> {
        // Setting up the map
        let mut map: HashMap<Position, Pipe> = HashMap::new();
        let input = fs::read_to_string(input_path).unwrap();
        let lines: Vec<(usize, String)> = (1..usize::MAX)
            .into_iter()
            .zip(input.lines().map(|x| x.to_owned()))
            .collect();

        let positions: Vec<Vec<(Position, char)>> =
            lines.iter().map(|(x, s)| apply_y_values(x, s)).collect();

        for element in positions.iter() {
            element.iter().for_each(|(p, c)| {
                map.entry(p.clone()).or_insert(apply_pipe(c.clone()));
            });
        }

        return map;
    }

    fn apply_y_values(y: &usize, pipes: &str) -> Vec<(Position, char)> {
        (1..usize::MAX)
            .into_iter()
            .zip(pipes.chars())
            .map(|(x, c)| (Position::new(x, y.clone()), c))
            .collect()
    }

    fn apply_pipe(c: char) -> Pipe {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::Down_Right,
            'J' => Pipe::Down_Left,
            'F' => Pipe::Up_Right,
            '7' => Pipe::Up_Left,
            'S' => Pipe::Start,
            _ => Pipe::Ground,
        }
    }

    fn count_loop(maze: &HashMap<Position, Pipe>) -> usize {
        let mut steps = 0;
        
        let mut position = maze.iter().filter(|(_, v)| **v == Pipe::Start).map(|(p, _)| p.clone()).last();



    }
}
