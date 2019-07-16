use std::collections::HashMap;

type Error = &'static str;

#[derive(Hash, PartialEq, Eq)]
enum Location {
    Home,
    Town(String),
}

struct TownMap {
    town_distances_from_home: HashMap<Location, f64>,
    ordered_towns: Vec<Location>,
    home_town_name: String,
}

impl TownMap {
    fn from_vec(vec: Vec<(String, f64)>, home_town_name: &str) -> Result<Self, Error> {
        let mut town_distances_from_home: HashMap<Location, f64> = HashMap::new();
        let mut ordered_towns = vec![];

        for (town_name, distance) in vec {
            if town_name == home_town_name {
                if distance != 0. {
                    return Err("Home town must be 0 miles away from itself");
                }
            } else {
                ordered_towns.push(Location::Town(town_name.clone()));
                town_distances_from_home.insert(Location::Town(town_name), distance);
            }
        }

        Ok(Self {
            town_distances_from_home,
            ordered_towns,
            home_town_name: home_town_name.to_string(),
        })
    }

    fn distance_to_home(&self, from: &Location) -> Result<f64, Error> {
        match self.town_distances_from_home.get(from) {
            Some(x) => Ok(*x),
            None => Err("We don't know where this town is"),
        }
    }

    fn distance_around_radius(&self, from: &Location, to: &Location) -> Result<f64, &'static str> {
        let index_1 = match self.ordered_towns.iter().position(|x| x == from) {
            Some(x) => Ok(x),
            None => Err("We don't know where this location is"),
        }?;
        let index_2 = match self.ordered_towns.iter().position(|x| x == to) {
            Some(x) => Ok(x),
            None => Err("We don't know where this location is"),
        }?;

        let (start_index, end_index) = if index_1 > index_2 {
            (index_2, index_1)
        } else {
            (index_1, index_2)
        };

        let mut current_distance = 0.;
        for i in start_index..end_index {
            let c = self.distance_to_home(&self.ordered_towns[i + 1])?;
            let a = self.distance_to_home(&self.ordered_towns[i])?;
            let b_squared = c.powi(2) - a.powi(2);
            if b_squared >= 0. {
                current_distance += b_squared.sqrt();
            } else {
                return Err("This map is invalid because of the right angle rule");
            }
        }

        Ok(current_distance)
    }

    fn distance(&self, from: &Location, to: &Location) -> Result<f64, &'static str> {
        match (from, to) {
            (Location::Home, Location::Home) => Ok(0.),
            (Location::Town(_), Location::Home) => Ok(self.distance_to_home(&from)?),
            (Location::Home, Location::Town(_)) => Ok(self.distance_to_home(&to)?),
            (Location::Town(_), Location::Town(_)) => {
                let town_to_town_distance = self.distance_around_radius(from, to)?;
                let home_in_middle_distance =
                    self.distance_to_home(from)? + self.distance_to_home(to)?;

                if town_to_town_distance > home_in_middle_distance {
                    Ok(home_in_middle_distance)
                } else {
                    Ok(town_to_town_distance)
                }
            }
        }
    }

    fn get_location_from_string(&self, name: &str) -> Location {
        if name == self.home_town_name {
            Location::Home
        } else {
            Location::Town(name.to_string())
        }
    }

    fn make_friend_map(&self, input: Vec<(String, String)>) -> HashMap<String, Location> {
        let mut output = HashMap::new();
        for x in input {
            output.insert(x.0.clone(), self.get_location_from_string(&x.1));
        }
        output
    }
}

pub fn total_distance(
    friends_to_visit: Vec<String>,
    friend_map: Vec<(String, String)>,
    town_map: Vec<(String, f64)>,
    home_town_name: Option<String>,
) -> Result<f64, Error> {
    let home_town_name = home_town_name.unwrap_or("Home".to_string());
    let town_map = TownMap::from_vec(town_map, &home_town_name)?;
    let friend_map = town_map.make_friend_map(friend_map);
    let locations_to_visit = {
        let mut output = vec![];
        for f in friends_to_visit {
            let friend_town = match friend_map.get(&f) {
                Some(x) => Ok(x),
                None => Err("We don't know where this friend lives"),
            }?;
            output.push(friend_town);
        }
        output.push(&Location::Home);

        output
    };

    let mut current_distance = 0.;
    let mut current_location = &Location::Home;
    for l in locations_to_visit {
        current_distance += town_map.distance(current_location, l)?;
        current_location = l;
    }

    Ok(current_distance)
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn is_within_tolerance_of(x: f64, y: f64) -> bool {
        let tolerance = 0.1;

        (x - tolerance) <= y && y <= (x + tolerance)
    }

    #[test]
    fn it_works_for_a_single_friend() {
        let friends_to_visit = vec!["friend1".to_string()];
        let friend_map = vec![("friend1".to_string(), "town1".to_string())];
        let town_map = vec![("town1".to_string(), 5.)];
        let home_town_name = None;
        let distance =
            total_distance(friends_to_visit, friend_map, town_map, home_town_name).unwrap();
        assert!(is_within_tolerance_of(distance, 10.));
    }

    #[test]
    fn it_works_for_a_two_friends_in_adjacent_towns() {
        let friends_to_visit = vec!["friend1".to_string(), "friend2".to_string()];
        let friend_map = vec![
            ("friend1".to_string(), "town1".to_string()),
            ("friend2".to_string(), "town2".to_string()),
        ];
        let town_map = vec![("town1".to_string(), 5.), ("town2".to_string(), 11.)];
        let home_town_name = None;
        let distance =
            total_distance(friends_to_visit, friend_map, town_map, home_town_name).unwrap();
        assert!(is_within_tolerance_of(distance, 25.79));
    }

    #[test]
    fn it_works_for_a_two_friends_non_adjacent_towns() {
        let friends_to_visit = vec!["friend1".to_string(), "friend2".to_string()];
        let friend_map = vec![
            ("friend1".to_string(), "town1".to_string()),
            ("friend2".to_string(), "town10".to_string()),
        ];
        let town_map = vec![
            ("town1".to_string(), 5.),
            ("town2".to_string(), 11.),
            ("town3".to_string(), 50.),
            ("town4".to_string(), 52.),
            ("town5".to_string(), 53.),
            ("town6".to_string(), 54.),
            ("town7".to_string(), 55.),
            ("town8".to_string(), 56.),
            ("town9".to_string(), 57.),
            ("town10".to_string(), 70.),
        ];
        let home_town_name = None;
        let distance =
            total_distance(friends_to_visit, friend_map, town_map, home_town_name).unwrap();
        assert!(
            is_within_tolerance_of(distance, 150.),
            format!("{}", distance)
        );
    }

    #[test]
    fn it_works_for_a_two_friends_almost_adjacent_towns() -> Result<(), Error> {
        let friends_to_visit = vec!["friend1".to_string(), "friend2".to_string()];
        let friend_map = vec![
            ("friend1".to_string(), "town1".to_string()),
            ("friend2".to_string(), "town4".to_string()),
        ];
        let town_map = vec![
            ("town1".to_string(), 5.),
            ("town2".to_string(), 6.),
            ("town3".to_string(), 7.),
            ("town4".to_string(), 8.),
        ];
        let home_town_name = None;
        let distance = total_distance(friends_to_visit, friend_map, town_map, home_town_name)?;
        assert!(
            is_within_tolerance_of(distance, 23.79),
            format!("{}", distance)
        );

        Ok(())
    }
}
