use std::fs;
use std::io::{prelude::*, BufReader};

fn get_input_lines(path: &str) -> Result<std::io::Lines<BufReader<fs::File>>, std::io::Error> {
    let file = fs::File::open(path)?;
    Ok(BufReader::new(file).lines())
}

pub fn day_1_1() -> u32{
    let lines = get_input_lines("src/inputs/day_1.txt").unwrap();

    let mut sum = 0;
    let mut max = 0;

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            if sum > max {
                max = sum;
            }

            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }

    max
}


pub fn day_1_2() -> u32 {
    let lines = get_input_lines("src/inputs/day_1.txt").unwrap();

    let mut elves = Vec::new();
    let mut sum = 0;

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }
    
    elves.sort();
    elves.reverse();
    elves.iter().take(3).sum()
}

