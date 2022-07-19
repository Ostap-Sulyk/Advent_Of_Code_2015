use std::{collections::HashMap, fs::read_to_string};

mod utils;
use utils::*;

fn main() {
    let data: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    // creating hash map to store wires
    let mut first_map: HashMap<&str, Option<u16>> = HashMap::new();

    // processing
    connect_wires(&data, &mut first_map, None);

    // saving "b" and value from "a"
    let (k, v) = ("b", *first_map.get("a").unwrap());

    // clearing map
    first_map.clear();
    first_map.insert(k, v);

    connect_wires(&data, &mut first_map, Some("b"));

    println!("{:#?}", first_map.get("a").unwrap().unwrap())
}
