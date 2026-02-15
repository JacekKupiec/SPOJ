use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut txt = String::new();

    input.read_line(&mut txt).unwrap();
    let n :i32 = txt.trim().parse().unwrap();

    for _ in 1..=n {
        txt.clear();
        input.read_line(&mut txt).unwrap();

        let sum_of_digits:i32 = txt.trim().as_bytes().iter().map(|b| *b as i32 - 48).sum();

        println!("{}", sum_of_digits);
    }
}
