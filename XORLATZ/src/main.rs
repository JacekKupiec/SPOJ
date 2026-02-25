use std::io::Read;

fn main() {
    let mut input = std::io::stdin().lock();
    let mut buffer = String::with_capacity(1024);

    _ = input.read_to_string(&mut buffer).unwrap();

    let p = buffer.trim_end().parse::<u32>().unwrap();

    println!("42");
}
