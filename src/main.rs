#![allow(dead_code)]

use proptest::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: usize,
}

#[derive(Debug, Clone)]
pub struct PersonDirectory {
    last_name_to_person_map: BTreeMap<String, Person>,
}

pub fn example_directory() -> PersonDirectory {
    let map = [
        Person {
            first_name: "Mick".to_string(),
            last_name: "Jagger".to_string(),
            age: 79,
        },
        Person {
            first_name: "Charles".to_string(),
            last_name: "Darwin".to_string(),
            age: 213,
        },
        Person {
            first_name: "Santa".to_string(),
            last_name: "Claus".to_string(),
            age: 1751,
        },
    ]
    .map(|person| (person.last_name.clone(), person))
    .into_iter()
    .collect();
    PersonDirectory {
        last_name_to_person_map: map,
    }
}

proptest! {
    #[test]
    fn deliberately_fail_test(_directory in Just(example_directory())) {
        prop_assert!(false);
    }
}

fn main() {
    println!("Hello, world!");
}
