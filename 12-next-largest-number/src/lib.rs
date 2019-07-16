pub fn next_largest(n: u32) -> Option<u32> {
    let s = n.to_string();

    let mut chars: Vec<_> = s.chars().collect();

    for i in (0..chars.len() - 1).rev() {
        let first = chars[i].to_digit(10).unwrap();
        let second = chars[i + 1].to_digit(10).unwrap();

        if second > first {
            chars.swap(i, i + 1);
            let s: String = chars.iter().collect();

            return Some(s.parse().unwrap());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_for_examples_that_have_no_largest() {
        assert_eq!(next_largest(4), None);
        assert_eq!(next_largest(100), None);
        assert_eq!(next_largest(9876), None);
    }

    #[test]
    fn it_works_for_the_examples() {
        assert_eq!(next_largest(12), Some(21));
        assert_eq!(next_largest(2019), Some(2091));
        assert_eq!(next_largest(513), Some(531));
    }

    #[test]
    fn it_works_for_large_numebrs() {
        assert_eq!(next_largest(36852367), Some(36852376));
        assert_eq!(next_largest(123456789), Some(123456798));
        assert_eq!(next_largest(5010), Some(5100));
    }
}
