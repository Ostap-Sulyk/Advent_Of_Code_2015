fn is_nice(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    let mut has_two_repeating_not_overlapping: bool = false;
    let mut has_two_the_same_letters_with_one_between: bool = false;

    for i in 0..chars.len() - 1 {
        let str_from_chars: String = [chars[i], chars[i + 1]].iter().collect();
        // something.matches(sub_str).count() will return number of non overlapping strings
        if s.matches(str_from_chars.as_str()).count() >= 2 {
            has_two_repeating_not_overlapping = true;
            break;
        }
    }
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            has_two_the_same_letters_with_one_between = true;
            break;
        }
    }

    has_two_the_same_letters_with_one_between && has_two_repeating_not_overlapping
}
fn main() {
    let mut count_nice = 0;
    let strings: Vec<&str> = include_str!("input.txt").split("\n").collect();

    for string in strings.iter() {
        if is_nice(string) {
            count_nice += 1;
        }
    }
    println!("{}", count_nice);
}
