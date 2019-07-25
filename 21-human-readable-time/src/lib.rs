pub fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        "now".to_string()
    } else {
        let units = [
            ("second", Some(60)),
            ("minute", Some(60)),
            ("hour", Some(24)),
            ("day", Some(365)),
            ("year", None),
        ];
        let mut cur = seconds;
        let mut times = vec![];
        for (label, number_before_next) in units.iter() {
            let remainder = match number_before_next {
                Some(x) => cur % x,
                None => cur,
            };
            cur = match number_before_next {
                Some(x) => cur / x,
                None => 0,
            };
            let maybe_plural_label = if remainder != 1 {
                format!("{}s", label)
            } else {
                label.to_string()
            };
            if remainder > 0 {
                times.push(format!("{} {}", remainder, maybe_plural_label));
            }
        }

        let mut iter = times.iter().cloned();
        let last_unit = iter.next().unwrap();
        let rest_units = iter.rev().collect::<Vec<_>>().join(", ");

        if rest_units.len() > 0 {
            format!("{} and {}", rest_units, last_unit)
        } else {
            last_unit
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::format_duration;

    #[test]
    fn it_for_zero() {
        assert_eq!(format_duration(0), "now".to_string());
    }

    #[test]
    fn it_for_the_small_example() {
        assert_eq!(format_duration(62), "1 minute and 2 seconds".to_string());
    }

    #[test]
    fn it_for_the_large_example() {
        assert_eq!(
            format_duration(3662),
            "1 hour, 1 minute and 2 seconds".to_string()
        );
    }

    #[test]
    fn it_works_for_e_chorobas_examples() {
        assert_eq!(
            format_duration(31539601),
            "1 year, 1 hour and 1 second".to_string()
        );
        assert_eq!(
            format_duration(66711841),
            "2 years, 42 days, 3 hours, 4 minutes and 1 second".to_string()
        );
        assert_eq!(format_duration(120), "2 minutes".to_string());
    }
}
