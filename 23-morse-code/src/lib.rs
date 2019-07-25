#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref MORSE_TO_CHAR: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert(".-", 'A');
        m.insert("-...", 'B');
        m.insert("-.-.", 'C');
        m.insert("-..", 'D');
        m.insert(".", 'E');
        m.insert("..-.", 'F');
        m.insert("--.", 'G');
        m.insert("....", 'H');
        m.insert("..", 'I');
        m.insert(".---", 'J');
        m.insert("-.-", 'K');
        m.insert(".-..", 'L');
        m.insert("--", 'M');
        m.insert("-.", 'N');
        m.insert("---", 'O');
        m.insert(".--.", 'P');
        m.insert("--.-", 'Q');
        m.insert(".-.", 'R');
        m.insert("...", 'S');
        m.insert("-", 'T');
        m.insert("..-", 'U');
        m.insert("...-", 'V');
        m.insert(".--", 'W');
        m.insert("-..-", 'X');
        m.insert("-.--", 'Y');
        m.insert("--..", 'Z');
        m.insert("-----", '0');
        m.insert(".----", '1');
        m.insert("..---", '2');
        m.insert("...--", '3');
        m.insert("....-", '4');
        m.insert(".....", '5');
        m.insert("-....", '6');
        m.insert("--...", '7');
        m.insert("---..", '8');
        m.insert("----.", '9');
        m
    };
}

lazy_static! {
    static ref CHAR_TO_MORSE: HashMap<char, &'static str> = {
        let mut m = HashMap::new();

        for (morse, c) in MORSE_TO_CHAR.iter() {
            m.insert(*c, *morse);
        }

        m
    };
}

pub fn encode_morse(morse_code: &str) -> String {
    morse_code
        .chars()
        .map(|c| {
            CHAR_TO_MORSE
                .get(&c)
                .expect("Invalid character for morse code")
                .to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode_morse(morse_code: &str) -> String {
    morse_code
        .split_whitespace()
        .map(|word| {
            MORSE_TO_CHAR
                .get(word)
                .expect("Invalid morse code")
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_to_decode_example() {
        assert_eq!(
            decode_morse(".... . -.-- .--- ..- -.. ."),
            "HEYJUDE".to_string()
        );
    }

    #[test]
    fn it_works_to_encode_example() {
        assert_eq!(
            encode_morse("HEYJUDE"),
            ".... . -.-- .--- ..- -.. .".to_string()
        );
    }

    #[test]
    fn it_is_reversible() {
        let input = "SOMETESTSTRING";

        assert_eq!(decode_morse(&encode_morse(input)), input.to_string());
    }
}
