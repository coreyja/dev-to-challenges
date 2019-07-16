fn foobar(goal: u32, starting_point: u32, curr: Vec<u32>) -> Option<Vec<u32>> {
    for i in (1..=starting_point).rev() {
        if goal >= i.pow(2) {
            let mut next = curr.clone();
            next.push(i);
            let test = foobar(goal - i.pow(2), starting_point - 1, next);

            if test.is_some() {
                return test;
            }
        }
    }

    None
}

fn decompose_starting_from(n: u32, starting_point: u32, curr: Vec<u32>) -> Option<Vec<u32>> {
    let mut curr_total = n.pow(2);
    let mut output = curr;

    for i in (1..=starting_point).rev() {
        if curr_total >= i.pow(2) {
            curr_total -= i.pow(2);
            output.push(i);
        }
    }

    if curr_total == 0 {
        output.sort();
        Some(output)
    } else {
        None
    }
}

pub fn decompose(n: u32) -> Option<Vec<u32>> {
    // for i in (1..n).rev() {
    //     let x = decompose_starting_from(n, i, vec![]);

    //     if x.is_some() {
    //         return x;
    //     }
    // }

    // None
    foobar(n.pow(2), n - 1, vec![])
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
