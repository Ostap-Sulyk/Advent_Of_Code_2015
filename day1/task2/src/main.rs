fn main() {
    let instructions: Vec<char> = include_str!("input").chars().collect();
    let mut floor = 0;
    let mut position = 0;
    for (pos, instruction) in instructions.iter().enumerate() {
        if *instruction == '(' {
            floor += 1;
        } else if *instruction == ')' {
            floor -= 1;
        }
        if floor == -1 {
            position = pos + 1;
            break;
        }
    }
    println!("{}", position);
}
