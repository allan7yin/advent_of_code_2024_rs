use crate::days::ChristmasSaver;
use std::{fs::File, io::{BufRead, BufReader}};
use std::collections::HashMap;

fn read_rows_7() -> HashMap<i64, Vec<i64>> {
    let path = "src/data_files/day7.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut eqs = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some((key, values)) = line.split_once(':') {
            let key: i64 = key.trim().parse().unwrap();
            let value: Vec<i64> = values.trim().split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect();
            eqs.insert(key, value);
        }
    }

    eqs
}

impl ChristmasSaver {
    pub fn find_poss_calibrations(&self) -> i64 {
        let mut sum = 0;
        let eqs = read_rows_7();
        for (key, value) in eqs {
            let result = Self::add_operators(&value, key, false);
            if result > 0 {
                sum += key;
            }
        }

        sum
    }

    pub fn find_poss_calibrations_2(&self) -> i64 {
        let mut sum = 0;
        let eqs = read_rows_7();
        for (key, value) in eqs {
            let result = Self::add_operators(&value, key, true);
            if result > 0 {
                sum += key;
            }
        }

        sum
    }

    pub fn add_operators(nums: &Vec<i64>, target: i64, part2: bool) -> i64 {
        let mut combinations: i64 = 0;
        let mut result = nums[0].to_string();
        Self::f(1, &nums, target, &mut result, nums[0], &mut combinations, part2);

        combinations
    }

    fn f(i: usize, nums: &Vec<i64>, target: i64, result: &mut String, sum: i64, combinations: &mut i64, part2: bool) {
        if i == nums.len() {
            if sum == target {
                *combinations += 1;
            }
            return;
        }

        Self::f(i + 1, nums, target, result, sum + nums[i], combinations, part2);
        Self::f(i + 1, nums, target, result, sum * nums[i], combinations, part2);

        if part2 {
            let digits = nums[i].to_string().len() as u32;
            let concatenated = sum * 10_i64.pow(digits) + nums[i];
            Self::f(i + 1, nums, target, result, concatenated, combinations, part2);
        }
    }
}