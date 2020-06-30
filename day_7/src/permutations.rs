// This seems like a really dumb way to do this.
// But damn it, Jim, I'm an engineer not a computer scientist/mathematician!
pub fn permutations<T>(source: &Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if source.len() == 1 {
        return vec![source.clone()];
    }

    let mut results = Vec::<Vec<T>>::new();
    for index in 0..source.len() {
        let head = source.get(index).unwrap();

        let tail: Vec<T> = source
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if i != index { Some(v.clone()) } else { None })
            .collect();

        for child_permutation in permutations(&tail) {
            let mut result = Vec::<T>::new();
            result.push(head.clone());

            for value in child_permutation {
                result.push(value);
            }
            results.push(result);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_3_items() {
        let result = permutations(&vec!["A", "B", "C"]);
        assert_eq!(result.len(), 6);
        assert!(result.iter().all(|p| p.len() == 3));
    }

    #[test]
    fn basic_4_items() {
        let result = permutations(&vec!["A", "B", "C", "D"]);
        assert_eq!(result.len(), 24);
        assert!(result.iter().all(|p| p.len() == 4));
    }

    #[test]
    fn basic_5_items() {
        let result = permutations(&vec!["A", "B", "C", "D", "E"]);
        assert_eq!(result.len(), 120);
        assert!(result.iter().all(|p| p.len() == 5));
    }
}
