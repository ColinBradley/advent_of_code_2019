pub struct AsteroidMap {
    #[cfg(test)]
    locations: Vec<bool>,
    pub asteroids: Vec<Position>,
    #[cfg(test)]
    width: usize,
    #[cfg(test)]
    height: usize,
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl AsteroidMap {
    pub fn parse(data: &str) -> AsteroidMap {
        let lines = data.trim().lines().collect::<Vec<&str>>();
        let width = lines.get(0).unwrap().len();
        let height = lines.len();
        let mut locations = Vec::with_capacity(width * height);
        let mut asteroids = Vec::with_capacity(locations.capacity());

        for (y, line) in lines.iter().enumerate() {
            for (x, character) in line.chars().enumerate() {
                match character {
                    '#' => {
                        locations.push(true);
                        asteroids.push(Position {
                            x: x as u16,
                            y: y as u16,
                        });
                    }
                    '.' => locations.push(false),
                    _ => panic!("Unknown character in map data"),
                }
            }
        }

        AsteroidMap {
            #[cfg(test)]
            locations,
            asteroids,
            #[cfg(test)]
            width,
            #[cfg(test)]
            height,
        }
    }

    #[cfg(test)]
    pub fn get(&self, x: usize, y: usize) -> Option<&bool> {
        self.locations.get(y * self.width + x)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_data;
    use super::*;

    #[test]
    fn map_parse_0() {
        let asteroids = AsteroidMap::parse(test_data::MAP_DATA_0);

        assert_eq!(asteroids.width, 5);
        assert_eq!(asteroids.height, 5);

        assert_eq!(asteroids.get(0, 0), Some(&false));
        assert_eq!(asteroids.get(1, 0), Some(&true));
        assert_eq!(asteroids.get(4, 4), Some(&true));

        assert_eq!(asteroids.asteroids.len(), 10);
    }
}
