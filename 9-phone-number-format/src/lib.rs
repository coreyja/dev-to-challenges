#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidLength,
    NotSingleDigitInput,
}

pub fn format_phone_numer(numbers: &[u32]) -> Result<String, Error> {
    if numbers.len() != 10 {
        Err(Error::InvalidLength)
    } else if numbers.iter().any(|x| *x > 9) {
        Err(Error::NotSingleDigitInput)
    } else {
        Ok(format!(
            "({}{}{}) {}{}{}-{}{}{}{}",
            numbers[0],
            numbers[1],
            numbers[2],
            numbers[3],
            numbers[4],
            numbers[5],
            numbers[6],
            numbers[7],
            numbers[8],
            numbers[9],
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_for_a_valid_phone_number() {
        assert_eq!(
            format_phone_numer(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            Ok("(123) 456-7890".to_string())
        );
        assert_eq!(
            format_phone_numer(&[3, 2, 1, 5, 5, 5, 3, 1, 6, 3]),
            Ok("(321) 555-3163".to_string())
        );
    }

    #[test]
    fn it_errors_invalid_length_phone_number() {
        assert_eq!(
            format_phone_numer(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 5, 6, 3]),
            Err(Error::InvalidLength)
        );
        assert_eq!(
            format_phone_numer(&[1, 2, 3, 4, 5, 6, 7,]),
            Err(Error::InvalidLength)
        );
    }

    #[test]
    fn it_errors_for_non_single_digit_numbers() {
        assert_eq!(
            format_phone_numer(&[10, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            Err(Error::NotSingleDigitInput)
        );
    }
}
