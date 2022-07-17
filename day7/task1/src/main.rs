use std::{collections::HashMap, fs::read_to_string};

mod utils;
use utils::connect_wires;

fn main() {
    let data: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    let mut map: HashMap<&str, Option<u16>> = HashMap::new();
    connect_wires(&data, &mut map);
    println!("{}", map.get("a").unwrap().unwrap());
}
