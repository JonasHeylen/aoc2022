use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let input_file = "input.txt";
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut calories_per_elve: Vec<u32> = Vec::new();
    let mut acc = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            calories_per_elve.push(acc);
            acc = 0;
        } else  {
            let calories: u32 = line.parse().unwrap();
            acc += calories;
        }
    }

    calories_per_elve.push(acc);

    let max_calories = calories_per_elve.iter().max().unwrap();

    println!("Number of elves: {} - max calories {}", calories_per_elve.len(), max_calories);

    calories_per_elve.sort();
    let top3: Vec<&u32> = calories_per_elve.iter().rev().take(3).collect();
    let sum_top3: u32 = top3.into_iter().sum();
    println!("Top 3 sum: {}", sum_top3);
}
