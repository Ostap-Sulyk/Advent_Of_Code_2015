fn main() {
    let input = include_str!("input")
        .lines()
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:#?}", input);
}
