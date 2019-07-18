fn find_index_iter(num_people: u64, soda_num: u64) -> u64 {
    let mut current_group_size = num_people;
    let mut total_size = num_people;
    let mut total_groups = 1;

    while soda_num > total_size {
        current_group_size += num_people;
        total_size += current_group_size;
        total_groups += 1;
    }

    let local_group_index = current_group_size - (total_size - soda_num) - 1;

    local_group_index / (total_groups + 0)
}

fn find_index_math(num_people: usize, soda_num: u64) -> usize {
    let soda_index = soda_num - 1;
    let number_of_groups =
        ((-1.0 + (1.0 + (8.0 / num_people as f64 * soda_num as f64)).sqrt()) / 2.0).ceil();

    let num_before_last_group =
        num_people as f64 / 2.0 * number_of_groups * (number_of_groups - 1.0);

    let local_group_index = soda_index as f64 - num_before_last_group;

    let ans = ((local_group_index / number_of_groups).floor()) as usize;

    ans
}

pub fn whos_soda<'a>(people: &'a [&str], soda_num: u64) -> &'a str {
    people[find_index_math(people.len(), soda_num)]
    // people[find_index_iter(people.len() as u64, soda_num) as usize]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_for_first_few_groups_with_3_names() {
        assert_eq!(whos_soda(&["a", "b", "c"], 1), "a");
        assert_eq!(whos_soda(&["a", "b", "c"], 2), "b");
        assert_eq!(whos_soda(&["a", "b", "c"], 3), "c");
        assert_eq!(whos_soda(&["a", "b", "c"], 4), "a");
        assert_eq!(whos_soda(&["a", "b", "c"], 5), "a");
        assert_eq!(whos_soda(&["a", "b", "c"], 6), "b");
        assert_eq!(whos_soda(&["a", "b", "c"], 7), "b");
        assert_eq!(whos_soda(&["a", "b", "c"], 8), "c");
        assert_eq!(whos_soda(&["a", "b", "c"], 9), "c");
    }

    #[test]
    fn it_for_first_few_groups_with_5_names() {
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 1), "a");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 2), "b");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 3), "c");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 4), "d");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 5), "e");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 6), "a");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 7), "a");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 8), "b");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 9), "b");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 10), "c");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 11), "c");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 12), "d");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 13), "d");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 14), "e");
        assert_eq!(whos_soda(&["a", "b", "c", "d", "e"], 15), "e");
    }
}

pub fn main() {
    let mut current_timing_iter = 0.0;
    let mut current_timing_math = 0.0;
    for people in 1..10 {
        for i in 1..2000000 {
            use std::time::Instant;
            let now = Instant::now();

            let math_ans = find_index_math(people as usize, i);

            let elapsed = now.elapsed();
            let math_sec =
                (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);

            let now = Instant::now();

            let iter_ans = find_index_iter(people, i) as usize;

            let elapsed = now.elapsed();
            let iter_sec =
                (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);

            current_timing_math += math_sec;
            current_timing_iter += iter_sec;

            assert_eq!(iter_ans, math_ans, "{}x{}", people, i);
        }
    }

    println!(
        "Math Took: {} Iter Took: {}",
        current_timing_math, current_timing_iter
    );
}
