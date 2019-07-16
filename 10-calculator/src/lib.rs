fn has_any_operands(input: &str) -> bool {
    // input.contains("+") || input.contains("-") || input.contains("*") || input.contains("/")
    input.contains("+")
}
pub fn calculate(input: &str) -> Result<u32, std::num::ParseIntError> {
    if !has_any_operands(input) {
        return input.parse();
    }

    let plus_operands = input.split("+");
    Ok(plus_operands
        .map(|x| {
            let z = calculate(input);
            if z.is_err() {
                return z;
            }
            z.unwrap()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_for_single_numbers() {
        assert_eq!(calculate("2"), Ok(2));
        assert_eq!(calculate("5"), Ok(5));
        assert_eq!(calculate("11"), Ok(11));
    }

    #[test]
    fn it_works_for_single_addition() {
        assert_eq!(calculate("2 + 2"), Ok(4));
        assert_eq!(calculate("5 + 4"), Ok(9));
        assert_eq!(calculate("11 + 4"), Ok(15));
    }

    #[test]
    fn it_works_for_multiple_addition() {
        assert_eq!(calculate("2 + 2 + 2"), Ok(6));
        assert_eq!(calculate("5 + 4 + 3"), Ok(12));
        assert_eq!(calculate("11 + 4 + 1"), Ok(16));
    }

    #[test]
    fn it_works_for_mixed_addition_and_subtraction() {
        assert_eq!(calculate("2 + 2 - 2"), Ok(2));
        assert_eq!(calculate("5 - 4 + 3"), Ok(8));
        assert_eq!(calculate("11 - 4 + 1"), Ok(12));
    }

    #[test]
    fn it_works_for_the_dev_to_example() {
        assert_eq!(calculate("2 / 2 + 3 * 4 - 6"), Ok(7));
    }
}
