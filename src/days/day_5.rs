use crate::days::ChristmasSaver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering;

fn read_rows_5() -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut top = true;
    let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut updates = vec![];
    let path = "src/data_files/day5.txt";

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            top = false;
            continue;
        }

        if top {
            let nums: Vec<i32> = line.split('|').map(|n| n.trim().parse::<i32>().unwrap()).collect();
            adj.entry(nums[0]).or_insert_with(HashSet::new).insert(nums[1]);
        } else {
            let update: Vec<i32> = line.split(',').map(|n| n.trim().parse::<i32>().unwrap()).collect();
            updates.push(update);
        }
        
    }

    (adj, updates)
}

fn compare_values(a: &i32, b: &i32, ordering: &HashMap<i32, HashSet<i32>>) -> Ordering {
    // If a must come before b according to ordering
    if ordering.get(a).map_or(false, |set| set.contains(b)) {
        return Ordering::Less;
    }
    // If b must come before a according to ordering
    if ordering.get(b).map_or(false, |set| set.contains(a)) {
        return Ordering::Greater;
    }
    // If no explicit ordering, maintain original order
    Ordering::Equal
}

impl ChristmasSaver {
    pub fn get_med_valid_updates(&self) -> i32 {
        let (adj, updates) = read_rows_5();
        let mut result = 0;
        for update in updates {
            if Self::check_update(&update, &adj) {
                let m = update.len() / 2;
                result += update[m];
            }
        }
        result
    }

    pub fn get_med_postfix(&self) -> i32 {
        let (adj, updates) = read_rows_5();
        let mut result = 0;
        for update in updates {
            if !Self::check_update(&update, &adj) {
                let mut sorted = update.clone();
                sorted.sort_by(|a, b| compare_values(a, b, &adj));
                let m = sorted.len() / 2;
                result += sorted[m];
            }
        }

        result
    }

    fn check_update(update: &Vec<i32>, adj: &HashMap<i32, HashSet<i32>>) -> bool {
        for i in 0..update.len() - 1 {
            let before = update[i];
            let next = update[i+1];

            if !adj.contains_key(&before) || !adj[&before].contains(&next) {
                return false;
            }
        }
        true
    }

}