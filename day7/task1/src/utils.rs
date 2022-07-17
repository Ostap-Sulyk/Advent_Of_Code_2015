use std::collections::HashMap;

pub fn parse_3(instructions: &[&str], map: &HashMap<&str, u16>) -> u16 {
    let (a, b) = (
        retrieve_val_from_map(map, instructions[0]),
        retrieve_val_from_map(map, instructions[2]),
    );

    match instructions[1].to_lowercase().as_str() {
        "lshift" => a << b,
        "rshift" => a >> b,
        "and" => a & b,
        _ => a | b,
    }
}

pub fn parse_2(instructions: &[&str], map: &HashMap<&str, u16>) -> u16 {
    !retrieve_val_from_map(map, instructions[1])
}

pub fn parse_1(instructions: &[&str], map: &HashMap<&str, u16>) -> u16 {
    retrieve_val_from_map(map, instructions[0])
}

fn retrieve_val_from_map(map: &HashMap<&str, u16>, key: &str) -> u16 {
    // this is redundant and unoptimized code, i could make it more efficient
    if map.get(key).is_some() {
        return *map.get(key).unwrap();
    } else if key.parse::<u16>().is_ok() {
        return key.parse::<u16>().unwrap();
    }
    0
}
