#[macro_use]
extern crate lazy_static;

pub fn triple_double(num1: u64, num2: u64) -> bool {
    use regex::Regex;

    lazy_static! {
        static ref first_num_regex: Regex = Regex::new(r"(\d{3})").unwrap();
    }

    let string1 = num1.to_string();
    let string2 = num2.to_string();

    for i in first_num_regex.captures_iter(&string1) {
        let regex_match = i.get(0).unwrap().as_str();

        if regex_match[0..1] == regex_match[1..2] && regex_match[1..2] == regex_match[2..3] {
            let second_layer_regex = Regex::new(&regex_match[0..2]).unwrap();

            if second_layer_regex.is_match(&string2) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_for_true_examples() {
        assert_eq!(triple_double(451999277, 41177722899), true);
        assert_eq!(triple_double(666789, 12345667), true);
    }

    #[test]
    fn it_for_false_examples() {
        assert_eq!(triple_double(1222345, 12345), false);
        assert_eq!(triple_double(12345, 12345), false);
    }
}
