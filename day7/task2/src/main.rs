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

    'outer: loop {
        for line in &data {
            let mut instructions: Vec<&str> = line.split_whitespace().collect();
            // retrieving and popping receiver wire
            let reciever = instructions.pop().unwrap();
            // now lets remove "->" element
            instructions.pop();

            let value;

            if instructions.len() == 3 {
                value = parse_3(&instructions, &first_map);
            } else if instructions.len() == 2 {
                value = parse_2(&instructions, &first_map);
            } else {
                value = parse_1(&instructions, &first_map);
            }
            first_map.insert(reciever, value);
        }
        if all_wires_connected(&first_map) {
            break 'outer;
        }
    }

    second_map.insert("b", *first_map.get("a").unwrap());

    'outer_2: loop {
        for line in &data {
            let mut instructions: Vec<&str> = line.split_whitespace().collect();
            let reciever = instructions.pop().unwrap();
            instructions.pop();

            let value;
            if instructions.len() == 3 {
                value = parse_3(&instructions, &second_map);
            } else if instructions.len() == 2 {
                value = parse_2(&instructions, &second_map);
            } else {
                value = parse_1(&instructions, &second_map);
            }
            if reciever == "b" {
                continue;
            }
            second_map.insert(reciever, value);
        }
        if all_wires_connected(&second_map) {
            break 'outer_2;
        }
    }
    println!("{:#?}", second_map.get("a").unwrap())
}
