mod space_map;
use space_map::*;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut objects = get_objects();

    let iterations = 1000;

    for _ in 0..iterations {
        update_space_objects(&mut objects);
    }

    let total_energy = objects.iter().fold(0, |acc, x| acc + x.get_total_energy());

    println!(
        "After {} iterations, the total energy is {}",
        iterations, total_energy
    );
}

fn part_2() {
    let mut objects = get_objects();

    let repeat_count = get_repeat_count(&mut objects);

    println!("Repeats after {} iterations", repeat_count);
}

fn get_objects() -> Vec<SpaceObject> {
    vec![
        SpaceObject::new(Vec3 { x: 1, y: 2, z: -9 }),
        SpaceObject::new(Vec3 {
            x: -1,
            y: -9,
            z: -4,
        }),
        SpaceObject::new(Vec3 { x: 17, y: 6, z: 8 }),
        SpaceObject::new(Vec3 { x: 12, y: 4, z: 2 }),
    ]
}
