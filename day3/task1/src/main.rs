struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn increase_x(&mut self) {
        self.x += 1;
    }
    fn increase_y(&mut self) {
        self.y += 1;
    }
    fn decrease_x(&mut self) {
        self.x -= 1;
    }
    fn decrease_y(&mut self) {
        self.y -= 1;
    }
    fn copy(&self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }
}

fn check_if_house_visited(houses: &Vec<Coordinate>, current_house_coordinate: &Coordinate) -> bool {
    for coordinate in houses.iter() {
        if coordinate.x == current_house_coordinate.x && coordinate.y == current_house_coordinate.y
        {
            return true;
        }
    }
    false
}
fn main() {
    let mut all_houses_visited: Vec<Coordinate> = Vec::new();
    let input_data: Vec<char> = include_str!("input").chars().collect();
    let mut current_coordinate = Coordinate { x: 0, y: 0 };

    all_houses_visited.push(current_coordinate.copy());

    for coordinate in input_data.iter() {
        if *coordinate == '>' {
            current_coordinate.increase_x();

            if !check_if_house_visited(&all_houses_visited, &current_coordinate) {
                all_houses_visited.push(current_coordinate.copy());
            }
        } else if *coordinate == '<' {
            current_coordinate.decrease_x();
            if !check_if_house_visited(&all_houses_visited, &current_coordinate) {
                all_houses_visited.push(current_coordinate.copy());
            }
        } else if *coordinate == '^' {
            current_coordinate.increase_y();

            if !check_if_house_visited(&all_houses_visited, &current_coordinate) {
                all_houses_visited.push(current_coordinate.copy())
            }
        } else if *coordinate == 'v' {
            current_coordinate.decrease_y();

            if !check_if_house_visited(&all_houses_visited, &current_coordinate) {
                all_houses_visited.push(current_coordinate.copy())
            }
        }
    }

    println!("{:?}", all_houses_visited.len());
}
