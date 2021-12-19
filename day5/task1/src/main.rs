fn has_3_vowels(line: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;
    for letter in line.chars() {
        if VOWELS.contains(&letter) {
            count += 1;
        }
        if count == 3 {
            return true;
        }
    }
    false
}
fn has_double_letter(line: &str) -> bool {
    let line: Vec<char> = line.chars().collect();
    for i in 0..line.len() - 1 {
        if line[i] == line[i + 1] {
            return true;
        }
    }

    false
}

fn has_forbidden_string(line: &str) -> bool {
    const FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];
    for forbidden_string in FORBIDDEN.iter() {
        if line.contains(forbidden_string) {
            return true;
        }
    }

    false
}
fn is_nice(line: &str) -> bool {
    has_3_vowels(&line) && has_double_letter(&line) && !has_forbidden_string(&line)
}
fn main() {
    let mut count_nice = 0;
    let strings: Vec<&str> = include_str!("input.txt").split("\n").collect();

    for line in 0..strings.len() {
        if is_nice(strings[line]) {
            count_nice += 1
        }
    }

    println!("{}", count_nice);
}
