use md5;
fn main() {
    // let digest = md5::compute(key.as_bytes());
    let mut number = 0;
    loop {
        let digest = md5::compute((format!("{}{}", "iwrupvqb", number)).as_bytes());
        let digest = format!("{:x}", digest);
        let first_five = &digest[0..6];

        if first_five == "000000" {
            break;
        }
        number += 1;
    }
    println!("{}", number);
}
