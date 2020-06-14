use std::collections::*;

pub fn get_checksum(input: &str) -> u32 {
    let mut objects = HashMap::<&str, &str>::new();

    for line in input.split("\n") {
        let mut parts = line.split(")");
        let parent = parts.next().unwrap();
        let child = parts.next().unwrap();

        let wat = objects.insert(child, parent);
        if wat.is_some() {
            panic!("Data corrupt");
        }
    }

    let mut count = 0;
    for object in objects.keys() {
        let mut current = object;

        while let Some(child) = objects.get(current) {
            count += 1;
            current = child;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let data = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

        assert_eq!(get_checksum(data), 42);
    }
}
