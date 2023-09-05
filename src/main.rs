use std::io;

fn main() {  // add loop
    print!("\x1B[2J\x1b[1;1H");

    println!("Simple Calculator");
    println!("Write a math problem: ");

    let mut input = String::new();
    io::stdin() 
    .read_line(&mut input)
    .expect("Invalid input");

    let input: String = input.split_whitespace().collect();
    let input: Vec<char> = input.chars().collect();
    let mut input = calculator::split_input(&input);
    
    let mut input = calculator::multi_div_opes(&mut input);

    let input = calculator::add_sub_opes(&mut input);

    println!("Answer: {}", input.num[0]);

    let mut end: String = String::new();

    io::stdin() 
    .read_line(&mut end)
    .expect("Fail");
}
