use std::collections::HashMap;
// does all the processing
pub fn connect_wires<'a>(
    data: &'a Vec<String>,
    map: &mut HashMap<&'a str, Option<u16>>,
) {
    'outer: loop {
        for line in data {
            let mut instructions: Vec<&str> = line.split_whitespace().collect();
            // each line ends with -> (wire_name)
            // retrieving (wire_name) and popping receiver wire
            let reciever = instructions.pop().unwrap();

            // now lets remove "->" element
            instructions.pop();

            let value;
            if instructions.len() == 3 {
                value = parse_3(&instructions, map);
            } else if instructions.len() == 2 {
                value = parse_2(&instructions, map);
            } else {
                value = parse_1(&instructions, map);
            }
            map.insert(reciever, value);
        }

        if all_wires_connected(map) {
            break 'outer;
        }
    }
}
pub fn parse_3(instructions: &[&str], map: &HashMap<&str, Option<u16>>) -> Option<u16> {
    let (a, b) = (
        retrieve_val_from_map(map, instructions[0]),
        retrieve_val_from_map(map, instructions[2]),
    );

    match (a, b) {
        (Some(a), Some(b)) => match instructions[1].to_lowercase().as_str() {
            "lshift" => Some(a << b),
            "rshift" => Some(a >> b),
            "and" => Some(a & b),
            _ => Some(a | b),
        },
        (_, _) => None,
    }
}

pub fn parse_2(instructions: &[&str], map: &HashMap<&str, Option<u16>>) -> Option<u16> {
    retrieve_val_from_map(map, instructions[1]).map(|v| !v)
}

pub fn parse_1(instructions: &[&str], map: &HashMap<&str, Option<u16>>) -> Option<u16> {
    retrieve_val_from_map(map, instructions[0])
}
pub fn all_wires_connected(map: &HashMap<&str, Option<u16>>) -> bool {
    for value in map.values() {
        if value.is_none() {
            return false;
        }
    }
    true
}

fn retrieve_val_from_map(map: &HashMap<&str, Option<u16>>, key: &str) -> Option<u16> {
    // this is redundant and unoptimized code, i could make it more efficient
    if map.get(key).is_some() {
        return *map.get(key).unwrap();
    } else if key.parse::<u16>().is_ok() {
        return Some(key.parse::<u16>().unwrap());
    }
    None
}
