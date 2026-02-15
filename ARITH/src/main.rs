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
                let separator = iter::repeat_n('-', line_length).collect::<String>();
                righ_to_print.insert_str(0, "+");

                println!("{:>width$}", left, width = line_length);
                println!("{:>width$}", righ_to_print, width = line_length);
                println!("{}", separator);
                print_number(&result, line_length);
                println!();
            },
            ArithmeticOperation::Subtract => {
                let result = subtract(&lhs, &rhs);
                let line_length = *[lhs.len(), rhs.len() + 1, result.len()].iter().max().unwrap();
                let separator = iter::repeat_n('-', line_length).collect::<String>();
                righ_to_print.insert_str(0, "-");

                println!("{:>width$}", left, width = line_length);
                println!("{:>width$}", righ_to_print, width = line_length);
                println!("{}", separator);
                print_number(&result, line_length);
                println!();
            },
            ArithmeticOperation::Multiply => {
                let (product, product_steps) = multiply(&lhs, &rhs);

                println!("{}\n{}", left, right);

                for product_step in product_steps {
                    print_number(&product_step, product_step.len());
                }

                print_number(&product, product.len());
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
    let offset = '0' as i32;
    let chars_to_print = iter::repeat_n(' ', num_spaces)
        .chain(number.iter().rev().map(|x| (x + offset) as u8 as char))
        .collect::<String>();

    println!("{:>width$}", chars_to_print, width = line_length);
}

// lhs is bigger and can handle the result
fn add_in_place(lhs: &mut Vec<i32>, rhs : &Vec<i32>) {
    for i in 0..(lhs.len() - 1) {
        let rhs_partial = rhs.get(i).copied().unwrap_or(0);
        let partial_sum =  lhs[i] + rhs_partial;

        lhs[i] = partial_sum % 10;
        lhs[i + 1] += partial_sum / 10;
    }

    lhs.pop_if(|x| *x == 0);
}

fn add(lhs: &Vec<i32>, rhs : &Vec<i32>) -> Vec<i32> {
    let sum_size = max(lhs.len(), rhs.len()) + 1;
    let mut sum = Vec::with_capacity(sum_size);

    sum.clone_from(lhs);

    while sum.len() < sum_size {
        sum.push(0);
    }

    add_in_place(&mut sum, rhs);

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

fn multiply_by(lhs: &Vec<i32>, rhs : &i32, shift_left: usize) -> Vec<i32> {
    let mul_idx_range = shift_left..(lhs.len() + shift_left);
    let indices_to_iterate = (0..lhs.len()).zip(mul_idx_range);

    let mul_size = lhs.len() + 1 + shift_left;
    let mut mul = vec![0; mul_size];

    for (idx, midx) in indices_to_iterate {
        let partial_mul = mul[midx] + lhs[idx] * rhs;
        mul[midx] = partial_mul % 10;
        mul[midx + 1] += partial_mul / 10;
    }

    mul.pop_if(|x| *x == 0);

    if mul.is_empty() {
        vec![0]
    } else {
        mul
    }

}

fn multiply(lhs: &Vec<i32>, rhs : &Vec<i32>) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut product_layout = Vec::with_capacity(rhs.len());
    let mut product = vec![0; lhs.len() + rhs.len()];

    for (power_of_ten_multiplier, multiplier) in rhs.iter().enumerate() {
        let partial_result = multiply_by(lhs, multiplier, power_of_ten_multiplier);
        add_in_place(&mut product, &partial_result);
        product_layout.push(partial_result);
    }

    (product, product_layout)
}

