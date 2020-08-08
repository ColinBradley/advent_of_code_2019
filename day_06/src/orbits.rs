use core::cell::RefCell;
use std::collections::*;
use std::rc::Rc;

pub struct SpaceObject<'a> {
    name: &'a str,
    parent: Option<Rc<RefCell<SpaceObject<'a>>>>,
    children: Vec<Rc<RefCell<SpaceObject<'a>>>>,
}

impl<'a> SpaceObject<'a> {
    fn new(name: &'a str) -> SpaceObject<'a> {
        SpaceObject {
            name,
            parent: None,
            children: Vec::<Rc<RefCell<SpaceObject>>>::new(),
        }
    }
}

pub fn parse<'a>(data: &'a str) -> HashMap<&'a str, Rc<RefCell<SpaceObject>>> {
    let mut objects_by_name = HashMap::<&'a str, Rc<RefCell<SpaceObject>>>::new();

    for line in data.split("\n") {
        let mut names = line.split(")");
        let parent_name = names.next().unwrap();
        let child_name = names.next().unwrap();

        let parent = Rc::clone(
            objects_by_name
                .entry(parent_name)
                .or_insert_with(|| Rc::new(RefCell::new(SpaceObject::new(parent_name)))),
        );

        let child = Rc::clone(
            objects_by_name
                .entry(child_name)
                .or_insert_with(|| Rc::new(RefCell::new(SpaceObject::new(child_name)))),
        );

        child.borrow_mut().parent = Some(Rc::clone(&parent));

        parent.try_borrow_mut().unwrap().children.push(child);
    }

    objects_by_name
}

pub fn get_checksum(data: &HashMap<&str, Rc<RefCell<SpaceObject>>>) -> u32 {
    data.values().map(|o| get_parent_count(o)).sum()
}

pub fn get_transfers<'a>(
    name_a: &str,
    name_b: &str,
    data: &HashMap<&'a str, Rc<RefCell<SpaceObject>>>,
) -> Option<u32> {
    let mut a_parents = Vec::new();
    get_parent_names(data.get(name_a)?, &mut a_parents);
    let mut b_parents = Vec::new();
    get_parent_names(data.get(name_b)?, &mut b_parents);

    let mut count = 0;
    for parent_a in a_parents {
        let mut b_count = 0;
        for parent_b in b_parents.iter() {
            if parent_a == *parent_b {
                count += b_count;
                return Some(count);
            }
            b_count += 1;
        }

        count += 1;
    }

    None
}

fn get_parent_count(object: &Rc<RefCell<SpaceObject>>) -> u32 {
    // Gone for recursion here to ensure that borrows live all the way through checking other parents
    // TODO: iterative way to do this?
    // Note: I feel the way the data is set up prevents simple iteration
    match &object.borrow().parent {
        Some(parent) => 1 + get_parent_count(parent),
        None => 0,
    }
}

fn get_parent_names<'a>(object: &Rc<RefCell<SpaceObject<'a>>>, names: &mut Vec<&'a str>) {
    // Gone for recursion here to ensure that borrows live all the way through checking other parents
    match &object.borrow().parent {
        Some(parent) => {
            names.push(parent.borrow().name);
            get_parent_names(parent, names)
        }
        None => (),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_checksum() {
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

        assert_eq!(get_checksum(&parse(data)), 42);
    }

    #[test]
    fn example_orbit_transfers() {
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
K)L
K)YOU
I)SAN";

        assert_eq!(get_transfers("YOU", "SAN", &parse(data)).unwrap(), 4);
    }
}
