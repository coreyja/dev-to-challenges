pub fn spin_words(input: &str) -> String {
    input
        .split(" ")
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_for_words_shorter_than_4_chars() {
        assert_eq!(spin_words("Test"), "Test".to_string());
        assert_eq!(spin_words("Te"), "Te".to_string());
        assert_eq!(spin_words("ant"), "ant".to_string());
        assert_eq!(
            spin_words("lots of tiny word"),
            "lots of tiny word".to_string()
        );
    }

    #[test]
    fn it_words_for_the_example() {
        assert_eq!(
            spin_words("Stop gninnipS My sdroW"),
            "Stop Spinning My Words".to_string()
        );
    }

    #[test]
    fn it_words_with_space_padded_strings() {
        assert_eq!(
            spin_words(" Stop gninnipS My sdroW "),
            " Stop Spinning My Words ".to_string()
        );
    }
}
