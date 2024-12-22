use std::fs::File;
use std::io::{self, BufRead};

use crate::days::ChristmasSaver;

fn read_rows_2() -> Vec<Vec<i32>> {
    let path = "src/data_files/day2.txt";

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut rows = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect();

        rows.push(numbers);
    }

    rows
}

pub fn is_row_safe(row: &Vec<i32>) -> bool {
    if row.len() == 1 { return true; }
    let increasing = row[1] > row[0];
    for i in 1..=row.len() - 1 {
        let diff = (row[i] - row[i-1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }

        if increasing != (row[i] > row[i-1]) {
            return false;
        }
    }
    true
}


impl ChristmasSaver {
    pub fn count_safe_reports(&self) ->  i32 {
        let mut count = 0;
        let rows = read_rows_2();

        for row in rows.iter() {
            if is_row_safe(row) {
                count += 1;
            }
        }

        count
    }
}