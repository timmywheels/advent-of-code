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

    const REQUIRED_BADGE_COUNT: i32 = 3;

    let mut sum_of_all_badge_priorities = 0;
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
        if line_count == REQUIRED_BADGE_COUNT {

            // check for intersections of items between the three elves
            let intersection_of_elves_one_and_two = elf_one_set.intersection(&elf_two_set).collect::<Vec<&char>>();
            let intersection_of_elves_two_and_three = elf_two_set.intersection(&elf_three_set).collect::<Vec<&char>>();

            // now work on determining if there's any intersecting values from the intersections above
            let set_of_intersecting_values_from_elves_one_and_two = HashSet::<&char>::from_iter(intersection_of_elves_one_and_two.clone());
            let set_of_intersecting_values_from_elves_two_and_three = HashSet::<&char>::from_iter(intersection_of_elves_two_and_three.clone());

            // based on the constraints
            // there should only be one intersection for each group of three elves
            // (if any at all) so extract index 0
            let intersection_of_elves_one_two_and_three: &&char = set_of_intersecting_values_from_elves_one_and_two.intersection(&set_of_intersecting_values_from_elves_two_and_three).nth(0).unwrap();


            if intersection_of_elves_one_two_and_three.is_ascii() {
                // calculate the badge priority for the value that intersects across all three elves
                // and add it to the running sum
                sum_of_all_badge_priorities += calculate_priority_from_char(**intersection_of_elves_one_two_and_three);
            }

            // reset values
            // for the next sequence of three elves
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
