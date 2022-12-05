#![feature(str_split_as_str)]
#![feature(str_split_whitespace_as_str)]

use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, Seek};
use std::str::Split;
use utils::get_file_reader;

/*
    initial crate arrangement

                        [L]     [H] [W]
                    [J] [Z] [J] [Q] [Q]
    [S]             [M] [C] [T] [F] [B]
    [P]     [H]     [B] [D] [G] [B] [P]
    [W]     [L] [D] [D] [J] [W] [T] [C]
    [N] [T] [R] [T] [T] [T] [M] [M] [G]
    [J] [S] [Q] [S] [Z] [W] [P] [G] [D]
    [Z] [G] [V] [V] [Q] [M] [L] [N] [R]
     1   2   3   4   5   6   7   8   9
*/


fn main() {
    let mut reader = get_file_reader("src/assets/input.txt");
    let mut stack_map = HashMap::new();

    let range_of_crate_stacks = 1..10;
    for i in range_of_crate_stacks {
        match i {
            1 => stack_map.insert(i, VecDeque::from(['S', 'P', 'W', 'N', 'J', 'Z'])),
            2 => stack_map.insert(i, VecDeque::from(['T', 'S', 'G'])),
            3 => stack_map.insert(i, VecDeque::from(['H', 'L', 'R', 'Q', 'V'])),
            4 => stack_map.insert(i, VecDeque::from(['D', 'T', 'S', 'V'])),
            5 => stack_map.insert(i, VecDeque::from(['J', 'M', 'B', 'D', 'T', 'Z', 'Q'])),
            6 => stack_map.insert(i, VecDeque::from(['L', 'Z', 'C', 'D', 'J', 'T', 'W', 'M'])),
            7 => stack_map.insert(i, VecDeque::from(['J', 'T', 'G', 'W', 'M', 'P', 'L'])),
            8 => stack_map.insert(i, VecDeque::from(['H', 'Q', 'F', 'B', 'T', 'M', 'G', 'N'])),
            9 => stack_map.insert(i, VecDeque::from(['W', 'Q','B', 'P', 'C', 'G', 'D', 'R'])),
            _ => stack_map.insert(i, VecDeque::new()),
        };
    }

    for current_line_of_file in reader.lines() {
        let mut curr_line = current_line_of_file.unwrap();

        let split = curr_line.split_whitespace().collect::<Vec<&str>>();

        let num_of_moves = split.get(0).unwrap().parse::<i32>().unwrap();
        let from = split.get(1).unwrap().parse::<i32>().unwrap();
        let to = split.get(2).unwrap().parse::<i32>().unwrap();

        let mut num_moves_left = num_of_moves;

        const DEFAULT_CHAR: char = '-';

        while num_moves_left > 0 {
            // if `from` stack is not empty
            let curr_item = match stack_map.get_mut(&from).unwrap().is_empty() {
                true => DEFAULT_CHAR,
                false => {
                    // pop the top item of the `from` stack
                    stack_map.get_mut(&from).unwrap().pop_front().unwrap()
                },
            };
            if curr_item != DEFAULT_CHAR {
                // push the item onto the `to` stack
                stack_map.get_mut(&to).unwrap().push_front(curr_item);
            }
            num_moves_left -= 1;
        }
    }

    for i in 1..10 {
        // pop then print top item from every stack
        let top_item = stack_map.get_mut(&i).unwrap().pop_front().unwrap();
        print!("{}", top_item);
    }
}
