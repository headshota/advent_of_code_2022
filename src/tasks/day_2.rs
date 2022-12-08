use std::fs;
use std::io::{prelude::*, BufReader};

pub fn part_1() -> u32 {
    let mut total_score = 0;
    let lines = get_input_lines("src/inputs/day_2.txt").unwrap();

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let enemy_choice = parts.next().unwrap();
        let our_choice = parts.next().unwrap();

        total_score += match (enemy_choice, our_choice) {
            ("A", "X") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Z") => 3 + 3,

            ("A", "Z") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("C", "Y") => 2 + 0,

            ("A", "Y") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,

            _ => panic!("Unknown outcome, can't score")
        };
    }

    total_score
}

pub fn part_2() -> u32 {
    let mut total_score = 0;
    let lines = get_input_lines("src/inputs/day_2.txt").unwrap();

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let enemy_choice = parts.next().unwrap();
        let round_result = parts.next().unwrap();

        total_score += match (enemy_choice, round_result) {
            ("A", "X") => 3 + 0,
            ("B", "X") => 1 + 0,
            ("C", "X") => 2 + 0,

            ("A", "Y") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 3 + 3,

            ("A", "Z") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 1 + 6,

            _ => panic!("Unknown outcome, can't score")
        };
    }

    total_score
}

fn get_input_lines(path: &str) -> Result<std::io::Lines<BufReader<fs::File>>, std::io::Error> {
    let file = fs::File::open(path)?;
    Ok(BufReader::new(file).lines())
}
