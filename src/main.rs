use std::io;

use calculator::calculate;

fn main() {
    loop {
        println!("Simple Calculator");
        println!("Write a math problem: ");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e1) => eprintln!("Failed reading input: {e1}"),
        };

        match calculate::calculate(input) {
            Ok(answer) => println!("{}", answer),
            Err(e2) => eprintln!("Application Error: {e2}"),
        };
        
        let mut end = String::new();
        match io::stdin().read_line(&mut end) {
            Ok(_) => (),
            Err(e3) => eprintln!("Failed reading input: {e3}"),
        };
        
        match clearscreen::clear() {
            Ok(_) => (),
            Err(e4) => eprintln!("Failed to clear screen: {e4}"),
        };
    }
}
