use std::cmp::max;
use std::io::{stdin, BufRead};
use std::iter;

enum ArithmeticOperation {
    Add,
    Subtract,
    Multiply
}

fn main() {
    let mut buffer = String::with_capacity(600);
    let stdin = stdin();
    let mut input = stdin.lock();

    println!("{number:>500}", number="1235469");

    input.read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let line = buffer.trim();

        let (left, right, operation) = if let Some((left, right)) = line.split_once('+') {
            (left, right, ArithmeticOperation::Add)
        } else if let Some((left, right)) = line.split_once('-') {
            (left, right, ArithmeticOperation::Subtract)
        } else if let Some((left, right)) = line.split_once('*') {
            (left, right, ArithmeticOperation::Multiply)
        } else {
            panic!("unrecognized line: {}", line);
        };

        let lhs = convert_to_number(left);
        let rhs = convert_to_number(right);
        let mut righ_to_print = right.to_owned();

        match operation {
            ArithmeticOperation::Add => {
                let result = add(&lhs, &rhs);
                let line_length = *[lhs.len(), rhs.len() + 1, result.len()].iter().max().unwrap();
                let separator = iter::repeat_n(' ', line_length).collect::<String>();
                righ_to_print.insert_str(0, "+");

                println!("{:>width$}", left, width = line_length);
                println!("{:>width$}", right, width = line_length);
                println!("{}", separator);
                print_number(&result, line_length);
            },
            ArithmeticOperation::Subtract => {
                let result = subtract(&lhs, &rhs);
                let line_length = *[lhs.len(), rhs.len() + 1, result.len()].iter().max().unwrap();
                let separator = iter::repeat_n(' ', line_length).collect::<String>();
                righ_to_print.insert_str(0, "-");

                println!("{:>width$}", left, width = line_length);
                println!("{:>width$}", right, width = line_length);
                println!("{}", separator);
                print_number(&result, line_length);
            },
            ArithmeticOperation::Multiply => {
                let result = multiply(&lhs, &rhs);
                print_number(&lhs);
                print_number(&rhs);
                print_number(&result);
            },
        }
    }
}

fn convert_to_number(text: &str) -> Vec<i32>
{
    text.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .rev()
        .collect()
}

fn print_number(number: &Vec<i32>, line_length: usize) {
    let num_spaces = line_length - number.len();
    let chars_to_print = iter::repeat_n(' ', num_spaces)
        .chain(number.iter().rev().map(|x| *x as u8 as char))
        .collect::<String>();

    println!("{:>width$}", chars_to_print, width = line_length);
}

fn add(lhs: &Vec<i32>, rhs : &Vec<i32>) -> Vec<i32> {
    let sum_size = max(lhs.len(), rhs.len()) + 1;
    let mut sum = vec![0; sum_size];

    for i in 0..(sum.len() - 1) {
        let lhs_partial = lhs.get(i).copied().unwrap_or(0);
        let rhs_partial = rhs.get(i).copied().unwrap_or(0);
        let partial_sum =  sum[i] + lhs_partial + rhs_partial;

        sum[i] = partial_sum % 10;
        sum[i + 1] = partial_sum / 10;
    }

    sum.pop_if(|x| *x == 0);
    sum
}

// lhs always bigger than rhs
fn subtract(lhs: &Vec<i32>, rhs : &Vec<i32>) -> Vec<i32> {
    let mut difference = Vec::with_capacity(lhs.len());
    difference.clone_from(lhs);

    for i in 0..difference.len() {
        let rhs_partial = rhs.get(i).copied().unwrap_or(0);

        difference[i] -= rhs_partial;

        if difference[i] < 0 {
            difference[i] += 10;
            difference[i + 1] -= 1;
        }
    }

    while let Some(_) = difference.pop_if(|x| *x == 0) { }

    difference
}

fn multiply(lhs: &Vec<i32>, rhs : &Vec<i32>) -> Vec<i32> {
    let product = Vec::new();

    product
}

#[cfg(test)]
mod test {

    #[test]
    fn my_first_test() {
        let mut s = String::from("abcdef");
        let insert_pos: usize = 1;  // Byte index where to insert
        let to_insert: &str = "kej";
        s.insert_str(insert_pos, to_insert);
        assert_eq!(s, "akejef");
    }
}

