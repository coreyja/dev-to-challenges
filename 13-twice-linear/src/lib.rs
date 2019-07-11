fn twice_linear(size: usize) -> Vec<u32> {
    let mut processed: Vec<u32> = vec![];
    let mut unprocessed: Vec<u32> = vec![];

    let mut current = 1;
    while processed.len() < size {
        processed.push(current);

        // This block of code is NOT the most effiecient
        // I should switch to a data store that can push/pop
        // from both ends effieciently, as to avoid needing
        // the sort AND the revserse
        unprocessed.push(current * 2 + 1);
        unprocessed.push(current * 3 + 1);
        unprocessed.sort();
        unprocessed = unprocessed.iter().rev().cloned().collect();

        current = unprocessed.pop().unwrap();
    }

    processed
}

pub fn twice_linear_at(u: usize) -> u32 {
    twice_linear(u + 1).pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works_for_the_base_cases() {
        assert_eq!(twice_linear_at(0), 1);
        assert_eq!(twice_linear_at(1), 3);
        assert_eq!(twice_linear_at(2), 4);
    }

    #[test]
    fn it_works_for_the_example() {
        assert_eq!(twice_linear_at(3), 7);
        assert_eq!(twice_linear_at(4), 9);
        assert_eq!(twice_linear_at(5), 10);
        assert_eq!(twice_linear_at(6), 13);
        assert_eq!(twice_linear_at(7), 15);
        assert_eq!(twice_linear_at(8), 19);
        assert_eq!(twice_linear_at(9), 21);
        assert_eq!(twice_linear_at(10), 22);
        assert_eq!(twice_linear_at(11), 27);
    }

    #[test]
    fn it_can_also_return_the_vec() {
        assert_eq!(
            twice_linear(12),
            vec![1, 3, 4, 7, 9, 10, 13, 15, 19, 21, 22, 27]
        );
    }
}
