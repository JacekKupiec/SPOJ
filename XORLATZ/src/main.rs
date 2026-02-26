use std::io::Read;

fn main() {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::with_capacity(1024);

    input.read_to_string(&mut buffer).unwrap();

    let mut p = buffer.trim_end().parse::<u32>().unwrap();
    let mut counter = 0;

    while p > 1 {
        p = if p % 2 == 1 {
            (p << 1) ^ p ^ 1
        } else {
            p >> 1
        };
        counter += 1;
    }

    print!("{}", counter);
}
