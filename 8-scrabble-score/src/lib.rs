type Error = &'static str;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use regex::Regex;

enum ScoreModifier {
    Double,
    Triple,
}

impl ScoreModifier {
    fn multiplier(&self) -> u32 {
        match self {
            ScoreModifier::Double => 2,
            ScoreModifier::Triple => 3,
        }
    }
}

struct Letter {
    c: char,
    score_modifier: Option<ScoreModifier>,
    blank_tile: bool,
}

impl Letter {
    fn score(&self) -> u32 {
        lazy_static! {
            static ref LETTER_SCORES: HashMap<char, u32> = {
                let mut m = HashMap::new();
                m.insert('a', 1);
                m.insert('b', 3);
                m.insert('c', 3);
                m.insert('d', 2);
                m.insert('e', 1);
                m.insert('f', 4);
                m.insert('g', 2);
                m.insert('h', 4);
                m.insert('i', 1);
                m.insert('j', 8);
                m.insert('k', 5);
                m.insert('l', 1);
                m.insert('m', 3);
                m.insert('n', 1);
                m.insert('o', 1);
                m.insert('p', 3);
                m.insert('q', 10);
                m.insert('r', 1);
                m.insert('s', 1);
                m.insert('t', 1);
                m.insert('u', 1);
                m.insert('v', 4);
                m.insert('w', 4);
                m.insert('x', 8);
                m.insert('y', 4);
                m.insert('z', 10);
                m
            };
        }

        if self.blank_tile {
            0
        } else {
            let raw_score = LETTER_SCORES
                .get(&self.c)
                .expect(&format!("This is not a scorable letter: {}", self.c))
                .clone();

            match &self.score_modifier {
                Some(m) => raw_score * m.multiplier(),
                None => raw_score,
            }
        }
    }

    fn letters_from_string(word: &str) -> Vec<Self> {
        lazy_static! {
            static ref LETTER_REGEX: Regex = Regex::new("([a-zA-Z])(\\*{0,2})(\\^?)").unwrap();
        }

        LETTER_REGEX
            .captures_iter(word)
            .map(|x| {
                let letter = x.get(1).unwrap().as_str();
                let multiplier = x.get(2).unwrap().as_str();

                let score_modifier = if multiplier == "" {
                    None
                } else if multiplier == "*" {
                    Some(ScoreModifier::Double)
                } else if multiplier == "**" {
                    Some(ScoreModifier::Triple)
                } else {
                    unreachable!("We shouldn't be able to reach this due to the regex we are using")
                };

                let blank_tile = x.get(3).unwrap().as_str() == "^";

                Letter {
                    c: letter.chars().next().unwrap(),
                    score_modifier,
                    blank_tile,
                }
            })
            .collect()
    }
}

impl ScoreModifier {
    fn from_word_modifier_string(s: &str) -> Vec<Self> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new("\\(([td])\\)").unwrap();
        }
        REGEX
            .captures_iter(s)
            .map(|x| {
                let letter = x.get(1).unwrap();
                println!("{:?}", x);

                match letter.as_str() {
                    "d" => ScoreModifier::Double,
                    "t" => ScoreModifier::Triple,
                    _ => panic!("Unreachable code block"),
                }
            })
            .collect()
    }
}

struct Word {
    letters: Vec<Letter>,
    modifiers: Vec<ScoreModifier>,
}

impl Word {
    fn from_string(word: &str) -> Result<Self, Error> {
        lazy_static! {
            static ref WORD_REGEX: Regex =
                Regex::new("^((?:[a-z]\\*{0,2}\\^?)*)((?:\\([td]\\))*)$").unwrap();
        }

        let trimmed_word = word.trim().to_ascii_lowercase();

        if WORD_REGEX.is_match(&trimmed_word) {
            let capture = WORD_REGEX.captures(&trimmed_word).unwrap();

            let letters = Letter::letters_from_string(capture.get(1).unwrap().as_str());
            let word_modifier_capture = capture.get(2);

            let modifiers = match word_modifier_capture {
                Some(c) => ScoreModifier::from_word_modifier_string(c.as_str()),
                None => vec![],
            };

            Ok(Word { letters, modifiers })
        } else {
            Err("This is not a valid scrabble word")
        }
    }

    fn score(&self) -> u32 {
        self.letters_score() * self.multiplier()
    }

    fn letters_score(&self) -> u32 {
        self.letters.iter().map(|l| l.score()).sum()
    }

    fn multiplier(&self) -> u32 {
        self.modifiers.iter().fold(1, |acc, x| acc * x.multiplier())
    }
}

pub fn scrabble_score(word: &str) -> Result<u32, Error> {
    Ok(Word::from_string(word)?.score())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_does_not_work_with_invalid_chars() {
        assert_eq!(
            scrabble_score("t&@%&est"),
            Err("This is not a valid scrabble word")
        );
        assert_eq!(
            scrabble_score("123"),
            Err("This is not a valid scrabble word")
        );
    }

    #[test]
    fn it_works_for_words_without_letter_modifiers() -> Result<(), Error> {
        assert_eq!(scrabble_score("test")?, 4);
        assert_eq!(scrabble_score("hello")?, 8);
        assert_eq!(scrabble_score("quiz")?, 22);

        Ok(())
    }

    #[test]
    fn it_works_for_mixed_case_words_without_modifiers() -> Result<(), Error> {
        assert_eq!(scrabble_score("tESt")?, 4);
        assert_eq!(scrabble_score("heLLO")?, 8);
        assert_eq!(scrabble_score("quIz")?, 22);

        Ok(())
    }

    #[test]
    fn it_works_with_double_letter_modifiers() -> Result<(), Error> {
        assert_eq!(scrabble_score("t*e*s*t*")?, 8);
        assert_eq!(scrabble_score("h*ello")?, 12);
        assert_eq!(scrabble_score("q*u*iz")?, 33);

        Ok(())
    }

    #[test]
    fn it_works_with_blank_letter_modifiers() -> Result<(), Error> {
        assert_eq!(scrabble_score("t**^e**s*t*")?, 7);
        assert_eq!(scrabble_score("h**^ello")?, 4);
        assert_eq!(scrabble_score("q*u*iz**^")?, 23);

        Ok(())
    }

    #[test]
    fn it_works_for_modified_words_without_letter_modifiers() -> Result<(), Error> {
        assert_eq!(scrabble_score("test(t)")?, 12);
        assert_eq!(scrabble_score("hello(d)")?, 16);
        assert_eq!(scrabble_score("quiz(d)(d)")?, 88);

        Ok(())
    }

    #[test]
    fn it_works_with_triple_letter_modifiers() -> Result<(), Error> {
        assert_eq!(scrabble_score("t**e**s*t*")?, 10);
        assert_eq!(scrabble_score("h**ello")?, 16);
        assert_eq!(scrabble_score("q*u*iz**")?, 53);

        Ok(())
    }

    #[test]
    fn it_works_with_mixed_word_and_letter_modifiers() -> Result<(), Error> {
        assert_eq!(scrabble_score("t**e**s*t*(d)")?, 20);
        assert_eq!(scrabble_score("h**ello(d)(d)")?, 64);
        assert_eq!(scrabble_score("q*u*iz**^(t)")?, 69);

        Ok(())
    }
}
