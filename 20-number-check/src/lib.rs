fn is_prime(num: u64) -> bool {
    if num == 0 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}

pub fn number_property(i: i64) -> (bool, bool, bool) {
    let is_even = i % 2 == 0;
    let is_divisible_by_ten = i % 10 == 0;

    let is_prime = if i > 0 { is_prime(i as u64) } else { false };

    (is_prime, is_even, is_divisible_by_ten)
}

#[cfg(test)]
mod tests {
    use crate::number_property;

    #[test]
    fn it_works_for_the_examples() {
        assert_eq!(number_property(7), (true, false, false));
        assert_eq!(number_property(10), (false, true, true));
    }

    #[test]
    fn it_works_for_negative_examples() {
        assert_eq!(number_property(-7), (false, false, false));
        assert_eq!(number_property(-10), (false, true, true));
    }
}
