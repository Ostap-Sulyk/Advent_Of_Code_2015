fn main() {
    let input: Vec<&str> = include_str!("input").split('\n').collect();
    let mut paper_needed = 0;

    for line in input.iter() {
        let data: Vec<usize> = line
            .split('x')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();

        let (l, w, h) = (data[0], data[1], data[2]);
        let dimension = vec![2 * l * w, 2 * w * h, 2 * h * l];
        let extra_paper = *dimension.iter().min().unwrap() / 2;

        paper_needed += extra_paper + dimension.iter().sum::<usize>();
    }
    println!("{}", paper_needed);
}
