use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use crate::days::ChristmasSaver;

fn read_rows_1() -> (Vec<i32>, Vec<i32>) {
    let path = "src/data_files/day1.txt";

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut col1 = vec![];
    let mut col2 = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        col1.push(numbers[0]);
        col2.push(numbers[1]);
    }

    col1.sort();
    col2.sort();
    (col1, col2)
}

impl ChristmasSaver {
    pub fn get_list_distance(&self) -> i32 {
        let mut distance = 0;
        let rows = read_rows_1();

        for (val1, val2) in rows.0.iter().zip(rows.1.iter()) {
            distance += (val1 - val2).abs();
        }
        
        distance
    }

    pub fn get_similarity_score(&self) -> i32 {
        let mut score = 0;
        let rows = read_rows_1();
        
        let mut right_count: HashMap<i32, i32> = HashMap::new();
        for &num in rows.1.iter() {
            *right_count.entry(num).or_insert(0) += 1;
        }

        for num in rows.0.iter() {
            if let Some(&value) = right_count.get(num) {
                score += *num * value;
            }
        }

        score
    }
}
