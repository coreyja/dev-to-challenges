use std::fmt;
use std::io;
use std::io::prelude::*;

fn parse_time_of_walk(user_input: &str) -> Result<u32, &'static str> {
    let parsed_input_result: Result<u32, _> = user_input.parse();
    match parsed_input_result {
        Ok(i) => {
            if i % 2 == 0 {
                Ok(i)
            } else {
                Err("Walk time must be an even number or we will not be able to get back to where we started")
            }
        }
        Err(_) => Err("Could not parse the walk time"),
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn will_take_me_home(&self, current_position: (i32, i32)) -> bool {
        match self {
            Direction::North => current_position.1 < 0,
            Direction::South => current_position.1 > 0,
            Direction::East => current_position.0 < 0,
            Direction::West => current_position.0 > 0,
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        }
    }

    fn direction_diff(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

fn options_availible(current_position: (i32, i32), minutes_left: u32) -> Option<Vec<Direction>> {
    if current_position == (0, 0) && minutes_left == 0 {
        return Some(vec![]);
    }

    let distance_from_origin = current_position.0.abs() + current_position.1.abs();

    if distance_from_origin > minutes_left as i32 {
        None
    } else {
        let all_options: Vec<Direction> = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        let can_take_detour = distance_from_origin < minutes_left as i32;

        let options = all_options
            .iter()
            .filter(|direction| direction.will_take_me_home(current_position) || can_take_detour)
            .cloned()
            .collect();
        Some(options)
    }
}

fn chosen_direction(input: &str) -> Option<Direction> {
    let lower = input.to_ascii_lowercase();
    if lower.starts_with("n") {
        Some(Direction::North)
    } else if lower.starts_with("e") {
        Some(Direction::East)
    } else if lower.starts_with("w") {
        Some(Direction::West)
    } else if lower.starts_with("s") {
        Some(Direction::South)
    } else {
        None
    }
}

fn directions_to_string(path: &Vec<Direction>) -> String {
    path.iter()
        .map(Direction::to_str)
        .collect::<Vec<_>>()
        .join(", ")
}

fn main() -> Result<(), &'static str> {
    println!("Hey I heard you want to take a walk!");
    println!("How long (in minutes) should we walk for?");
    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    let user_input_minutes = stdin_lines.next().unwrap().unwrap();

    let mut minutes_left = parse_time_of_walk(&user_input_minutes)?;
    let mut current_position = (0, 0);
    let mut path: Vec<Direction> = vec![];

    let mut options = options_availible(current_position, minutes_left);
    while minutes_left > 0 && options.is_some() {
        let unwrapped_options = options.unwrap();

        println!("\nYou have {} minutes left", minutes_left);
        println!("Which direction would you like to go?");
        println!(
            "So far you'r path has been: {}",
            directions_to_string(&path)
        );
        println!(
            "Your current options are: {}",
            directions_to_string(&unwrapped_options)
        );

        let mut chosen_direction_option = chosen_direction(&stdin_lines.next().unwrap().unwrap());
        while chosen_direction_option.is_none() {
            chosen_direction_option = chosen_direction(&stdin_lines.next().unwrap().unwrap());
        }

        let chosen_direction = chosen_direction_option.unwrap();
        println!("You chose: {}", chosen_direction);

        current_position.0 += chosen_direction.direction_diff().0;
        current_position.1 += chosen_direction.direction_diff().1;
        path.push(chosen_direction);
        minutes_left -= 1;
        options = options_availible(current_position, minutes_left);
    }

    if minutes_left == 0 {
        println!(
            "YAY we made it! Here is the path we took: {}",
            directions_to_string(&path)
        );
    } else if options.is_none() {
        println!("OH NO! We won't be able to make it back in time! Try another path.");
        println!(
            "Here is what you tried this time: {}",
            directions_to_string(&path)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_time_of_walk_test() {
        assert_eq!(
            parse_time_of_walk("3"),
            Err("Walk time must be an even number or we will not be able to get back to where we started")
        );
        assert_eq!(
            parse_time_of_walk("five"),
            Err("Could not parse the walk time")
        );
        assert_eq!(parse_time_of_walk("4"), Ok(4));
    }

    #[test]
    fn test_all_options_availible() {
        let output = options_availible((0, 0), 10);
        let all_options = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        assert_eq!(output, Some(all_options));
    }

    #[test]
    fn test_impossible_options() {
        assert_eq!(options_availible((0, -10), 1), None);
        assert_eq!(options_availible((1, 10), 10), None);
        assert_eq!(options_availible((-1, 10), 10), None);
    }

    #[test]
    fn test_already_at_origin_and_no_minutes_left() {
        assert_eq!(options_availible((0, 0), 0), Some(vec![]));
    }

    #[test]
    fn test_north_only_option() {
        let output = options_availible((0, -1), 1);
        assert_eq!(output, Some(vec![Direction::North]));
    }
}
