fn concat_char(c: char, n: i32) -> String {
    (0..n).map(|_x| c).collect()
}

fn diamond_line(number_of_asteriks: i32, number_of_padding_spaces: i32) -> String {
    let spaces = concat_char(' ', number_of_padding_spaces);
    let asteriks = concat_char('*', number_of_asteriks);
    format!("{}{}{}\n", spaces, asteriks, spaces)
}

fn diamond(size: i32) -> Option<String> {
    if size <= 0 {
        None
    } else if size % 2 == 0 {
        None
    } else {
        let midpoint_index = (size - 1) / 2;

        let output: String = (0..size)
            .map(|line_number| {
                let number_of_padding_spaces = (line_number - midpoint_index).abs();
                let number_of_asteriks = size - number_of_padding_spaces * 2;

                diamond_line(number_of_asteriks, number_of_padding_spaces)
            })
            .collect();
        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::diamond;

    #[test]
    fn it_works_for_even_inputs() {
        assert_eq!(diamond(2), None);
        assert_eq!(diamond(4), None);
        assert_eq!(diamond(60), None);
    }

    #[test]
    fn it_works_for_negative_inputs() {
        assert_eq!(diamond(-2), None);
        assert_eq!(diamond(-5), None);
        assert_eq!(diamond(-11), None);
    }

    #[test]
    fn it_works_for_zero() {
        // This is not defined in the spec
        assert_eq!(diamond(0), None);
    }

    #[test]
    fn a_single_asterik_is_a_basic_diamond() {
        let expected_output = "*\n".to_string();
        assert_eq!(diamond(1), Some(expected_output));
    }

    #[test]
    fn it_works_with_a_small_diamond() {
        let expected_output = " * \n***\n * \n".to_string();
        assert_eq!(diamond(3), Some(expected_output));
    }

    #[test]
    fn it_works_with_a_large_diamond() {
        let expected_output = "     *     \n    ***    \n   *****   \n  *******  \n ********* \n***********\n ********* \n  *******  \n   *****   \n    ***    \n     *     \n".to_string();
        assert_eq!(diamond(11), Some(expected_output));
    }
}
