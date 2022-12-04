use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::collections::btree_set::Intersection;
use std::env;
use std::fmt::{Display, Formatter};
use std::fs::{File, read};
use std::io::{BufRead, BufReader};
use std::ptr::addr_of;

fn main() -> std::io::Result<()> {
    let curr_dir = env::current_dir()?.display().to_string();
    let path_to_input = format!("{}/src/assets/input.txt", curr_dir);

    let file = File::open(path_to_input)?;
    let reader = BufReader::new(file);

    let mut sum_of_all_badge_priorities = 0;

    const DEFAULT_BADGE_COUNT: i32 = 1;
    const REQUIRED_BADGE_COUNT: i32 = 3;

    let mut line_count = 1;

    let mut elf_one_set: HashSet<char> = HashSet::new();
    let mut elf_two_set: HashSet<char> = HashSet::new();
    let mut elf_three_set: HashSet<char> = HashSet::new();

    for line in reader.lines() {
        let curr_line = line.unwrap();

        for character in curr_line.chars() {
            match line_count {
                1 => elf_one_set.insert(character),
                2 => elf_two_set.insert(character),
                3 => elf_three_set.insert(character),
                _ => true
            };
        }

        // update line count accordingly
        if line_count == 3 {

            // it does not matter the permutation we use to determine the intersection
            // as long as we check all three of the sets in some way
            let intersection_of_one_and_two = elf_one_set.intersection(&elf_two_set).collect::<Vec<&char>>();
            let intersection_of_two_and_three = elf_two_set.intersection(&elf_three_set).collect::<Vec<&char>>();

            let intersect_one = HashSet::<&char>::from_iter(intersection_of_one_and_two.clone());
            let intersect_two = HashSet::<&char>::from_iter(intersection_of_two_and_three.clone());

            // there should only be one intersection for each group of three elves
            let intersection_of_intersections: &&char = intersect_one.intersection(&intersect_two).nth(0).unwrap();

            if intersection_of_intersections.is_ascii() {
                // calculate priority for the intersection value
                sum_of_all_badge_priorities += calculate_priority_from_char(**intersection_of_intersections);
            }

            // reset
            line_count = 1;
            elf_one_set = HashSet::new();
            elf_two_set = HashSet::new();
            elf_three_set = HashSet::new();
        } else {
            line_count += 1;
        }

    }

    println!("Sum of all priorities: {}", sum_of_all_badge_priorities);
    Ok(())
}

fn calculate_priority_from_char(c: char) -> i32 {
    const LOWERCASE_OFFSET: i32 = 96;
    const UPPERCASE_OFFSET: i32 = 38;

    let mut priority_for_char: i32 = 0;

    if c.is_ascii_lowercase() {
        priority_for_char = (c as i32) - LOWERCASE_OFFSET;
        return priority_for_char;
    }
    priority_for_char = (c as i32) - UPPERCASE_OFFSET;
    priority_for_char
}