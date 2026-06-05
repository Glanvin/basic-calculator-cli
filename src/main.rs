use std::{io};

fn main() {
    println!("Hello, Welcome to Basic Calculator!");
    println!("Here put your numbers separated by commas(e.g. 1,2,3):");
    calculate();
}

fn calculate() {
    let mut numbers = Vec::new();
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap_or_default();
    let inputs: Vec<&str> = inputs.trim().split(',').collect();
    for input in inputs {
        if let Ok(num) = input.trim().parse::<i64>() {
            numbers.push(num);
        }
    }
    println!("Your numbers are {:?}", numbers);
    println!("Choose an operation +, /, -, %");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap_or_default();
    let operation = operation.trim();
    let result = match operation {
        "+" => numbers.iter().sum::<i64>(),
        "-" => numbers.iter().fold(0, |acc, &num| acc - num),
        "*" => numbers.iter().fold(1, |acc, &num| acc * num),
        "/" => numbers.iter().fold(1, |acc, &num| acc / num),
        _ => {
            println!("Invalid operation");
            return;
        }
    };
    println!("Result: {}", result);
}
