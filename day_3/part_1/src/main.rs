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

        println!("{}", &curr_line);
        println!("{} | {}", &compartment_one, &compartment_two);
        println!("-----");

        // if &curr_line.len() % 2 != 0 {
        //     println!("unequal length: {}", &curr_line.len())
        // }

        let mut map: HashMap<char, Compartment> = HashMap::new();

        for item in compartment_one.chars() {
            // add all items from an arbitrary compartment
            // to the map, in this case we will add all items
            // from compartment_one
            map.insert(item, Compartment::ONE);
        }

        for item in compartment_two.chars() {
            // now check for duplicates that exist
            // in both compartments
            let another = &item;
            if map.contains_key(&item) && map.get(&item) != Some(&Compartment::TWO) {
                // update running sum of priority
                sum_of_all_priorities += calculate_priority_from_char(item);
                // duplicate item in rucksack found
                // so we can stop computation
                break;
            } else {
                map.insert(item, Compartment::TWO);
            }
        }


        // println!("current_line: {}", &curr_line);
        // println!("compartment_one: {}", compartment_one);
        // println!("compartment_two: {}", compartment_two);
        // println!("-----");

        // 7501
        // 60284


    }

    println!("Sum of all priorities: {}", sum_of_all_priorities);
    Ok(())
}

fn calculate_priority_from_char(c: char) -> i32 {

    const LOWERCASE_OFFSET: i32 = 96;
    const UPPERCASE_OFFSET: i32 = 38;

    let mut priority_for_char: i32 = 0;

    if c.is_ascii_lowercase() {
        // println!("lowercase");
        // println!("char: {}", c);
        // println!("char as num: {}", (c as i32) - LOWERCASE_OFFSET);
        // println!("---");
        priority_for_char = (c as i32) - LOWERCASE_OFFSET;
        // println!("Priority: {}", priority_for_char);
        println!("priority_for_char '{}': {}", &c, &priority_for_char);
        return priority_for_char
    }

    // println!("uppercase");
    // println!("char: {}", c);
    // println!("char as num: {}", (c as i32) - UPPERCASE_OFFSET);
    // println!("---");
    priority_for_char = (c as i32) - UPPERCASE_OFFSET;

    println!("priority_for_char '{}': {}", &c, &priority_for_char);
    priority_for_char
}

#[derive(PartialEq)]
enum Compartment {
    ONE,
    TWO
}