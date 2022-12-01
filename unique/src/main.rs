use std::collections::hash_map::HashMap;

fn main() {
    println!("Hello, world!");
}


fn unique<T: Ord + std::hash::Hash + Copy>(elements: Vec<T>) -> Vec<T> {
    let mut map = HashMap::new();
    let mut out: Vec<T> = vec![];

    for element in elements.iter() {
        let dup = map.contains_key(element);
        if !dup {
            map.insert(element, true);
            out.push(*element);
        }
    }

    out
}

#[test]
fn empty_list(){
    let input: Vec<i32> = vec![];
    let expected = vec![];
    let actual = unique(input);
    assert_eq!(actual, expected)
}

#[test]
fn already_unique(){
    let input = vec![4, 2, 8, 6];
    let expected = vec![4, 2, 8, 6];
    let actual = unique(input);
    assert_eq!(actual, expected)
}

#[test]
fn sorted_duplicate(){
    let input = vec![2, 4, 4, 6, 8];
    let expected = vec![2, 4, 6, 8];
    let actual = unique(input);
    assert_eq!(actual, expected)
}

#[test]
fn unsorted_duplicate(){
    let input = vec![4, 2, 2, 8, 6, 8, 2];
    let expected = vec![4, 2, 8, 6];
    let actual = unique(input);
    assert_eq!(actual, expected)
}
