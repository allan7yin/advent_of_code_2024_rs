use std::fs::File;
use std::io::{self, BufRead};

use crate::days::ChristmasSaver;
use regex::Regex;

fn read_rows_3() -> String {
    let path = "src/data_files/day3.txt";
    let mut input = String::new();

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        input.push_str(&line);
    }

    input
}

fn clean_input(input: &String) -> String {
    let pattern = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let cleaned = pattern.replace_all(input, "");
    return cleaned.to_string();
}

impl ChristmasSaver {
    fn find_mult_sum(input: &String) -> i32 {
        let pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let matches: Vec<&str> = pattern.find_iter(&input).map(|mat| mat.as_str()).collect();

        let mut sum = 0;
        for mat in matches {
            let numbers = &mat[4..mat.len() - 1];
            let parts: Vec<i32> = numbers.split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            
            sum += parts[0] * parts[1];
        }

        sum 
    }
    pub fn sum_of_mults(&self) -> i32 {
        let input = read_rows_3();
        return Self::find_mult_sum(&input);
        
    }

    pub fn sum_of_mults_2(&self) -> i32 {
        let input = read_rows_3();
        let cleaned_input = clean_input(&input);
        return Self::find_mult_sum(&cleaned_input);
    }
}