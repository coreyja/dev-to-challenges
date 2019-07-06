#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use regex::Regex;

enum LetterScoreModifier {
    Single,
    Double,
    Triple,
}

struct Letter {
    c: char,
    score_modifier: LetterScoreModifier,
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

        let raw_score = LETTER_SCORES
            .get(&self.c)
            .expect(&format!("This is not a scorable letter: {}", self.c))
            .clone();

        let muptilpier = match self.score_modifier {
            LetterScoreModifier::Single => 1,
            LetterScoreModifier::Double => 2,
            LetterScoreModifier::Triple => 3,
        };

        raw_score * muptilpier
    }
}

fn string_to_letters(s: &str) -> Vec<Letter> {
    lazy_static! {
        static ref LETTER_REGEX: Regex = Regex::new("[a-zA-Z]\\*{0,2}").unwrap();
    }
    LETTER_REGEX
        .captures_iter(s)
        .map(|x| {
            let only_capture = x.get(0).unwrap().as_str();

            let score_modifier = if only_capture.len() == 1 {
                LetterScoreModifier::Single
            } else if only_capture.len() == 2 {
                LetterScoreModifier::Double
            } else if only_capture.len() == 3 {
                LetterScoreModifier::Triple
            } else {
                panic!("We shouldn't be able to reach this due to the regex we are using")
            };

            Letter {
                c: only_capture.chars().next().unwrap(),
                score_modifier,
            }
        })
        .collect()
}

pub fn scrabble_score(word: &str) -> u32 {
    string_to_letters(&word.trim().to_ascii_lowercase())
        .iter()
        .map(|l| l.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_for_words_without_modifiers() {
        assert_eq!(scrabble_score("test"), 4);
        assert_eq!(scrabble_score("hello"), 8);
        assert_eq!(scrabble_score("quiz"), 22);
    }

    #[test]
    fn it_works_for_mixed_case_words_without_modifiers() {
        assert_eq!(scrabble_score("tESt"), 4);
        assert_eq!(scrabble_score("heLLO"), 8);
        assert_eq!(scrabble_score("quIz"), 22);
    }

    #[test]
    fn it_works_with_double_letter_modifiers() {
        assert_eq!(scrabble_score("t*e*s*t*"), 8);
        assert_eq!(scrabble_score("h*ello"), 12);
        assert_eq!(scrabble_score("q*u*iz"), 33);
    }

    #[test]
    fn it_works_with_triple_letter_modifiers() {
        assert_eq!(scrabble_score("t**e**s*t*"), 10);
        assert_eq!(scrabble_score("h**ello"), 16);
        assert_eq!(scrabble_score("q*u*iz**"), 53);
    }
}
