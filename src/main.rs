use std::io;

use calculator::Operator;
use calculator::split_input;

fn main() {
    println!("Simple Calculator"); 
    println!("Write a math problem: ");

    let mut input = String::new(); 
    io::stdin() 
    .read_line(&mut input)
    .expect("Invalid input");

    let input: String = input.split_whitespace().collect();
    let input: Vec<char> = input.chars().collect();
    let mut input = split_input(&input);
    
    let mut ope_index = 0;
    let ope_clone = input.ope.clone();
    for op in ope_clone {
        let switch: bool;
        (input.num[ope_index], ope_index, input.ope, switch) = match op {
            Operator::Multi => (input.num[ope_index] * input.num[ope_index + 1], 
                ope_index, 
                input.ope, 
                true), 
            Operator::Div => (input.num[ope_index] / input.num[ope_index + 1], 
                ope_index, 
                input.ope, 
                true), 
            _ => (input.num[ope_index], 
                ope_index + 1,
                input.ope, 
                false), 
        };
        if switch {
            input.ope.remove(ope_index);
            input.num.remove(ope_index + 1);
        }
    }

    let mut ope_index = 0;
    let ope_clone = input.ope.clone();
    for op in ope_clone {
        let switch: bool;
        (input.num[ope_index], ope_index, input.ope, switch) = match op {
            Operator::Add => (input.num[ope_index] + input.num[ope_index + 1], 
                ope_index, 
                input.ope, 
                true), 
            Operator::Sub => (input.num[ope_index] - input.num[ope_index + 1], 
                ope_index, 
                input.ope, 
                true), 
            _ => (input.num[ope_index], 
                ope_index + 1, 
                input.ope, 
                false), 
        };
        if switch {
            input.ope.remove(ope_index);
            input.num.remove(ope_index + 1);
        }
    }
    println!("Answer: {}", input.num[0]);

    let mut end = String::new();
    io::stdin() 
    .read_line(&mut end)
    .expect("Fail");
}

//fn run() {

//}
