pub fn is_cubic(n: u32) -> bool {
    if n >= 1000 {
        return false;
    }

    let mut sum = 0;
    let mut cur = n;

    while cur > 0 {
        sum += (cur % 10).pow(3);
        cur /= 10;
    }

    sum == n
}

pub fn find_cubics(s: &str) -> Vec<u32> {
    let chars: Vec<_> = s.chars().collect();
    let mut cubics = vec![];

    let mut i = 0;
    while i < chars.len() {
        if chars.get(i).unwrap().is_digit(10) {
            let mut j = i + 1;
            while j < chars.len() && chars.get(j).unwrap().is_digit(10) {
                j += 1;
            }

            let string_num = &s[i..j];
            let num: u32 = string_num.parse().unwrap();
            if is_cubic(num) {
                cubics.push(num);
            }

            i = j;
        }

        i += 1;
    }

    cubics
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_returns_false_for_most_numbers() {
        assert_eq!(is_cubic(2), false);
        assert_eq!(is_cubic(22), false);
        assert_eq!(is_cubic(500), false);
    }

    #[test]
    fn it_returns_false_for_numbers_over_999() {
        assert_eq!(is_cubic(1000), false);
        assert_eq!(is_cubic(5000), false);
    }

    #[test]
    fn it_works_for_the_examples() {
        assert_eq!(is_cubic(0), true);
        assert_eq!(is_cubic(1), true);
        assert_eq!(is_cubic(153), true);
    }

    #[test]
    fn find_cubics_works_too() {
        assert_eq!(find_cubics("aqdf& 0 1 xyz 153 777.777"), vec![0, 1, 153])
    }
}
