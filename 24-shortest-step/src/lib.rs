pub fn shortest_step_exhaustive_recurse(num: u64) -> Option<u64> {
    fn recursive_helper(cur: u64, goal: u64) -> Option<u64> {
        if cur == goal {
            Some(0)
        } else if cur > goal {
            None
        } else {
            [cur + 1, cur * 2]
                .iter()
                .filter_map(|next_try| recursive_helper(*next_try, goal))
                .map(|attempt| attempt + 1)
                .min()
        }
    }

    recursive_helper(1, num)
}

pub fn shortest_step_count_down(num: u64) -> Option<u64> {
    if num == 0 {
        None
    } else if num == 1 {
        Some(0)
    } else if num % 2 == 0 {
        Some(1 + shortest_step_count_down(num / 2)?)
    } else {
        Some(1 + shortest_step_count_down(num - 1)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn all_algos_work_for_three() {
        assert_eq!(shortest_step_exhaustive_recurse(3), Some(2));
        assert_eq!(shortest_step_count_down(3), Some(2));
    }

    #[test]
    fn all_algos_work_for_twelve() {
        assert_eq!(shortest_step_exhaustive_recurse(12), Some(4));
        assert_eq!(shortest_step_count_down(12), Some(4));
    }

    #[test]
    fn all_algos_match_up_to_200() {
        for i in 0..200 {
            assert_eq!(
                shortest_step_count_down(i),
                shortest_step_exhaustive_recurse(i),
                "Algos don't match for {}",
                i
            );
        }
    }
}
