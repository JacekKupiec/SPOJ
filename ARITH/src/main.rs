use std::cmp::max;
use std::io::{stdin, BufRead};
use std::iter::repeat;

enum ArithmeticOperation {
    Add,
    Subtract,
    Multiply
}

fn main() {
    let mut buffer = String::with_capacity(1200);
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
        let mut right = right.to_owned();

        match operation {
            ArithmeticOperation::Add => {
                let result = add(&lhs, &rhs);
                let line_length = *[lhs.len(), rhs.len() + 1, result.len()].iter().max().unwrap();
                let separator_length = max(rhs.len() + 1, result.len());
                let separator = repeat('-').take(separator_length).collect::<String>();
                right.insert_str(0, "+");

                println!("{:>width$}", left, width = line_length);
                println!("{:>width$}", right, width = line_length);
                println!("{:>width$}", separator, width = line_length);
                print_number(&result, line_length);
                print!("\n");
            },
            ArithmeticOperation::Subtract => {
                let result = subtract(&lhs, &rhs);
                let line_length = *[lhs.len(), rhs.len() + 1, result.len()].iter().max().unwrap();
                let separator_length = max(rhs.len() + 1, result.len());
                let separator = repeat('-').take(separator_length).collect::<String>();
                right.insert_str(0, "-");

                println!("{:>width$}", left, width = line_length);
                println!("{:>width$}", right, width = line_length);
                println!("{:>width$}", separator, width = line_length);
                print_number(&result, line_length);
                print!("\n");
            },
            ArithmeticOperation::Multiply => {
                right.insert_str(0, "*");

                let (product, product_steps) = multiply(&lhs, &rhs);
                let line_length = product_steps.iter()
                    .enumerate()
                    .map(|(idx, s)| s.len() + idx)
                    .chain([lhs.len(), right.len(), product.len()])
                    .max()
                    .unwrap();

                println!("{:>width$}", left, width = line_length);
                println!("{:>width$}", right, width = line_length);

                if product_steps.len() > 1 {
                    let separator_shorter = repeat('-')
                        .take(max(right.len(), product_steps[0].len()))
                        .collect::<String>();
                    println!("{:>width$}", separator_shorter, width = line_length);

                    for (idx, product_step) in product_steps.iter().enumerate() {
                        print_number(&product_step, line_length - idx);
                    }

                    let len_last_step = product_steps.last().unwrap().len() + product_steps.len() - 1;
                    let separator_longer = repeat('-')
                        .take(
                            max(len_last_step, product.len()))
                        .collect::<String>();
                    println!("{:>width$}", separator_longer, width = line_length);
                }
                else {
                    let separator_longer = repeat('-')
                        .take(max(right.len(), product.len()))
                        .collect::<String>();

                    println!("{:>width$}", separator_longer, width = line_length);
                }

                print_number(&product, line_length);
                print!("\n");
            },
        }
    }
}

fn convert_to_number(text: &str) -> Vec<i32> {
    text.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .rev()
        .collect()
}

fn print_number(number: &Vec<i32>, line_length: usize) {
    let num_spaces = line_length - number.len();
    let offset = '0' as i32;
    let chars_to_print = repeat(' ').take(num_spaces)
        .chain(number.iter().rev().map(|x| (x + offset) as u8 as char))
        .collect::<String>();

    println!("{:>width$}", chars_to_print, width = line_length);
}

// lhs is bigger and can handle the result
fn add_in_place(lhs: &mut Vec<i32>, rhs : &Vec<i32>) {
    let mut quotient = 0;

    for i in 0..lhs.len() {
        let rhs_partial = rhs.get(i).copied().unwrap_or(0);
        let partial_sum =  lhs[i] + rhs_partial + quotient;

        lhs[i] = partial_sum % 10;
        quotient = partial_sum / 10;
    }
}

fn add(lhs: &Vec<i32>, rhs : &Vec<i32>) -> Vec<i32> {
    let sum_size = max(lhs.len(), rhs.len()) + 1;
    let mut sum = Vec::with_capacity(sum_size);

    sum.clone_from(lhs);

    while sum.len() < sum_size {
        sum.push(0);
    }

    add_in_place(&mut sum, rhs);

    if let Some(e) = sum.last() {
        if *e == 0 {
            sum.pop();
        }
    }

    // sum.pop_if(|x| *x == 0);
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

    while let Some(e) = difference.last() {
        if *e == 0 {
            difference.pop();
        } else {
            break;
        }
    }

    if difference.is_empty() {
        difference.push(0);
    }

    // while let Some(_) = difference.pop_if(|x| *x == 0) { }

    difference
}

fn multiply_by(lhs: &Vec<i32>, rhs : &i32, power_of_ten_multiplier: usize) -> Vec<i32> {
    if *rhs == 0 {
        return vec![0; power_of_ten_multiplier + 1]
    }

    let mul_idx_range = power_of_ten_multiplier..(lhs.len() + power_of_ten_multiplier);
    let indices_to_iterate = (0..lhs.len()).zip(mul_idx_range);

    let mul_size = lhs.len() + 1 + power_of_ten_multiplier;
    let mut mul = vec![0; mul_size];

    for (idx, midx) in indices_to_iterate {
        let partial_mul = mul[midx] + lhs[idx] * rhs;
        mul[midx] = partial_mul % 10;
        mul[midx + 1] += partial_mul / 10;
    }

    if let Some(e) = mul.last() {
        if *e == 0 {
            mul.pop();
        }
    }

    // mul.pop_if(|x| *x == 0);
    mul
}

fn multiply(lhs: &Vec<i32>, rhs : &Vec<i32>) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut product_layout = Vec::with_capacity(rhs.len());
    let mut product = vec![0; lhs.len() + rhs.len()];

    for (power_of_ten_multiplier, multiplier) in rhs.iter().enumerate() {
        let partial_result = multiply_by(lhs, multiplier, power_of_ten_multiplier);
        add_in_place(&mut product, &partial_result);

        let partial_result_print = Vec::from(&partial_result[power_of_ten_multiplier..]);
        product_layout.push(partial_result_print);
    }

    while let Some(e) = product.last() {
        if *e == 0 {
            product.pop();
        } else {
            break;
        }
    }

    // while let Some(_) = product.pop_if(|x| *x == 0) { }

    if product.is_empty() || *product.last().unwrap() == -1 {
        product.push(0);
    }

    (product, product_layout)
}

