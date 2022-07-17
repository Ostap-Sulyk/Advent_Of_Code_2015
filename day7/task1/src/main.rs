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

    let mut has_signal: HashMap<&str, Option<u16>> = HashMap::new();

    'outer: loop {
        for line in &data {
            let mut instructions: Vec<&str> = line.split_whitespace().collect();
            // retrieving and popping receiver wire
            let reciever = instructions.pop().unwrap();
            // now lets remove "->" element
            instructions.pop();

            let value;

            if instructions.len() == 3 {
                value = parse_3(&instructions, &has_signal);
            } else if instructions.len() == 2 {
                value = parse_2(&instructions, &has_signal);
            } else {
                value = parse_1(&instructions, &has_signal);
            }
            has_signal.insert(reciever, value);
        }
        if all_wires_connected(&has_signal) {
            break 'outer;
        }
    }
    println!("{}", has_signal.get("a").unwrap().unwrap());
}
