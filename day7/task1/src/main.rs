#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i16> = HashMap::new();

    let input = include_str!("test_input.txt")
        .lines()
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for item in input {
        if item.len() == 3 {
            insert_or_update(&mut map, &item);
        } else if item.len() == 4 {
            insert_or_redirect_with_not(&mut map, &item);
        } else if item.len() == 5 {
            // insert_or_redirect_with_two_wires(&mut map, &item);
        }
    }

    println!("{:#?}", map);
}

fn insert_or_update(map: &mut HashMap<String, i16>, item: &Vec<&str>) {
    if let Some(value) = map.get_mut(item[2]) {
        *value += item[0].parse::<i16>().unwrap();
        return;
    }
    map.insert(item[2].to_string(), item[0].parse::<i16>().unwrap());
}

fn insert_or_redirect_with_not(map: &mut HashMap<String, i16>, item: &Vec<&str>) {
    if let Some(value) = map.get_mut(item[3]) {
        *value += !(item[1].parse::<i16>().unwrap());
        return;
    }
    map.insert(item[3].to_string(), !(*map.get(item[1]).unwrap()));
}
// fn insert_or_redirect_with_two_wires(map: &mut HashMap<String, i16>, item: &Vec<&str>) {
//     let a = map.get(item[0]).unwrap().clone();
//     let b = map.get(item[2]).unwrap().clone();
//     
//     match item[1] {
//         "AND" => {
//             if let Some(value) = map.get_mut(item[4]) {
//                 *value += (a) & (b);
//             } else {
//                 map.insert(item[4].to_string(), (a) & (b));
//             }
//         }
//         "OR" => {
//             if let Some(value) = map.get_mut(item[4]) {
//                 *value += (a) | (b);
//             } else {
//                 map.insert(item[4].to_string(), (a) | (b));
//             }
//         }
//         "LSHIFT" => {
//             let b = item[2].parse::<i16>().unwrap();
//             if let Some(value) = map.get_mut(item[4]) {
//                 *value += a << b;
//             } else {
//                 map.insert(item[4].to_string(), (a) << (b));
//             }
//         }
//         _ => {
//             let b = item[2].parse::<i16>().unwrap();
//             if let Some(value) = map.get_mut(item[4]) {
//                 *value += a >> b;
//             } else {
//                 map.insert(item[4].to_string(), (a) >> (b));
//             }
//         }
//     }
// }
