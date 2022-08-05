use std::collections::HashSet;
use std::io;

use std::format;

use colored::Colorize;

const MAX_GUESSES: usize = 6;
const WORD_LENGTH: usize = 5;

pub struct Game {
    word: String,
    guesses: Vec<String>,
    current_guess: String,
    letters_used: HashSet<char>,
}

impl Game {
    pub fn new() -> Self {

        // Load the words into a Vector
        let word_dict = Game::load_word_dict();

        // Just get the second word for now
        let word = &word_dict[2].to_uppercase().to_owned();

        Self {
            word: word.to_owned(),
            guesses: Vec::new(),
            letters_used: HashSet::new(),
            current_guess: String::new(),
        }
    }

    pub fn wait_for_guess(&mut self) -> String{
        let mut guess = String::new();
        
        let mut valid_guess: bool = false;
        let mut sanitized_guess = String::new();

        while !valid_guess {
            guess = "".to_owned();

            println!("Guess: ({} More)", (MAX_GUESSES - self.guesses.len()));
            io::stdin().read_line(&mut guess).unwrap();

            // Sanitize the guess so we don't have rogue new lines etc
            sanitized_guess = guess.trim().to_uppercase().to_string();

            // Check that the guess is valid
            valid_guess = self.check_guess(sanitized_guess.clone());

        }

        // The guess was OK handle it
        self.handle_guess(sanitized_guess.clone());
        
        return sanitized_guess;
    }

    fn handle_guess(&mut self, guess: String) {
        // Push the current guess into the guesses vector
        self.guesses.push(guess.clone());

        // Add the letters used into the HashSet
        guess.chars().enumerate().for_each(|(_pos, c)| {
            self.letters_used.insert(c);
        });

        // Set the current guess
        self.current_guess = guess;
    }

    pub fn show_guesses(&mut self) {
        println!("Your Guesses");

        let guesses_remaining = MAX_GUESSES - self.guesses.len();
        
        for g in &self.guesses {
            let mut output = String::new();
            g.chars().enumerate().for_each(|(_pos, c)| {

                let formatted_string;
                
                if self.word.contains(c) {
                    if self.word.find(c) == Some(_pos) {
                        // Only show green if correct letter in right place
                        formatted_string = format!("[{}]", c.to_string().green().bold());
                    } else {
                        // Yellow if its in the word but wrong palce
                        formatted_string = format!("[{}]", c.to_string().yellow().bold()); 
                    }
                } else {
                    // Grey if not in word
                    formatted_string = format!("[{}]", c.to_string().dimmed());
                }

                output.push_str(&formatted_string);
            });
            println!("{}", output);
        }

        (0..guesses_remaining).for_each(|_n: usize| {
            println!("[ ][ ][ ][ ][ ]");
        });

        println!("\n");
    }

    pub fn check_guess(&mut self, guess: String) -> bool {
        
        if guess.len() != WORD_LENGTH {
            println!("Your guess is '{}' letters. It should have {} letters.", guess.len(), WORD_LENGTH);
            return false;
        }

        // Have we already had this guess?
        for g in &self.guesses {
            if g.to_string() == guess {
                println!("You have already guessed '{}'", guess);
                return false;
            }
        }

        return true;
    }
 
    pub fn game_over(&mut self) -> bool {
        // Game over id;
        // We have run out of guesses
        // We have got the right answer
        if &self.current_guess == &self.word {
            println!("Correct! You guessed the word correctly!");
            return true;
        } else if self.guesses.len() >= MAX_GUESSES {
            println!("You ran out of guesses!");
            return true;
        } else {
                return false;
            }   
        }

    fn load_word_dict() -> Vec<String> {
        let mut dict = Vec::new();
        println!("Loading word dict...");

        dict.push("Hello".to_owned());
        dict.push("Brown".to_owned());
        dict.push("Child".to_owned());
        dict.push("Owner".to_owned());
        dict.push("Thing".to_owned());

        return dict;
    }
}