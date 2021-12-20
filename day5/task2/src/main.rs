fn two_double_letters_not_overlapping(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        let str_from_chars: String = [chars[i], chars[i + 1]].iter().collect();
        // something.matches(sub_str).count() will return number of non overlapping strings
        if s.matches(str_from_chars.as_str()).count() >= 2 {
            return true;
        }
    }
    false
}
// fn one_letter_with_one_between(s: &str) -> bool {
//     unimplemented!()
// }
fn main() {
    let strings: Vec<&str> = include_str!("test_input.txt").split("\n").collect();

    println!("{}", two_double_letters_not_overlapping(strings[0]));

    // println!("{:?}", strings);
}
