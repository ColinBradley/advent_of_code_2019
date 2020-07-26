use super::map::*;
use std::collections::HashMap;

pub fn get_vaporization_order<'a>(from: &Position, field: &'a AsteroidField) -> Vec<&'a Position> {
    let mut angles = field.keys().collect::<Vec<_>>();

    angles.sort_by(|a, b| {
        a.parse::<f64>()
            .unwrap()
            .partial_cmp(&b.parse::<f64>().unwrap())
            .unwrap()
    });

    let get_distance = |p: &Position| {
        (((from.x as i16 - p.x as i16).pow(2) + (from.y as i16 - p.y as i16).pow(2)) as f64).sqrt()
    };

    // If we collected the positions in order, then we'd not have to do this
    // Order being: from the center point outwards
    let mut fields_with_sorted_positions = HashMap::new();
    for angle in angles.iter() {
        let mut asteroid_positions = field.get(*angle).unwrap().clone();

        asteroid_positions.sort_by(|a, b| get_distance(a).partial_cmp(&get_distance(b)).unwrap());

        fields_with_sorted_positions.insert(angle, asteroid_positions);
    }

    let mut results = Vec::<&Position>::new();
    let mut ring_index = 0;
    loop {
        let mut found = 0;
        for angle in angles.iter() {
            let asteroid_positions = fields_with_sorted_positions.get(angle).unwrap();
            if let Some(asteroid_position) = asteroid_positions.get(ring_index) {
                results.push(asteroid_position);
                found += 1;
            }
        }

        if found == 0 {
            break;
        }

        ring_index += 1;
    }

    results
}

pub fn get_most_visible<'a>(
    asteroids: &'a Vec<(&'a Position, AsteroidField)>,
) -> &'a (&'a Position, AsteroidField<'a>) {
    let mut largest: Option<&(&'a Position, AsteroidField)> = None;

    for asteroid in asteroids {
        largest = match largest {
            None => Some(asteroid),
            Some(field) if field.1.len() < asteroid.1.len() => Some(asteroid),
            _ => largest,
        }
    }

    largest.unwrap()
}

pub fn get_visible_fields<'a>(map: &'a AsteroidMap) -> Vec<(&Position, AsteroidField<'a>)> {
    map.asteroids
        .iter()
        .map(|p| (p, get_asteroids_by_angle(map, p)))
        .collect()
}

pub type AsteroidField<'a> = HashMap<String, Vec<&'a Position>>;

pub fn get_asteroids_by_angle<'a>(
    map: &'a AsteroidMap,
    position: &'a Position,
) -> AsteroidField<'a> {
    // Unique angle means that something is visible
    let mut angles_to_positions = HashMap::<String, Vec<&Position>>::new();

    for asteroid in map.asteroids.iter() {
        if asteroid == position {
            continue;
        }

        let x_diff = (asteroid.x as i16 - position.x as i16) as f64;
        let y_diff = (asteroid.y as i16 - position.y as i16) as f64;

        let angle_rad = y_diff.atan2(x_diff);

        let angle_deg = angle_rad * RAD_TO_DEG;

        // "up" should be 0 (rather than "right" being 0)
        let mut angle_deg_rotated = (angle_deg + 90f64) % 360f64;

        if angle_deg_rotated < 0f64 {
            angle_deg_rotated += 360f64;
        }

        angles_to_positions
            .entry(angle_deg_rotated.to_string())
            .or_insert_with(|| Vec::new())
            .push(asteroid);
    }

    angles_to_positions
}

const RAD_TO_DEG: f64 = 180f64 / std::f64::consts::PI;

#[cfg(test)]
mod tests {
    use super::super::test_data;
    use super::*;

    #[test]
    fn get_visible_counts_data_0() {
        let map = AsteroidMap::parse(test_data::MAP_DATA_0);

        let fields = get_visible_fields(&map);
        let counts = fields.iter().map(|r| r.1.len()).collect::<Vec<usize>>();

        assert_eq!(counts, vec![7, 7, 6, 7, 7, 7, 5, 7, 8, 7]);
    }

    #[test]
    fn get_most_visible_data_1() {
        let map = AsteroidMap::parse(test_data::MAP_DATA_1);

        let fields = get_visible_fields(&map);
        let best = get_most_visible(&fields);

        assert_eq!(*best.0, Position { x: 5, y: 8 });
        assert_eq!(best.1.len(), 33);
    }

    #[test]
    fn get_vaporization_order_data_2() {
        let map = AsteroidMap::parse(test_data::MAP_DATA_2);

        let fields = get_visible_fields(&map);
        let best = get_most_visible(&fields);
        let positions_in_vaporization_order = get_vaporization_order(&best.0, &best.1);

        assert_eq!(best.0, &Position { x: 11, y: 13 });
        assert_eq!(positions_in_vaporization_order.len(), 299);

        assert_eq!(
            positions_in_vaporization_order.get(0),
            Some(&&Position { x: 11, y: 12 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(1),
            Some(&&Position { x: 12, y: 1 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(2),
            Some(&&Position { x: 12, y: 2 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(9),
            Some(&&Position { x: 12, y: 8 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(19),
            Some(&&Position { x: 16, y: 0 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(49),
            Some(&&Position { x: 16, y: 9 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(99),
            Some(&&Position { x: 10, y: 16 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(198),
            Some(&&Position { x: 9, y: 6 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(199),
            Some(&&Position { x: 8, y: 2 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(200),
            Some(&&Position { x: 10, y: 9 })
        );
        assert_eq!(
            positions_in_vaporization_order.get(298),
            Some(&&Position { x: 11, y: 1 })
        );
    }
}
