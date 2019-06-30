#[macro_use]
extern crate lazy_static;

pub fn vowel_count(some_string: &str) -> u32 {
    lazy_static! {
        static ref VOWELS: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    }

    let mut counter = 0;
    for c in some_string.chars() {
        if VOWELS.contains(&c) {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use crate::vowel_count;

    #[test]
    fn it_works_with_an_empty_string() {
        assert_eq!(vowel_count(""), 0);
    }

    #[test]
    fn it_works_non_vowel_strings() {
        assert_eq!(vowel_count("d"), 0);
        assert_eq!(vowel_count("drthpCVM  *&^"), 0);
        assert_eq!(vowel_count("1234567890!@#$%^&*()--__+="), 0);
        assert_eq!(vowel_count("NPlkv.,<>?/"), 0);
    }

    #[test]
    fn it_works_for_strings_of_just_vowels() {
        assert_eq!(vowel_count("a"), 1);
        assert_eq!(vowel_count("A"), 1);
        assert_eq!(vowel_count("AaeEiIoOuU"), 10);
        assert_eq!(vowel_count("eoiuioEAUIAEoieaiAoe"), 20);
    }

    #[test]
    fn it_works_for_mixed_strings() {
        assert_eq!(vowel_count("deadpool"), 4);
        assert_eq!(
            vowel_count("This is just a sentence! With some words and symbols #$%"),
            13
        );
        assert_eq!(vowel_count("TESTING OUT YELLING WITH ALL CAPS"), 9);
        assert_eq!(
            vowel_count("!@#      \nThis is THE MOST COMPLICATED test SoO farrrr"),
            12
        );
    }
}
