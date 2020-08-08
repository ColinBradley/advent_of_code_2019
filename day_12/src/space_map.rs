use std::cmp::Ordering;

pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn get_absolute_sum(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    #[cfg(test)]
    fn to_string(&self) -> String {
        format!("<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

pub struct SpaceObject {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl SpaceObject {
    pub fn new(position: Vec3) -> Self {
        SpaceObject {
            position,
            velocity: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    pub fn get_total_energy(&self) -> i64 {
        self.get_potential_energy() * self.get_kinetic_energy()
    }

    fn get_potential_energy(&self) -> i64 {
        self.position.get_absolute_sum()
    }

    fn get_kinetic_energy(&self) -> i64 {
        self.velocity.get_absolute_sum()
    }

    #[cfg(test)]
    fn to_string(&self) -> String {
        format!(
            "pos={}, vel={}",
            self.position.to_string(),
            self.velocity.to_string()
        )
    }
}

pub fn update_space_objects(objects: &mut Vec<SpaceObject>) {
    // Update velocities
    for current_index in 0..objects.len() {
        for other_index in 0..objects.len() {
            if current_index == other_index {
                continue;
            }

            let current = objects.get(current_index).unwrap();
            let other = objects.get(other_index).unwrap();

            let new_velocity = current.velocity.add(&Vec3 {
                x: get_delta(&current.position.x, &other.position.x),
                y: get_delta(&current.position.y, &other.position.y),
                z: get_delta(&current.position.z, &other.position.z),
            });

            objects.get_mut(current_index).unwrap().velocity = new_velocity;
        }
    }

    // Update positions
    for object in objects.iter_mut() {
        object.position = object.position.add(&object.velocity);
    }
}

pub fn update_space_objects_axis<F, S>(
    objects: &mut Vec<SpaceObject>,
    axis_fetcher: F,
    axis_setter: S,
) where
    F: Fn(&Vec3) -> i64,
    S: Fn(&mut Vec3, i64),
{
    // Update velocities
    for current_index in 0..objects.len() {
        for other_index in 0..objects.len() {
            if current_index == other_index {
                continue;
            }

            let current = objects.get(current_index).unwrap();
            let other = objects.get(other_index).unwrap();

            let new_velocity = axis_fetcher(&current.velocity)
                + get_delta(
                    &axis_fetcher(&current.position),
                    &axis_fetcher(&other.position),
                );

            axis_setter(
                &mut objects.get_mut(current_index).unwrap().velocity,
                new_velocity,
            );
        }
    }

    // Update positions
    for object in objects.iter_mut() {
        object.position = object.position.add(&object.velocity);
    }
}

pub fn get_repeat_count_axis<F, S>(
    objects: &mut Vec<SpaceObject>,
    axis_fetcher: &F,
    axis_setter: &S,
) -> u64
where
    F: Fn(&Vec3) -> i64,
    S: Fn(&mut Vec3, i64),
{
    let initial_positions = objects
        .iter()
        .map(|o| axis_fetcher(&o.position))
        .collect::<Vec<i64>>();

    let mut index = 0;
    'outer: loop {
        update_space_objects_axis(objects, axis_fetcher, axis_setter);
        index = index + 1;

        for object_entry in objects.iter().enumerate() {
            let object = object_entry.1;

            if object.get_kinetic_energy() != 0 {
                continue 'outer;
            }

            if initial_positions.get(object_entry.0).unwrap() != &axis_fetcher(&object.position) {
                continue 'outer;
            }
        }

        break;
    }

    index
}

pub fn get_repeat_count(objects: &mut Vec<SpaceObject>) -> u64 {
    // So... this can be a REALLY big number (Hi, 64 bit numbers!)
    // A few things: repeats are not arbitrary - they come back to the initial value, so that's all we have to look for
    // We can optimize by checking each axis individually for repeat count then finding the lowest common multiplier of each

    let x_repeat_count = get_repeat_count_axis(objects, &|o| o.x, &|o, v| o.x = v);
    let y_repeat_count = get_repeat_count_axis(objects, &|o| o.y, &|o, v| o.y = v);
    let z_repeat_count = get_repeat_count_axis(objects, &|o| o.z, &|o, v| o.z = v);

    num_integer::lcm(
        x_repeat_count,
        num_integer::lcm(y_repeat_count, z_repeat_count),
    )
}

fn get_delta(a: &i64, b: &i64) -> i64 {
    match a.partial_cmp(b).unwrap() {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_1() -> Vec<SpaceObject> {
        vec![
            SpaceObject::new(Vec3 { x: -1, y: 0, z: 2 }),
            SpaceObject::new(Vec3 {
                x: 2,
                y: -10,
                z: -7,
            }),
            SpaceObject::new(Vec3 { x: 4, y: -8, z: 8 }),
            SpaceObject::new(Vec3 { x: 3, y: 5, z: -1 }),
        ]
    }

    fn get_test_data_2() -> Vec<SpaceObject> {
        vec![
            SpaceObject::new(Vec3 {
                x: -8,
                y: -10,
                z: 0,
            }),
            SpaceObject::new(Vec3 { x: 5, y: 5, z: 10 }),
            SpaceObject::new(Vec3 { x: 2, y: -7, z: 3 }),
            SpaceObject::new(Vec3 { x: 9, y: -8, z: -3 }),
        ]
    }

    #[test]
    fn example_1_10_runs() {
        let mut objects = get_test_data_1();

        for _ in 0..10 {
            update_space_objects(&mut objects);
        }

        assert_eq!(
            objects[0].to_string(),
            "pos=<x=2, y=1, z=-3>, vel=<x=-3, y=-2, z=1>"
        );
        assert_eq!(
            objects[1].to_string(),
            "pos=<x=1, y=-8, z=0>, vel=<x=-1, y=1, z=3>"
        );
        assert_eq!(
            objects[2].to_string(),
            "pos=<x=3, y=-6, z=1>, vel=<x=3, y=2, z=-3>"
        );
        assert_eq!(
            objects[3].to_string(),
            "pos=<x=2, y=0, z=4>, vel=<x=1, y=-1, z=-1>"
        );

        assert_eq!(objects[0].get_total_energy(), 36);
        assert_eq!(objects[1].get_total_energy(), 45);
        assert_eq!(objects[2].get_total_energy(), 80);
        assert_eq!(objects[3].get_total_energy(), 18);
    }

    #[test]
    fn example_2_10_runs() {
        let mut objects = get_test_data_2();

        for _ in 0..100 {
            update_space_objects(&mut objects);
        }

        assert_eq!(
            objects[0].to_string(),
            "pos=<x=8, y=-12, z=-9>, vel=<x=-7, y=3, z=0>"
        );
        assert_eq!(
            objects[1].to_string(),
            "pos=<x=13, y=16, z=-3>, vel=<x=3, y=-11, z=-5>"
        );
        assert_eq!(
            objects[2].to_string(),
            "pos=<x=-29, y=-11, z=-1>, vel=<x=-3, y=7, z=4>"
        );
        assert_eq!(
            objects[3].to_string(),
            "pos=<x=16, y=-13, z=23>, vel=<x=7, y=1, z=1>"
        );

        assert_eq!(objects[0].get_total_energy(), 290);
        assert_eq!(objects[1].get_total_energy(), 608);
        assert_eq!(objects[2].get_total_energy(), 574);
        assert_eq!(objects[3].get_total_energy(), 468);
    }

    #[test]
    fn example_1_repeat_count() {
        let mut objects = get_test_data_1();

        assert_eq!(get_repeat_count(&mut objects), 2772);
    }

    #[test]
    fn example_2_repeat_count() {
        let mut objects = get_test_data_2();

        assert_eq!(get_repeat_count(&mut objects), 4686774924);
    }
}
