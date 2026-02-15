use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let numbers : Vec<&str> = buffer.trim_end().splitn(3,' ').collect();
        let a : i32 = numbers[0].parse().unwrap();
        let b : i32 = numbers[1].parse().unwrap();
        let c : i32 = numbers[2].parse().unwrap();

        if a == 0 && b == 0 && c == 0 {
            break;
        }

        if a*c == b*b {
            let next = c + (b - a);
            println!("AP {}", next);
        } else {
            let next = c * (b / a);
            println!("GP {}", next);
        }
    }
}
