#![allow(unused, clippy::never_loop)]
use regex::{Captures, Regex};
use std::{alloc, collections::HashMap, fs::read_to_string};

mod utils;
use utils::*;

    /// last word in each line is wire is wire
    /// before last word there is ->
    /// on the left there is operation to do on that word

    /// split each line in two parts to extract last wire
    /// then delete -> from lines,
    /// then trim the line
    /// then split split_whitespace
    /// if len is 3
    ///    extract thre values wire, operation, wire
    ///  else if len is 2
    ///      extract use not
    ///  else
    ///  try to extract wire from map and assign to receiving wire
    ///

fn main() {
    let data: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    //    let wire_regex = Regex::new(r"[a-z]+").unwrap();
    //    let operation_regex = Regex::new(r"[A-Z]+").unwrap();

    let mut map: HashMap<&str, u16> = HashMap::new();

    for line in &data {
        let mut instructions: Vec<&str> = line.split_whitespace().collect();
        // retrieving and popping receiver wire
        let reciever = instructions.pop().unwrap();
        let mut value: u16 = 0;
        // now lets remove "->" element
        instructions.pop();

        if instructions.len() == 3 {
            value = parse_3(&instructions, &map);
        } else if instructions.len() == 2 {
            value = parse_2(&instructions, &map);
        } else {
            value = parse_1(&instructions, &map);
        }
        map.insert(reciever, value);
    
    }
    println!("{}", map.get("a").unwrap());
}
