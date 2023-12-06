use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut answer: u32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let mut num_char_vec: Vec<char> = vec![];
                let char_vec: Vec<char> = line.chars().collect();
                for char in char_vec {
                    if char.is_digit(10) {
                        num_char_vec.push(char);
                    }
                }
                let first_num: char = *num_char_vec.first().unwrap();
                let last_num: char = *num_char_vec.last().unwrap();
                let calibration_chars: Vec<char> = vec![first_num, last_num];
                let num_string: String = calibration_chars.into_iter().collect();
                // println!("{}", num_string);
                let number = num_string.parse::<u32>().unwrap();
                answer = answer + number;
            }
        }
        println!("answer 1: {}", answer)
    }
}

fn part_two() {
    let number_words = vec![("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];
    if let Ok(lines) = read_lines("./input.txt") {
        // for each line
        for line in lines {
            
        }
    }
}