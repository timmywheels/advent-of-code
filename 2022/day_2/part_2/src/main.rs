use std::{env, fs};
use std::ops::AddAssign;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let curr_dir = env::current_dir()?.display().to_string();
    let path_to_input = format!("{}/src/assets/input.txt", curr_dir);
    let data = fs::read_to_string(path_to_input).expect("Error reading input file");
    let str_arr = parse_data_into_arr(data);

    let mut my_score: u64 = 0;
    let mut opponent_score: u64 = 0;

    const LOSE_SCORE: u64 = 0;
    const DRAW_SCORE: u64 = 3;
    const WIN_SCORE: u64 = 6;

    const ROCK_SCORE: u64 = 1;
    const PAPER_SCORE: u64 = 2;
    const SCISSORS_SCORE: u64 = 3;


    for val in str_arr.iter() {
        println!("{}", val);
        let opponent_choice = OpponentChoices::from_str(&val.chars().nth(0).unwrap().to_string()[..]);
        let my_choice = MyChoices::from_str(&val.chars().nth(1).unwrap().to_string()[..]);

        let outcome = match opponent_choice {
            Ok(OpponentChoices::ROCK) => {
                opponent_score += ROCK_SCORE;
                if my_choice == Ok(MyChoices::DRAW) {
                    my_score += ROCK_SCORE + DRAW_SCORE;
                    opponent_score += DRAW_SCORE;
                } else if my_choice == Ok(MyChoices::WIN) {
                    my_score += PAPER_SCORE + WIN_SCORE;
                    opponent_score += LOSE_SCORE;
                } else {
                    my_score += SCISSORS_SCORE + LOSE_SCORE;
                    opponent_score += WIN_SCORE;
                }
                Ok(())
            },
            Ok(OpponentChoices::PAPER) => {
                opponent_score += PAPER_SCORE;
                if my_choice == Ok(MyChoices::DRAW) {
                    my_score += PAPER_SCORE + DRAW_SCORE;
                    opponent_score += DRAW_SCORE;
                } else if my_choice == Ok(MyChoices::LOSE) {
                    my_score += ROCK_SCORE + LOSE_SCORE;
                    opponent_score += WIN_SCORE;
                } else {
                    my_score += SCISSORS_SCORE + WIN_SCORE;
                    opponent_score += LOSE_SCORE
                }
                Ok(())
            },
            Ok(OpponentChoices::SCISSORS) => {
                opponent_score += SCISSORS_SCORE;
                if my_choice == Ok(MyChoices::DRAW) {
                    my_score += SCISSORS_SCORE + DRAW_SCORE;
                    opponent_score += DRAW_SCORE;
                } else if my_choice == Ok(MyChoices::WIN) {
                    my_score += ROCK_SCORE + WIN_SCORE;
                    opponent_score += LOSE_SCORE;
                } else {
                    my_score += PAPER_SCORE + LOSE_SCORE;
                    opponent_score += WIN_SCORE
                }
                Ok(())
            },
            _ => Err(()),
        };

    }

    println!("My Score: {}", my_score);
    println!("Opponent Score: {}", opponent_score);
    Ok(())
}

fn parse_data_into_arr(data: String) -> Vec<String> {
    data.split('\n')
        .map(|round| round.split_whitespace().collect())
        .collect()
}

#[derive(PartialEq)]
enum MyChoices {
    WIN,
    LOSE,
    DRAW,
}

enum OpponentChoices {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Outcomes {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

impl FromStr for MyChoices {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MyChoices::LOSE),
            "Y" => Ok(MyChoices::DRAW),
            "Z" => Ok(MyChoices::WIN),
            _ => Err(()),
        }
    }
}

impl FromStr for OpponentChoices {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(OpponentChoices::ROCK),
            "B" => Ok(OpponentChoices::PAPER),
            "C" => Ok(OpponentChoices::SCISSORS),
            _ => Err(()),
        }
    }
}


/*
    Scoring

    Choices:
    1 for Rock
    2 for Paper
    3 for Scissors

    Outcomes:
    0 for a loss
    3 for a draw
    6 for a win
*/
