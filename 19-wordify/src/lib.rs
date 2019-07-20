type Error = &'static str;

fn digit_to_string(s: &str) -> Option<&'static str> {
    match s {
        "0" => Some(""),
        "1" => Some("one"),
        "2" => Some("two"),
        "3" => Some("three"),
        "4" => Some("four"),
        "5" => Some("five"),
        "6" => Some("six"),
        "7" => Some("seven"),
        "8" => Some("eight"),
        "9" => Some("nine"),
        _ => None,
    }
}

fn tens_digit_to_string(s: &str) -> Option<&'static str> {
    match s {
        "2" => Some("twenty"),
        "3" => Some("thirty"),
        "4" => Some("fourty"),
        "5" => Some("fifty"),
        "6" => Some("sixty"),
        "7" => Some("seventy"),
        "8" => Some("eighty"),
        "9" => Some("ninety"),
        _ => None,
    }
}

fn double_digit_to_string(s: &str) -> Option<String> {
    if &s[0..1] == "1" {
        Some(
            match s {
                "10" => "ten",
                "11" => "eleven",
                "12" => "twelve",
                "13" => "thirteen",
                "14" => "fourteen",
                "15" => "fifteen",
                "16" => "sixteen",
                "17" => "seventeen",
                "18" => "eighteen",
                "19" => "nineteen",
                _ => unreachable!("Can't get here"),
            }
            .to_string(),
        )
    } else {
        Some(
            format!(
                "{} {}",
                tens_digit_to_string(&s[0..1]).unwrap(),
                digit_to_string(&s[1..2]).unwrap()
            )
            .trim()
            .to_string(),
        )
    }
}

pub fn wordify(num: u32) -> Result<String, Error> {
    if num == 0 || num >= 1000 {
        return Err("Invalid Number");
    }

    let num_string = num.to_string();

    match num_string.len() {
        1 => Ok(digit_to_string(&num_string).unwrap().to_string()),
        2 => Ok(double_digit_to_string(&num_string).unwrap()),
        3 => Ok(format!(
            "{} hundred {}",
            digit_to_string(&num_string[0..1]).unwrap(),
            double_digit_to_string(&num_string[1..3]).unwrap()
        )
        .trim()
        .to_string()),
        _ => unreachable!("Already check for numbers larger"),
    }
}

#[cfg(test)]
mod tests {
    use crate::wordify;

    #[test]
    fn it_return_an_error_for_invalid_numbers() {
        assert_eq!(wordify(0), Err("Invalid Number"));
        assert_eq!(wordify(1000), Err("Invalid Number"));
        assert_eq!(wordify(1111), Err("Invalid Number"));
    }

    #[test]
    fn it_works_for_the_examples() {
        assert_eq!(wordify(1), Ok("one".to_string()));
        assert_eq!(wordify(12), Ok("twelve".to_string()));
        assert_eq!(wordify(17), Ok("seventeen".to_string()));
        assert_eq!(wordify(56), Ok("fifty six".to_string()));
        assert_eq!(wordify(90), Ok("ninety".to_string()));
        assert_eq!(wordify(326), Ok("three hundred twenty six".to_string()));
    }
}
