use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::addr_of;

fn main() -> std::io::Result<()> {
    let curr_dir = env::current_dir()?.display().to_string();
    let path_to_input = format!("{}/src/assets/input.txt", curr_dir);

    let file = File::open(path_to_input)?;
    let reader = BufReader::new(file);

    let mut sum_of_all_priorities = 0;

    for current_line_of_file in reader.lines() {
        let curr_line = current_line_of_file.unwrap();
        let index_of_middle_of_string = &curr_line.len() / 2;

        let compartment_one = &curr_line[..index_of_middle_of_string];
        let compartment_two = &curr_line[index_of_middle_of_string..];

        let mut map: HashMap<char, Compartment> = HashMap::new();

        for item in compartment_one.chars() {
            // add all items from an arbitrary compartment
            // to the map, in this case we will add all items
            // from compartment_one
            map.insert(item, Compartment::ONE);
        }

        for item in compartment_two.chars() {
            // now check for duplicates that
            // may exist in both compartments
            if map.contains_key(&item) && map.get(&item) != Some(&Compartment::TWO) {
                // duplicate item in rucksack found
                // update running sum of priority
                sum_of_all_priorities += calculate_priority_from_char(item);
                // we can stop computation
                break;
            } else {
                map.insert(item, Compartment::TWO);
            }
        }
    }

    println!("Sum of all priorities: {}", sum_of_all_priorities);
    Ok(())
}

fn calculate_priority_from_char(c: char) -> i32 {

    const LOWERCASE_OFFSET: i32 = 96;
    const UPPERCASE_OFFSET: i32 = 38;

    let mut priority_for_char: i32 = 0;

    if c.is_ascii_lowercase() {
        priority_for_char = (c as i32) - LOWERCASE_OFFSET;
        return priority_for_char
    }

    priority_for_char = (c as i32) - UPPERCASE_OFFSET;
    priority_for_char
}

#[derive(PartialEq)]
enum Compartment {
    ONE,
    TWO
}