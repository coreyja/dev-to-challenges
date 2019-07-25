fn pigify_word(word: &str) -> String {
    println!("{}", word);
    if word.chars().all(|x| x.is_alphanumeric()) {
        let mut chars = word.chars();
        let first_letter = chars.next().unwrap();
        let rest_of_letters: String = chars.collect();

        format!("{}{}ay", rest_of_letters, first_letter)
    } else if word.chars().all(|x| !x.is_alphanumeric()) {
        word.to_string()
    } else {
        let symbol_index = word.find(|c: char| !c.is_alphanumeric()).unwrap();
        let alphanumeric_index = word.find(|c: char| c.is_alphanumeric()).unwrap();

        let index = symbol_index.max(alphanumeric_index);

        [word[0..index].to_string(), word[index..].to_string()]
            .iter()
            .map(|x| pigify_word(x))
            .collect()
    }
}

pub fn pig_it(english_input: &str) -> String {
    english_input
        .split(" ")
        .map(pigify_word)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_without_punctuation() {
        assert_eq!(
            pig_it("Pig latin is cool"),
            "igPay atinlay siay oolcay".to_string()
        );
    }

    #[test]
    fn it_works_with_spaced_out_punctuation() {
        assert_eq!(pig_it("Hello world !"), "elloHay orldway !".to_string());
    }

    #[test]
    fn it_works_with_word_ending_punctuation() {
        assert_eq!(
            pig_it("Hello world! How is it going today?"),
            "elloHay orldway! owHay siay tiay oinggay odaytay?".to_string()
        );
    }

    #[test]
    fn it_works_with_word_mid_punctuation() {
        assert_eq!(
            pig_it("Hello world! How's it going today?"),
            "elloHay orldway! owHay'say tiay oinggay odaytay?".to_string()
        );
    }
}
