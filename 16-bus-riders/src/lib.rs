pub fn riders_left(bus_stops: &[(u32, u32)]) -> Result<u32, &str> {
    bus_stops.iter().fold(Ok(0), |sum, (on, off)| {
        let new_sum = sum? + on;

        if off > &new_sum {
            Err("Somehow more people left the bus than got on")
        } else {
            Ok(new_sum - off)
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_returns_zero_for_an_empty_array() {
        assert_eq!(riders_left(&[]), Ok(0))
    }

    #[test]
    fn it_for_a_semi_complicated_array_example() {
        assert_eq!(riders_left(&[(10, 0), (5, 9), (6, 6), (2, 6)]), Ok(2));
    }

    #[test]
    fn it_for_a_semi_complicated_vec_example() {
        assert_eq!(riders_left(&vec![(10, 0), (5, 9), (6, 6), (2, 6)]), Ok(2));
    }

    #[test]
    fn it_for_a_broken_example() {
        assert_eq!(
            riders_left(&vec![(10, 0), (5, 9), (6, 6), (2, 12)]),
            Err("Somehow more people left the bus than got on")
        );
    }
}
