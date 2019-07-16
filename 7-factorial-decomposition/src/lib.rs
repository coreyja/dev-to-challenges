use std::collections::HashMap;

fn prime_decomposition(n: u32) -> Vec<u32> {
    let mut output = vec![];
    let mut curr = n;

    for i in 2..(n + 1) {
        while curr % i == 0 {
            output.push(i);
            curr = curr / i;
        }
    }

    output
}

fn count_occurances(list: Vec<u32>) -> Vec<(u32, u32)> {
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for x in list {
        let count = counts.entry(x).or_insert(0);
        *count += 1;
    }

    counts
        .keys()
        .map(|key| (*key, *counts.get(key).unwrap()))
        .collect()
}

pub fn factorial_decomposition(n: u32) -> Vec<(u32, u32)> {
    if n == 0 || n == 1 {
        vec![(1, 1)]
    } else {
        let mut factors: Vec<u32> = vec![];

        for i in 2..(n + 1) {
            factors.append(&mut prime_decomposition(i));
        }

        let mut output = count_occurances(factors);
        output.sort();
        output
    }
}

// "n = 12; decomp(12) -> \"2^10 * 3^5 * 5^2 * 7 * 11\""
pub fn fac_decomp_string(n: u32) -> String {
    fn format_single_factorial(x: &(u32, u32)) -> String {
        if x.1 == 1 {
            format!("{}", x.0)
        } else {
            format!("{}^{}", x.0, x.1)
        }
    }

    let factorial_decomposition = factorial_decomposition(n);
    let fac_string = factorial_decomposition
        .iter()
        .map(format_single_factorial)
        .collect::<Vec<String>>()
        .join(" * ");

    format!(
        "n = {n}; decomp({n}) -> \"{decomp}\"",
        n = n,
        decomp = fac_string
    )
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn sorted_counted_prime_decomp(n: u32) -> Vec<(u32, u32)> {
        let mut output: Vec<(u32, u32)> = count_occurances(prime_decomposition(n));
        output.sort();
        output
    }

    #[test]
    fn prime_decomposition_works_for_prime_numbers() {
        assert_eq!(sorted_counted_prime_decomp(2), vec![(2, 1)]);
        assert_eq!(sorted_counted_prime_decomp(3), vec![(3, 1)]);
        assert_eq!(sorted_counted_prime_decomp(5), vec![(5, 1)]);
        assert_eq!(sorted_counted_prime_decomp(7), vec![(7, 1)]);
        assert_eq!(sorted_counted_prime_decomp(11), vec![(11, 1)]);
    }

    #[test]
    fn prime_decomposition_works_for_factors_of_2() {
        assert_eq!(sorted_counted_prime_decomp(2), vec![(2, 1)]);
        assert_eq!(sorted_counted_prime_decomp(4), vec![(2, 2)]);
        assert_eq!(sorted_counted_prime_decomp(8), vec![(2, 3)]);
        assert_eq!(sorted_counted_prime_decomp(16), vec![(2, 4)]);
    }

    #[test]
    fn prime_decomposition_works_for_more_complex_numbers() {
        assert_eq!(sorted_counted_prime_decomp(10), vec![(2, 1), (5, 1)]);
        assert_eq!(sorted_counted_prime_decomp(6), vec![(2, 1), (3, 1)]);
        assert_eq!(sorted_counted_prime_decomp(15), vec![(3, 1), (5, 1)]);
    }

    #[test]
    fn simple_factorial_decomposition() {
        assert_eq!(factorial_decomposition(2), vec![(2, 1)]);
        assert_eq!(factorial_decomposition(3), vec![(2, 1), (3, 1)]);
        assert_eq!(factorial_decomposition(4), vec![(2, 3), (3, 1)]);
        assert_eq!(factorial_decomposition(5), vec![(2, 3), (3, 1), (5, 1)]);
    }

    #[test]
    fn factorial_decomposition_of_edge_cases() {
        assert_eq!(factorial_decomposition(1), vec![(1, 1)]);
        assert_eq!(factorial_decomposition(0), vec![(1, 1)]);
    }

    #[test]
    fn factorial_decomposition_of_examples() {
        assert_eq!(
            factorial_decomposition(12),
            vec![(2, 10), (3, 5), (5, 2), (7, 1), (11, 1)]
        );
        assert_eq!(
            factorial_decomposition(22),
            vec![
                (2, 19),
                (3, 9),
                (5, 4),
                (7, 3),
                (11, 2),
                (13, 1),
                (17, 1),
                (19, 1)
            ]
        );
        assert_eq!(
            factorial_decomposition(25),
            vec![
                (2, 22),
                (3, 10),
                (5, 6),
                (7, 3),
                (11, 2),
                (13, 1),
                (17, 1),
                (19, 1),
                (23, 1),
            ]
        );
    }

    #[test]
    fn formatting_works() {
        assert_eq!(fac_decomp_string(0), "n = 0; decomp(0) -> \"1\"");
        assert_eq!(fac_decomp_string(1), "n = 1; decomp(1) -> \"1\"");
        assert_eq!(
            fac_decomp_string(12),
            "n = 12; decomp(12) -> \"2^10 * 3^5 * 5^2 * 7 * 11\""
        );
        assert_eq!(
            fac_decomp_string(22),
            "n = 22; decomp(22) -> \"2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19\""
        );
        assert_eq!(
            fac_decomp_string(25),
            "n = 25; decomp(25) -> \"2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23\""
        );
    }
}
