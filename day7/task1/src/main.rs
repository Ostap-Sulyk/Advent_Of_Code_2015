#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, u16> = HashMap::new();

    let input = include_str!("input.txt")
        .lines()
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for item in input {
        if extract_data(&mut map, &item).is_some() {
            let data = extract_data(&mut map, &item).unwrap();

            if let Some(x) = map.get_mut(data.wire.as_str()){
                *x += data.signal;
            } else {
                map.insert(data.wire, data.signal);
            }
        };
    }
    println!("{:?}", map);
}
fn extract_data(map: &mut HashMap<String, u16>, item: &Vec<&str>) -> Option<Data> {
    match item.len() {
        3 => {
            let number = item[0].parse::<u16>().unwrap();
            let wire = item[2].to_string();
            Some(Data {
                signal: number,
                wire,
            })
        }
        4 => {
            let number = map.get(item[1]).unwrap();
            let wire = item[3].to_string();
            Some(Data {
                signal: !number,
                wire,
            })
        }
        5 => {
            let number_from_map = match map.get(item[0]){
                Some(x) => *x,
                None => 0,
            };
            let number_from_data = match item[2].parse::<u16>() {
                Ok(x) => x,
                Err(e) => *map.get(item[2]).unwrap(),
            };

            let number = perform_calculation(number_from_map, number_from_data, item[1]);
            let wire = item[4].to_string();

            Some(Data {
                signal: number.unwrap(),
                wire,
            })
        }
        _ => None,
    }
}
struct Data {
    signal: u16,
    wire: String,
}

fn perform_calculation(n1: u16, n2: u16, operation: &str) -> Option<u16> {
    match operation {
        "AND" => Some(n1 & n2),
        "OR" => Some(n1 | n2),
        "LSHIFT" => Some(n1 << n2),
        "RSHIFT" => Some(n1 >> n2),
        _ => None,
    }
}
