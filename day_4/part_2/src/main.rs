#![feature(str_split_as_str)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let curr_dir = env::current_dir()?.display().to_string();
    let path_to_input = format!("{}/src/assets/input.txt", curr_dir);

    let file = File::open(path_to_input)?;
    let reader = BufReader::new(file);

    let mut num_of_partially_contained_pairs = 0;

    for current_line_of_file in reader.lines() {
        let curr_line = current_line_of_file.unwrap_or_default();
        let mut assignment_pairs = curr_line.split(',').collect::<Vec<&str>>();

        let pair_one = assignment_pairs.get(0).unwrap();
        let pair_two = assignment_pairs.get(1).unwrap();

        let pair_one_range = pair_one.split('-').collect::<Vec<&str>>();
        let pair_one_start = pair_one_range.get(0).unwrap().to_string().parse::<i32>().unwrap();
        let pair_one_finish = pair_one_range.get(1).unwrap().to_string().parse::<i32>().unwrap();

        let pair_two_range = pair_two.split('-').collect::<Vec<&str>>();
        let pair_two_start = pair_two_range.get(0).unwrap().to_string().parse::<i32>().unwrap();
        let pair_two_finish = pair_two_range.get(1).unwrap().to_string().parse::<i32>().unwrap();

        if pair_one_start <= pair_two_finish && pair_two_start <= pair_one_finish {
            num_of_partially_contained_pairs += 1;
        }
    }
    println!("num_of_fully_contained_pairs: {}", num_of_partially_contained_pairs);
    Ok(())
}
