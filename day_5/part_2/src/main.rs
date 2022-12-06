#![feature(str_split_as_str)]
#![feature(str_split_whitespace_as_str)]

use std::collections::{HashMap, VecDeque};
use std::io::{BufRead};
use utils::get_file_reader;

const COLUMN_WIDTH: i32 = 3;
const COLUMN_SPACING: i32 = 1;

const INITIAL_CRATE_POSITION: i32 = 1;

fn get_column_from_cursor_position(cursor_position: i32) -> i32 {
    match cursor_position {
        1 => 1,
        5 => 2,
        9 => 3,
        13 => 4,
        17 => 5,
        21 => 6,
        25 => 7,
        29 => 8,
        33 => 9,
        _ => 0
    }
}

fn generate_stack_map<'hello>() -> HashMap<i32, VecDeque<char>> {
    let mut stack_map: HashMap<i32, VecDeque<char>> = HashMap::new();
    for column in 1..10 {
        stack_map.insert(column, VecDeque::new());
    }
    stack_map
}

fn parse_crate_diagram_input<'hello>() -> HashMap<i32, VecDeque<char>> {
    let reader = get_file_reader("src/assets/input.txt");
    let mut stack_map = generate_stack_map();

    let mut total_lines_in_diagram = 8;

    for line_of_diagram in reader.lines() {
        let mut cursor_position = 0;

        if total_lines_in_diagram < 0 {
            break;
        }

        for character in line_of_diagram.unwrap().chars() {
            if character.is_ascii_alphabetic() {
                let column = get_column_from_cursor_position(cursor_position);
                stack_map.get_mut(&column).unwrap().push_back(character);
            }
            cursor_position += 1;
        }
        total_lines_in_diagram -= 1;
    }
    stack_map
}

fn parse_move_from_to_line(line: String) -> (i32, i32, i32) {
    let move_split = line.split("move").nth(1).unwrap_or_default().trim();
    let from_split_parts = move_split.split("from").collect::<Vec<&str>>();

    let num_moves = from_split_parts.get(0).unwrap().trim().to_string();
    let from_to_str = match from_split_parts.get(1) {
        None => {
            "".to_string()
        },
        Some(part) => {
            part.trim().to_string()
        }
    };
    let mut from_to_split = from_to_str.split_whitespace();
    let from = match from_to_split.nth(0) {
        None => {
            "".to_string()
        },
        Some(part) => {
            part.trim().to_string()
        }
    };
    let to = match from_to_split.nth(1) {
        None => {
            "".to_string()
        },
        Some(part) => {
            part.trim().to_string()
        }
    };

    (
        num_moves.parse::<i32>().unwrap_or_default(),
        from.parse::<i32>().unwrap_or_default(),
        to.parse::<i32>().unwrap_or_default()
    )
}

fn main() {
    let reader = get_file_reader("src/assets/input.txt");
    let mut stack_map = parse_crate_diagram_input();

    let mut line_count = 0;
    let num_lines_to_skip = 9;

    for current_line_of_file in reader.lines() {
        if line_count < num_lines_to_skip {
            line_count += 1;
            continue;
        }

        let curr_line = current_line_of_file.unwrap();

        if curr_line.is_empty() {
            continue
        }

        let (num_of_moves, from, to) = parse_move_from_to_line(curr_line);
        let mut num_moves_left = num_of_moves;

        const DEFAULT_CHAR: char = '-';

        let mut intermediate_stack = VecDeque::new();

        while num_moves_left > 0 {
            let curr_item = match stack_map.get_mut(&from).unwrap().is_empty() {
                true => DEFAULT_CHAR,
                false => {
                    // pop the top item of the `from` stack
                    stack_map.get_mut(&from).unwrap().pop_front().unwrap()
                }
            };

            if curr_item != DEFAULT_CHAR {
                // push the item onto the intermediate stack
                intermediate_stack.push_front(curr_item);
            }
            num_moves_left -= 1;
        }

        // pop all items added to intermediate stack so that they
        // come out reversed, according to problem spec
        while !intermediate_stack.is_empty() {
            let top_item = intermediate_stack.pop_front().unwrap();
            // push the item onto the `to` stack
            stack_map.get_mut(&to).unwrap().push_front(top_item);
        }
    }

    for i in 1..10 {
        // pop then print top item from every stack
        let top_item = stack_map.get_mut(&i).unwrap().pop_front().unwrap();
        print!("{}", top_item);
    }
}
