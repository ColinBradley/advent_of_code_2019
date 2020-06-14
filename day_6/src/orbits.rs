use core::cell::RefCell;
use std::collections::*;
use std::rc::Rc;

struct SpaceObject<'a> {
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

pub fn get_checksum(input: &str) -> u32 {
    let result = parse(input);

    result.values().map(|o| get_parent_count(o)).sum()
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

fn parse<'a>(data: &'a str) -> HashMap<&'a str, Rc<RefCell<SpaceObject>>> {
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
