use std::io::BufRead;
use utils::get_file_reader;

fn main() -> std::io::Result<()> {
    let reader = get_file_reader("src/assets/input.txt");

    let mut calories_for_current_elf: i32 = 0;
    let mut most_calories_for_any_elf: i32 = 0;

    for current_line_of_file in reader.lines() {
        let line = &current_line_of_file.unwrap();
        let reached_end_of_calories_for_current_elf = line.is_empty();
        if reached_end_of_calories_for_current_elf {
            if calories_for_current_elf > most_calories_for_any_elf {
                most_calories_for_any_elf = calories_for_current_elf
            }
            calories_for_current_elf = 0;
        } else {
            let calories = &line.parse::<i32>().unwrap();
            calories_for_current_elf += calories;
        }
    }
    println!("Most Calories for Any Elf: {}", most_calories_for_any_elf);
    Ok(())
}


