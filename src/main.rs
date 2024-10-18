use std::io::{stdin, stdout, Stdin, Stdout, Write};

use ansi_term::enable_ansi_support;
use rand::{seq::IteratorRandom, thread_rng};

mod display;
mod util;

use display::ResultDisplayer;
use util::Result;

const LIST: &str = include_str!("../dict.txt");

fn main() -> Result<()> {
    enable_ansi_support().map_err(|e| e.to_string())?;

    let mut io = IoController::new();
    let word = get_word().ok_or("Failed to get word.")?;

    let mut guess = ResultDisplayer::new(word);

    let mut tries = 6u8;

    loop {
        let guessed = io.input("Guess a word")?;
        guess.guess(guessed);
        guess.display(&mut io);

        if guess.win() {
            println!("You won!");
            break;
        } else {
            tries -= 1;
            println!(
                "Incorrect answer. {} {} left",
                tries,
                if tries == 1 { "try" } else { "tries" }
            );
            if tries == 0 {
                println!("You lost! The word was: {}", word);
                break;
            }
        }
    }

    Ok(())
}

fn get_word() -> Option<&'static str> {
    let mut rng = thread_rng();
    LIST.lines().map(|s| s.trim()).choose(&mut rng)
}

struct IoController {
    pub stdin: Stdin,
    pub stdout: Stdout,
}

impl IoController {
    fn new() -> Self {
        Self {
            stdin: stdin(),
            stdout: stdout(),
        }
    }

    pub fn input(&mut self, msg: &str) -> Result<String> {
        print!("{}: ", msg);
        self.stdout.flush()?;

        let mut buf = String::new();
        self.stdin.read_line(&mut buf)?;

        Ok(buf.trim().to_string())
    }

    pub fn flush(&mut self) -> Result<()> {
        println!();
        self.stdout.flush()?;
        Ok(())
    }
}
