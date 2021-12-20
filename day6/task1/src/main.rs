#[allow(dead_code, unknown_lints, unused_variables)]
fn change(lights: &mut [[bool; 999]; 999], to_do: &str, range: ((i32, i32), (i32, i32))) {}
#[derive(Debug)]

enum Action {
    OnOff(bool),
    Toggle,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Operation {
    action: Action,
    starting_coords: Vec<usize>,
    ending_coords: Vec<usize>,
}

fn extract_data(line: &str) -> Operation {
    let line = line.split(' ').collect::<Vec<_>>();

    if line.len() == 4 {
        return Operation {
            action: Action::Toggle,
            starting_coords: line[1].split(',').map(|x| x.parse().unwrap()).collect(),
            ending_coords: line[3].split(',').map(|x| x.parse().unwrap()).collect(),
        };
    } else {
        return Operation {
            action: Action::OnOff(if line[1] == "on" { true } else { false }),
            starting_coords: line[2].split(',').map(|x| x.parse().unwrap()).collect(),
            ending_coords: line[4].split(',').map(|x| x.parse().unwrap()).collect(),
        };
    }
}

fn change_lights(lights: &mut Vec<Vec<bool>>, instructions: &Vec<Operation>) {
    for operation in instructions {
        match operation.action {
            Action::OnOff(true_or_false) => {
                let starting_coords = &operation.starting_coords;
                let ending_coords = &operation.ending_coords;
                for i in starting_coords[0]..=ending_coords[0] {
                    for j in starting_coords[1]..=ending_coords[1] {
                        lights[i][j] = true_or_false;
                    }
                }
            }
            Action::Toggle => {
                let starting_coords = &operation.starting_coords;
                let ending_coords = &operation.ending_coords;
                for x in starting_coords[0]..=ending_coords[0] {
                    for y in starting_coords[1]..=ending_coords[1] {
                        lights[x][y] = !lights[x][y];
                    }
                }
            }
        }
    }
}

fn main() {
    let mut count_on = 0;
    let _instructions: Vec<&str> = include_str!("input").split("\n").collect();
    let mut _lights = vec![vec![false; 1000]; 1000];

    let mut sanitized_instructions = Vec::new();
    for line in _instructions.iter() {
        sanitized_instructions.push(extract_data(line))
    }

    change_lights(&mut _lights, &sanitized_instructions);
    for light_line in _lights {
        for light_bulb in light_line {
            if light_bulb {
                count_on += 1;
            }
        }
    }
    println!("{}", count_on);
}
