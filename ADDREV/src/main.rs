use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let n :i32 = buffer.trim().parse().unwrap();

    (1..=n).for_each(|_| {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let (left_reversed, right_reversed) = buffer.trim().split_once(' ').unwrap();

        let left_str = left_reversed.chars().rev().collect::<String>();
        let left : i32 = left_str.trim_start_matches('0').parse().unwrap();

        let right_str = right_reversed.chars().rev().collect::<String>();
        let right : i32 = right_str.trim_start_matches('0').parse().unwrap();

        let sum = left + right;
        let sum_str = format!("{}", sum).chars().rev().collect::<String>();
        println!("{}", sum_str.trim_start_matches('0'));
    })
}
