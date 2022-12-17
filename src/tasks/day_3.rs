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
        let matching_chars = find_same_char(part_1, part_2);
        let ch = matching_chars.first().unwrap();
        sum += char_to_score(*ch);
    }

    sum
}


pub fn part_2() -> u32 {
    let lines = get_input_lines("src/inputs/day_3.txt").unwrap();
    let mut sum = 0;

    let mut group = Vec::new();
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        group.push(line);

        if (i + 1) % 3 == 0 {
            let mut iter = group.iter();
            let (line1, line2, line3) = ( iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap() );

            let matching_chars_1 = find_same_char(&line1[..], &line2[..]).iter().collect::<String>();
            let matching_chars_2 = find_same_char(&matching_chars_1[..], &line3[..]);


            let ch = matching_chars_2.first().unwrap();
            sum += char_to_score(*ch);
            group.clear();
        }
    }

    sum
}

fn find_same_char(part_1: &str, part_2: &str) -> Vec<char> {
    part_1.chars().filter(|&c| part_2.contains(c) ).collect()
}

fn char_to_score(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32 - 0x30) + 10 
    } else {
        (c as u32 - 0x30) - 48
    }
}
