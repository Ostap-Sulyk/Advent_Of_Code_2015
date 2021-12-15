fn main() {
    // get input from file
    // convert split string and iterate over it to and decrease or increase floor level
    let instructions: Vec<char> = include_str!("input").chars().collect();
    let mut floor = 0;
    for instruction in instructions.iter() {
        if *instruction == '(' {
            floor += 1;
        } else if *instruction == ')' {
            floor -= 1;
        }
    }
    println!("{}", floor);
}