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

    let mut first_map: HashMap<&str, Option<u16>> = HashMap::new();
    let mut second_map: HashMap<&str, Option<u16>> = HashMap::new();
    // overriding b with a

    connect_wires(&data, &mut first_map, None);

    second_map.insert("b", *first_map.get("a").unwrap());

    connect_wires(&data, &mut second_map, Some("b"));

    println!("{:#?}", second_map.get("a").unwrap().unwrap())
}
