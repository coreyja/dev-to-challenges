pub fn vowel_count(some_string: &str) -> u32 {
    0
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
}
