#![feature(str_split_as_str)]

use std::io::BufRead;
use utils::get_file_reader;

fn main() -> std::io::Result<()> {
    let reader = get_file_reader("src/assets/input.txt");

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
