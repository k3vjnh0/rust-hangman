use colored::*;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool,
}

enum GameProgress {
    InProgress,
    Won,
    Lost,
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    println!("{}", "Welcome to Hangman!".red().bold());

    loop {
        println!(
            "\nYou have {} turns left.",
            turns_left.to_string().yellow().bold()
        );
        display_progress(&letters);

        println!("\nPlease enter a letter to guess:");
        let user_char = read_user_input_character();

        if user_char == '*' {
            break;
        }

        let mut at_least_one_revealed = false;

        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        if !at_least_one_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats, you won!");
                break;
            }
            GameProgress::Lost => {
                println!("\nSorry, you lost!");
                break;
            }
        }
    }

    println!(
        "Selected word was {}.\nGood bye!\n",
        selected_word.green().bold()
    );
}

fn select_word() -> String {
    let mut file = File::open("words.txt").expect("Could not open file!");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error has occurred while reading this file!");

    let available_words: Vec<&str> = file_contents.trim().split("\n").collect();

    let random_index = rand::thread_rng().gen_range(0..available_words.len());

    String::from(available_words[random_index])
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false,
        });
    }

    letters
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }
    }

    println!("{} ({} characters)", display_string, letters.len());
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => c,
            None => '*',
        },
        Err(_) => '*',
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_revealed = true;

    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
