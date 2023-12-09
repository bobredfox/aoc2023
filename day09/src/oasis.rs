pub mod oasis_part1 {
    use std::fs;

    struct Oasis {
        pub history_data: Vec<isize>,
    }

    impl Oasis {
        fn new_history(str_line: &str) -> Oasis {
            let values: Vec<isize> = str_line
                .split_whitespace()
                .map(|x| x.to_string().parse::<isize>().unwrap())
                .collect();
            Oasis {
                history_data: values,
            }
        }

        fn next_value(history: Vec<isize>) -> isize {
            let test: Vec<isize> = history
                .iter()
                .zip(history.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect();

            match test.iter().all(|x| x.clone() == 0) {
                true => test.last().unwrap().clone(),
                false => test.last().unwrap() + Oasis::next_value(test.clone()),
            }
        }

        fn prev_value(history: Vec<isize>) -> isize {
            let test: Vec<isize> = history
                .iter()
                .zip(history.iter().skip(1))
                .map(|(x, y)| y - x)
                .collect();

            println!("{}", test.first().unwrap());
            match test.iter().all(|x| x.clone() == 0) {
                true => test.first().unwrap().clone(),
                false => test.first().unwrap() - Oasis::prev_value(test.clone()),
            }
        }
    }

    pub fn part_one(input_file: &str) -> isize {
        let lines = fs::read_to_string(input_file).unwrap();
        let hist_lines = lines
            .lines()
            .map(|l| Oasis::new_history(l))
            .collect::<Vec<Oasis>>();
        hist_lines
            .iter()
            .map(|o| {
                Oasis::next_value(o.history_data.clone()) + o.history_data.last().unwrap().clone()
            })
            .sum()
    }

    pub fn part_two(input_file: &str) -> isize {
        let lines = fs::read_to_string(input_file).unwrap();
        let hist_lines = lines
            .lines()
            .map(|l| Oasis::new_history(l))
            .collect::<Vec<Oasis>>();

        hist_lines
            .iter()
            .map(|o| {
                o.history_data.first().unwrap() - Oasis::prev_value(o.history_data.clone())
            }).sum()
    }
}
