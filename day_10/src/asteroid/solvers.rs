use super::map::*;
use std::collections::HashSet;

pub fn get_most_visible<'a>(counts: &'a Vec<(&'a Position, usize)>) -> &'a (&'a Position, usize) {
    let mut largest: Option<&(&Position, usize)> = None;

    for asteroid in counts {
        largest = match largest {
            None => Some(asteroid),
            Some((_, largest_count)) if *largest_count < asteroid.1 => Some(asteroid),
            _ => largest,
        }
    }

    largest.unwrap()
}

pub fn get_visible_counts(map: &AsteroidMap) -> Vec<(&Position, usize)> {
    map.asteroids
        .iter()
        .map(|p| (p, get_visible_count(map, p)))
        .collect()
}

pub fn get_visible_count(map: &AsteroidMap, position: &Position) -> usize {
    // Unique angle means that something is visible
    let mut angles = HashSet::new();

    for asteroid in map.asteroids.iter() {
        if asteroid == position {
            continue;
        }

        let x_diff = (asteroid.x as i16 - position.x as i16) as f64;
        let y_diff = (asteroid.y as i16 - position.y as i16) as f64;

        let angle = y_diff.atan2(x_diff);

        angles.insert(angle.to_string());
    }

    angles.len()
}

#[cfg(test)]
mod tests {
    use super::super::test_data;
    use super::*;

    #[test]
    fn get_visible_counts_0() {
        let map = AsteroidMap::parse(test_data::MAP_DATA_0);

        let counts = get_visible_counts(&map)
            .iter()
            .map(|r| r.1)
            .collect::<Vec<usize>>();

        assert_eq!(counts, vec![7, 7, 6, 7, 7, 7, 5, 7, 8, 7]);
    }

    #[test]
    fn get_visible_counts_1() {
        let map = AsteroidMap::parse(test_data::MAP_DATA_1);

        let counts = get_visible_counts(&map);
        let most_visible = get_most_visible(&counts);

        assert_eq!(*most_visible.0, Position { x: 5, y: 8 });
        assert_eq!(most_visible.1, 33);
    }
}
