use std::fs;
use std::io::{prelude::*, BufReader};

fn get_input_lines(path: &str) -> Result<std::io::Lines<BufReader<fs::File>>, std::io::Error> {
    let file = fs::File::open(path)?;
    Ok(BufReader::new(file).lines())
}


pub fn part_1() -> u32{
    let lines = get_input_lines("src/inputs/day_3.txt").unwrap();
    let mut sum = 0;

    for line in lines {
        let line = line.unwrap();

        let part_1 = &line[0..line.len() / 2];
        let part_2 = &line[line.len() / 2..line.len()];
        let ch = find_same_char(part_1, part_2).unwrap();
        sum += char_to_score(ch);
    }

    sum
}


pub fn part_2() -> u32 {
    todo!()
}

fn find_same_char(part_1: &str, part_2: &str) -> Option<char> {
    for c in part_1.chars() {
        if part_2.contains(c) {
            return Some(c)
        }
    }

    None
}

fn char_to_score(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32 - 0x30) + 10 
    } else {
        (c as u32 - 0x30) - 48
    }
}
