fn decompose_recursive(goal: u32, i: u32, curr: Vec<u32>) -> Option<Vec<u32>> {
    if goal == 0 {
        return Some(curr);
    }

    if i <= 0 {
        return None;
    }

    if goal >= i.pow(2) {
        let mut next = curr.clone();
        next.push(i);
        let attempt = decompose_recursive(goal - i.pow(2), i - 1, next);

        if attempt.is_some() {
            return attempt;
        }
    }

    let attempt2 = decompose_recursive(goal, i - 1, curr);
    if attempt2.is_some() {
        return attempt2;
    }

    None
}

pub fn decompose(n: u32) -> Option<Vec<u32>> {
    let reverse_vec = decompose_recursive(n.pow(2), n - 1, vec![])?;
    let vec = reverse_vec.iter().rev().cloned().collect();
    Some(vec)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_for_a_nonexistant_example() {
        assert_eq!(decompose(4), None);
    }

    #[test]
    fn it_works_for_the_dev_to_example() {
        assert_eq!(decompose(11), Some(vec![1, 2, 4, 10]));
    }

    #[test]
    fn it_works_for_50() {
        assert_eq!(decompose(50), Some(vec![1, 3, 5, 8, 49]));
    }
}
