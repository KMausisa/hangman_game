use rand::seq::IteratorRandom;
use std::io;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "words.txt";

struct Letter {
    character: char,
    revealed: bool,
}


struct BodyPart {
    body: String,
    used: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn find_word() -> String {
    let f = File::open(FILENAME).unwrap_or_else(|e| panic!("File not found: {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had not lines")
}

/*
 * Reads a character from user input. If multiple characters are given,
 * the first index is returned.
 */
fn read_user_input() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => {
                return c;
            }
            None => {
                return '*';
            }
        },
        Err(_) => {
            return '*';
        }
    }
}

fn store_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }
    return letters;
}

fn store_stick_man(stick_man_body_parts: &Vec<String>) -> Vec<BodyPart> {

    let mut stick_man: Vec<BodyPart> = Vec::new();

    for body_part in stick_man_body_parts {
        stick_man.push(BodyPart {
            body: body_part.to_owned(),
            used: false
        });
    }
    return stick_man;
}

fn draw_stick_man(stick_man: &Vec<BodyPart>) {
    for body_part in stick_man {
        if body_part.used {
            println!("{}", body_part.body);
        }
    }
} 

/* Displays the progress of the game based off Vec<Letter>
Example output: l _ n g _ _ g _ */
fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:"); // Example: Progress: _ a _ a _ y

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

/* Checks the current state (progress) of the game and returns the appropriate
GameProgress member */
fn check_progress(stick_man: &Vec<BodyPart>, letters: &Vec<Letter>) -> GameProgress {
    /* Determine if all letters have been revealed */
    let mut all_letters_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_letters_revealed = false;
        }
    }

    // Determine if all body parts were drawn
    let mut all_parts_used = true;
    for body_part in stick_man {
        if !body_part.used {
            all_parts_used = false;
        }
    }

    if all_letters_revealed {
        return GameProgress::Won;
    }
    
    if all_parts_used {
        return GameProgress::Lost;
    }

    return GameProgress::InProgress;
}

fn main() {
    // welcome the player to hangman
    println!("Welcome to hangman!");

    // create a stick man template
    let mut stick_man_body_parts: Vec<String> = Vec::new();
    stick_man_body_parts.push(String::from("  O"));
    stick_man_body_parts.push(String::from(" /|\\"));
    stick_man_body_parts.push(String::from("/ | \\ "));
    stick_man_body_parts.push(String::from(" / \\"));
    stick_man_body_parts.push(String::from("/   \\")); 

    let mut stick_man = store_stick_man(&stick_man_body_parts);
    // create a vector that will draw the 
    // stickman for every incorrect guess

    // generate random word from words.txt file
    let rand_word = find_word();

    // Store each letter in a vec
    let mut letters = store_letters(&rand_word);

    // print new line
    println!("");

    // game loop
    loop {
        draw_stick_man(&stick_man);
        display_progress(&letters);
        // take user input: read and evaluate
        println!("Please input your guess.");
        let guess = read_user_input();
        // print out the guess
        println!("Your guess was: {}", guess);

        // update the revealed state of each letter
        let mut revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == guess {
                letter.revealed = true;
                revealed = true;
            }
        }

        // Draw stickman if user makes an incorrect guess
        if !revealed {
            for body_part in stick_man.iter_mut() {
                if body_part.used == false {
                    body_part.used = true;
                    break;
                }
            }
        }
        
        // Check game progress
        match check_progress(&stick_man, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                display_progress(&letters);
                println!("\nCongrats! You won! ☺");
                break;
            }
            GameProgress::Lost => {
                draw_stick_man(&stick_man);
                println!("\nYou lost! ☹");
                break;
            }
        }

    }
}
