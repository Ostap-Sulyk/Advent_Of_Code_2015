#[derive(Debug)]
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
fn coord_exits(list: &Vec<Coordinate>, coordinate: &Coordinate) -> bool {
    for i in list.iter() {
        let coord_from_list = i.copy();
        if coord_from_list.x == coordinate.x && coord_from_list.y == coordinate.y {
            return true;
        }
    }
    false
}

fn get_unique_coords(santa: &Vec<Coordinate>, robo: &Vec<Coordinate>) -> Vec<Coordinate> {
    let mut unique_coords = Vec::new();

    for i in santa.iter() {
        if !coord_exits(&unique_coords, i) {
            unique_coords.push(i.copy());
        }
    }
    for coord in robo.iter() {
        if !coord_exits(&unique_coords, coord) {
            unique_coords.push(coord.copy())
        }
    }

    unique_coords
}

fn main() {
    // declaration
    let all_houses = include_str!("input").chars().collect::<Vec<char>>();

    let mut santa_coords = Coordinate { x: 0, y: 0 };
    let mut robo_santa_coords = Coordinate { x: 0, y: 0 };

    let mut santa_visited: Vec<Coordinate> = Vec::new();
    let mut robo_santa_visited: Vec<Coordinate> = Vec::new();

    robo_santa_visited.push(robo_santa_coords.copy());
    santa_visited.push(santa_coords.copy());
    // we store coordinates for each santa separately
    // and then look for values in santa_visited and robo_santa_visited that are not in santa_visited

    let mut iter = 0;
    for coordinate in all_houses.iter() {
        if iter == 0 || iter % 2 == 0 {
            match coordinate {
                '>' => {
                    santa_coords.increase_x();
                    santa_visited.push(santa_coords.copy());
                }
                '<' => {
                    santa_coords.decrease_x();
                    santa_visited.push(santa_coords.copy());
                }
                '^' => {
                    santa_coords.increase_y();
                    santa_visited.push(santa_coords.copy())
                }
                _ => {
                    santa_coords.decrease_y();
                    santa_visited.push(santa_coords.copy())
                }
            }
            iter += 1;
        } else {
            match coordinate {
                '>' => {
                    robo_santa_coords.increase_x();
                    robo_santa_visited.push(robo_santa_coords.copy());
                }
                '<' => {
                    robo_santa_coords.decrease_x();
                    robo_santa_visited.push(robo_santa_coords.copy());
                }
                '^' => {
                    robo_santa_coords.increase_y();
                    robo_santa_visited.push(robo_santa_coords.copy())
                }
                _ => {
                    robo_santa_coords.decrease_y();
                    robo_santa_visited.push(robo_santa_coords.copy())
                }
            }
            iter += 1;
        }
    }

    let all_houses_with_presents = get_unique_coords(&santa_visited, &robo_santa_visited);
    println!("{:?}", all_houses_with_presents.len());
}
