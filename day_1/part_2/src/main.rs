use std::io::BufRead;
use utils::get_file_reader;

fn main() -> std::io::Result<()> {
    let reader = get_file_reader("src/assets/input.txt");

    let mut calories_for_current_elf: i32 = 0;
    let mut top_three_elf_calorie_counts: [i32; 3] = [0; 3];
    let mut sum_of_top_three_elf_calorie_counts: i32 = 0;

    for current_line_of_file in reader.lines() {
        let line = &current_line_of_file.unwrap();
        let reached_end_of_calories_for_current_elf = line.is_empty();
        if reached_end_of_calories_for_current_elf {
            let elf_with_most_calories = top_three_elf_calorie_counts[0];
            let elf_with_second_most_calories = top_three_elf_calorie_counts[1];
            let elf_with_third_most_calories = top_three_elf_calorie_counts[2];

            // shift original values down when a larger calorie count is found
            if calories_for_current_elf > elf_with_most_calories {
                top_three_elf_calorie_counts[0] = calories_for_current_elf;
                top_three_elf_calorie_counts[1] = elf_with_most_calories;
                top_three_elf_calorie_counts[2] = elf_with_second_most_calories;

            } else if calories_for_current_elf > elf_with_second_most_calories {
                top_three_elf_calorie_counts[1] = calories_for_current_elf;
                top_three_elf_calorie_counts[2] = elf_with_second_most_calories;
            } else if calories_for_current_elf > elf_with_third_most_calories {
                top_three_elf_calorie_counts[2] = calories_for_current_elf;
            }
            calories_for_current_elf = 0;
        } else {
            let calories = &line.parse::<i32>().unwrap();
            calories_for_current_elf += calories;
        }
    }

    for calorie_count in top_three_elf_calorie_counts {
        sum_of_top_three_elf_calorie_counts += calorie_count
    }
    println!("Total Calories for Top 3 Elves: {}", sum_of_top_three_elf_calorie_counts);
    Ok(())
}


