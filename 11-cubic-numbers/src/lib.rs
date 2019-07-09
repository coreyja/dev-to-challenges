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

fn find_next_digit(vec: &Vec<char>, start: usize) -> Option<usize> {
    let mut cur = start;
    while cur < vec.len() && !vec.get(cur).unwrap().is_digit(10) {
        cur += 1;
    }

    if cur < vec.len() {
        Some(cur)
    } else {
        None
    }
}

fn find_last_digit(vec: &Vec<char>, start: usize) -> usize {
    let mut cur = start;
    while cur < vec.len() && vec.get(cur).unwrap().is_digit(10) {
        cur += 1;
    }

    cur
}

pub fn find_cubics(s: &str) -> Vec<u32> {
    let chars: Vec<_> = s.chars().collect();
    let mut cubics = vec![];

    let mut next_digit_index = find_next_digit(&chars, 0);
    while next_digit_index.is_some() {
        let start = next_digit_index.unwrap();
        let end = find_last_digit(&chars, start);

        let num: u32 = (&s[start..end]).parse().unwrap();
        if is_cubic(num) {
            cubics.push(num);
        }

        next_digit_index = find_next_digit(&chars, end + 1);
    }

    cubics
}

pub fn all_cubics() -> Vec<u32> {
    (0..1000).filter(|x| is_cubic(*x)).collect()
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
        assert_eq!(find_cubics("aqdf& 0 1 xyz 153 777.777"), vec![0, 1, 153]);
        assert_eq!(find_cubics("370&371h xyz 15 407.777"), vec![370, 371, 407]);
    }

    #[test]
    fn can_find_all_cubics() {
        assert_eq!(all_cubics(), vec![0, 1, 153, 370, 371, 407]);
    }
}
