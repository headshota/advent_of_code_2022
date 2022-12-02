use std::fs;
use std::io::{prelude::*, BufReader};

pub fn day_1() -> u32{
    let file = fs::File::open("src/inputs/day_1.txt")
        .expect("Failed to read input file");

    let lines = BufReader::new(file).lines();

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
