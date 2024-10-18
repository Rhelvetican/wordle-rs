use ansi_term::Colour;

use crate::IoController;

pub struct ResultDisplayer {
    answer: &'static str,
    guessed: String,
}

impl ResultDisplayer {
    pub fn new(answer: &'static str) -> Self {
        Self {
            answer,
            guessed: String::new(),
        }
    }

    pub fn guess(&mut self, guess: String) {
        self.guessed = guess;
    }

    pub fn win(&self) -> bool {
        self.guessed == self.answer
    }

    pub fn display(&self, io: &mut IoController) {
        self.guessed
            .chars()
            .zip(self.answer.chars())
            .for_each(|(g, a)| {
                if g == a {
                    print!("{}", Colour::Green.paint(g.to_string()));
                } else if self.answer.contains(g) {
                    print!("{}", Colour::Yellow.paint(g.to_string()));
                } else {
                    print!("{}", Colour::Red.paint(g.to_string()));
                }
            });
        io.flush().unwrap();
    }
}
