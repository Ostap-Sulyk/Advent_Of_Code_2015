#[allow(dead_code, unknown_lints, unused_variables)]
fn change(lights: &mut [[bool; 999]; 999], to_do: &str, range: ((i32, i32), (i32, i32))) {}
struct Operation {
    action: i32,
    starting_coords: Vec<usize>,
    ending_coords: Vec<usize>,
}

fn extract_data(line: &str) -> Operation {
    let line = line.split(' ').collect::<Vec<_>>();

    if line.len() == 4 {
        return Operation {
            action: 2,
            starting_coords: line[1].split(',').map(|x| x.parse().unwrap()).collect(),
            ending_coords: line[3].split(',').map(|x| x.parse().unwrap()).collect(),
        };
    } else {
        return Operation {
            action: if line[1] == "on" { 1 } else { -1 },
            starting_coords: line[2].split(',').map(|x| x.parse().unwrap()).collect(),
            ending_coords: line[4].split(',').map(|x| x.parse().unwrap()).collect(),
        };
    }
}

fn change_lights(lights: &mut Vec<Vec<i32>>, instructions: &Vec<Operation>) {
    for operation in instructions {
        let starting_coords = &operation.starting_coords;
        let ending_coords = &operation.ending_coords;
        for i in starting_coords[0]..=ending_coords[0] {
            for j in starting_coords[1]..=ending_coords[1] {
                lights[i][j] += operation.action;
                if lights[i][j] < 0 {
                    lights[i][j] = 0
                }
            }
        }
    }
}

fn main() {
    let mut total_brightness = 0;
    let _instructions: Vec<&str> = include_str!("input").split("\n").collect();
    let mut _lights = vec![vec![0; 1000]; 1000];

    let mut sanitized_instructions = Vec::new();
    for line in _instructions.iter() {
        sanitized_instructions.push(extract_data(line))
    }

    change_lights(&mut _lights, &sanitized_instructions);

    for light_line in &_lights {
        for light_bulb in light_line {
            total_brightness += light_bulb;
        }
    }
    println!("{}", total_brightness);
}
